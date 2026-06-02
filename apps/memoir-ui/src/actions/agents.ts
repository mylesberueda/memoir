'use server';

import { create } from '@bufbuild/protobuf';
import { ListAgentsRequestSchema } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { memoryClient } from '@/lib/grpc/client';
import { resolveScopeAndToken } from '@/lib/grpc/scope';

import type { ActionResult } from '.';

export interface ListAgentsResult {
	agentIds: string[];
}

export async function listAgents(): Promise<ActionResult<ListAgentsResult>> {
	// agent_id is irrelevant to ListAgents — it's what we're discovering. We
	// reuse the scope resolver only for the caller's org_id, user_id, and token.
	const resolved = await resolveScopeAndToken('_');
	if (!resolved.ok) return resolved.failure;

	const request = create(ListAgentsRequestSchema, {
		orgId: resolved.scope.orgId,
		userId: resolved.scope.userId,
	});

	try {
		const response = await memoryClient().listAgents(request, {
			headers: { authorization: `Bearer ${resolved.accessToken}` },
		});
		return { success: true, data: { agentIds: response.agentIds } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'List agents request failed' };
	}
}
