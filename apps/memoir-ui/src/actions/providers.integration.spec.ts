import { create } from '@bufbuild/protobuf';
import { createClient } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import { DeleteAccountRequestSchema, UserService } from '@polypixel/memoir-sdk/api-service/api/v1/users_pb';
import { createTestUser, deleteTestUser, generateTestEmail } from '@test-utils';
import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest';

// Test tokens - populated in beforeAll
let testAccessToken: string;
let testIdToken: string;
let testUserId: string;

// Mock next/headers
vi.mock('next/headers', () => ({
	cookies: () => ({
		get: () => undefined,
		set: vi.fn(),
		delete: vi.fn(),
	}),
}));

// Mock next/cache
vi.mock('next/cache', () => ({
	revalidatePath: vi.fn(),
}));

// Mock auth actions to return test tokens
vi.mock('@actions/auth', () => ({
	getAccessToken: () => Promise.resolve(testAccessToken),
	getIdToken: () => Promise.resolve(testIdToken),
	getCurrentUser: () =>
		Promise.resolve({
			id: testUserId,
			email: 'test@example.com',
			name: 'Test User',
		}),
}));

// Import actions — vi.mock() is hoisted above imports by Vitest
import { getProviders as getProvidersData } from '@actions/providers';

/**
 * Integration tests for provider server actions.
 *
 * These tests call the real server actions with mocked Next.js context.
 * They require:
 * - RIG service running at RIG_SERVICE_URL
 * - API service running at API_SERVICE_URL
 * - Zitadel running at ZITADEL_URL
 *
 * Run with: pnpm nx run web:test:integration
 */

// Helper to create a direct gRPC client for cleanup operations
function createUserClient(accessToken: string, idToken?: string) {
	const apiServiceUrl = process.env.API_SERVICE_URL;
	if (!apiServiceUrl) throw new Error('API_SERVICE_URL not set');

	const transport = createGrpcTransport({
		baseUrl: apiServiceUrl,
		interceptors: [
			(next) => async (req) => {
				req.header.set('Authorization', `Bearer ${accessToken}`);
				if (idToken) {
					req.header.set('x-id-token', idToken);
				}
				return next(req);
			},
		],
	});
	return createClient(UserService, transport);
}

describe('Providers Integration Tests', () => {
	beforeAll(async () => {
		const testUser = await createTestUser(generateTestEmail('provider-test'), 'TestPassword123!', 'Provider Test User');
		testUserId = testUser.userId;
		testAccessToken = testUser.accessToken;
		testIdToken = testUser.idToken;
	}, 30000);

	afterAll(async () => {
		// Cleanup test user from api-service database
		if (testAccessToken) {
			try {
				const userClient = createUserClient(testAccessToken, testIdToken);
				await userClient.deleteAccount(create(DeleteAccountRequestSchema, {}));
			} catch {
				// Ignore cleanup errors
			}
		}

		// Cleanup test user from Zitadel
		if (testUserId) {
			await deleteTestUser(testUserId);
		}
	}, 30000);

	describe('getProvidersData', () => {
		it('should return list of active providers for authenticated user', async () => {
			const result = await getProvidersData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(Array.isArray(result.data.providers)).toBe(true);
		});

		it('should include system providers accessible to all users', async () => {
			const result = await getProvidersData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			// System providers should always be present (e.g., Ollama for local inference)
			expect(result.data.providers.length).toBeGreaterThan(0);

			// Each provider should have a PID identifier
			for (const provider of result.data.providers) {
				expect(provider.identifier.case).toBe('pid');
				expect(provider.identifier.value).toBeDefined();
			}
		});

		it('should not expose credentials in response', async () => {
			const result = await getProvidersData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			// Verify no provider has exposed API keys or secrets
			for (const provider of result.data.providers) {
				// The Provider proto should not include credential fields in the response
				// Check that common credential patterns are not present
				// Use a replacer to handle BigInt values from protobuf
				const providerJson = JSON.stringify(provider, (_, v) => (typeof v === 'bigint' ? v.toString() : v));

				// Should not contain API key patterns
				expect(providerJson).not.toMatch(/api[_-]?key/i);
				expect(providerJson).not.toMatch(/secret/i);
				expect(providerJson).not.toMatch(/sk-[a-zA-Z0-9]+/); // OpenAI key pattern
				expect(providerJson).not.toMatch(/Bearer\s+[a-zA-Z0-9]+/i);
			}
		});
	});

	describe('getProvidersData (unauthenticated)', () => {
		it('should return error when not authenticated', async () => {
			// Temporarily override the mock to simulate no authentication
			const originalAccessToken = testAccessToken;
			testAccessToken = '';

			try {
				const result = await getProvidersData();

				// The action returns success: false with error when client is null
				expect(result.success).toBe(false);
				if (result.success) return;

				expect(result.error).toBe('Authentication required');
			} finally {
				// Restore the token
				testAccessToken = originalAccessToken;
			}
		});
	});
});
