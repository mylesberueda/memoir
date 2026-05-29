'use server';

import { create } from '@bufbuild/protobuf';
import { timestampFromDate } from '@bufbuild/protobuf/wkt';
import { type Memory, TimelineRequestSchema } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { memoryClient } from '@/lib/grpc/client';
import { resolveScopeAndToken } from '@/lib/grpc/scope';

import type { ActionResult } from '.';

export type { Memory };

export type KindFilter = 'episodic' | 'semantic' | 'both';

export interface TimelineParams {
	agentId: string;
	kind?: KindFilter;
	createdAfter?: Date;
	createdBefore?: Date;
	eventAtAfter?: Date;
	eventAtBefore?: Date;
	excludeSuperseded?: boolean;
	/** Default false — newest-first. */
	ascending?: boolean;
	/** 0 / unset uses the service default. */
	limit?: number;
}

export interface TimelineResult {
	memories: Memory[];
}

export async function getTimeline(params: TimelineParams): Promise<ActionResult<TimelineResult>> {
	const resolved = await resolveScopeAndToken(params.agentId);
	if (!resolved.ok) return resolved.failure;

	const request = create(TimelineRequestSchema, {
		scope: resolved.scope,
		kinds: { episodic: params.kind === 'episodic', semantic: params.kind === 'semantic' },
		createdAfter: params.createdAfter ? timestampFromDate(params.createdAfter) : undefined,
		createdBefore: params.createdBefore ? timestampFromDate(params.createdBefore) : undefined,
		eventAtAfter: params.eventAtAfter ? timestampFromDate(params.eventAtAfter) : undefined,
		eventAtBefore: params.eventAtBefore ? timestampFromDate(params.eventAtBefore) : undefined,
		excludeSuperseded: params.excludeSuperseded ?? false,
		ascending: params.ascending ?? false,
		limit: params.limit ?? 0,
	});

	try {
		const response = await memoryClient().timeline(request, {
			headers: { authorization: `Bearer ${resolved.accessToken}` },
		});
		return { success: true, data: { memories: response.memories } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Timeline request failed' };
	}
}
