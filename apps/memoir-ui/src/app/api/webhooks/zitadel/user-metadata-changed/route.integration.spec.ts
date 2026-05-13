import { createHmac, randomUUID } from 'node:crypto';
import Redis from 'ioredis';
import { afterAll, beforeAll, beforeEach, describe, expect, it } from 'vitest';

/**
 * Integration tests for Zitadel user-metadata-changed webhook.
 *
 * These tests require:
 * - Redis running at REDIS_URL
 * - ZITADEL_WEBHOOK_SECRET set
 *
 * Run with: pnpm nx run web:test:integration
 */

const SESSION_PREFIX = 'web:session:';
const USER_SESSIONS_PREFIX = 'web:user:';
const SESSION_TTL_SECONDS = 24 * 60 * 60;

let redis: Redis;
let webhookSecret: string;
let baseUrl: string;

// Track created sessions for cleanup
const createdSessionIds: string[] = [];
const createdUserIds: string[] = [];

// Helper to create a valid Zitadel signature
function createSignature(body: string, secret: string, timestamp?: number): string {
	const ts = timestamp ?? Math.floor(Date.now() / 1000);
	const signedPayload = `${ts}.${body}`;
	const signature = createHmac('sha256', secret).update(signedPayload).digest('hex');
	return `t=${ts},v1=${signature}`;
}

// Helper to create a Zitadel event payload
function createEventPayload(userId: string, metadataKey: string): string {
	return JSON.stringify({
		aggregateID: userId,
		aggregateType: 'user',
		resourceOwner: '318571900000000001',
		instanceID: '318571800000000001',
		version: 'v1',
		sequence: 42,
		event_type: 'user.metadata.set',
		created_at: new Date().toISOString(),
		userID: userId,
		event_payload: {
			key: metadataKey,
			value: 'dGVzdA==', // base64 "test"
		},
	});
}

// Helper to create a session directly in Redis
async function createTestSession(userId: string): Promise<string> {
	const sessionId = randomUUID();
	const sessionKey = `${SESSION_PREFIX}${sessionId}`;
	const userSessionsKey = `${USER_SESSIONS_PREFIX}${userId}:sessions`;

	const sessionData = {
		accessToken: 'test-access-token',
		refreshToken: 'test-refresh-token',
		idToken: 'test-id-token',
		userId,
		expiresAt: Math.floor(Date.now() / 1000) + 3600,
		forceRefresh: false,
	};

	await redis.set(sessionKey, JSON.stringify(sessionData), 'EX', SESSION_TTL_SECONDS);
	await redis.sadd(userSessionsKey, sessionId);
	await redis.expire(userSessionsKey, SESSION_TTL_SECONDS);

	createdSessionIds.push(sessionId);
	if (!createdUserIds.includes(userId)) {
		createdUserIds.push(userId);
	}

	return sessionId;
}

// Helper to get session from Redis
async function getSession(sessionId: string): Promise<Record<string, unknown> | null> {
	const data = await redis.get(`${SESSION_PREFIX}${sessionId}`);
	return data ? JSON.parse(data) : null;
}

describe('Zitadel webhook integration tests', () => {
	beforeAll(() => {
		const redisUrl = process.env.REDIS_URL;
		if (!redisUrl) {
			throw new Error('REDIS_URL environment variable is not set');
		}

		webhookSecret = process.env.ZITADEL_WEBHOOK_SECRET || '';
		if (!webhookSecret) {
			throw new Error('ZITADEL_WEBHOOK_SECRET environment variable is not set');
		}

		baseUrl = process.env.NEXT_PUBLIC_APP_URL || 'http://localhost:3000';

		redis = new Redis(redisUrl, {
			maxRetriesPerRequest: 3,
			lazyConnect: true,
		});
	});

	afterAll(async () => {
		if (!redis) return;

		// Cleanup created sessions
		for (const sessionId of createdSessionIds) {
			await redis.del(`${SESSION_PREFIX}${sessionId}`);
		}

		// Cleanup user session sets
		for (const userId of createdUserIds) {
			await redis.del(`${USER_SESSIONS_PREFIX}${userId}:sessions`);
		}

		await redis.quit();
	});

	beforeEach(() => {
		// Clear tracking arrays for each test
		createdSessionIds.length = 0;
		createdUserIds.length = 0;
	});

	describe('tier metadata changes', () => {
		it('should mark session for refresh when tier metadata changes', async () => {
			const userId = `test-user-${randomUUID()}`;
			const sessionId = await createTestSession(userId);

			// Verify initial state
			const initialSession = await getSession(sessionId);
			expect(initialSession?.forceRefresh).toBe(false);

			// Call webhook
			const body = createEventPayload(userId, 'tier');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);
			const json = await response.json();
			expect(json.sessionsUpdated).toBe(1);

			// Verify session was marked for refresh
			const updatedSession = await getSession(sessionId);
			expect(updatedSession?.forceRefresh).toBe(true);
		});

		it('should mark multiple sessions for refresh', async () => {
			const userId = `test-user-${randomUUID()}`;

			// Create 3 sessions for the same user
			const sessionIds = await Promise.all([
				createTestSession(userId),
				createTestSession(userId),
				createTestSession(userId),
			]);

			// Call webhook
			const body = createEventPayload(userId, 'tier');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);
			const json = await response.json();
			expect(json.sessionsUpdated).toBe(3);

			// Verify all sessions were marked for refresh
			for (const sessionId of sessionIds) {
				const session = await getSession(sessionId);
				expect(session?.forceRefresh).toBe(true);
			}
		});

		it('should handle user with no sessions', async () => {
			const userId = `test-user-${randomUUID()}`;
			// Don't create any sessions

			const body = createEventPayload(userId, 'tier');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);
			const json = await response.json();
			expect(json.sessionsUpdated).toBe(0);
		});
	});

	describe('non-triggering metadata changes', () => {
		it('should NOT mark session for refresh when arbitrary metadata changes', async () => {
			const userId = `test-user-${randomUUID()}`;
			const sessionId = await createTestSession(userId);

			// Call webhook with non-triggering key
			const body = createEventPayload(userId, 'user_preferences');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);
			const json = await response.json();
			expect(json.reason).toBe('metadata key does not trigger refresh');

			// Verify session was NOT marked for refresh
			const session = await getSession(sessionId);
			expect(session?.forceRefresh).toBe(false);
		});

		it('should NOT mark session for refresh when onboarding metadata changes', async () => {
			const userId = `test-user-${randomUUID()}`;
			const sessionId = await createTestSession(userId);

			const body = createEventPayload(userId, 'onboarding_completed');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);

			// Verify session was NOT marked for refresh
			const session = await getSession(sessionId);
			expect(session?.forceRefresh).toBe(false);
		});
	});

	describe('other triggering keys', () => {
		it('should mark session for refresh when billing_cycle_start changes', async () => {
			const userId = `test-user-${randomUUID()}`;
			const sessionId = await createTestSession(userId);

			const body = createEventPayload(userId, 'billing_cycle_start');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);

			const session = await getSession(sessionId);
			expect(session?.forceRefresh).toBe(true);
		});

		it('should mark session for refresh when stripe_customer_id changes', async () => {
			const userId = `test-user-${randomUUID()}`;
			const sessionId = await createTestSession(userId);

			const body = createEventPayload(userId, 'stripe_customer_id');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);

			const session = await getSession(sessionId);
			expect(session?.forceRefresh).toBe(true);
		});

		it('should mark session for refresh when stripe_subscription_id changes', async () => {
			const userId = `test-user-${randomUUID()}`;
			const sessionId = await createTestSession(userId);

			const body = createEventPayload(userId, 'stripe_subscription_id');
			const signature = createSignature(body, webhookSecret);

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': signature,
				},
				body,
			});

			expect(response.status).toBe(200);

			const session = await getSession(sessionId);
			expect(session?.forceRefresh).toBe(true);
		});
	});

	describe('signature verification', () => {
		it('should reject requests with invalid signature', async () => {
			const userId = `test-user-${randomUUID()}`;
			await createTestSession(userId);

			const body = createEventPayload(userId, 'tier');
			const invalidSignature = 't=1234567890,v1=invalidsignature';

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'zitadel-signature': invalidSignature,
				},
				body,
			});

			expect(response.status).toBe(400);
			const json = await response.json();
			expect(json.error).toBe('Invalid signature');
		});

		it('should reject requests without signature header', async () => {
			const userId = `test-user-${randomUUID()}`;
			const body = createEventPayload(userId, 'tier');

			const response = await fetch(`${baseUrl}/api/webhooks/zitadel/user-metadata-changed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body,
			});

			expect(response.status).toBe(400);
			const json = await response.json();
			expect(json.error).toBe('Missing signature header');
		});
	});
});
