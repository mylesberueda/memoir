'use server';

import { create } from '@bufbuild/protobuf';
import { type QueryHit, QueryRequestSchema, type Ranking } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { getAccessToken } from '@/actions/auth';
import { memoryClient } from '@/lib/grpc/client';
import { getOrganizationContext } from '@/lib/grpc/transport';
import { getSession } from '@/lib/session';

import type { ActionResult } from '.';
import type { KindFilter } from './timeline';

export type { QueryHit, Ranking };

export interface QueryParams {
	/** Required by memoir's scope tuple; the operator supplies the agent persona id. */
	agentId: string;
	query: string;
	kind?: KindFilter;
	/** 0 / unset uses the service default. */
	limit?: number;
}

export interface QueryResult {
	hits: QueryHit[];
	/** The ranking the service actually ran, with library defaults filled in. */
	rankingUsed?: Ranking;
}

/**
 * Runs a hybrid-ranked query over a scope and returns the ranked hits plus
 * the ranking the service actually applied.
 *
 * Leaves `ranking` unset on the request, so memoir-core's default hybrid
 * strategy runs; the response echoes it back as `ranking_used`. Scope and
 * auth are assembled exactly as in the timeline action.
 */
export async function runQuery(params: QueryParams): Promise<ActionResult<QueryResult>> {
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

	const query = params.query.trim();
	if (!query) {
		return { success: false, error: 'Query is required' };
	}

	const accessToken = await getAccessToken();
	if (!accessToken) {
		return { success: false, error: 'Not authenticated' };
	}

	const request = create(QueryRequestSchema, {
		scope: { agentId, orgId, userId: session.userId },
		query,
		kinds: { episodic: params.kind === 'episodic', semantic: params.kind === 'semantic' },
		limit: params.limit ?? 0,
	});

	try {
		const response = await memoryClient().query(request, {
			headers: { authorization: `Bearer ${accessToken}` },
		});
		return { success: true, data: { hits: response.hits, rankingUsed: response.rankingUsed } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Query request failed' };
	}
}
