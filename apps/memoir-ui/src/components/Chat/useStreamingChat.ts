'use client';

import { deleteDocument, fetchDocument, uploadDocument } from '@actions/documents';
import { cancelInference, createConversation } from '@actions/infer';
import { create } from '@bufbuild/protobuf';
import { DocumentStatus, type MessageAttachment, type MessagePart } from '@lib/chat-state';
import { streamInferenceEvents } from '@lib/streaming';
import { streamingStore } from '@lib/streaming-store';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
	ToolCallSchema,
	ToolExecutionStatus,
	ToolResultSchema,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { useCallback, useRef, useState } from 'react';

type MessageStatus = 'sending' | 'sent' | 'failed' | 'processing' | 'complete' | 'cancelled';

export interface StreamingChatOptions {
	agentId: string | undefined;
	sessionId: string | undefined;
	addMessage: (message: {
		role: 'user' | 'assistant';
		status: MessageStatus;
		parts: MessagePart[];
		attachments?: MessageAttachment[];
	}) => string;
	updateMessage: (
		id: string,
		updates: Partial<{
			status: MessageStatus;
			parts: MessagePart[];
			hasThinking: boolean;
			attachments: MessageAttachment[];
		}>,
	) => void;
	setLoading: (loading: boolean) => void;
	setError: (error: string | null) => void;
	setSessionId: (sessionId: string) => void;
	onSessionCreated?: (sessionId: string) => void;
}

export interface SubmitOptions {
	prompt: string;
	files?: File[];
}

export interface UseStreamingChatReturn {
	submit: (options: SubmitOptions) => Promise<void>;
	stop: () => Promise<void>;
	isStreaming: boolean;
	retryAttachment: (messageId: string, attachmentId: string, currentAttachments: MessageAttachment[]) => Promise<void>;
	deleteAttachment: (messageId: string, attachmentId: string, currentAttachments: MessageAttachment[]) => Promise<void>;
}

/**
 * Upload a single file via the uploadDocument RPC, then poll until ingestion completes.
 * Returns true when the file reaches READY, false on any failure.
 */
async function uploadSingleFile(
	file: { name: string; type: string; content: Uint8Array },
	attachmentIndex: number,
	updateAttachment: (index: number, updates: Partial<MessageAttachment>) => void,
	conversationPid: string | undefined,
): Promise<boolean> {
	const result = await uploadDocument({
		filename: file.name,
		contentType: file.type,
		content: file.content,
		conversationPid,
	});

	if (!result.success || !result.data?.document) {
		updateAttachment(attachmentIndex, { status: DocumentStatus.FAILED });
		return false;
	}

	const { document } = result.data;
	updateAttachment(attachmentIndex, { id: document.pid, status: DocumentStatus.PROCESSING });

	// Poll until ingestion completes (READY or FAILED)
	const maxPollMs = 60_000;
	const pollIntervalMs = 500;
	const start = Date.now();

	while (Date.now() - start < maxPollMs) {
		await new Promise((r) => setTimeout(r, pollIntervalMs));
		const docResult = await fetchDocument(document.pid);
		if (!docResult.success || !docResult.data?.document) break;

		const docStatus = docResult.data.document.status;
		if (docStatus === DocumentStatus.READY || docStatus === DocumentStatus.FAILED) {
			updateAttachment(attachmentIndex, { status: docStatus });
			return docStatus === DocumentStatus.READY;
		}
	}

	// Timed out
	updateAttachment(attachmentIndex, { status: DocumentStatus.FAILED });
	return false;
}

export function useStreamingChat({
	agentId,
	sessionId,
	addMessage,
	updateMessage,
	setLoading,
	setError,
	setSessionId,
	onSessionCreated,
}: StreamingChatOptions): UseStreamingChatReturn {
	const [isStreaming, setIsStreaming] = useState(false);
	const requestIdRef = useRef<string | null>(null);
	const abortControllerRef = useRef<AbortController | null>(null);
	const isCancelledRef = useRef(false);
	const currentAssistantMessageIdRef = useRef<string | null>(null);

	// Retain pre-read file payloads across the submit → retry lifecycle
	type FilePayload = { name: string; type: string; content: Uint8Array };
	const fileMapRef = useRef<Map<string, FilePayload>>(new Map());
	// Store the prompt text for each user message so retry/delete can trigger inference
	const pendingPromptsRef = useRef<Map<string, string>>(new Map());

	// Use ref to track sessionId for cancellation to avoid stale closure
	const sessionIdRef = useRef(sessionId);
	sessionIdRef.current = sessionId;

	const stop = useCallback(async () => {
		isCancelledRef.current = true;
		setIsStreaming(false);
		setLoading(false);

		// Immediately mark the assistant message as cancelled in the UI,
		// flushing accumulated parts so the partial response stays visible.
		if (currentAssistantMessageIdRef.current) {
			const cancelledParts = streamingStore.flush();
			updateMessage(currentAssistantMessageIdRef.current, {
				status: 'cancelled',
				parts: cancelledParts,
				hasThinking: cancelledParts.some((p) => p.kind === MessagePartKind.THINKING),
			});
		}

		if (abortControllerRef.current) {
			abortControllerRef.current.abort();
			abortControllerRef.current = null;
		}

		if (requestIdRef.current) {
			cancelInference(requestIdRef.current, sessionIdRef.current).catch((err) => {
				console.error('Failed to cancel inference:', err);
			});
		}
	}, [setLoading, updateMessage]);

	/**
	 * Run the inference streaming phase for a given user message.
	 * Extracted so both submit() and retry/delete auto-triggers can reuse it.
	 */
	const runInference = useCallback(
		async (userMessageId: string, prompt: string, currentSessionId: string | undefined, documentPids?: string[]) => {
			if (!agentId) return;

			setLoading(true);
			setIsStreaming(true);
			setError(null);
			isCancelledRef.current = false;

			const requestId = `req_${Date.now()}_${Math.random().toString(36).slice(2)}`;
			requestIdRef.current = requestId;

			const abortController = new AbortController();
			abortControllerRef.current = abortController;

			try {
				const assistantMessageId = addMessage({
					role: 'assistant',
					status: 'processing',
					parts: [],
				});
				currentAssistantMessageIdRef.current = assistantMessageId;

				streamingStore.start();

				for await (const res of streamInferenceEvents(
					{
						agentPid: agentId,
						conversationPid: currentSessionId,
						message: prompt,
						requestId,
						documentPids,
					},
					abortController.signal,
				)) {
					if (isCancelledRef.current) {
						break;
					}

					switch (res.event.case) {
						case 'acknowledged': {
							const ack = res.event.value;
							updateMessage(userMessageId, { status: 'sent' });
							if (ack.conversationPid && !currentSessionId) {
								currentSessionId = ack.conversationPid;
								setSessionId(currentSessionId);
								onSessionCreated?.(currentSessionId);
							}
							break;
						}
						case 'partStart': {
							const startEvent = res.event.value;
							const newPart = create(MessagePartSchema, {
								id: startEvent.partId,
								kind: startEvent.kind,
								status: MessagePartStatus.STREAMING,
								content:
									startEvent.kind === MessagePartKind.TEXT || startEvent.kind === MessagePartKind.THINKING
										? ''
										: undefined,
								toolCall:
									startEvent.kind === MessagePartKind.TOOL_CALL
										? create(ToolCallSchema, {
												id: startEvent.partId,
												name: startEvent.toolName || '',
												arguments: {},
												status: ToolExecutionStatus.CALLING,
											})
										: undefined,
								toolResult:
									startEvent.kind === MessagePartKind.TOOL_RESULT
										? create(ToolResultSchema, {
												toolCallId: startEvent.toolCallId || '',
												result: '',
												status: ToolExecutionStatus.RUNNING,
											})
										: undefined,
							});
							streamingStore.addPart(newPart);
							break;
						}
						case 'partDelta': {
							const delta = res.event.value;
							if (delta.delta.case === 'content') {
								const contentValue = delta.delta.value;
								streamingStore.updateDelta(delta.partId, (part) => {
									if (part.kind === MessagePartKind.TOOL_RESULT && part.toolResult) {
										part.toolResult.result = (part.toolResult.result || '') + contentValue;
									} else {
										part.content = (part.content || '') + contentValue;
									}
								});
							} else if (delta.delta.case === 'arguments') {
								const argsValue = delta.delta.value;
								streamingStore.updateDelta(delta.partId, (part) => {
									if (part.toolCall) {
										const existingArgs = part.toolCall.arguments || {};
										part.toolCall.arguments = { ...existingArgs, ...argsValue };
									}
								});
							}
							break;
						}
						case 'partEnd': {
							const partEnd = res.event.value;
							streamingStore.endPart(partEnd.partId, partEnd.status, (part) => {
								if (part.toolCall) {
									part.toolCall.status =
										partEnd.status === MessagePartStatus.COMPLETE
											? ToolExecutionStatus.COMPLETED
											: ToolExecutionStatus.FAILED;
								}
								if (part.toolResult) {
									part.toolResult.status =
										partEnd.status === MessagePartStatus.COMPLETE
											? ToolExecutionStatus.COMPLETED
											: ToolExecutionStatus.FAILED;
								}
							});
							break;
						}
						case 'complete': {
							const complete = res.event.value;
							const finalParts = complete.message?.parts || streamingStore.flush();
							updateMessage(assistantMessageId, {
								status: 'complete',
								parts: finalParts,
								hasThinking: finalParts.some((p) => p.kind === MessagePartKind.THINKING),
							});
							if (complete.conversationPid && !currentSessionId) {
								currentSessionId = complete.conversationPid;
								setSessionId(currentSessionId);
								onSessionCreated?.(currentSessionId);
							}
							break;
						}
						case 'cancelled': {
							console.log('Received cancellation from backend:', res.event.value);
							const cancelledParts = streamingStore.flush();
							updateMessage(assistantMessageId, {
								status: 'cancelled',
								parts: cancelledParts,
								hasThinking: cancelledParts.some((p) => p.kind === MessagePartKind.THINKING),
							});
							break;
						}
					}
				}

				if (isCancelledRef.current) {
					const cancelledParts = streamingStore.flush();
					updateMessage(userMessageId, { status: 'sent' });
					updateMessage(assistantMessageId, {
						status: 'cancelled',
						parts: cancelledParts,
						hasThinking: cancelledParts.some((p) => p.kind === MessagePartKind.THINKING),
					});
				}
			} catch (error) {
				if (isCancelledRef.current) {
					// User-initiated cancellation — the error (e.g. "Controller is
					// already closed") is a side effect of the abort racing with the
					// stream close. Flush accumulated parts so the partial response
					// stays visible.
					const cancelledParts = streamingStore.flush();
					updateMessage(userMessageId, { status: 'sent' });
					if (currentAssistantMessageIdRef.current) {
						updateMessage(currentAssistantMessageIdRef.current, {
							status: 'cancelled',
							parts: cancelledParts,
							hasThinking: cancelledParts.some((p) => p.kind === MessagePartKind.THINKING),
						});
					}
				} else {
					console.error('Streaming error:', error);
					updateMessage(userMessageId, { status: 'failed' });
					setError(error instanceof Error ? error.message : 'Failed to send message');
				}
			} finally {
				streamingStore.reset();
				setLoading(false);
				setIsStreaming(false);
				requestIdRef.current = null;
				abortControllerRef.current = null;
				currentAssistantMessageIdRef.current = null;
				// Clean up pending prompt on successful inference completion
				pendingPromptsRef.current.delete(userMessageId);
			}
		},
		[agentId, addMessage, updateMessage, setLoading, setError, setSessionId, onSessionCreated],
	);

	const submit = useCallback(
		async ({ prompt, files }: SubmitOptions) => {
			if (!agentId || !prompt.trim()) return;

			// Show user message immediately with pending attachments
			const initialAttachments: MessageAttachment[] = (files ?? []).map((file) => ({
				id: `att_${Date.now()}_${Math.random().toString(36).slice(2)}`,
				name: file.name,
				type: file.type,
				size: file.size,
				status: DocumentStatus.PENDING,
			}));

			const userMessageId = addMessage({
				role: 'user',
				status: files && files.length > 0 ? 'sending' : 'sending',
				attachments: initialAttachments.length > 0 ? initialAttachments : undefined,
				parts: [
					create(MessagePartSchema, {
						id: `part_${Date.now()}`,
						kind: MessagePartKind.TEXT,
						status: MessagePartStatus.COMPLETE,
						content: prompt,
					}),
				],
			});

			// Store prompt for retry/delete lifecycle (file payloads stored after arrayBuffer below)
			if (files && files.length > 0) {
				pendingPromptsRef.current.set(userMessageId, prompt);
			}

			// Upload files and wait for ingestion before starting inference.
			// Use a shared mutable array so each concurrent upload can update its own
			// slot and trigger a UI refresh showing the latest status of all attachments.
			const attachments: MessageAttachment[] = [...initialAttachments];

			const updateAttachment = (index: number, updates: Partial<MessageAttachment>) => {
				// Re-key fileMapRef when the attachment gets a real server pid
				if (updates.id && updates.id !== attachments[index].id) {
					const file = fileMapRef.current.get(attachments[index].id);
					if (file) {
						fileMapRef.current.delete(attachments[index].id);
						fileMapRef.current.set(updates.id, file);
					}
				}
				attachments[index] = { ...attachments[index], ...updates };
				updateMessage(userMessageId, { attachments: [...attachments] });
			};

			// Eagerly create conversation before file uploads so documents
			// are automatically attached via conversation_pid.
			let currentSessionId = sessionId;
			if (files && files.length > 0 && !currentSessionId) {
				const convResult = await createConversation(agentId);
				if (convResult.success && convResult.data.conversation?.pid) {
					currentSessionId = convResult.data.conversation.pid;
					setSessionId(currentSessionId);
					onSessionCreated?.(currentSessionId);
				}
			}

			if (files && files.length > 0) {
				try {
					// Read file bytes upfront before any polling timers start
					const filePayloads = await Promise.all(
						files.map(async (file) => ({
							name: file.name,
							type: file.type,
							content: new Uint8Array(await file.arrayBuffer()),
						})),
					);

					// Store payloads for retry lifecycle
					for (let i = 0; i < filePayloads.length; i++) {
						fileMapRef.current.set(initialAttachments[i].id, filePayloads[i]);
					}

					await Promise.all(
						filePayloads.map(async (file, index) => {
							const success = await uploadSingleFile(file, index, updateAttachment, currentSessionId);
							// Clean up fileMapRef for files that reached READY
							if (success) {
								fileMapRef.current.delete(attachments[index].id);
							}
						}),
					);

					// Bail if any attachment failed
					if (attachments.some((a) => a.status === DocumentStatus.FAILED)) {
						setError('One or more files failed to process');
						updateMessage(userMessageId, { status: 'failed' });
						return;
					}

					// All files succeeded — clean up pending prompt (inference will set its own)
					// Don't delete yet; runInference's finally block handles cleanup
				} catch (error) {
					console.error('File upload failed:', error);
					setError('File upload failed');
					updateMessage(userMessageId, { status: 'failed' });
					return;
				}
			}

			const readyPids = attachments.filter((a) => a.status === DocumentStatus.READY).map((a) => a.id);

			await runInference(userMessageId, prompt, currentSessionId, readyPids.length > 0 ? readyPids : undefined);
		},
		[agentId, sessionId, addMessage, updateMessage, setError, setSessionId, onSessionCreated, runInference],
	);

	const retryAttachment = useCallback(
		async (messageId: string, attachmentId: string, currentAttachments: MessageAttachment[]) => {
			const file = fileMapRef.current.get(attachmentId);
			if (!file) {
				console.warn(`retryAttachment: no File found for attachment ${attachmentId}`);
				return;
			}

			const attachmentIndex = currentAttachments.findIndex((a) => a.id === attachmentId);
			if (attachmentIndex === -1) return;

			// Work with a mutable copy of attachments
			const attachments = [...currentAttachments];

			const updateAttachment = (index: number, updates: Partial<MessageAttachment>) => {
				// Re-key fileMapRef when the attachment gets a real server pid
				if (updates.id && updates.id !== attachments[index].id) {
					const f = fileMapRef.current.get(attachments[index].id);
					if (f) {
						fileMapRef.current.delete(attachments[index].id);
						fileMapRef.current.set(updates.id, f);
					}
				}
				attachments[index] = { ...attachments[index], ...updates };
				updateMessage(messageId, { attachments: [...attachments] });
			};

			// Reset to PENDING
			updateAttachment(attachmentIndex, { status: DocumentStatus.PENDING });

			const success = await uploadSingleFile(file, attachmentIndex, updateAttachment, sessionIdRef.current);
			if (success) {
				fileMapRef.current.delete(attachments[attachmentIndex].id);
			}

			// Check if all attachments are now non-FAILED
			const anyFailed = attachments.some((a) => a.status === DocumentStatus.FAILED);
			if (!anyFailed) {
				const prompt = pendingPromptsRef.current.get(messageId);
				if (prompt) {
					updateMessage(messageId, { status: 'sending' });
					await runInference(messageId, prompt, sessionIdRef.current);
				}
			}
		},
		[updateMessage, runInference],
	);

	const deleteAttachment = useCallback(
		async (messageId: string, attachmentId: string, currentAttachments: MessageAttachment[]) => {
			// Remove File ref
			fileMapRef.current.delete(attachmentId);

			// If it's a real server document (not a temp att_ id), delete server-side
			if (!attachmentId.startsWith('att_')) {
				deleteDocument(attachmentId).catch((err) => {
					console.error('Failed to delete document:', err);
				});
			}

			// Filter out the deleted attachment
			const remaining = currentAttachments.filter((a) => a.id !== attachmentId);
			updateMessage(messageId, {
				attachments: remaining.length > 0 ? remaining : (undefined as unknown as MessageAttachment[]),
			});

			// Check if we should auto-trigger inference
			const anyFailed = remaining.some((a) => a.status === DocumentStatus.FAILED);
			if (!anyFailed) {
				const prompt = pendingPromptsRef.current.get(messageId);
				if (prompt) {
					updateMessage(messageId, { status: 'sending' });
					await runInference(messageId, prompt, sessionIdRef.current);
				}
			}
		},
		[updateMessage, runInference],
	);

	return { submit, stop, isStreaming, retryAttachment, deleteAttachment };
}
