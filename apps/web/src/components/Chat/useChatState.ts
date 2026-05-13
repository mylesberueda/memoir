'use client';

import { type ChatStateActions, chatInitialState, chatReducer, useChatActions } from '@lib/chat-state';
import { useReducer } from 'react';

/**
 * Local (page-scoped) chat state hook. Uses the same reducer and interface
 * as the global AssistantChatProvider, but state lives in a local useReducer
 * and is discarded on unmount. Use this for agent conversation pages.
 */
export function useChatState(): ChatStateActions {
	const [state, dispatch] = useReducer(chatReducer, chatInitialState);
	const actions = useChatActions(dispatch);
	return { state, ...actions };
}
