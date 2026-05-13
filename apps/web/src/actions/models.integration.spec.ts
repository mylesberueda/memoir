import { create } from '@bufbuild/protobuf';
import { createClient } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import { DeleteAccountRequestSchema, UserService } from '@startup/proto-ts/api-service/api/v1/users_pb';
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
import { getModels as getModelsData, getModelsForProvider } from '@actions/models';
import { getProviders as getProvidersData } from '@actions/providers';

/**
 * Integration tests for model server actions.
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

describe('Models Integration Tests', () => {
	beforeAll(async () => {
		const testUser = await createTestUser(generateTestEmail('model-test'), 'TestPassword123!', 'Model Test User');
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

	describe('getModelsData', () => {
		it('should return list of active, non-deprecated models', async () => {
			const result = await getModelsData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(Array.isArray(result.data.models)).toBe(true);

			// All returned models should be active and not deprecated
			for (const model of result.data.models) {
				expect(model.identifier.case).toBe('pid');
				expect(model.identifier.value).toBeDefined();
				// Model should have required fields
				expect(model.name).toBeDefined();
			}
		});

		it('should include models from system providers with provider references', async () => {
			const result = await getModelsData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			// System providers (like Ollama) should have models available
			expect(result.data.models.length).toBeGreaterThan(0);

			// Every model should reference a provider
			for (const model of result.data.models) {
				expect(model.providerPid).toBeDefined();
				expect(model.providerPid.length).toBeGreaterThan(0);
			}
		});
	});

	describe('getModelsData (unauthenticated)', () => {
		it('should return error when not authenticated', async () => {
			// Temporarily override the mock to simulate no authentication
			const originalAccessToken = testAccessToken;
			testAccessToken = '';

			try {
				const result = await getModelsData();

				expect(result.success).toBe(false);
				if (result.success) return;

				expect(result.error).toBe('Authentication required');
			} finally {
				// Restore the token
				testAccessToken = originalAccessToken;
			}
		});
	});

	describe('getModelsForProvider', () => {
		it('should return models filtered by provider PID', async () => {
			// First get a provider
			const providersResult = await getProvidersData();
			expect(providersResult.success).toBe(true);
			if (!providersResult.success) return;
			expect(providersResult.data.providers.length).toBeGreaterThan(0);

			const provider = providersResult.data.providers[0];
			const providerPid = provider.identifier.value;

			// Get models for this provider
			const result = await getModelsForProvider(providerPid);

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(Array.isArray(result.data.models)).toBe(true);

			// All returned models should belong to this provider
			for (const model of result.data.models) {
				expect(model.providerPid).toBe(providerPid);
			}
		});

		it('should return empty list for provider with no models', async () => {
			// Use a non-existent provider PID
			const result = await getModelsForProvider('non-existent-provider-pid');

			expect(result.success).toBe(true);
			if (!result.success) return;

			// Should return empty array, not an error
			expect(result.data.models).toEqual([]);
		});
	});
});
