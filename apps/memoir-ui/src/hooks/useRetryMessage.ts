import { sendInferenceMessage } from '@actions/infer';
import { type ChatStateActions, getTextContent, type Message } from '@lib/chat-state';
import { useCallback } from 'react';

interface RetryOptions {
	chat: ChatStateActions;
	assistantId?: string;
	sessionId?: string;
	onRetryStart?: (messageId: string) => void;
	onRetrySuccess?: (messageId: string) => void;
	onRetryError?: (messageId: string, error: string) => void;
}

export default function useRetryMessage(options: RetryOptions) {
	const {
		chat: {
			state: { messages },
			updateMessage,
			addMessage,
			setError,
			setSessionId,
		},
	} = options;

	const retryMessage = useCallback(
		async (messageId: string) => {
			if (!options.assistantId) {
				console.warn('[useRetryMessage]', 'No assistant available');
				return;
			}

			const message = messages.find((m) => m.id === messageId);
			if (!message || message.role !== 'user') {
				console.error('Cannot retry: message not found or not a user message');
				return;
			}

			if (!options.assistantId) {
				console.error('Cannot retry: no assistant ID provided');
				return;
			}

			// Mark message as retrying
			updateMessage(messageId, { status: 'sending' });
			options.onRetryStart?.(messageId);

			try {
				const result = await sendInferenceMessage({
					agentPid: options.assistantId,
					conversationPid: options.sessionId,
					message: getTextContent(message.parts),
				});

				if (result.success) {
					// Update user message as sent
					updateMessage(messageId, { status: 'sent' });

					// Add new assistant response
					addMessage({
						role: 'assistant',
						status: 'complete',
						parts: result.data.message?.parts ?? [],
					});

					// Store thread ID if new
					if (result.data.conversationPid && !options.sessionId) {
						setSessionId(result.data.conversationPid);
					}

					options.onRetrySuccess?.(messageId);
				} else {
					throw new Error(result.error || 'Failed to retry message');
				}
			} catch (error) {
				// Mark message as failed again
				updateMessage(messageId, { status: 'failed' });
				const errorMessage = error instanceof Error ? error.message : 'Failed to retry message';
				setError(errorMessage);
				options.onRetryError?.(messageId, errorMessage);
			}
		},
		[messages, options, updateMessage, addMessage, setError, setSessionId],
	);

	const canRetry = useCallback(
		(message: Message) => {
			if (!options.assistantId) {
				return false;
			}
			return message.role === 'user' && message.status === 'failed';
		},
		[options.assistantId],
	);

	return {
		retryMessage,
		canRetry,
	};
}
