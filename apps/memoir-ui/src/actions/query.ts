'use server';

import { create } from '@bufbuild/protobuf';
import { type QueryHit, QueryRequestSchema, type Ranking } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { memoryClient } from '@/lib/grpc/client';
import { resolveScopeAndToken } from '@/lib/grpc/scope';

import type { ActionResult } from '.';
import type { KindFilter } from './timeline';

export type { QueryHit, Ranking };

export interface QueryParams {
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

export async function runQuery(params: QueryParams): Promise<ActionResult<QueryResult>> {
	const query = params.query.trim();
	if (!query) {
		return { success: false, error: 'Query is required' };
	}

	const resolved = await resolveScopeAndToken(params.agentId);
	if (!resolved.ok) return resolved.failure;

	const request = create(QueryRequestSchema, {
		scope: resolved.scope,
		query,
		kinds: { episodic: params.kind === 'episodic', semantic: params.kind === 'semantic' },
		limit: params.limit ?? 0,
	});

	try {
		const response = await memoryClient().query(request, {
			headers: { authorization: `Bearer ${resolved.accessToken}` },
		});
		return { success: true, data: { hits: response.hits, rankingUsed: response.rankingUsed } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Query request failed' };
	}
}
