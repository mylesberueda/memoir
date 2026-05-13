'use client';

import type { UserAssistantResponse } from '@actions/agents';
import { getUserAssistant, updateAgent } from '@actions/agents';
import { deleteConversation, fetchConversationMessages, fetchConversations } from '@actions/infer';
import type { ListModelsResponse } from '@actions/models';
import { getModels } from '@actions/models';
import {
	ChatLayout,
	type ConversationListItem,
	transformMessagesToChatProps,
	useStreamingChat,
} from '@components/Chat';
import type { PromptBoxForm, PromptInputRef } from '@components/Chat/PromptInput';
import useCrossTabSync from '@hooks/useCrossTabSync';
import useRetryMessage from '@hooks/useRetryMessage';
import useToast from '@hooks/useToast';
import {
	type ChatSession,
	getTextContent,
	type Message,
	MessagePartKind,
	useAssistantChatState,
} from '@lib/chat-state';
import {
	type Conversation as ConversationProto,
	MessageStatus,
	type Message as ProtoMessage,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { useCallback, useEffect, useMemo, useRef, useState, useTransition } from 'react';

function mapProtoStatus(status: MessageStatus): Message['status'] {
	switch (status) {
		case MessageStatus.CANCELLED:
			return 'cancelled';
		case MessageStatus.ERROR:
			return 'failed';
		default:
			return 'complete';
	}
}

interface AssistantPageProps {
	initialSessions?: ChatSession[];
	initialAssistant?: UserAssistantResponse;
	initialModels?: ListModelsResponse;
}

export default function AssistantPage({ initialSessions = [], initialAssistant, initialModels }: AssistantPageProps) {
	const [selected, setSelected] = useState<ConversationListItem>();
	const [isUpdatingModel, setIsUpdatingModel] = useState(false);
	const [assistant, setAssistant] = useState<UserAssistantResponse | undefined>(initialAssistant);
	const [sessions, setSessions] = useState<ChatSession[]>(initialSessions);
	const [models, setModels] = useState<ListModelsResponse | undefined>(initialModels);
	const [modelsLoading, setModelsLoading] = useState(false);
	const [modelsError, setModelsError] = useState<string | null>(null);
	const [assistantLoading, setAssistantLoading] = useState(false);
	const [assistantError, setAssistantError] = useState<string | null>(null);
	const [_isPending, startTransition] = useTransition();

	const chatInputRef = useRef<PromptInputRef>(null);
	const toast = useToast();
	const chat = useAssistantChatState();
	const {
		state: { messages, isLoading, error, sessionId },
		addMessage,
		updateMessage,
		setMessages,
		setLoading,
		setError,
		setSessionId,
		setAssistantId,
		clearChat,
	} = chat;

	useCrossTabSync(chat);

	const refreshSessions = useCallback(() => {
		startTransition(() => {
			fetchConversations({
				agentPid: assistant?.agent.identifier.value,
			})
				.then((result) => {
					if (result.success && result.data.conversations) {
						const mappedSessions: ChatSession[] = result.data.conversations.map((thread: ConversationProto) => {
							const lastMessage =
								thread.messages.length > 0 ? getTextContent(thread.messages[thread.messages.length - 1].parts) : '';

							return {
								id: thread.pid,
								title: thread.title || 'Untitled',
								last_message: lastMessage,
								timestamp: thread.lastMessageAt
									? new Date(thread.lastMessageAt).toISOString()
									: new Date().toISOString(),
								agent_id: thread.agentPid,
							};
						});
						setSessions(mappedSessions);
					} else if (!result.success && result.error) {
						console.error('Failed to refresh sessions:', result.error);
					}
				})
				.catch((error) => {
					console.error('Failed to refresh sessions:', error);
				});
		});
	}, [assistant?.agent.identifier.value]);

	const { submit, stop, isStreaming, retryAttachment, deleteAttachment } = useStreamingChat({
		agentId: assistant?.agent?.identifier.value,
		sessionId,
		addMessage,
		updateMessage,
		setLoading,
		setError,
		setSessionId,
		onSessionCreated: refreshSessions,
	});

	useEffect(() => {
		setError(null);
	}, [setError]);

	useEffect(() => {
		if (assistant?.agent) {
			setAssistantId(assistant.agent.identifier.value);
		}
	}, [assistant, setAssistantId]);

	useEffect(() => {
		if (!initialAssistant) {
			setAssistantLoading(true);
			getUserAssistant()
				.then((result) => {
					if (result.success) {
						setAssistant(result.data);
						setAssistantId(result.data.agent.identifier.value);
					} else {
						console.error('Failed to load assistant:', result.error);
						setAssistantError(result.error);
					}
				})
				.catch((error) => {
					console.error('Failed to load assistant:', error);
					setAssistantError(error instanceof Error ? error.message : 'Failed to load assistant');
				})
				.finally(() => {
					setAssistantLoading(false);
				});
		}
	}, [initialAssistant, setAssistantId]);

	useEffect(() => {
		if (!initialModels) {
			setModelsLoading(true);
			getModels()
				.then((result) => {
					if (result.success) {
						setModels(result.data);
					} else {
						console.error('Failed to load models:', result.error);
						setModelsError(result.error);
					}
				})
				.finally(() => {
					setModelsLoading(false);
				});
		}
	}, [initialModels]);

	const handleModelChange = useCallback(
		async (modelId: string) => {
			if (!assistant?.agent || !modelId || modelId === assistant.agent.model?.pid) {
				return;
			}

			setIsUpdatingModel(true);
			setError(null);

			try {
				await updateAgent(
					{
						pid: assistant.agent.identifier.value,
						modelPid: modelId,
					},
					[],
				);

				const result = await getUserAssistant();
				if (result.success) {
					setAssistant(result.data);
					const modelName = models?.models.find((m) => m.identifier.value === modelId)?.name ?? modelId;
					toast.success(`Agent "${assistant.agent.name}" updated to use ${modelName}`);
				} else {
					toast.error(result.error);
				}
			} catch (error) {
				console.error('Failed to update agent model:', error);
				toast.error(error instanceof Error ? error.message : 'Failed to update agent model');
			} finally {
				setIsUpdatingModel(false);
			}
		},
		[assistant, models, setError, toast],
	);

	const conversations: ConversationListItem[] = useMemo(
		() =>
			sessions.map((session) => ({
				id: session.id,
				title: session.title,
				lastMessage: session.last_message,
				timestamp: new Date(session.timestamp),
				messages: [],
			})),
		[sessions],
	);

	const { retryMessage, canRetry } = useRetryMessage({
		chat,
		assistantId: assistant?.agent?.identifier.value,
		sessionId,
	});

	const chatMessages = useMemo(
		() =>
			transformMessagesToChatProps({
				messages,
				assistantName: assistant?.agent?.name || 'Assistant',
				canRetry,
				onRetry: retryMessage,
			}),
		[messages, assistant?.agent?.name, canRetry, retryMessage],
	);

	const handleRetry = useCallback(
		(messageId: string) => {
			retryMessage(messageId);
		},
		[retryMessage],
	);

	const handleCopy = useCallback((_messageId: string, content: string) => {
		navigator.clipboard.writeText(content);
	}, []);

	const handleFeedback = useCallback((messageId: string, type: 'like' | 'dislike') => {
		console.log('Feedback:', type, 'for message:', messageId);
	}, []);

	const handleRetryAttachment = useCallback(
		(messageId: string, attachmentId: string) => {
			const msg = messages.find((m) => m.id === messageId);
			retryAttachment(messageId, attachmentId, msg?.attachments ?? []);
		},
		[messages, retryAttachment],
	);

	const handleDeleteAttachment = useCallback(
		(messageId: string, attachmentId: string) => {
			const msg = messages.find((m) => m.id === messageId);
			deleteAttachment(messageId, attachmentId, msg?.attachments ?? []);
		},
		[messages, deleteAttachment],
	);

	const handleOnSubmit = useCallback(
		async (form: PromptBoxForm) => {
			await submit({ prompt: form.prompt, files: form.files });
		},
		[submit],
	);

	const handleOnNewConversation = useCallback(() => {
		clearChat();
		setSelected(undefined);
		refreshSessions();
		setTimeout(() => {
			chatInputRef.current?.focus();
		}, 0);
	}, [clearChat, refreshSessions]);

	const handleOnSelectConversation = useCallback(
		async (conversation: ConversationListItem) => {
			setSelected(conversation);
			setSessionId(conversation.id);
			setLoading(true);
			setError(null);

			try {
				const result = await fetchConversationMessages(conversation.id);

				if (!result.success) {
					throw new Error(result.error || 'Failed to load conversation messages');
				}

				const messagesData = result.data.messages || [];
				const loadedMessages = messagesData.map((msg: ProtoMessage) => ({
					id: msg.pid,
					role: msg.role as 'user' | 'assistant',
					status: mapProtoStatus(msg.status),
					timestamp: new Date(msg.createdAt),
					parts: msg.parts,
					hasThinking: msg.parts.some((p) => p.kind === MessagePartKind.THINKING),
				}));

				setMessages(loadedMessages);
			} catch (error) {
				console.error('Failed to load conversation messages:', error);
				setError('Failed to load conversation messages');
				setMessages([]);
			} finally {
				setLoading(false);
			}
		},
		[setSessionId, setLoading, setError, setMessages],
	);

	const handleOnDeleteConversation = useCallback(
		async (conversationId: string) => {
			try {
				const result = await deleteConversation(conversationId);
				if (!result.success) {
					toast.error(result.error || 'Failed to delete conversation');
					return;
				}

				if (selected?.id === conversationId) {
					clearChat();
					setSelected(undefined);
				}

				refreshSessions();
				toast.success('Conversation deleted');
			} catch (error) {
				console.error('Failed to delete conversation:', error);
				toast.error(error instanceof Error ? error.message : 'Failed to delete conversation');
			}
		},
		[selected?.id, clearChat, refreshSessions, toast],
	);

	if (assistantLoading) {
		return (
			<div className="flex h-full items-center justify-center">
				<div className="text-center">
					<div className="loading loading-spinner loading-lg" />
					<p className="mt-2">Loading assistant...</p>
				</div>
			</div>
		);
	}

	if (assistantError && !assistant) {
		return (
			<div className="flex h-full items-center justify-center">
				<div className="text-center">
					<p className="text-error">Failed to load assistant</p>
					<p className="mt-1 text-sm text-gray-500">{assistantError}</p>
				</div>
			</div>
		);
	}

	return (
		<ChatLayout
			chatRef={chatInputRef}
			error={error}
			sidebar={{
				title: assistant?.agent?.name || 'Assistant',
				conversations,
				selectedConversation: selected,
				onNewConversation: handleOnNewConversation,
				onSelectConversation: handleOnSelectConversation,
				onDeleteConversation: handleOnDeleteConversation,
			}}
			chatProps={{
				id: 'assistant_chat',
				messages: chatMessages,
				onSubmit: handleOnSubmit,
				disabled: isLoading || isStreaming || isUpdatingModel || !assistant?.agent?.identifier.value,
				isLoading,
				isStreaming,
				onStopStreaming: stop,
				showLoading: true,
				enableFileUpload: true,
				enableWebSearch: true,
				models: models?.models,
				modelsLoading,
				modelsError: modelsError ? new Error(modelsError) : undefined,
				currentAssistantModel: assistant?.agent?.model?.pid,
				onModelChange: handleModelChange,
				onRetry: handleRetry,
				onCopy: handleCopy,
				onFeedback: handleFeedback,
				onRetryAttachment: handleRetryAttachment,
				onDeleteAttachment: handleDeleteAttachment,
			}}
		/>
	);
}
