import { create } from '@bufbuild/protobuf';
import { createClient } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import { DeleteAccountRequestSchema, UserService } from '@polypixel/proto-ts/api-service/api/v1/users_pb';
import { createTestUser, deleteTestUser, generateTestEmail } from '@test-utils';
import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest';

let testAccessToken: string;
let testIdToken: string;
let testUserId: string;

let mockOrganizationId: string | undefined;

function setOrgContext(orgPid: string | undefined) {
	mockOrganizationId = orgPid;
}

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

vi.mock('next/cache', () => ({
	revalidatePath: vi.fn(),
}));

vi.mock('@actions/auth', () => ({
	getAccessToken: () => Promise.resolve(testAccessToken),
	getIdToken: () => Promise.resolve(testIdToken),
}));

import { getOrganizations } from '@actions/organizations';

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

describe('SSR org context — stale cookie regression', () => {
	beforeAll(async () => {
		const testUser = await createTestUser(
			generateTestEmail('ssr-org-ctx'),
			'TestPassword123!',
			'SSR Context Test User',
		);
		testUserId = testUser.userId;
		testAccessToken = testUser.accessToken;
		testIdToken = testUser.idToken;
	}, 30000);

	afterAll(async () => {
		setOrgContext(undefined);
		if (testAccessToken) {
			try {
				const userClient = createUserClient(testAccessToken, testIdToken);
				await userClient.deleteAccount(create(DeleteAccountRequestSchema, {}));
			} catch {
				// ignore cleanup failures
			}
		}
		if (testUserId) {
			await deleteTestUser(testUserId);
		}
	}, 30000);

	it('should fail when the org cookie points at a nonexistent org', async () => {
		setOrgContext('nonexistent-org-pid-12345');

		const result = await getOrganizations();

		expect(result.success).toBe(false);
		if (result.success) return;
		expect(result.error).toContain('Organization not found');

		setOrgContext(undefined);
	});

	it('should succeed when no org cookie is set (middleware passthrough)', async () => {
		setOrgContext(undefined);

		const result = await getOrganizations();

		expect(result.success).toBe(true);
		if (!result.success) return;
		expect(Array.isArray(result.data.organizations)).toBe(true);
	});
});
