'use server';

import { create } from '@bufbuild/protobuf';
import { timestampFromDate } from '@bufbuild/protobuf/wkt';
import { EditRequestSchema, type Memory } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
import { getAccessToken } from '@/actions/auth';
import { memoryClient } from '@/lib/grpc/client';
import { getSession } from '@/lib/session';

import type { ActionResult } from '.';

export interface EditParams {
	pid: string;
	content?: string;
	eventAt?: Date;
}

export async function editMemory(params: EditParams): Promise<ActionResult<{ memory: Memory }>> {
	const session = await getSession();
	if (!session) {
		return { success: false, error: 'Not authenticated' };
	}
	const accessToken = await getAccessToken();
	if (!accessToken) {
		return { success: false, error: 'Not authenticated' };
	}

	const request = create(EditRequestSchema, {
		pid: params.pid,
		content: params.content,
		eventAt: params.eventAt ? timestampFromDate(params.eventAt) : undefined,
	});

	try {
		const response = await memoryClient().edit(request, {
			headers: { authorization: `Bearer ${accessToken}` },
		});
		if (!response.memory) {
			return { success: false, error: 'Edit returned no memory' };
		}
		return { success: true, data: { memory: response.memory } };
	} catch (err) {
		return { success: false, error: err instanceof Error ? err.message : 'Edit request failed' };
	}
}
