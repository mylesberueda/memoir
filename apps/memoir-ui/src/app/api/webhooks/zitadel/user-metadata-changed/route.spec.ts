import { createHmac } from 'node:crypto';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

// Mock the session module before importing the route
vi.mock('@lib/session', () => ({
	forceRefreshAllUserSessions: vi.fn(),
}));

import { forceRefreshAllUserSessions } from '@lib/session';
import { POST } from './route';

const mockForceRefresh = vi.mocked(forceRefreshAllUserSessions);

// Test constants
const TEST_SECRET = 'test-webhook-secret-for-signing';
const TEST_USER_ID = '318571912345678901';

// Helper to create a valid Zitadel signature
function createSignature(body: string, secret: string, timestamp?: number): string {
	const ts = timestamp ?? Math.floor(Date.now() / 1000);
	const signedPayload = `${ts}.${body}`;
	const signature = createHmac('sha256', secret).update(signedPayload).digest('hex');
	return `t=${ts},v1=${signature}`;
}

// Helper to create a Zitadel event payload
function createEventPayload(overrides: Record<string, unknown> = {}): string {
	const event = {
		aggregateID: TEST_USER_ID,
		aggregateType: 'user',
		resourceOwner: '318571900000000001',
		instanceID: '318571800000000001',
		version: 'v1',
		sequence: 42,
		event_type: 'user.metadata.set',
		created_at: new Date().toISOString(),
		userID: TEST_USER_ID,
		event_payload: {
			key: 'tier',
			value: 'cHJv', // base64 "pro"
		},
		...overrides,
	};
	return JSON.stringify(event);
}

// Helper to create a mock NextRequest
function createMockRequest(body: string, signatureHeader?: string): Request {
	const headers = new Headers({
		'content-type': 'application/json',
	});
	if (signatureHeader) {
		headers.set('zitadel-signature', signatureHeader);
	}

	return new Request('http://localhost:3000/api/webhooks/zitadel/user-metadata-changed', {
		method: 'POST',
		headers,
		body,
	});
}

describe('Zitadel user-metadata-changed webhook', () => {
	const originalEnv = process.env;

	beforeEach(() => {
		vi.clearAllMocks();
		process.env = {
			...originalEnv,
			ZITADEL_WEBHOOK_SECRET: TEST_SECRET,
		};
		mockForceRefresh.mockResolvedValue(2);
	});

	afterEach(() => {
		process.env = originalEnv;
	});

	describe('signature verification', () => {
		it('should reject requests without signature header', async () => {
			const body = createEventPayload();
			const request = createMockRequest(body);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(400);
			expect(json.error).toBe('Missing signature header');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should reject requests with invalid signature', async () => {
			const body = createEventPayload();
			const request = createMockRequest(body, 't=123456789,v1=invalidsignature');

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(400);
			expect(json.error).toBe('Invalid signature');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should reject requests with expired timestamp', async () => {
			const body = createEventPayload();
			const expiredTimestamp = Math.floor(Date.now() / 1000) - 600; // 10 minutes ago
			const signature = createSignature(body, TEST_SECRET, expiredTimestamp);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(400);
			expect(json.error).toBe('Invalid signature');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should accept requests with valid signature', async () => {
			const body = createEventPayload();
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);

			expect(response.status).toBe(200);
			expect(mockForceRefresh).toHaveBeenCalledWith(TEST_USER_ID);
		});
	});

	describe('metadata key filtering', () => {
		it('should refresh sessions when tier metadata changes', async () => {
			const body = createEventPayload({
				event_payload: { key: 'tier', value: 'cHJv' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(200);
			expect(json.message).toBe('Sessions marked for refresh');
			expect(json.sessionsUpdated).toBe(2);
			expect(mockForceRefresh).toHaveBeenCalledWith(TEST_USER_ID);
		});

		it('should refresh sessions when billing_cycle_start metadata changes', async () => {
			const body = createEventPayload({
				event_payload: { key: 'billing_cycle_start', value: 'MTcwNTMxMjAwMA==' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);

			expect(response.status).toBe(200);
			expect(mockForceRefresh).toHaveBeenCalledWith(TEST_USER_ID);
		});

		it('should refresh sessions when stripe_customer_id metadata changes', async () => {
			const body = createEventPayload({
				event_payload: { key: 'stripe_customer_id', value: 'Y3VzX3h4eA==' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);

			expect(response.status).toBe(200);
			expect(mockForceRefresh).toHaveBeenCalledWith(TEST_USER_ID);
		});

		it('should refresh sessions when stripe_subscription_id metadata changes', async () => {
			const body = createEventPayload({
				event_payload: { key: 'stripe_subscription_id', value: 'c3ViX3h4eA==' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);

			expect(response.status).toBe(200);
			expect(mockForceRefresh).toHaveBeenCalledWith(TEST_USER_ID);
		});

		it('should NOT refresh sessions for arbitrary metadata keys', async () => {
			const body = createEventPayload({
				event_payload: { key: 'user_preferences', value: 'eyJ0aGVtZSI6ImRhcmsifQ==' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(200);
			expect(json.message).toBe('Event acknowledged but not processed');
			expect(json.reason).toBe('metadata key does not trigger refresh');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should NOT refresh sessions for onboarding_completed metadata', async () => {
			const body = createEventPayload({
				event_payload: { key: 'onboarding_completed', value: 'dHJ1ZQ==' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(200);
			expect(json.reason).toBe('metadata key does not trigger refresh');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should NOT refresh sessions when event_payload is missing key', async () => {
			const body = createEventPayload({
				event_payload: { value: 'c29tZXZhbHVl' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(200);
			expect(json.reason).toBe('metadata key does not trigger refresh');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});
	});

	describe('event validation', () => {
		it('should ignore non-user aggregate types', async () => {
			const body = createEventPayload({
				aggregateType: 'organization',
				event_payload: { key: 'tier', value: 'cHJv' },
			});
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(200);
			expect(json.message).toBe('Event acknowledged but not processed');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should handle malformed JSON gracefully', async () => {
			const body = 'not valid json';
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(400);
			expect(json.error).toBe('Invalid JSON payload');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should handle missing aggregateID', async () => {
			const event = {
				aggregateType: 'user',
				event_type: 'user.metadata.set',
				event_payload: { key: 'tier', value: 'cHJv' },
			};
			const body = JSON.stringify(event);
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(200);
			expect(json.message).toBe('Event acknowledged but not processed');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});
	});

	describe('error handling', () => {
		it('should return 500 when ZITADEL_WEBHOOK_SECRET is not set', async () => {
			delete process.env.ZITADEL_WEBHOOK_SECRET;

			const body = createEventPayload();
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(500);
			expect(json.error).toBe('Server configuration error');
			expect(mockForceRefresh).not.toHaveBeenCalled();
		});

		it('should return 500 when forceRefreshAllUserSessions throws', async () => {
			mockForceRefresh.mockRejectedValue(new Error('Redis connection failed'));

			const body = createEventPayload();
			const signature = createSignature(body, TEST_SECRET);
			const request = createMockRequest(body, signature);

			const response = await POST(request as never);
			const json = await response.json();

			expect(response.status).toBe(500);
			expect(json.error).toBe('Internal server error');
		});
	});
});
