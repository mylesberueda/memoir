'use server';

import { create } from '@bufbuild/protobuf';
import { timestampFromDate } from '@bufbuild/protobuf/wkt';
import { type Memory, RecallAsOfRequestSchema } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { memoryClient } from '@/lib/grpc/client';
import { resolveScopeAndToken } from '@/lib/grpc/scope';

import type { ActionResult } from '.';
import type { KindFilter } from './timeline';

export interface RecallAsOfParams {
	agentId: string;
	asOf: Date;
	kind?: KindFilter;
	/** 0 / unset uses the service default. */
	limit?: number;
}

export interface RecallAsOfResult {
	memories: Memory[];
}

export async function recallAsOf(params: RecallAsOfParams): Promise<ActionResult<RecallAsOfResult>> {
	const resolved = await resolveScopeAndToken(params.agentId);
	if (!resolved.ok) return resolved.failure;

	const request = create(RecallAsOfRequestSchema, {
		scope: resolved.scope,
		asOf: timestampFromDate(params.asOf),
		kinds: { episodic: params.kind === 'episodic', semantic: params.kind === 'semantic' },
		limit: params.limit ?? 0,
	});

	try {
		const response = await memoryClient().recallAsOf(request, {
			headers: { authorization: `Bearer ${resolved.accessToken}` },
		});
		return { success: true, data: { memories: response.memories } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Recall-as-of request failed' };
	}
}
