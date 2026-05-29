'use server';

import { create } from '@bufbuild/protobuf';
import { timestampFromDate } from '@bufbuild/protobuf/wkt';
import { type Memory, RecallAsOfRequestSchema } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { getAccessToken } from '@/actions/auth';
import { memoryClient } from '@/lib/grpc/client';
import { getOrganizationContext } from '@/lib/grpc/transport';
import { getSession } from '@/lib/session';

import type { ActionResult } from '.';
import type { KindFilter } from './timeline';

export interface RecallAsOfParams {
	/** Required by memoir's scope tuple; the operator supplies the agent persona id. */
	agentId: string;
	/** The instant to reconstruct memoir's knowledge as of. */
	asOf: Date;
	kind?: KindFilter;
	/** 0 / unset uses the service default. */
	limit?: number;
}

export interface RecallAsOfResult {
	memories: Memory[];
}

/**
 * Reconstructs the active-as-of set for a scope at a point in time.
 *
 * A future `asOf` yields current state; one before any memory existed
 * yields empty — both fall out of the library semantics. Scope and auth
 * are assembled exactly as in the timeline action.
 */
export async function recallAsOf(params: RecallAsOfParams): Promise<ActionResult<RecallAsOfResult>> {
	const session = await getSession();
	if (!session) {
		return { success: false, error: 'Not authenticated' };
	}

	const orgId = await getOrganizationContext();
	if (!orgId) {
		return { success: false, error: 'No organization selected' };
	}

	const agentId = params.agentId.trim();
	if (!agentId) {
		return { success: false, error: 'Agent id is required' };
	}

	const accessToken = await getAccessToken();
	if (!accessToken) {
		return { success: false, error: 'Not authenticated' };
	}

	const request = create(RecallAsOfRequestSchema, {
		scope: { agentId, orgId, userId: session.userId },
		asOf: timestampFromDate(params.asOf),
		kinds: { episodic: params.kind === 'episodic', semantic: params.kind === 'semantic' },
		limit: params.limit ?? 0,
	});

	try {
		const response = await memoryClient().recallAsOf(request, {
			headers: { authorization: `Bearer ${accessToken}` },
		});
		return { success: true, data: { memories: response.memories } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Recall-as-of request failed' };
	}
}
