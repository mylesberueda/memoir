import { randomUUID } from 'node:crypto';
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

// Organization context — set in beforeAll after user provisioning
let testOrganizationPid: string | undefined;
let secondOrgPid: string | undefined;

// Mock next/headers
vi.mock('next/headers', () => ({
	cookies: () => ({
		get: (name: string) => {
			if (name === 'x-organization-id' && testOrganizationPid) {
				return { value: testOrganizationPid };
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
import {
	createAgent as createAgentAction,
	getAgents as getAgentsData,
	getUserAssistant as getUserAssistantAction,
	updateAgent as updateAgentAction,
} from '@actions/agents';
import { getModels as getModelsData } from '@actions/models';
import { createOrg, deleteOrg, getOrganizations } from '@actions/organizations';

/**
 * Integration tests for agent server actions.
 *
 * These tests call the real server actions with mocked Next.js context.
 * They require:
 * - RIG service running at RIG_SERVICE_URL
 * - API service running at API_SERVICE_URL
 * - Zitadel running at ZITADEL_URL
 *
 * Run with: pnpm nx run web:test:integration
 */

// Track created agents for cleanup
const createdAgentPids: string[] = [];

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

describe('Agents Integration Tests', () => {
	beforeAll(async () => {
		const testUser = await createTestUser(generateTestEmail('agent-test'), 'TestPassword123!', 'Agent Test User');
		testUserId = testUser.userId;
		testAccessToken = testUser.accessToken;
		testIdToken = testUser.idToken;

		// Fetch the user's personal org (auto-created on first auth via api-service middleware)
		const orgsResult = await getOrganizations();
		if (orgsResult.success && orgsResult.data.organizations.length > 0) {
			testOrganizationPid = orgsResult.data.organizations[0].pid;
		}

		// Create a second org for cross-org isolation tests
		const secondOrgResult = await createOrg('Agent Test Org B', `agent-test-org-b-${randomUUID().slice(0, 8)}`);
		if (secondOrgResult.success) {
			secondOrgPid = secondOrgResult.data.organization?.pid;
		}
	}, 30000);

	afterAll(async () => {
		// Cleanup created agents by deactivating them
		for (const pid of createdAgentPids) {
			try {
				await updateAgentAction({ pid, isActive: false }, []);
			} catch {
				// Ignore cleanup errors
			}
		}

		// Cleanup second org
		if (secondOrgPid) {
			try {
				testOrganizationPid = secondOrgPid;
				await deleteOrg(secondOrgPid, []);
			} catch {
				// Ignore cleanup errors
			}
			testOrganizationPid = undefined;
		}

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

	describe('getAgentsData', () => {
		it('should return list of active agents for authenticated user', async () => {
			const result = await getAgentsData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(Array.isArray(result.data.agents)).toBe(true);
		});

		it('should return no user-created agents for a fresh user', async () => {
			const result = await getAgentsData();

			expect(result.success).toBe(true);
			if (!result.success) return;

			// Fresh user has no manually created agents (assistant is excluded from listAgents by default)
			const userCreatedAgents = result.data.agents.filter((a) => a.name !== 'Assistant');
			expect(userCreatedAgents.length).toBe(0);
		});
	});

	describe('createAgentAction', () => {
		it('should create agent with provided name, model, and system prompt', async () => {
			// First get an available model
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;
			expect(modelsResult.data.models.length).toBeGreaterThan(0);

			const model = modelsResult.data.models[0];
			const uniqueName = `Test Agent ${randomUUID().slice(0, 8)}`;

			const result = await createAgentAction(
				{
					name: uniqueName,
					modelPid: model.identifier.value,
					systemPrompt: 'You are a helpful test assistant.',
				},
				[],
			);

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(result.data.name).toBe(uniqueName);
			expect(result.data.systemPrompt).toBe('You are a helpful test assistant.');
			expect(result.data.identifier.case).toBe('pid');

			createdAgentPids.push(result.data.identifier.value);
		});

		it('should return error when model does not exist', async () => {
			const result = await createAgentAction(
				{
					name: 'Invalid Model Agent',
					modelPid: 'non-existent-model-pid',
					systemPrompt: 'Test prompt',
				},
				[],
			);

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();
		});
	});

	describe('updateAgentAction', () => {
		it('should update only agent name when only name is provided', async () => {
			// First create an agent
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const originalName = `Original Agent ${randomUUID().slice(0, 8)}`;
			const originalPrompt = 'You are a helpful test assistant.';
			const originalTemp = 50;

			const createResult = await createAgentAction(
				{
					name: originalName,
					modelPid: model.identifier.value,
					systemPrompt: originalPrompt,
					temperature: originalTemp,
				},
				[],
			);

			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			// Update ONLY the name - other fields should be preserved
			const newName = `Updated Agent ${randomUUID().slice(0, 8)}`;
			const updateResult = await updateAgentAction({ pid: agentPid, name: newName }, []);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;

			// Name should be updated
			expect(updateResult.data.name).toBe(newName);
			// Other fields should be preserved
			expect(updateResult.data.systemPrompt).toBe(originalPrompt);
			expect(updateResult.data.temperature).toBe(originalTemp);
			expect(updateResult.data.model?.pid).toBe(model.identifier.value);
		});

		it('should update only temperature when only temperature is provided', async () => {
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const originalName = `Temp Agent ${randomUUID().slice(0, 8)}`;
			const originalPrompt = 'Original prompt.';

			const createResult = await createAgentAction(
				{
					name: originalName,
					modelPid: model.identifier.value,
					systemPrompt: originalPrompt,
					temperature: 50,
				},
				[],
			);

			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			// Update ONLY the temperature
			const updateResult = await updateAgentAction({ pid: agentPid, temperature: 80 }, []);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;

			// Temperature should be updated
			expect(updateResult.data.temperature).toBe(80);
			// Other fields should be preserved
			expect(updateResult.data.name).toBe(originalName);
			expect(updateResult.data.systemPrompt).toBe(originalPrompt);
		});

		it('should update only model when only modelPid is provided', async () => {
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			// Need at least 2 models to test model switching
			if (modelsResult.data.models.length < 2) {
				console.log('Skipping model update test - need at least 2 models');
				return;
			}

			const [model1, model2] = modelsResult.data.models;
			const originalName = `Model Switch Agent ${randomUUID().slice(0, 8)}`;
			const originalPrompt = 'Original system prompt.';
			const originalTemp = 60;

			const createResult = await createAgentAction(
				{
					name: originalName,
					modelPid: model1.identifier.value,
					systemPrompt: originalPrompt,
					temperature: originalTemp,
				},
				[],
			);

			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			// Update ONLY the model
			const updateResult = await updateAgentAction({ pid: agentPid, modelPid: model2.identifier.value }, []);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;

			// Model should be updated
			expect(updateResult.data.model?.pid).toBe(model2.identifier.value);
			// Other fields should be preserved
			expect(updateResult.data.name).toBe(originalName);
			expect(updateResult.data.systemPrompt).toBe(originalPrompt);
			expect(updateResult.data.temperature).toBe(originalTemp);
		});

		it('should update only system prompt when only systemPrompt is provided', async () => {
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const originalName = `Prompt Agent ${randomUUID().slice(0, 8)}`;
			const originalTemp = 70;

			const createResult = await createAgentAction(
				{
					name: originalName,
					modelPid: model.identifier.value,
					systemPrompt: 'Original prompt.',
					temperature: originalTemp,
				},
				[],
			);

			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			// Update ONLY the system prompt
			const newPrompt = 'Updated system prompt with new instructions.';
			const updateResult = await updateAgentAction({ pid: agentPid, systemPrompt: newPrompt }, []);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;

			// System prompt should be updated
			expect(updateResult.data.systemPrompt).toBe(newPrompt);
			// Other fields should be preserved
			expect(updateResult.data.name).toBe(originalName);
			expect(updateResult.data.temperature).toBe(originalTemp);
		});

		it('should update multiple fields at once', async () => {
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const originalName = `Multi Update Agent ${randomUUID().slice(0, 8)}`;

			const createResult = await createAgentAction(
				{
					name: originalName,
					modelPid: model.identifier.value,
					systemPrompt: 'Original prompt.',
					temperature: 50,
				},
				[],
			);

			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			// Update name and temperature together
			const newName = `Renamed Agent ${randomUUID().slice(0, 8)}`;
			const updateResult = await updateAgentAction({ pid: agentPid, name: newName, temperature: 90 }, []);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;

			// Both fields should be updated
			expect(updateResult.data.name).toBe(newName);
			expect(updateResult.data.temperature).toBe(90);
			// Untouched fields should be preserved
			expect(updateResult.data.systemPrompt).toBe('Original prompt.');
		});

		it('should deactivate agent when only isActive is set to false', async () => {
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const originalName = `Deactivate Agent ${randomUUID().slice(0, 8)}`;

			const createResult = await createAgentAction(
				{
					name: originalName,
					modelPid: model.identifier.value,
					systemPrompt: 'Test prompt.',
				},
				[],
			);

			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const agentPid = createResult.data.identifier.value;
			createdAgentPids.push(agentPid);

			// Agent should start as active
			expect(createResult.data.isActive).toBe(true);

			// Deactivate with only isActive field
			const updateResult = await updateAgentAction({ pid: agentPid, isActive: false }, []);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;

			// Should be deactivated
			expect(updateResult.data.isActive).toBe(false);
			// Other fields should be preserved
			expect(updateResult.data.name).toBe(originalName);
		});

		it('should return error for non-existent agent', async () => {
			const result = await updateAgentAction({ pid: 'non-existent-agent-pid', name: 'New Name' }, []);

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();
		});
	});

	describe('getUserAssistantAction', () => {
		it('should return or create default assistant for user', async () => {
			const result = await getUserAssistantAction();

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(result.data.agent).toBeDefined();
			expect(result.data.agent.identifier.case).toBe('pid');
			expect(result.data.agent.name).toBeDefined();

			// Track for cleanup
			createdAgentPids.push(result.data.agent.identifier.value);
		});

		it('should return existing assistant on subsequent calls', async () => {
			// First call - creates or gets existing
			const firstResult = await getUserAssistantAction();
			expect(firstResult.success).toBe(true);
			if (!firstResult.success) return;

			const firstPid = firstResult.data.agent.identifier.value;

			// Second call - should return same assistant
			const secondResult = await getUserAssistantAction();
			expect(secondResult.success).toBe(true);
			if (!secondResult.success) return;

			expect(secondResult.data.agent.identifier.value).toBe(firstPid);
		});
	});

	describe('org isolation', () => {
		it('should not list agents from a different org', async () => {
			// Create agent in the personal org
			const modelsResult = await getModelsData();
			expect(modelsResult.success).toBe(true);
			if (!modelsResult.success) return;

			const model = modelsResult.data.models[0];
			const agentName = `Org A Agent ${randomUUID().slice(0, 8)}`;
			const createResult = await createAgentAction(
				{ name: agentName, modelPid: model.identifier.value, systemPrompt: 'Test' },
				[],
			);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;
			createdAgentPids.push(createResult.data.identifier.value);

			// Switch to second org and list agents — should not see the agent from personal org
			testOrganizationPid = secondOrgPid;
			const listResult = await getAgentsData();
			testOrganizationPid = secondOrgPid; // keep in second org for assertion

			expect(listResult.success).toBe(true);
			if (!listResult.success) {
				testOrganizationPid = secondOrgPid; // restore before returning
				return;
			}

			const found = listResult.data.agents.find((a) => a.name === agentName);
			expect(found).toBeUndefined();

			// Restore to personal org for subsequent tests
			const orgsResult = await getOrganizations();
			if (orgsResult.success && orgsResult.data.organizations.length > 0) {
				testOrganizationPid = orgsResult.data.organizations[0].pid;
			}
		});

		it('should reject agent creation without org context', async () => {
			const savedOrg = testOrganizationPid;
			testOrganizationPid = undefined;

			const result = await createAgentAction(
				{
					name: `No Org Agent ${randomUUID().slice(0, 8)}`,
					modelPid: 'any-model',
					systemPrompt: 'Test',
				},
				[],
			);

			testOrganizationPid = savedOrg;

			expect(result.success).toBe(false);
			if (result.success) return;
			expect(result.error).toContain('Organization');
		});

		it('should reject getUserAssistant without org context', async () => {
			const savedOrg = testOrganizationPid;
			testOrganizationPid = undefined;

			const result = await getUserAssistantAction();

			testOrganizationPid = savedOrg;

			expect(result.success).toBe(false);
			if (result.success) return;
			expect(result.error).toContain('Organization');
		});
	});
});
