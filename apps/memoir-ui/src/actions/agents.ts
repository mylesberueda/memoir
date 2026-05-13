'use server';

import { create } from '@bufbuild/protobuf';
import { agentServiceClient } from '@lib/grpc/clients';
import { actionLogger } from '@lib/logger';
import {
	CreateAgentRequestSchema,
	DeleteAgentRequestSchema,
	GetAgentRequestSchema,
	GetUserAssistantRequestSchema,
	ListAgentsRequestSchema,
	type Agent as ProtoAgent,
	type ListAgentsResponse as ProtoListAgentsResponse,
	type UpdateAgentRequest,
	UpdateAgentRequestSchema,
	type UpdateAgentToolRequest,
	UpdateAgentToolRequestSchema,
} from '@polypixel/proto-ts/rig-service/rig/v1/agent_pb';
import { revalidatePath } from 'next/cache';

import type { ActionResult } from '.';
import { getCurrentUser } from './auth';

type AgentIdentifier = ProtoAgent['identifier'];
type PidIdentifier = Extract<AgentIdentifier, { case: 'pid' }>;
export type Agent = Omit<ProtoAgent, 'identifier'> & { identifier: PidIdentifier };

export type ListAgentsResponse = Omit<ProtoListAgentsResponse, 'agents'> & { agents: Agent[] };

export async function getAgent(pid: string): Promise<ActionResult<Agent>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetAgentRequestSchema, { pid });
		const res = await client.getAgent(req);

		if (!res.agent || res.agent.identifier.case !== 'pid') {
			return { success: false, error: 'Agent not found' };
		}

		return { success: true, data: res.agent as Agent };
	} catch (error) {
		actionLogger.error('getAgent failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function getAgents(): Promise<ActionResult<ListAgentsResponse>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListAgentsRequestSchema, { isActive: true });
		const res = await client.listAgents(req);

		const agents = res.agents.filter((a): a is Agent => a.identifier.case === 'pid');

		return { success: true, data: { ...res, agents } };
	} catch (error) {
		actionLogger.error('getAgentsData failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export type CreateAgentInput = {
	name: string;
	modelPid: string;
	systemPrompt?: string;
	providerPid?: string;
	temperature?: number;
	toolPids?: string[];
};

export async function createAgent(
	input: CreateAgentInput,
	revalidatePages: string[] = ['/agents'],
): Promise<ActionResult<Agent>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const user = await getCurrentUser();
		if (!user) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CreateAgentRequestSchema, {
			name: input.name,
			modelPid: input.modelPid,
			systemPrompt: input.systemPrompt ?? '',
			providerPid: input.providerPid,
			temperature: input.temperature ?? 70,
			toolPids: input.toolPids ?? [],
		});

		const res = await client.createAgent(req);

		if (!res.agent || res.agent.identifier.case !== 'pid') {
			return { success: false, error: 'Failed to create agent' };
		}

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: res.agent as Agent };
	} catch (error) {
		actionLogger.error('createAgentAction failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

/**
 * Input for updating an agent. All fields except `pid` are optional.
 * Only provided fields will be updated; omitted fields retain their current values.
 */
export type UpdateAgentInput = Omit<UpdateAgentRequest, '$typeName' | '$unknown' | 'tools'> & {
	tools?: Omit<UpdateAgentToolRequest, '$typeName' | '$unknown'>[];
};

export async function updateAgent(
	input: UpdateAgentInput,
	revalidatePages: string[] = ['/agents'],
): Promise<ActionResult<Agent>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UpdateAgentRequestSchema, {
			pid: input.pid,
			name: input.name,
			slug: input.slug,
			modelPid: input.modelPid,
			temperature: input.temperature,
			systemPrompt: input.systemPrompt,
			isActive: input.isActive,
			providerPid: input.providerPid,
			tools: input.tools?.map((t) => create(UpdateAgentToolRequestSchema, t)) ?? [],
		});

		const res = await client.updateAgent(req);

		if (!res.agent || res.agent.identifier.case !== 'pid') {
			return { success: false, error: 'Failed to update agent' };
		}

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: res.agent as Agent };
	} catch (error) {
		actionLogger.error('updateAgentAction failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function deleteAgent(pid: string, revalidatePages: string[] = ['/agents']): Promise<ActionResult<void>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(DeleteAgentRequestSchema, { pid });
		await client.deleteAgent(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: undefined };
	} catch (error) {
		actionLogger.error('deleteAgentAction failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export type UserAssistantResponse = { agent: Agent };

export async function getUserAssistant(): Promise<ActionResult<UserAssistantResponse>> {
	try {
		const client = await agentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const user = await getCurrentUser();
		if (!user) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetUserAssistantRequestSchema, { userId: user.id });
		const res = await client.getUserAssistant(req);

		if (!res.agent || res.agent.identifier.case !== 'pid') {
			return { success: false, error: 'Failed to get user assistant' };
		}

		return { success: true, data: { agent: res.agent as Agent } };
	} catch (error) {
		actionLogger.error('getUserAssistantAction failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
