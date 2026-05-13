'use client';

import type { Agent } from '@actions/agents';
import { getAgent, updateAgent } from '@actions/agents';
import { deleteConversation, fetchConversationMessages, fetchConversations } from '@actions/infer';
import type { ListModelsResponse } from '@actions/models';
import { getOrgMembers } from '@actions/organizations';
import type { ListProvidersResponse } from '@actions/providers';
import type { ListToolsResponse } from '@actions/tools';
import {
	ChatLayout,
	type ConversationListItem,
	transformMessagesToChatProps,
	useChatState,
	useStreamingChat,
} from '@components/Chat';
import type { PromptBoxForm, PromptInputRef } from '@components/Chat/PromptInput';
import AgentModal, { type AgentFormData } from '@components/Modal/AgentModal';
import ShareModal from '@components/Modal/ShareModal/ShareModal';
import useCrossTabSync from '@hooks/useCrossTabSync';
import useRetryMessage from '@hooks/useRetryMessage';
import useToast from '@hooks/useToast';
import { type ChatSession, getTextContent, type Message, MessagePartKind } from '@lib/chat-state';
import { useOrganizations } from '@providers/OrganizationContextProvider';
import type { OrganizationMember } from '@polypixel/proto-ts/api-service/api/v1/organizations_pb';
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

interface AgentConversationClientProps {
	agent: Agent;
	initialSessions?: ChatSession[];
	initialSessionId?: string;
	initialModels?: ListModelsResponse;
	initialProviders?: ListProvidersResponse;
	initialTools?: ListToolsResponse;
}

function updateUrl(agentPid: string, sessionId?: string) {
	const url = sessionId ? `/conversations/${agentPid}?session=${sessionId}` : `/conversations/${agentPid}`;
	window.history.replaceState(window.history.state, '', url);
}

export default function AgentConversationClient({
	agent: initialAgent,
	initialSessions = [],
	initialSessionId,
	initialModels,
	initialProviders,
	initialTools,
}: AgentConversationClientProps) {
	const { currentOrgPid } = useOrganizations();
	const [selected, setSelected] = useState<ConversationListItem>();
	const [isUpdatingModel, setIsUpdatingModel] = useState(false);
	const [agent, setAgent] = useState<Agent>(initialAgent);
	const [sessions, setSessions] = useState<ChatSession[]>(initialSessions);
	const [models] = useState<ListModelsResponse | undefined>(initialModels);
	const [modelsLoading] = useState(false);
	const [modelsError] = useState<string | null>(null);
	const [isAgentModalOpen, setIsAgentModalOpen] = useState(false);
	const [isShareModalOpen, setIsShareModalOpen] = useState(false);
	const [orgMembers, setOrgMembers] = useState<OrganizationMember[]>([]);
	const [isAgentUpdating, startAgentUpdateTransition] = useTransition();
	const [_isPending, startTransition] = useTransition();

	const chatInputRef = useRef<PromptInputRef>(null);
	const toast = useToast();
	const chat = useChatState();
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

	// Set agent ID on mount
	useEffect(() => {
		setAssistantId(agent.identifier.value);
	}, [agent.identifier.value, setAssistantId]);

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

	// Load the initial session from the server-provided prop (from ?session= query param).
	// biome-ignore lint/correctness/useExhaustiveDependencies: This only runs on mount — after that, the URL is write-only from this component.
	useEffect(() => {
		if (initialSessionId) {
			const session = sessions.find((s) => s.id === initialSessionId);
			if (session) {
				handleOnSelectConversation({
					id: session.id,
					title: session.title,
					lastMessage: session.last_message,
					timestamp: new Date(session.timestamp),
					messages: [],
				});
			}
		}
	}, []);

	const refreshSessions = useCallback(() => {
		startTransition(() => {
			fetchConversations({ agentPid: agent.identifier.value })
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
	}, [agent.identifier.value]);

	const { submit, stop, isStreaming } = useStreamingChat({
		agentId: agent.identifier.value,
		sessionId,
		addMessage,
		updateMessage,
		setLoading,
		setError,
		setSessionId,
		onSessionCreated: (newSessionId: string) => {
			updateUrl(agent.identifier.value, newSessionId);
			setSelected({
				id: newSessionId,
				title: 'New conversation',
				lastMessage: '',
				timestamp: new Date(),
				messages: [],
			});
			refreshSessions();
		},
	});

	useEffect(() => {
		setError(null);
	}, [setError]);

	const handleModelChange = useCallback(
		async (modelId: string) => {
			if (!modelId || modelId === agent.model?.pid) {
				return;
			}

			setIsUpdatingModel(true);
			setError(null);

			try {
				const result = await updateAgent({ pid: agent.identifier.value, modelPid: modelId }, []);
				if (result.success) {
					setAgent(result.data);
					const modelName = models?.models.find((m) => m.identifier.value === modelId)?.name ?? modelId;
					toast.success(`Agent "${agent.name}" updated to use ${modelName}`);
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
		[agent, models, setError, toast],
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
		assistantId: agent.identifier.value,
		sessionId,
	});

	const chatMessages = useMemo(
		() =>
			transformMessagesToChatProps({
				messages,
				assistantName: agent.name || 'Agent',
				canRetry,
				onRetry: retryMessage,
			}),
		[messages, agent.name, canRetry, retryMessage],
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

	const handleOnSubmit = useCallback(
		async (form: PromptBoxForm) => {
			await submit({ prompt: form.prompt, files: form.files });

			if (!sessionId) {
				console.log(sessionId);
			}
		},
		[submit, sessionId],
	);

	const handleOnNewConversation = useCallback(() => {
		clearChat();
		setSelected(undefined);
		updateUrl(agent.identifier.value);

		setTimeout(() => {
			chatInputRef.current?.focus();
		}, 0);
	}, [clearChat, agent.identifier.value]);

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
					updateUrl(agent.identifier.value);
				}

				refreshSessions();
				toast.success('Conversation deleted');
			} catch (error) {
				console.error('Failed to delete conversation:', error);
				toast.error(error instanceof Error ? error.message : 'Failed to delete conversation');
			}
		},
		[selected?.id, clearChat, refreshSessions, toast, agent.identifier.value],
	);

	const handleAgentModalSubmit = useCallback(
		async (data: AgentFormData) => {
			startAgentUpdateTransition(async () => {
				const result = await updateAgent({
					pid: agent.identifier.value,
					name: data.name,
					modelPid: data.model,
					systemPrompt: data.system_prompt || '',
					tools: data.tools?.map((pid) => ({ pid, isActive: true })),
					providerPid: data.provider_id || undefined,
				});

				if (!result.success) {
					toast.error(`Failed to update agent: ${result.error}`);
					return;
				}

				// Refresh agent data
				const refreshed = await getAgent(agent.identifier.value);
				if (refreshed.success) {
					setAgent(refreshed.data);
				}

				toast.success('Agent updated successfully');
				setIsAgentModalOpen(false);
			});
		},
		[agent.identifier.value, toast],
	);

	const handleShareAgent = useCallback(async () => {
		if (!currentOrgPid) return;
		const result = await getOrgMembers(currentOrgPid);
		if (result.success) {
			setOrgMembers(result.data.members);
		}
		setIsShareModalOpen(true);
	}, [currentOrgPid]);

	return (
		<>
			<ChatLayout
				chatRef={chatInputRef}
				error={error}
				sidebar={{
					title: agent.name,
					conversations,
					selectedConversation: selected,
					onNewConversation: handleOnNewConversation,
					onSelectConversation: (conversation: ConversationListItem) => {
						updateUrl(agent.identifier.value, conversation.id);
						handleOnSelectConversation(conversation);
					},
					onDeleteConversation: handleOnDeleteConversation,
					onEditAgent: () => setIsAgentModalOpen(true),
					onShareAgent: handleShareAgent,
				}}
				chatProps={{
					id: 'agent_chat',
					messages: chatMessages,
					onSubmit: handleOnSubmit,
					disabled: isLoading || isStreaming || isUpdatingModel,
					isLoading,
					isStreaming,
					onStopStreaming: stop,
					showLoading: true,
					enableFileUpload: true,
					enableWebSearch: true,
					models: models?.models,
					modelsLoading,
					modelsError: modelsError ? new Error(modelsError) : undefined,
					currentAssistantModel: agent.model?.pid,
					onModelChange: handleModelChange,
					onRetry: handleRetry,
					onCopy: handleCopy,
					onFeedback: handleFeedback,
				}}
			/>

			<AgentModal
				isOpen={isAgentModalOpen}
				onClose={() => setIsAgentModalOpen(false)}
				mode="edit"
				agent={agent}
				onSubmit={handleAgentModalSubmit}
				isSubmitting={isAgentUpdating}
				models={initialModels ? { success: true, data: initialModels } : undefined}
				providers={initialProviders ? { success: true, data: initialProviders } : undefined}
				tools={initialTools ? { success: true, data: initialTools } : undefined}
			/>

			<ShareModal
				isOpen={isShareModalOpen}
				onClose={() => setIsShareModalOpen(false)}
				resourceType="agent"
				resourcePid={agent.identifier.value}
				ownerUserId={agent.createdByUserId}
				members={orgMembers}
			/>
		</>
	);
}
