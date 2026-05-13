'use server';

import { create } from '@bufbuild/protobuf';
import { toolServiceClient } from '@lib/grpc/clients';
import { actionLogger } from '@lib/logger';
import {
	ListToolsRequestSchema,
	type ListToolsResponse as ProtoListToolsResponse,
	type Tool as ProtoTool,
} from '@polypixel/memoir-sdk/rig-service/rig/v1/tool_pb';

import type { ActionResult } from '.';

export type Tool = ProtoTool;

export type ListToolsResponse = ProtoListToolsResponse;

export async function getTools(): Promise<ActionResult<ListToolsResponse>> {
	try {
		const client = await toolServiceClient();
		if (!client) {
			actionLogger.warn('getToolsData: Authentication required');
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListToolsRequestSchema, {
			isActive: true,
			toolType: 'system', // Only show system tools for agent creation (not assistant-only tools)
			page: BigInt(1),
			pageSize: BigInt(100),
		});

		const res = await client.listTools(req);

		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('getToolsData failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
