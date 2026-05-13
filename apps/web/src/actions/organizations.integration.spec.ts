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

// Mock organization context (can be set in tests to simulate org-scoped requests)
let mockOrganizationId: string | undefined;

/** Set the organization context for subsequent API calls */
function setOrgContext(orgPid: string | undefined) {
	mockOrganizationId = orgPid;
}

// Mock next/headers
vi.mock('next/headers', () => ({
	cookies: () => ({
		get: (name: string) => {
			if (name === 'x-organization-id' && mockOrganizationId) {
				return { value: mockOrganizationId };
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
}));

// Import actions — vi.mock() is hoisted above imports by Vitest
import {
	createOrg,
	deleteOrg,
	getOrganizationByPid,
	getOrganizations,
	getOrgMembers,
	updateOrg,
} from '@actions/organizations';

/**
 * Integration tests for organization actions.
 *
 * These tests call the real server actions with mocked Next.js context.
 * They require:
 * - API service running at API_SERVICE_URL
 * - Zitadel running at ZITADEL_URL
 * - Valid service account credentials in ZITADEL_WEB_SERVICE_KEY
 *
 * Run with: pnpm nx run web:test:integration
 */

// Test user - created once per test suite
let testUserId: string;

// Track created orgs for cleanup
const createdOrgPids: string[] = [];

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

describe('Organizations Integration Tests', () => {
	beforeAll(async () => {
		const testUser = await createTestUser(generateTestEmail('org-test'), 'TestPassword123!', 'Org Test User');
		testUserId = testUser.userId;
		testAccessToken = testUser.accessToken;
		testIdToken = testUser.idToken;
	}, 30000);

	afterAll(async () => {
		// Cleanup created organizations using the real action
		for (const pid of createdOrgPids) {
			try {
				setOrgContext(pid);
				await deleteOrg(pid, []);
			} catch {
				// Ignore cleanup errors
			}
		}
		setOrgContext(undefined);

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

	describe('getOrganizations', () => {
		it('should return a list of organizations for authenticated user', async () => {
			const result = await getOrganizations();

			expect(result.success).toBe(true);
			if (!result.success) return;
			expect(Array.isArray(result.data.organizations)).toBe(true);
		});
	});

	describe('createOrg', () => {
		it('should create a new organization', async () => {
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;

			const result = await createOrg('Test Organization', uniqueSlug, []);

			expect(result.success).toBe(true);
			if (!result.success) return;

			expect(result.data.organization).toBeDefined();
			expect(result.data.organization?.name).toBe('Test Organization');
			expect(result.data.organization?.slug).toBe(uniqueSlug);

			if (result.data.organization?.pid) {
				createdOrgPids.push(result.data.organization.pid);
			}
		});

		it('should fail to create organization with duplicate slug', async () => {
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;

			// Create first org
			const firstResult = await createOrg('First Org', uniqueSlug, []);
			expect(firstResult.success).toBe(true);
			if (firstResult.success && firstResult.data.organization?.pid) {
				createdOrgPids.push(firstResult.data.organization.pid);
			}

			// Try to create second org with same slug
			const secondResult = await createOrg('Second Org', uniqueSlug, []);
			expect(secondResult.success).toBe(false);
		});
	});

	describe('getOrganizationByPid', () => {
		it('should get an organization by PID', async () => {
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;

			const createResult = await createOrg('Get Test Org', uniqueSlug, []);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const orgPid = createResult.data.organization?.pid;
			expect(orgPid).toBeDefined();
			if (!orgPid) return;
			createdOrgPids.push(orgPid);

			// Set org context for the get request
			setOrgContext(orgPid);
			const getResult = await getOrganizationByPid(orgPid);
			setOrgContext(undefined);

			expect(getResult.success).toBe(true);
			if (!getResult.success) return;
			expect(getResult.data.organization?.pid).toBe(orgPid);
			expect(getResult.data.organization?.name).toBe('Get Test Org');
		});

		it('should fail to get non-existent organization', async () => {
			// Need some org context to make the request, but the org won't exist
			// First create a real org to have valid context
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;
			const createResult = await createOrg('Context Org', uniqueSlug, []);
			if (createResult.success && createResult.data.organization?.pid) {
				createdOrgPids.push(createResult.data.organization.pid);
				setOrgContext(createResult.data.organization.pid);
			}

			const result = await getOrganizationByPid('non-existent-pid');
			setOrgContext(undefined);
			expect(result.success).toBe(false);
		});
	});

	describe('updateOrg', () => {
		it('should update an organization name', async () => {
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;

			const createResult = await createOrg('Original Name', uniqueSlug, []);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const orgPid = createResult.data.organization?.pid;
			expect(orgPid).toBeDefined();
			if (!orgPid) return;
			createdOrgPids.push(orgPid);

			// Set org context for the update request
			setOrgContext(orgPid);
			const updateResult = await updateOrg(orgPid, { name: 'Updated Name' }, []);
			setOrgContext(undefined);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;
			expect(updateResult.data.organization?.name).toBe('Updated Name');
		});

		it('should update an organization slug', async () => {
			const originalSlug = `test-org-${randomUUID().slice(0, 8)}`;
			const newSlug = `updated-org-${randomUUID().slice(0, 8)}`;

			const createResult = await createOrg('Slug Update Test', originalSlug, []);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const orgPid = createResult.data.organization?.pid;
			expect(orgPid).toBeDefined();
			if (!orgPid) return;
			createdOrgPids.push(orgPid);

			// Set org context for the update request
			setOrgContext(orgPid);
			const updateResult = await updateOrg(orgPid, { slug: newSlug }, []);
			setOrgContext(undefined);

			expect(updateResult.success).toBe(true);
			if (!updateResult.success) return;
			expect(updateResult.data.organization?.slug).toBe(newSlug);
		});
	});

	describe('deleteOrg', () => {
		it('should delete an organization', async () => {
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;

			const createResult = await createOrg('Delete Test Org', uniqueSlug, []);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const orgPid = createResult.data.organization?.pid;
			expect(orgPid).toBeDefined();
			if (!orgPid) return;

			// Set org context for the delete request
			setOrgContext(orgPid);
			const deleteResult = await deleteOrg(orgPid, []);
			expect(deleteResult.success).toBe(true);

			// Verify it's gone (still in org context, but org no longer exists)
			const getResult = await getOrganizationByPid(orgPid);
			setOrgContext(undefined);
			expect(getResult.success).toBe(false);
		});
	});

	describe('getOrgMembers', () => {
		it('should list members of an organization', async () => {
			const uniqueSlug = `test-org-${randomUUID().slice(0, 8)}`;

			const createResult = await createOrg('Members Test Org', uniqueSlug, []);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const orgPid = createResult.data.organization?.pid;
			expect(orgPid).toBeDefined();
			if (!orgPid) return;
			createdOrgPids.push(orgPid);

			// Set org context for the list members request
			setOrgContext(orgPid);
			const membersResult = await getOrgMembers(orgPid);
			setOrgContext(undefined);

			expect(membersResult.success).toBe(true);
			if (!membersResult.success) return;
			expect(Array.isArray(membersResult.data.members)).toBe(true);
			// Creator should automatically be a member
			expect(membersResult.data.members.length).toBeGreaterThanOrEqual(1);
		});

		it('should return enriched user data (email + display name) for members', async () => {
			const uniqueSlug = `test-org-enriched-${randomUUID().slice(0, 8)}`;

			const createResult = await createOrg('Enriched Members Org', uniqueSlug, []);
			expect(createResult.success).toBe(true);
			if (!createResult.success) return;

			const orgPid = createResult.data.organization?.pid;
			expect(orgPid).toBeDefined();
			if (!orgPid) return;
			createdOrgPids.push(orgPid);

			setOrgContext(orgPid);
			const membersResult = await getOrgMembers(orgPid);
			setOrgContext(undefined);

			expect(membersResult.success).toBe(true);
			if (!membersResult.success) return;
			expect(membersResult.data.members.length).toBeGreaterThanOrEqual(1);

			// The creator (test user) should have email populated
			const creator = membersResult.data.members[0];
			expect(creator.email).toBeDefined();
			expect(creator.email.length).toBeGreaterThan(0);
			expect(creator.email).toContain('@');
		});
	});
});
