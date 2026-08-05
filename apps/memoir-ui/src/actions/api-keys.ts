'use server';

import { create } from '@bufbuild/protobuf';
import {
	type ApiKey,
	ApiKeyRole,
	CreateApiKeyRequestSchema,
	ListApiKeysRequestSchema,
	RevokeApiKeyRequestSchema,
	RotateApiKeyRequestSchema,
} from '@polypixel/memoir-sdk/memoir/v1/auth_pb';

import { getAccessToken } from '@/actions/auth';
import { authClient } from '@/lib/grpc/client';

import type { ActionResult } from '.';

export interface ListApiKeysResult {
	keys: ApiKey[];
	nextCursor?: string;
}

/**
 * The plaintext half of a create/rotate response.
 *
 * `plaintext` is the only moment the full `mk.<key_id>.<secret>` string
 * exists anywhere — the service persists an Argon2 hash and cannot return it
 * again. Callers must surface it to the operator immediately.
 */
export interface ApiKeySecretResult {
	key: ApiKey;
	plaintext: string;
}

async function authorization(): Promise<{ authorization: string } | null> {
	const accessToken = await getAccessToken();
	if (!accessToken) return null;
	return { authorization: `Bearer ${accessToken}` };
}

function failure(err: unknown, fallback: string): ActionResult<never> {
	return { success: false, error: err instanceof Error ? err.message : fallback };
}

export async function listApiKeys(cursor?: string): Promise<ActionResult<ListApiKeysResult>> {
	const headers = await authorization();
	if (!headers) return { success: false, error: 'Not authenticated' };

	const request = create(ListApiKeysRequestSchema, { limit: 100, cursor });

	try {
		const response = await authClient().listApiKeys(request, { headers });
		return { success: true, data: { keys: response.keys, nextCursor: response.nextCursor } };
	} catch (err) {
		return failure(err, 'List API keys request failed');
	}
}

export async function createApiKey(
	name: string,
	role: ApiKeyRole,
	orgId?: string,
): Promise<ActionResult<ApiKeySecretResult>> {
	const headers = await authorization();
	if (!headers) return { success: false, error: 'Not authenticated' };

	const trimmedName = name.trim();
	if (!trimmedName) return { success: false, error: 'Name is required' };
	if (role === ApiKeyRole.UNSPECIFIED) return { success: false, error: 'Role is required' };

	const trimmedOrgId = orgId?.trim();
	const request = create(CreateApiKeyRequestSchema, {
		name: trimmedName,
		role,
		orgId: trimmedOrgId ? trimmedOrgId : undefined,
	});

	try {
		const response = await authClient().createApiKey(request, { headers });
		if (!response.key) return { success: false, error: 'Service returned no key' };
		return { success: true, data: { key: response.key, plaintext: response.plaintext } };
	} catch (err) {
		return failure(err, 'Create API key request failed');
	}
}

export async function rotateApiKey(pid: string): Promise<ActionResult<ApiKeySecretResult>> {
	const headers = await authorization();
	if (!headers) return { success: false, error: 'Not authenticated' };
	if (!pid) return { success: false, error: 'Key id is required' };

	const request = create(RotateApiKeyRequestSchema, { pid });

	try {
		const response = await authClient().rotateApiKey(request, { headers });
		if (!response.key) return { success: false, error: 'Service returned no key' };
		return { success: true, data: { key: response.key, plaintext: response.plaintext } };
	} catch (err) {
		return failure(err, 'Rotate API key request failed');
	}
}

export async function revokeApiKey(pid: string): Promise<ActionResult<void>> {
	const headers = await authorization();
	if (!headers) return { success: false, error: 'Not authenticated' };
	if (!pid) return { success: false, error: 'Key id is required' };

	const request = create(RevokeApiKeyRequestSchema, { pid });

	try {
		await authClient().revokeApiKey(request, { headers });
		return { success: true, data: undefined };
	} catch (err) {
		return failure(err, 'Revoke API key request failed');
	}
}
