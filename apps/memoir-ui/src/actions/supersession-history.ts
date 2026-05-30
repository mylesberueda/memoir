'use server';

import { create } from '@bufbuild/protobuf';
import { type SupersessionEvent, SupersessionHistoryRequestSchema } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

import { getAccessToken } from '@/actions/auth';
import { memoryClient } from '@/lib/grpc/client';
import { getSession } from '@/lib/session';

import type { ActionResult } from '.';

export interface SupersessionHistoryParams {
	pid: string;
}

export interface SupersessionHistoryResult {
	events: SupersessionEvent[];
}

export async function getSupersessionHistory(
	params: SupersessionHistoryParams,
): Promise<ActionResult<SupersessionHistoryResult>> {
	const session = await getSession();
	if (!session) {
		return { success: false, error: 'Not authenticated' };
	}
	const accessToken = await getAccessToken();
	if (!accessToken) {
		return { success: false, error: 'Not authenticated' };
	}

	const pid = params.pid.trim();
	if (!pid) {
		return { success: false, error: 'Pid is required' };
	}

	const request = create(SupersessionHistoryRequestSchema, { pid });

	try {
		const response = await memoryClient().supersessionHistory(request, {
			headers: { authorization: `Bearer ${accessToken}` },
		});
		return { success: true, data: { events: response.events } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Supersession history request failed' };
	}
}
