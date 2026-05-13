'use client';

import { DocumentStatus } from '@polypixel/proto-ts/rig-service/rig/v1/document_pb';
import {
	type MessagePart,
	MessagePartKind,
	MessagePartStatus,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { createContext, type ReactNode, useCallback, useContext, useReducer } from 'react';
import { getTextContent, hasThinkingParts } from './message';

// Re-export message utilities for convenience (they're in a separate file for server compatibility)
export { getTextContent, hasThinkingParts };
export { DocumentStatus };
export { type MessagePart, MessagePartStatus, MessagePartKind };

export interface Message {
	id: string;
	role: 'user' | 'assistant';
	timestamp: Date;
	status: 'sending' | 'sent' | 'failed' | 'processing' | 'complete' | 'cancelled';
	attachments?: MessageAttachment[];
	parts: MessagePart[];
	// Cached flag for whether message has thinking content
	hasThinking?: boolean;
	// Timing information for thinking duration
	thinkingDuration?: number; // in seconds
}

export interface MessageAttachment {
	id: string;
	name: string;
	type: string;
	size: number;
	url?: string;
	status: DocumentStatus;
}

export interface ChatState {
	messages: Message[];
	isLoading: boolean;
	error?: string;
	sessionId?: string;
	assistantId: string | null;
	pendingFiles: File[];
}

export type ChatAction =
	| {
			type: 'ADD_MESSAGE';
			message: Omit<Message, 'id' | 'timestamp'>;
			id: string;
	  }
	| { type: 'UPDATE_MESSAGE'; id: string; updates: Partial<Message> }
	| { type: 'SET_MESSAGES'; messages: Message[] }
	| { type: 'DELETE_MESSAGE'; id: string }
	| { type: 'SET_LOADING'; loading: boolean }
	| { type: 'SET_ERROR'; error: string | null }
	| { type: 'SET_SESSION_ID'; sessionId: string }
	| { type: 'SET_ASSISTANT_ID'; assistantId: string }
	| { type: 'SET_PENDING_FILES'; files: File[] }
	| { type: 'CLEAR_CHAT' };

export const chatInitialState: ChatState = {
	messages: [],
	isLoading: false,
	assistantId: null,
	pendingFiles: [],
};

/**
 * Helper to enrich message with computed properties
 */
function enrichMessage(message: Message): Message {
	const parts = message.parts;

	// Only enrich assistant messages
	if (message.role !== 'assistant') {
		return message;
	}

	// Compute hasThinking from parts
	const hasThinking = hasThinkingParts(parts);

	const result: Message = {
		...message,
		parts,
		hasThinking,
	};

	// Handle thinking timing - estimate duration based on thinking content length
	if (hasThinking && !message.thinkingDuration && message.status === 'complete') {
		const thinkingText = parts
			.filter((part) => part.kind === MessagePartKind.THINKING)
			.map((part) => part.content)
			.join('');

		// Calculate estimated duration (min 2s, max 15s, roughly 1s per 200 chars)
		const estimatedDuration = Math.min(15, Math.max(2, Math.ceil(thinkingText.length / 200)));
		result.thinkingDuration = estimatedDuration;
	}

	return result;
}

export function chatReducer(state: ChatState, action: ChatAction): ChatState {
	switch (action.type) {
		case 'ADD_MESSAGE': {
			const baseMessage: Message = {
				...action.message,
				id: action.id,
				timestamp: new Date(),
			};
			const enrichedMessage = enrichMessage(baseMessage);
			return {
				...state,
				messages: [...state.messages, enrichedMessage],
			};
		}
		case 'UPDATE_MESSAGE': {
			// Check if the message exists and needs updating
			const messageIndex = state.messages.findIndex((msg) => msg.id === action.id);
			if (messageIndex === -1) {
				// Message not found, return state unchanged
				return state;
			}

			// Always update the message to ensure streaming updates are reflected
			const updatedMessages = state.messages.map((msg) => {
				if (msg.id === action.id) {
					const updatedMessage = { ...msg, ...action.updates };
					// Re-enrich if parts or status were updated
					if (action.updates.parts !== undefined || action.updates.status !== undefined) {
						return enrichMessage(updatedMessage);
					}
					return updatedMessage;
				}
				return msg;
			});
			return {
				...state,
				messages: updatedMessages,
			};
		}
		case 'SET_MESSAGES':
			return {
				...state,
				messages: action.messages.map((msg) => enrichMessage(msg)),
			};
		case 'DELETE_MESSAGE':
			return {
				...state,
				messages: state.messages.filter((msg) => msg.id !== action.id),
			};
		case 'SET_LOADING':
			return { ...state, isLoading: action.loading };
		case 'SET_ERROR':
			return { ...state, error: action.error ?? undefined };
		case 'SET_SESSION_ID':
			return { ...state, sessionId: action.sessionId };
		case 'SET_ASSISTANT_ID':
			return { ...state, assistantId: action.assistantId };
		case 'SET_PENDING_FILES':
			return { ...state, pendingFiles: action.files };
		case 'CLEAR_CHAT':
			return {
				...state,
				messages: [],
				isLoading: false,
				pendingFiles: [],
				sessionId: undefined,
			};
		default:
			return state;
	}
}

export interface ChatStateActions {
	state: ChatState;
	addMessage: (message: Omit<Message, 'id' | 'timestamp'>) => string;
	updateMessage: (id: string, updates: Partial<Message>) => void;
	deleteMessage: (id: string) => void;
	setMessages: (messages: Message[]) => void;
	setLoading: (loading: boolean) => void;
	setError: (error: string | null) => void;
	setSessionId: (sessionId: string) => void;
	setAssistantId: (assistantId: string) => void;
	setPendingFiles: (files: File[]) => void;
	clearChat: () => void;
}

/**
 * Creates memoized action creators from a dispatch function.
 * Shared between the global AssistantChatProvider and the local useChatState hook.
 */
export function useChatActions(dispatch: React.Dispatch<ChatAction>): Omit<ChatStateActions, 'state'> {
	const addMessage = useCallback(
		(message: Omit<Message, 'id' | 'timestamp'>) => {
			const id = `msg_${Date.now()}_${Math.random().toString(36).slice(2)}`;
			dispatch({ type: 'ADD_MESSAGE', message, id });
			return id;
		},
		[dispatch],
	);

	const updateMessage = useCallback(
		(id: string, updates: Partial<Message>) => {
			dispatch({ type: 'UPDATE_MESSAGE', id, updates });
		},
		[dispatch],
	);

	const deleteMessage = useCallback(
		(id: string) => {
			dispatch({ type: 'DELETE_MESSAGE', id });
		},
		[dispatch],
	);

	const setMessages = useCallback(
		(messages: Message[]) => {
			dispatch({ type: 'SET_MESSAGES', messages });
		},
		[dispatch],
	);

	const setLoading = useCallback(
		(loading: boolean) => {
			dispatch({ type: 'SET_LOADING', loading });
		},
		[dispatch],
	);

	const setError = useCallback(
		(error: string | null) => {
			dispatch({ type: 'SET_ERROR', error });
		},
		[dispatch],
	);

	const setSessionId = useCallback(
		(sessionId: string) => {
			dispatch({ type: 'SET_SESSION_ID', sessionId });
		},
		[dispatch],
	);

	const setAssistantId = useCallback(
		(assistantId: string) => {
			dispatch({ type: 'SET_ASSISTANT_ID', assistantId });
		},
		[dispatch],
	);

	const setPendingFiles = useCallback(
		(files: File[]) => {
			dispatch({ type: 'SET_PENDING_FILES', files });
		},
		[dispatch],
	);

	const clearChat = useCallback(() => {
		dispatch({ type: 'CLEAR_CHAT' });
	}, [dispatch]);

	return {
		addMessage,
		updateMessage,
		deleteMessage,
		setMessages,
		setLoading,
		setError,
		setSessionId,
		setAssistantId,
		setPendingFiles,
		clearChat,
	};
}

const AssistantChatContext = createContext<ChatStateActions | null>(null);

export function AssistantChatProvider({ children }: { children: ReactNode }) {
	const [state, dispatch] = useReducer(chatReducer, chatInitialState);
	const actions = useChatActions(dispatch);

	return <AssistantChatContext.Provider value={{ state, ...actions }}>{children}</AssistantChatContext.Provider>;
}

export function useAssistantChatState(): ChatStateActions {
	const context = useContext(AssistantChatContext);
	if (!context) {
		throw new Error('useAssistantChatState must be used within an AssistantChatProvider');
	}
	return context;
}

// TypeScript interface for chat sessions
export interface ChatSession {
	id: string;
	title: string;
	last_message: string;
	timestamp: string;
	agent_id: string;
}
