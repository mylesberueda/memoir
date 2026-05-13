import { randomUUID } from 'node:crypto';
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

// Second test user for isolation tests
let testAccessToken2: string;
let testIdToken2: string;
let testUserId2: string;

// Organization context per user — set in beforeAll after user provisioning
let testOrganizationPid1: string | undefined;
let testOrganizationPid2: string | undefined;

// Mock next/headers
vi.mock('next/headers', () => ({
	cookies: () => ({
		get: (name: string) => {
			if (name === 'x-organization-id') {
				const orgPid = activeUserContext === 1 ? testOrganizationPid1 : testOrganizationPid2;
				if (orgPid) return { value: orgPid };
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

// Track which user context to use
let activeUserContext = 1;

// Mock auth actions to return test tokens based on active context
vi.mock('@actions/auth', () => ({
	getAccessToken: () => Promise.resolve(activeUserContext === 1 ? testAccessToken : testAccessToken2),
	getIdToken: () => Promise.resolve(activeUserContext === 1 ? testIdToken : testIdToken2),
	getCurrentUser: () =>
		Promise.resolve({
			id: activeUserContext === 1 ? testUserId : testUserId2,
			email: 'test@example.com',
			name: 'Test User',
		}),
}));

/** Switch to user 1 context */
function switchToUser1() {
	activeUserContext = 1;
}

/** Switch to user 2 context */
function switchToUser2() {
	activeUserContext = 2;
}

import { createAgent as createAgentAction, updateAgent as updateAgentAction } from '@actions/agents';
// Import actions — vi.mock() is hoisted above imports by Vitest
import {
	createConversation,
	deleteConversation,
	fetchConversationMessages,
	fetchConversations,
	sendInferenceMessage,
} from '@actions/infer';
import { getModels as getModelsData } from '@actions/models';
import { getOrganizations } from '@actions/organizations';

/**
 * Integration tests for inference and conversation server actions.
 *
 * These tests call the real server actions with mocked Next.js context.
 * They require:
 * - RIG service running at RIG_SERVICE_URL
 * - API service running at API_SERVICE_URL
 * - Zitadel running at ZITADEL_URL
 * - Ollama running (for inference tests)
 *
 * Run with: pnpm nx run web:test:integration
 */

// Track created resources for cleanup
const createdConversationPids: string[] = [];
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

describe('Inference Integration Tests', () => {
	// Store agent PID for tests
	let testAgentPid: string;

	beforeAll(async () => {
		// Create first test user
		const testUser1 = await createTestUser(generateTestEmail('infer-test-1'), 'TestPassword123!', 'Infer Test User 1');
		testUserId = testUser1.userId;
		testAccessToken = testUser1.accessToken;
		testIdToken = testUser1.idToken;

		// Create second test user for isolation tests
		const testUser2 = await createTestUser(generateTestEmail('infer-test-2'), 'TestPassword123!', 'Infer Test User 2');
		testUserId2 = testUser2.userId;
		testAccessToken2 = testUser2.accessToken;
		testIdToken2 = testUser2.idToken;

		// Provision both users and fetch their personal orgs (auto-created on first auth via api-service middleware)

		switchToUser1();
		const orgsResult1 = await getOrganizations();
		if (orgsResult1.success && orgsResult1.data.organizations.length > 0) {
			testOrganizationPid1 = orgsResult1.data.organizations[0].pid;
		}

		switchToUser2();
		const orgsResult2 = await getOrganizations();
		if (orgsResult2.success && orgsResult2.data.organizations.length > 0) {
			testOrganizationPid2 = orgsResult2.data.organizations[0].pid;
		}

		// Create a test agent — fetch models and find the one matching OLLAMA_TEST_MODEL
		switchToUser1();
		const ollamaTestModel = process.env.OLLAMA_TEST_MODEL;
		if (!ollamaTestModel) {
			throw new Error('OLLAMA_TEST_MODEL environment variable is required for integration tests');
		}

		const modelsResult = await getModelsData();
		if (!modelsResult.success || modelsResult.data.models.length === 0) {
			throw new Error('No models available for testing');
		}

		const model = modelsResult.data.models.find((m) => m.modelId === ollamaTestModel);
		if (!model) {
			throw new Error(
				`Model "${ollamaTestModel}" not found — available: ${modelsResult.data.models.map((m) => m.modelId).join(', ')}`,
			);
		}

		const agentResult = await createAgentAction(
			{
				name: `Infer Test Agent ${randomUUID().slice(0, 8)}`,
				modelPid: model.identifier.value,
				systemPrompt: 'You are a test assistant. Keep responses very brief.',
			},
			[],
		);

		if (!agentResult.success) {
			throw new Error(`Failed to create test agent: ${agentResult.error}`);
		}

		testAgentPid = agentResult.data.identifier.value;
		createdAgentPids.push(testAgentPid);
	}, 60000);

	afterAll(async () => {
		// Cleanup conversations
		switchToUser1();
		for (const pid of createdConversationPids) {
			try {
				await deleteConversation(pid);
			} catch {
				// Ignore cleanup errors
			}
		}

		// Cleanup agents
		for (const pid of createdAgentPids) {
			try {
				await updateAgentAction({ pid, isActive: false }, []);
			} catch {
				// Ignore cleanup errors
			}
		}

		// Cleanup test users from api-service database
		if (testAccessToken) {
			try {
				const userClient = createUserClient(testAccessToken, testIdToken);
				await userClient.deleteAccount(create(DeleteAccountRequestSchema, {}));
			} catch {
				// Ignore cleanup errors
			}
		}

		if (testAccessToken2) {
			try {
				const userClient = createUserClient(testAccessToken2, testIdToken2);
				await userClient.deleteAccount(create(DeleteAccountRequestSchema, {}));
			} catch {
				// Ignore cleanup errors
			}
		}

		// Cleanup test users from Zitadel
		if (testUserId) {
			await deleteTestUser(testUserId);
		}
		if (testUserId2) {
			await deleteTestUser(testUserId2);
		}
	}, 30000);

	describe('createConversation', () => {
		it('should create conversation for valid agent', async () => {
			switchToUser1();
			const result = await createConversation(testAgentPid, 'Test Conversation');

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(result.data.conversation).toBeDefined();
			expect(result.data.conversation?.pid).toBeDefined();
			expect(result.data.conversation?.title).toBe('Test Conversation');

			if (result.data.conversation?.pid) {
				createdConversationPids.push(result.data.conversation.pid);
			}
		});

		it('should return error for invalid agent PID', async () => {
			switchToUser1();
			const result = await createConversation('non-existent-agent-pid');

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();
		});
	});

	describe('fetchConversations', () => {
		it('should return list of user conversations', async () => {
			switchToUser1();

			// First create a conversation
			const createResult = await createConversation(testAgentPid, `List Test ${randomUUID().slice(0, 8)}`);
			expect(createResult.success).toBe(true);
			if (createResult.success && createResult.data.conversation?.pid) {
				createdConversationPids.push(createResult.data.conversation.pid);
			}

			// Fetch conversations
			const result = await fetchConversations();

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(Array.isArray(result.data.conversations)).toBe(true);
			expect(result.data.conversations.length).toBeGreaterThan(0);
		});

		it('should not return other users conversations', async () => {
			// Create a conversation as user 1
			switchToUser1();
			const user1ConvResult = await createConversation(testAgentPid, `User1 Conv ${randomUUID().slice(0, 8)}`);
			expect(user1ConvResult.success).toBe(true);
			const user1ConvPid = user1ConvResult.success ? user1ConvResult.data.conversation?.pid : undefined;
			if (user1ConvPid) {
				createdConversationPids.push(user1ConvPid);
			}

			// Fetch as user 2 — should not see user 1's conversation
			switchToUser2();
			const user2Convs = await fetchConversations();
			expect(user2Convs.success).toBe(true);
			if (!user2Convs.success) return;

			const foundUser1Conv = user2Convs.data.conversations.find((c) => c.pid === user1ConvPid);
			expect(foundUser1Conv).toBeUndefined();

			switchToUser1();
		});

		it('should filter by agent PID when provided', async () => {
			switchToUser1();

			// Create conversations with test agent
			const conv1Result = await createConversation(testAgentPid, `Filter Test 1 ${randomUUID().slice(0, 8)}`);
			expect(conv1Result.success).toBe(true);
			if (conv1Result.success && conv1Result.data.conversation?.pid) {
				createdConversationPids.push(conv1Result.data.conversation.pid);
			}

			// Fetch with agent filter
			const result = await fetchConversations({ agentPid: testAgentPid });

			expect(result.success).toBe(true);
			if (!result.success) return;

			// All returned conversations should be for this agent
			for (const conv of result.data.conversations) {
				expect(conv.agentPid).toBe(testAgentPid);
			}
		});
	});

	describe('fetchConversationMessages', () => {
		it('should return conversation with messages', async () => {
			switchToUser1();

			// Create a conversation
			const createResult = await createConversation(testAgentPid, `Messages Test ${randomUUID().slice(0, 8)}`);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const convPid = createResult.data.conversation?.pid;
			expect(convPid).toBeDefined();
			if (!convPid) return;
			createdConversationPids.push(convPid);

			// Fetch the conversation
			const result = await fetchConversationMessages(convPid);

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(result.data.conversation).toBeDefined();
			expect(result.data.conversation?.pid).toBe(convPid);
			expect(Array.isArray(result.data.messages)).toBe(true);
		});

		it('should return error for non-existent conversation', async () => {
			switchToUser1();
			const result = await fetchConversationMessages('non-existent-conversation-pid');

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();
		});

		it('should return error for other users conversation', async () => {
			// Create as user 1
			switchToUser1();
			const createResult = await createConversation(testAgentPid, `Private Conv ${randomUUID().slice(0, 8)}`);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const convPid = createResult.data.conversation?.pid;
			expect(convPid).toBeDefined();
			if (!convPid) return;
			createdConversationPids.push(convPid);

			// Try to access as user 2
			switchToUser2();
			const result = await fetchConversationMessages(convPid);

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();

			// Switch back to user 1
			switchToUser1();
		});
	});

	describe('deleteConversation', () => {
		it('should soft-delete conversation', async () => {
			switchToUser1();

			// Create a conversation
			const createResult = await createConversation(testAgentPid, `Delete Test ${randomUUID().slice(0, 8)}`);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const convPid = createResult.data.conversation?.pid;
			expect(convPid).toBeDefined();
			if (!convPid) return;
			// Don't add to cleanup list since we're deleting it

			// Delete it
			const deleteResult = await deleteConversation(convPid);
			expect(deleteResult.success).toBe(true);

			// Should not be able to fetch it anymore
			const fetchResult = await fetchConversationMessages(convPid);
			expect(fetchResult.success).toBe(false);
		});

		it('should return error for non-existent conversation', async () => {
			switchToUser1();
			const result = await deleteConversation('non-existent-conversation-pid');

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();
		});
	});

	describe('sendInferenceMessage', () => {
		it('should return complete response with content', async () => {
			switchToUser1();

			const result = await sendInferenceMessage({
				agentPid: testAgentPid,
				message: 'Say hello',
			});

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(result.data.content).toBeDefined();
			expect(result.data.content.length).toBeGreaterThan(0);
			expect(result.data.conversationPid).toBeDefined();

			if (result.data.conversationPid) {
				createdConversationPids.push(result.data.conversationPid);
			}
		}, 60000);

		it('should create new conversation when none provided', async () => {
			switchToUser1();

			const result = await sendInferenceMessage({
				agentPid: testAgentPid,
				message: 'Hello',
			});

			expect(result.success).toBe(true);
			if (!result.success) return;

			// Should have created a new conversation
			expect(result.data.conversationPid).toBeDefined();

			if (result.data.conversationPid) {
				createdConversationPids.push(result.data.conversationPid);
			}
		}, 60000);

		it('should add to existing conversation when PID provided', async () => {
			switchToUser1();

			// Create a conversation first
			const createResult = await createConversation(testAgentPid, `Existing Conv ${randomUUID().slice(0, 8)}`);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const existingConvPid = createResult.data.conversation?.pid;
			expect(existingConvPid).toBeDefined();
			if (!existingConvPid) return;
			createdConversationPids.push(existingConvPid);

			// Send a message to existing conversation
			const result = await sendInferenceMessage({
				agentPid: testAgentPid,
				conversationPid: existingConvPid,
				message: 'Follow up message',
			});

			expect(result.success).toBe(true);
			if (!result.success) return;

			// Should use the same conversation
			expect(result.data.conversationPid).toBe(existingConvPid);
		}, 60000);

		it('should return error for invalid agent', async () => {
			switchToUser1();

			const result = await sendInferenceMessage({
				agentPid: 'non-existent-agent-pid',
				message: 'Hello',
			});

			expect(result.success).toBe(false);
			if (result.success) return;

			expect(result.error).toBeDefined();
		});
	});
});
