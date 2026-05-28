'use server';

import { create } from '@bufbuild/protobuf';
import { timestampFromDate } from '@bufbuild/protobuf/wkt';
import {
	type Memory,
	TimelineRequestSchema,
} from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { getAccessToken } from '@/actions/auth';
import { getOrganizationContext } from '@/lib/grpc/transport';
import { memoryClient } from '@/lib/grpc/client';
import { getSession } from '@/lib/session';

import type { ActionResult } from '.';

export type { Memory };

/** Kind filter for a timeline read. `both` (the default) retrieves all kinds. */
export type KindFilter = 'episodic' | 'semantic' | 'both';

export interface TimelineParams {
	/** Required by memoir's scope tuple; the operator supplies the agent persona id. */
	agentId: string;
	kind?: KindFilter;
	createdAfter?: Date;
	createdBefore?: Date;
	eventAtAfter?: Date;
	eventAtBefore?: Date;
	/** Default false — timeline includes superseded rows (the audit view). */
	excludeSuperseded?: boolean;
	/** Default false — newest-first. */
	ascending?: boolean;
	/** 0 / unset uses the service default. */
	limit?: number;
}

export interface TimelineResult {
	memories: Memory[];
}

/**
 * Reads the chronological memory event-log for a scope.
 *
 * Scope is assembled from the operator-supplied `agentId`, the session's
 * user, and the active organization context. Requires an authenticated
 * session; the access token is attached as a bearer header.
 */
export async function getTimeline(params: TimelineParams): Promise<ActionResult<TimelineResult>> {
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

	const kinds =
		params.kind === 'episodic'
			? { episodic: true, semantic: false }
			: params.kind === 'semantic'
				? { episodic: false, semantic: true }
				: undefined;

	const request = create(TimelineRequestSchema, {
		scope: { agentId, orgId, userId: session.userId },
		kinds,
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
			headers: { authorization: `Bearer ${accessToken}` },
		});
		return { success: true, data: { memories: response.memories } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Timeline request failed' };
	}
}
