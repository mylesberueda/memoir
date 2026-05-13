'use server';

import { create } from '@bufbuild/protobuf';
import { agentServiceClient, inferenceServiceClient } from '@lib/grpc/clients';
import { actionLogger } from '@lib/logger';
import {
	ListAgentSharesRequestSchema,
	type ListAgentSharesResponse,
	ShareAgentRequestSchema,
	type ShareAgentResponse,
	UnshareAgentRequestSchema,
	type UnshareAgentResponse,
} from '@polypixel/memoir-sdk/rig-service/rig/v1/agent_pb';
import {
	ListConversationSharesRequestSchema,
	type ListConversationSharesResponse,
	ShareConversationRequestSchema,
	type ShareConversationResponse,
	UnshareConversationRequestSchema,
	type UnshareConversationResponse,
} from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';

import type { ActionResult } from '.';

export async function shareAgent(
	agentPid: string,
	userId: string,
	permissions: number,
): Promise<ActionResult<ShareAgentResponse>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ShareAgentRequestSchema, { agentPid, userId, permissions });
		const res = await client.shareAgent(req);
		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('shareAgent failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function unshareAgent(agentPid: string, userId: string): Promise<ActionResult<UnshareAgentResponse>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UnshareAgentRequestSchema, { agentPid, userId });
		const res = await client.unshareAgent(req);
		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('unshareAgent failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function listAgentShares(
	agentPid: string,
	pageSize?: number,
	cursor?: string,
): Promise<ActionResult<ListAgentSharesResponse>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListAgentSharesRequestSchema, { agentPid, pageSize, cursor });
		const res = await client.listAgentShares(req);
		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('listAgentShares failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function shareConversation(
	conversationPid: string,
	userId: string,
	permissions: number,
): Promise<ActionResult<ShareConversationResponse>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ShareConversationRequestSchema, { conversationPid, userId, permissions });
		const res = await client.shareConversation(req);
		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('shareConversation failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function unshareConversation(
	conversationPid: string,
	userId: string,
): Promise<ActionResult<UnshareConversationResponse>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UnshareConversationRequestSchema, { conversationPid, userId });
		const res = await client.unshareConversation(req);
		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('unshareConversation failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function listConversationShares(
	conversationPid: string,
	pageSize?: number,
	cursor?: string,
): Promise<ActionResult<ListConversationSharesResponse>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListConversationSharesRequestSchema, { conversationPid, pageSize, cursor });
		const res = await client.listConversationShares(req);
		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('listConversationShares failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
