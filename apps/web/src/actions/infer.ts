'use server';

import { create } from '@bufbuild/protobuf';
import { inferenceServiceClient } from '@lib/grpc/clients';
import { createChildLogger } from '@lib/logger';
import {
	CancelInferenceRequestSchema,
	type Conversation,
	CreateConversationRequestSchema,
	DeleteConversationRequestSchema,
	GetConversationRequestSchema,
	type InferRequest,
	InferRequestSchema,
	type InferResponse,
	ListConversationsRequestSchema,
	type Message,
} from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import type { ActionResult } from '.';

const log = createChildLogger({ action: 'infer' });

export async function fetchConversations(options?: {
	isActive?: boolean;
	agentPid?: string;
	page?: number;
	pageSize?: number;
}): Promise<ActionResult<{ conversations: Conversation[]; total: number }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListConversationsRequestSchema, {
			isActive: options?.isActive,
			agentPid: options?.agentPid,
			page: options?.page ?? 1,
			pageSize: options?.pageSize ?? 50,
		});

		const res = await client.listConversations(req);

		log.debug('fetchConversations response', {
			total: res.total,
			conversationCount: res.conversations.length,
			conversations: res.conversations.map((c) => ({
				pid: c.pid,
				title: c.title,
				messageCount: c.messageCount,
				lastMessageAt: c.lastMessageAt?.toString(),
				lastMessageAtType: typeof c.lastMessageAt,
				createdAt: c.createdAt?.toString(),
				createdAtType: typeof c.createdAt,
			})),
		});

		return {
			success: true,
			data: {
				conversations: res.conversations,
				total: res.total,
			},
		};
	} catch (error) {
		log.error('fetchConversations error', { error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function fetchConversationMessages(
	conversationPid: string,
): Promise<ActionResult<{ conversation: Conversation | undefined; messages: Message[] }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetConversationRequestSchema, { pid: conversationPid });
		const res = await client.getConversation(req);

		const messages = res.conversation?.messages ?? [];

		log.debug('fetchConversationMessages response', {
			conversationPid,
			messageCount: messages.length,
			messages: messages.map((m) => ({
				pid: m.pid,
				role: m.role,
				createdAt: m.createdAt,
				partsCount: m.parts.length,
			})),
		});

		return {
			success: true,
			data: {
				conversation: res.conversation,
				messages,
			},
		};
	} catch (error) {
		log.error('fetchConversationMessages error', {
			conversationPid,
			error: error instanceof Error ? error.message : error,
		});
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

/**
 * Server action to create a new conversation thread
 */
export async function createConversation(
	agentPid: string,
	title?: string,
): Promise<ActionResult<{ conversation: Conversation | undefined }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CreateConversationRequestSchema, {
			agentPid,
			title,
		});

		const res = await client.createConversation(req);

		return {
			success: true,
			data: {
				conversation: res.conversation,
			},
		};
	} catch (error) {
		console.error('Create conversation error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

/**
 * Server action to delete a conversation thread
 */
export async function deleteConversation(
	conversationPid: string,
): Promise<{ success: true } | { success: false; error: string }> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(DeleteConversationRequestSchema, { pid: conversationPid });
		await client.deleteConversation(req);

		return { success: true };
	} catch (error) {
		console.error('Delete conversation error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

/**
 * Server action for non-streaming inference (fallback when streaming is disabled)
 * Collects all streaming events and returns the complete response
 */
export async function sendInferenceMessage(input: {
	agentPid: string;
	conversationPid?: string;
	message: string;
}): Promise<
	ActionResult<{
		content: string;
		conversationPid: string | undefined;
		messagePid: string | undefined;
		message: Message | undefined;
	}>
> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const requestId = `req_${Date.now()}_${Math.random().toString(36).slice(2)}`;
		const req = create(InferRequestSchema, {
			agentPid: input.agentPid,
			conversationPid: input.conversationPid,
			message: input.message,
			requestId,
		});

		// Collect all events from the streaming response
		let content = '';
		let conversationPid: string | undefined;
		let messagePid: string | undefined;
		let finalMessage: Message | undefined;

		for await (const res of client.infer(req)) {
			switch (res.event.case) {
				case 'partDelta':
					if (res.event.value.delta.case === 'content') {
						content += res.event.value.delta.value;
					}
					break;
				case 'complete':
					conversationPid = res.event.value.conversationPid;
					messagePid = res.event.value.message?.pid;
					finalMessage = res.event.value.message;
					break;
			}
		}

		return {
			success: true,
			data: {
				content,
				conversationPid,
				messagePid,
				message: finalMessage,
			},
		};
	} catch (error) {
		console.error('Inference error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

type StreamInferenceInput = Omit<InferRequest, '$typeName' | '$unknown'>;

/**
 * Streaming inference server action using async generator.
 * Yields InferResponse events directly from the gRPC stream.
 * Client consumes via `for await...of` loop.
 */
async function* generateInference(input: StreamInferenceInput): AsyncGenerator<InferResponse, void, unknown> {
	const client = await inferenceServiceClient();
	if (!client) {
		throw new Error('Authentication required');
	}

	const req = create(InferRequestSchema, {
		agentPid: input.agentPid,
		conversationPid: input.conversationPid,
		message: input.message,
		requestId: input.requestId,
		documentPids: input.documentPids,
	});

	log.debug('generateInference starting', {
		agentPid: input.agentPid,
		conversationPid: input.conversationPid,
		requestId: input.requestId,
		documentPids: input.documentPids,
	});

	for await (const res of client.infer(req)) {
		// Log complete events which contain the final message with timestamp
		if (res.event.case === 'complete') {
			const msg = res.event.value.message;
			log.debug('generateInference complete event', {
				conversationPid: res.event.value.conversationPid,
				message: msg
					? {
							pid: msg.pid,
							role: msg.role,
							createdAt: msg.createdAt,
							partsCount: msg.parts.length,
						}
					: null,
			});
		}
		yield res;
	}
}

/**
 * Returns [`generateInference`]; workaround because Nextjs' Typescript plugin
 * requires anything exported from a file with the `use server;` directive
 * absolutely _must_ return a Promise.
 */
export async function streamInference(input: StreamInferenceInput) {
	return generateInference(input);
}

/**
 * Server action to cancel an in-progress inference request
 */
export async function cancelInference(
	requestId: string,
	conversationPid?: string,
): Promise<ActionResult<{ cancelled: boolean }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CancelInferenceRequestSchema, {
			requestId,
			conversationPid: conversationPid || '',
		});
		const res = await client.cancelInference(req);

		return {
			success: true,
			data: {
				cancelled: res.cancelled,
			},
		};
	} catch (error) {
		console.error('Cancel inference error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}
