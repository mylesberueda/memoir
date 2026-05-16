import { createHmac, timingSafeEqual } from 'node:crypto';
import { authLogger } from '@lib/logger';
import { forceRefreshAllUserSessions } from '@lib/session';
import { type NextRequest, NextResponse } from 'next/server';

const SIGNATURE_HEADER = 'zitadel-signature';
const TIMESTAMP_TOLERANCE_SECONDS = 300; // 5 minutes

// Only these metadata keys trigger a session refresh.
// Other metadata changes (e.g., user preferences) don't affect JWT claims.
const REFRESH_TRIGGERING_KEYS = ['tier', 'billing_cycle_start', 'stripe_customer_id', 'stripe_subscription_id'];

interface ZitadelEvent {
	aggregateID: string; // userId
	aggregateType: string; // "user"
	resourceOwner: string; // orgId
	instanceID: string;
	version: string;
	sequence: number;
	event_type: string; // "user.metadata.set"
	created_at: string; // ISO 8601
	userID: string; // same as aggregateID for user events
	event_payload: {
		key: string;
		value: string; // base64 encoded
	};
}

/**
 * Verify Zitadel webhook signature.
 * Header format: "t=<unix-timestamp>,v1=<hex-signature>"
 * Signature: HMAC-SHA256(timestamp + "." + body, signingKey)
 */
function verifyZitadelSignature(body: string, signatureHeader: string, secret: string): boolean {
	// Parse header: "t=1234567890,v1=abc123..."
	const parts: Record<string, string> = {};
	for (const part of signatureHeader.split(',')) {
		const eqIndex = part.indexOf('=');
		if (eqIndex > 0) {
			const key = part.slice(0, eqIndex);
			const value = part.slice(eqIndex + 1);
			parts[key] = value;
		}
	}

	const timestamp = parts.t;
	const signature = parts.v1;

	if (!timestamp || !signature) {
		return false;
	}

	// Check timestamp tolerance (prevent replay attacks)
	const now = Math.floor(Date.now() / 1000);
	const timestampNum = parseInt(timestamp, 10);
	if (Number.isNaN(timestampNum) || Math.abs(now - timestampNum) > TIMESTAMP_TOLERANCE_SECONDS) {
		return false;
	}

	// Compute expected signature: HMAC-SHA256(timestamp + "." + body, secret)
	const signedPayload = `${timestamp}.${body}`;
	const expectedSignature = createHmac('sha256', secret).update(signedPayload).digest('hex');

	// Timing-safe comparison
	const sigBuffer = Buffer.from(signature, 'hex');
	const expectedBuffer = Buffer.from(expectedSignature, 'hex');

	if (sigBuffer.length !== expectedBuffer.length) {
		return false;
	}

	return timingSafeEqual(sigBuffer, expectedBuffer);
}

/**
 * Zitadel webhook endpoint for user metadata changes.
 * Called by Zitadel Actions when user.metadata.set event fires.
 * Marks all user sessions for force refresh so they get new JWT with updated claims.
 */
export async function POST(request: NextRequest) {
	const webhookSecret = process.env.ZITADEL_WEBHOOK_SECRET;
	if (!webhookSecret) {
		authLogger.error('ZITADEL_WEBHOOK_SECRET environment variable is not set');
		return NextResponse.json({ error: 'Server configuration error' }, { status: 500 });
	}

	// Get signature header
	const signatureHeader = request.headers.get(SIGNATURE_HEADER);
	if (!signatureHeader) {
		return NextResponse.json({ error: 'Missing signature header' }, { status: 400 });
	}

	// Read body as text for signature verification
	let body: string;
	try {
		body = await request.text();
	} catch {
		return NextResponse.json({ error: 'Invalid request body' }, { status: 400 });
	}

	// Verify signature
	if (!verifyZitadelSignature(body, signatureHeader, webhookSecret)) {
		authLogger.warn('Zitadel webhook signature verification failed');
		return NextResponse.json({ error: 'Invalid signature' }, { status: 400 });
	}

	// Parse event payload
	let event: ZitadelEvent;
	try {
		event = JSON.parse(body) as ZitadelEvent;
	} catch {
		return NextResponse.json({ error: 'Invalid JSON payload' }, { status: 400 });
	}

	// Validate required fields
	if (!event.aggregateID || event.aggregateType !== 'user') {
		authLogger.warn('Zitadel webhook received non-user event', { aggregateType: event.aggregateType });
		// Return 200 to acknowledge - we just don't process non-user events
		return NextResponse.json({ message: 'Event acknowledged but not processed' }, { status: 200 });
	}

	const userId = event.aggregateID;
	const metadataKey = event.event_payload?.key;

	// Only refresh sessions for metadata keys that affect JWT claims
	if (!metadataKey || !REFRESH_TRIGGERING_KEYS.includes(metadataKey)) {
		authLogger.info('Zitadel webhook ignored non-triggering metadata key', {
			userId,
			eventType: event.event_type,
			metadataKey,
		});
		return NextResponse.json({
			message: 'Event acknowledged but not processed',
			reason: 'metadata key does not trigger refresh',
		});
	}

	try {
		const updatedCount = await forceRefreshAllUserSessions(userId);

		authLogger.info('Zitadel webhook processed', {
			userId,
			eventType: event.event_type,
			metadataKey,
			sessionsMarked: updatedCount,
		});

		return NextResponse.json({
			message: 'Sessions marked for refresh',
			sessionsUpdated: updatedCount,
		});
	} catch (error) {
		authLogger.error('Zitadel webhook processing failed', { error, userId });
		return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
	}
}
