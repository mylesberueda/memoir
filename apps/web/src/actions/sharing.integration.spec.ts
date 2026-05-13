import { randomUUID } from 'node:crypto';
import { create } from '@bufbuild/protobuf';
import { createClient } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import {
	DeleteAccountRequestSchema,
	MeRequestSchema,
	UserService,
} from '@polypixel/proto-ts/api-service/api/v1/users_pb';
import { createTestUser, deleteTestUser, generateTestEmail } from '@test-utils';
import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest';

// Test tokens for User A (owner) - populated in beforeAll
let userAAccessToken: string;
let userAIdToken: string;
let userAId: string;
let userAEmail: string;

// Test tokens for User B (recipient) - populated in beforeAll
let userBAccessToken: string;
let userBIdToken: string;
let userBId: string;
let userBEmail: string;
const userBDisplayName = 'Share Recipient User';

// Organization context
let personalOrgPid: string | undefined;

// Active token context — swap between users
let activeAccessToken: string;
let activeIdToken: string;

// Mock next/headers
vi.mock('next/headers', () => ({
	cookies: () => ({
		get: (name: string) => {
			if (name === 'x-organization-id' && personalOrgPid) {
				return { value: personalOrgPid };
			}
			return undefined;
		},
		set: vi.fn(),
		delete: vi.fn(),
	}),
}));

// Mock next/cache
vi.mock('next/cache', () => ({
	revalidatePath: vi.fn(),
}));

// Mock auth actions — uses the active token context
vi.mock('@actions/auth', () => ({
	getAccessToken: () => Promise.resolve(activeAccessToken),
	getIdToken: () => Promise.resolve(activeIdToken),
	getCurrentUser: () =>
		Promise.resolve({
			id: userAId,
			email: userAEmail,
			name: 'Share Owner User',
		}),
}));

// Import actions — vi.mock() is hoisted above imports by Vitest
import { createAgent as createAgentAction, updateAgent as updateAgentAction } from '@actions/agents';
import { getModels as getModelsData } from '@actions/models';
import { addOrgMember, getOrganizations } from '@actions/organizations';
import { listAgentShares, shareAgent, unshareAgent } from '@actions/sharing';

// Helper to create a direct gRPC client for cleanup
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

/**
 * Integration tests for sharing server actions.
 *
 * Tests the full cross-service flow:
 *   Frontend action → rig-service (ShareAgent/ListAgentShares)
 *   → rig-service calls api-service (GetUsers) for display enrichment
 *   → Frontend receives email + displayName on share records
 *
 * Requires all services running:
 *   - API service at API_SERVICE_URL
 *   - RIG service at RIG_SERVICE_URL
 *   - Zitadel at ZITADEL_URL
 */

const createdAgentPids: string[] = [];

// Shared agent pid — created once, used across CRUD tests
let sharedAgentPid: string;

describe('Sharing Integration Tests', () => {
	beforeAll(async () => {
		// Create User A (agent owner)
		const userA = await createTestUser(generateTestEmail('share-owner'), 'TestPassword123!', 'Share Owner User');
		userAId = userA.userId;
		userAEmail = userA.email;
		userAAccessToken = userA.accessToken;
		userAIdToken = userA.idToken;

		// Set User A as active for initial setup
		activeAccessToken = userAAccessToken;
		activeIdToken = userAIdToken;

		// Get User A's personal org
		const orgsResult = await getOrganizations();
		if (orgsResult.success && orgsResult.data.organizations.length > 0) {
			personalOrgPid = orgsResult.data.organizations[0].pid;
		}
		expect(personalOrgPid).toBeDefined();

		// Create User B (share recipient)
		const userB = await createTestUser(generateTestEmail('share-recipient'), 'TestPassword123!', userBDisplayName);
		userBId = userB.userId;
		userBEmail = userB.email;
		userBAccessToken = userB.accessToken;
		userBIdToken = userB.idToken;

		// Provision User B in api-service by calling Me (no org context needed)
		const userBClient = createUserClient(userBAccessToken, userBIdToken);
		await userBClient.me(create(MeRequestSchema, {}));

		if (!personalOrgPid) {
			fail('Missing personal org pid');
		}

		// Add User B as a member of User A's personal org
		await addOrgMember(personalOrgPid, userBId, 'member');

		// Create a shared agent for CRUD tests
		const modelsResult = await getModelsData();
		expect(modelsResult.success).toBe(true);
		if (!modelsResult.success) throw new Error('Failed to get models');
		expect(modelsResult.data.models.length).toBeGreaterThan(0);

		const model = modelsResult.data.models[0];
		const createResult = await createAgentAction(
			{
				name: `CRUD Share Agent ${randomUUID().slice(0, 8)}`,
				modelPid: model.identifier.value,
				systemPrompt: 'Agent for sharing CRUD tests.',
			},
			[],
		);
		expect(createResult.success).toBe(true);
		if (!createResult.success) throw new Error('Failed to create agent');
		sharedAgentPid = createResult.data.identifier.value;
		createdAgentPids.push(sharedAgentPid);
	}, 60000);

	afterAll(async () => {
		activeAccessToken = userAAccessToken;
		activeIdToken = userAIdToken;

		// Deactivate created agents
		for (const pid of createdAgentPids) {
			try {
				await updateAgentAction({ pid, isActive: false }, []);
			} catch {
				// Ignore cleanup errors
			}
		}

		// Cleanup User A from api-service + Zitadel
		if (userAAccessToken) {
			try {
				const client = createUserClient(userAAccessToken, userAIdToken);
				await client.deleteAccount(create(DeleteAccountRequestSchema, {}));
			} catch {
				// Ignore cleanup errors
			}
		}
		if (userAId) {
			await deleteTestUser(userAId);
		}

		// Cleanup User B from api-service + Zitadel
		if (userBAccessToken) {
			try {
				const client = createUserClient(userBAccessToken, userBIdToken);
				await client.deleteAccount(create(DeleteAccountRequestSchema, {}));
			} catch {
				// Ignore cleanup errors
			}
		}
		if (userBId) {
			await deleteTestUser(userBId);
		}
	}, 30000);

	describe('shareAgent (create)', () => {
		it('should share an agent with another org member', async () => {
			const result = await shareAgent(sharedAgentPid, userBId, 1); // READ
			if (!result.success) {
				console.error('shareAgent failed:', result.error);
			}
			expect(result.success).toBe(true);

			// Verify via list
			const listResult = await listAgentShares(sharedAgentPid);
			expect(listResult.success).toBe(true);
			if (!listResult.success) return;

			expect(listResult.data.shares.length).toBe(1);
			expect(listResult.data.shares[0].userId).toBe(userBId);
			expect(listResult.data.shares[0].permissions).toBe(1);
			expect(listResult.data.shares[0].sharedBy).toBe(userAId);
		});

		it('should reject sharing with a user not in the org', async () => {
			const result = await shareAgent(sharedAgentPid, 'nonexistent-user-id', 1);
			expect(result.success).toBe(false);
		});
	});

	describe('shareAgent (update — permission upsert)', () => {
		it('should update permissions when resharing with same user', async () => {
			// First ensure a share exists with READ (1)
			await shareAgent(sharedAgentPid, userBId, 1);

			// Reshare with RWX (7) — should upsert, not duplicate
			const result = await shareAgent(sharedAgentPid, userBId, 7);
			expect(result.success).toBe(true);

			// Verify: one share record with updated permissions
			const listResult = await listAgentShares(sharedAgentPid);
			expect(listResult.success).toBe(true);
			if (!listResult.success) return;

			expect(listResult.data.shares.length).toBe(1);
			expect(listResult.data.shares[0].permissions).toBe(7);
		});
	});

	describe('listAgentShares (read)', () => {
		it('should return enriched user data on share records', async () => {
			// Ensure share exists
			await shareAgent(sharedAgentPid, userBId, 5); // read + execute

			const listResult = await listAgentShares(sharedAgentPid);
			expect(listResult.success).toBe(true);
			if (!listResult.success) return;

			expect(listResult.data.shares.length).toBe(1);

			const share = listResult.data.shares[0];
			expect(share.userId).toBe(userBId);
			expect(share.permissions).toBe(5);
			expect(share.email).toBe(userBEmail);
			expect(share.displayName).toBe(userBDisplayName);
			expect(share.sharedBy).toBe(userAId);
			expect(share.createdAt).toBeDefined();
		});

		it('should return empty list when no shares exist', async () => {
			// Create a fresh agent with no shares
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const createResult = await createAgentAction(
				{
					name: `No Shares Agent ${randomUUID().slice(0, 8)}`,
					modelPid: model.identifier.value,
					systemPrompt: 'Test.',
				},
				[],
			);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			const listResult = await listAgentShares(agentPid);
			expect(listResult.success).toBe(true);
			if (!listResult.success) return;

			expect(listResult.data.shares.length).toBe(0);
		});
	});

	describe('unshareAgent (delete)', () => {
		it('should remove a share', async () => {
			// Ensure share exists
			await shareAgent(sharedAgentPid, userBId, 7);

			// Verify it exists
			const beforeResult = await listAgentShares(sharedAgentPid);
			expect(beforeResult.success).toBe(true);
			if (!beforeResult.success) return;
			expect(beforeResult.data.shares.length).toBe(1);

			// Unshare
			const unshareResult = await unshareAgent(sharedAgentPid, userBId);
			expect(unshareResult.success).toBe(true);

			// Verify it's gone
			const afterResult = await listAgentShares(sharedAgentPid);
			expect(afterResult.success).toBe(true);
			if (!afterResult.success) return;
			expect(afterResult.data.shares.length).toBe(0);
		});

		it('should succeed even when no share exists (idempotent)', async () => {
			// Ensure no share exists
			await unshareAgent(sharedAgentPid, userBId);

			// Unshare again — should not error
			const result = await unshareAgent(sharedAgentPid, userBId);
			expect(result.success).toBe(true);
		});
	});
});
