/**
 * Integration tests for billing/tier upgrade session continuity.
 *
 * These tests verify that when a user's tier changes:
 * 1. The session is NOT deleted (user stays logged in)
 * 2. Token refresh happens transparently
 * 3. The new JWT contains updated tier information
 *
 * Requirements:
 * - Redis running at REDIS_URL
 * - Zitadel running at ZITADEL_URL
 * - Valid service account credentials
 *
 * Run with: pnpm nx run web:test:integration
 */

import { createHmac, randomUUID } from 'node:crypto';
import { getServiceAccountToken, getZitadelUrl, refreshTokens } from '@lib/zitadel';
import { createTestUser, deleteTestUser, generateTestEmail } from '@test-utils';
import Redis from 'ioredis';
import { decodeJwt } from 'jose';
import { afterAll, beforeAll, beforeEach, describe, expect, it } from 'vitest';

// Redis session constants (must match session.ts)
const SESSION_PREFIX = 'web:session:';
const USER_SESSIONS_PREFIX = 'web:user:';
const SESSION_TTL_SECONDS = 24 * 60 * 60;

let redis: Redis;
let webhookSecret: string;
let baseUrl: string;

// Test user - created once per suite
let testUserId: string;
let _testEmail: string;
let testAccessToken: string;
let testRefreshToken: string;
let testIdToken: string;

// Track created sessions for cleanup
const createdSessionIds: string[] = [];

// Helper to create a valid Zitadel webhook signature
function createWebhookSignature(body: string, secret: string, timestamp?: number): string {
	const ts = timestamp ?? Math.floor(Date.now() / 1000);
	const signedPayload = `${ts}.${body}`;
	const signature = createHmac('sha256', secret).update(signedPayload).digest('hex');
	return `t=${ts},v1=${signature}`;
}

// Helper to create Zitadel metadata change event payload
function createMetadataEventPayload(userId: string, metadataKey: string): string {
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
			value: Buffer.from('plus').toString('base64'),
		},
	});
}

// Helper to create a session in Redis
async function createTestSession(
	userId: string,
	accessToken: string,
	refreshToken: string,
	idToken: string,
): Promise<string> {
	const sessionId = randomUUID();
	const sessionKey = `${SESSION_PREFIX}${sessionId}`;
	const userSessionsKey = `${USER_SESSIONS_PREFIX}${userId}:sessions`;

	const sessionData = {
		accessToken,
		refreshToken,
		idToken,
		userId,
		expiresAt: Math.floor(Date.now() / 1000) + 3600,
		forceRefresh: false,
	};

	await redis.set(sessionKey, JSON.stringify(sessionData), 'EX', SESSION_TTL_SECONDS);
	await redis.sadd(userSessionsKey, sessionId);
	await redis.expire(userSessionsKey, SESSION_TTL_SECONDS);

	createdSessionIds.push(sessionId);

	return sessionId;
}

// Helper to get session from Redis
async function getSession(sessionId: string): Promise<Record<string, unknown> | null> {
	const data = await redis.get(`${SESSION_PREFIX}${sessionId}`);
	return data ? JSON.parse(data) : null;
}

// Helper to update user tier metadata in Zitadel
async function setUserTierMetadata(userId: string, tier: string): Promise<void> {
	const zitadelUrl = await getZitadelUrl();
	const saToken = await getServiceAccountToken();

	const metadata = [
		{
			key: 'tier',
			value: Buffer.from(tier).toString('base64'),
		},
	];

	const response = await fetch(`${zitadelUrl}/v2/users/${userId}/metadata`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${saToken}`,
		},
		body: JSON.stringify({ metadata }),
	});

	if (!response.ok) {
		const error = await response.text();
		throw new Error(`Failed to set user metadata: ${response.status} ${error}`);
	}
}

// Helper to decode and log JWT contents
function decodeAndLogJwt(token: string, label: string): Record<string, unknown> {
	const decoded = decodeJwt(token);
	console.log(`\n========== ${label} ==========`);
	console.log(JSON.stringify(decoded, null, 2));
	console.log('==========================================\n');
	return decoded as Record<string, unknown>;
}

describe('Billing/Tier Upgrade Session Continuity', () => {
	beforeAll(async () => {
		// Setup Redis
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

		// Create test user with real Zitadel tokens
		const testUser = await createTestUser(generateTestEmail('billing-test'), 'TestPassword123!', 'Billing Test User');
		testUserId = testUser.userId;
		_testEmail = testUser.email;
		testAccessToken = testUser.accessToken;
		testRefreshToken = testUser.refreshToken;
		testIdToken = testUser.idToken;

		console.log('\n🔍 INITIAL TOKEN INSPECTION');
		decodeAndLogJwt(testIdToken, 'INITIAL ID TOKEN');
		decodeAndLogJwt(testAccessToken, 'INITIAL ACCESS TOKEN');
	}, 60000);

	afterAll(async () => {
		if (!redis) return;

		// Cleanup sessions
		for (const sessionId of createdSessionIds) {
			await redis.del(`${SESSION_PREFIX}${sessionId}`);
		}
		await redis.del(`${USER_SESSIONS_PREFIX}${testUserId}:sessions`);

		// Cleanup test user
		if (testUserId) {
			await deleteTestUser(testUserId);
		}

		await redis.quit();
	});

	beforeEach(() => {
		createdSessionIds.length = 0;
	});

	describe('JWT tier propagation', () => {
		it('should include updated tier in JWT after metadata change and token refresh', async () => {
			// Set tier metadata in Zitadel
			await setUserTierMetadata(testUserId, 'plus');

			// Wait a moment for Zitadel to process
			await new Promise((resolve) => setTimeout(resolve, 500));

			// Refresh tokens to get new JWT with updated metadata
			const newTokens = await refreshTokens(testRefreshToken);

			// Verify ID token contains metadata
			expect(newTokens.id_token).toBeDefined();
			const idClaims = decodeJwt(newTokens.id_token as string);
			const metadataClaim = 'urn:zitadel:iam:user:metadata';

			expect(idClaims[metadataClaim]).toBeDefined();

			const metadata = idClaims[metadataClaim] as Record<string, string>;
			expect(metadata.tier).toBeDefined();

			// Zitadel stores metadata values as base64
			const tierValue = Buffer.from(metadata.tier, 'base64').toString('utf-8');
			expect(tierValue).toBe('plus');

			// Verify access token also contains metadata
			const accessClaims = decodeJwt(newTokens.access_token);
			expect(accessClaims[metadataClaim]).toBeDefined();

			const accessMetadata = accessClaims[metadataClaim] as Record<string, string>;
			const accessTierValue = Buffer.from(accessMetadata.tier, 'base64').toString('utf-8');
			expect(accessTierValue).toBe('plus');
		});
	});

	describe('session continuity on tier change', () => {
		it('should mark session for refresh when tier webhook fires', async () => {
			// Create a session in Redis with real tokens
			const sessionId = await createTestSession(testUserId, testAccessToken, testRefreshToken, testIdToken);

			// Verify initial state
			const initialSession = await getSession(sessionId);
			expect(initialSession?.forceRefresh).toBe(false);

			// Trigger the tier webhook
			const body = createMetadataEventPayload(testUserId, 'tier');
			const signature = createWebhookSignature(body, webhookSecret);

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

			// Verify session was marked for refresh (NOT deleted)
			const updatedSession = await getSession(sessionId);
			expect(updatedSession).not.toBeNull();
			expect(updatedSession?.forceRefresh).toBe(true);
			expect(updatedSession?.userId).toBe(testUserId);

			console.log('\n✅ Session marked for refresh, user stays logged in');
		});
	});
});
