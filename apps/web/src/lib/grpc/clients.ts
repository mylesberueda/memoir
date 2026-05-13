'use server';

import { type Client, createClient } from '@connectrpc/connect';
import { AdminService } from '@startup/proto-ts/api-service/api/v1/admin_pb';
import { BillingService } from '@startup/proto-ts/api-service/api/v1/billing_pb';
import { OrganizationService } from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { UserService } from '@startup/proto-ts/api-service/api/v1/users_pb';
import { ChannelService } from '@startup/proto-ts/chat-service/chat/v1/channel_pb';
import { ChatService } from '@startup/proto-ts/chat-service/chat/v1/chat_pb';
import { ModerationService } from '@startup/proto-ts/chat-service/chat/v1/moderation_pb';
import { NotificationService } from '@startup/proto-ts/notification-service/notification/v1/notification_pb';
import { AgentService } from '@startup/proto-ts/rig-service/rig/v1/agent_pb';
import {
	DocumentGroupService,
	DocumentSearchService,
	DocumentService,
} from '@startup/proto-ts/rig-service/rig/v1/document_pb';
import { InferenceService } from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import { ModelService, ProviderService } from '@startup/proto-ts/rig-service/rig/v1/provider_pb';
import { ToolService } from '@startup/proto-ts/rig-service/rig/v1/tool_pb';
import { createAuthenticatedTransport } from './transport';

function requireEnv(name: string): string {
	const value = process.env[name];
	if (!value) {
		throw new Error(`Missing required environment variable: ${name}`);
	}
	return value;
}

export async function userServiceClient(): Promise<Client<typeof UserService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('API_SERVICE_URL'));
	if (!transport) return null;
	return createClient(UserService, transport);
}

export async function organizationServiceClient(): Promise<Client<typeof OrganizationService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('API_SERVICE_URL'));
	if (!transport) return null;
	return createClient(OrganizationService, transport);
}

export async function adminServiceClient(): Promise<Client<typeof AdminService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('API_SERVICE_URL'));
	if (!transport) return null;
	return createClient(AdminService, transport);
}

export async function agentServiceClient(): Promise<Client<typeof AgentService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(AgentService, transport);
}

export async function inferenceServiceClient(): Promise<Client<typeof InferenceService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(InferenceService, transport);
}

export async function providerServiceClient(): Promise<Client<typeof ProviderService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(ProviderService, transport);
}

export async function modelServiceClient(): Promise<Client<typeof ModelService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(ModelService, transport);
}

export async function toolServiceClient(): Promise<Client<typeof ToolService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(ToolService, transport);
}

export async function chatServiceClient(): Promise<Client<typeof ChatService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('CHAT_SERVICE_URL'));
	if (!transport) return null;
	return createClient(ChatService, transport);
}

export async function channelServiceClient(): Promise<Client<typeof ChannelService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('CHAT_SERVICE_URL'));
	if (!transport) return null;
	return createClient(ChannelService, transport);
}

export async function moderationServiceClient(): Promise<Client<typeof ModerationService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('CHAT_SERVICE_URL'));
	if (!transport) return null;
	return createClient(ModerationService, transport);
}

export async function notificationServiceClient(): Promise<Client<typeof NotificationService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('NOTIFICATION_SERVICE_URL'));
	if (!transport) return null;
	return createClient(NotificationService, transport);
}

export async function billingServiceClient(): Promise<Client<typeof BillingService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('API_SERVICE_URL'));
	if (!transport) return null;
	return createClient(BillingService, transport);
}

export async function documentServiceClient(): Promise<Client<typeof DocumentService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(DocumentService, transport);
}

export async function documentGroupServiceClient(): Promise<Client<typeof DocumentGroupService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(DocumentGroupService, transport);
}

export async function documentSearchServiceClient(): Promise<Client<typeof DocumentSearchService> | null> {
	const transport = await createAuthenticatedTransport(requireEnv('RIG_SERVICE_URL'));
	if (!transport) return null;
	return createClient(DocumentSearchService, transport);
}
