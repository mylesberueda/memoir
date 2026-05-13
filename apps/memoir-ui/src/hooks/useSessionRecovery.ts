import { fetchConversationMessages } from '@actions/infer';
import type { ChatStateActions, Message } from '@lib/chat-state';
import type { Message as ProtoMessage } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import { MessagePartKind, MessageStatus } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import { useEffect, useRef, useState } from 'react';

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

export default function useSessionRecovery(chat: ChatStateActions) {
	const { state, setMessages, setSessionId } = chat;
	const { sessionId, messages } = state;
	const hasAttemptedRecovery = useRef(false);
	const [hasBackup, setHasBackup] = useState(false);
	const [isClient, setIsClient] = useState(false);

	// Set client-side flag on mount
	useEffect(() => {
		setIsClient(true);
		// Check for backup once client-side
		if (typeof window !== 'undefined') {
			try {
				setHasBackup(localStorage.getItem('chat_messages_backup') !== null);
			} catch (error) {
				console.error('Failed to check localStorage backup:', error);
				setHasBackup(false);
			}
		}
	}, []);

	useEffect(() => {
		// Prevent multiple recovery attempts
		if (hasAttemptedRecovery.current) {
			return;
		}

		const recoverSession = async () => {
			// Only run on client-side
			if (typeof window === 'undefined') return;

			// Check for session ID in localStorage first
			let storedSessionId: string | null = null;
			try {
				storedSessionId = localStorage.getItem('chat_session_id');
			} catch (error) {
				console.error('Failed to access localStorage for session recovery:', error);
				return;
			}

			if (storedSessionId && !sessionId && messages.length === 0) {
				try {
					// Try to fetch session history from API using server action
					const result = await fetchConversationMessages(storedSessionId);

					if (result.success && result.data.messages) {
						if (Array.isArray(result.data.messages)) {
							// Convert API messages to chat state format (parts are already proto type)
							const restoredMessages = result.data.messages.map((msg: ProtoMessage) => {
								// Deduplicate parts by ID (keep first occurrence)
								const seenIds = new Set<string>();
								const uniqueParts = msg.parts.filter((part) => {
									if (seenIds.has(part.id)) {
										console.warn('Filtering duplicate part during recovery:', part.id, part.kind);
										return false;
									}
									seenIds.add(part.id);
									return true;
								});

								return {
									id: `msg_${msg.pid}`,
									role: msg.role as 'user' | 'assistant',
									status: mapProtoStatus(msg.status),
									timestamp: new Date(msg.createdAt),
									parts: uniqueParts,
									hasThinking: uniqueParts.some((p) => p.kind === MessagePartKind.THINKING),
								};
							});

							setMessages(restoredMessages);
							setSessionId(storedSessionId);

							console.log(`Recovered session ${storedSessionId} with ${restoredMessages.length} messages`);
						}
					} else if (!result.success) {
						// Session not found or error occurred, clean up localStorage
						console.error('Failed to recover session:', result.error);
						try {
							localStorage.removeItem('chat_session_id');
						} catch (error) {
							console.error('Failed to remove session ID from localStorage:', error);
						}
					}
				} catch (error) {
					console.error('Failed to recover session:', error);
					// Remove invalid session ID from localStorage
					try {
						localStorage.removeItem('chat_session_id');
					} catch (localStorageError) {
						console.error('Failed to remove session ID from localStorage:', localStorageError);
					}
				}
			}
		};

		// Only attempt recovery once on mount
		hasAttemptedRecovery.current = true;
		recoverSession();
	}, [sessionId, messages.length, setMessages, setSessionId]);

	// Save session ID to localStorage when it changes (client-side only)
	useEffect(() => {
		if (typeof window === 'undefined') return;

		try {
			if (sessionId) {
				localStorage.setItem('chat_session_id', sessionId);
			} else {
				localStorage.removeItem('chat_session_id');
			}
		} catch (error) {
			console.error('Failed to save session ID to localStorage:', error);
		}
	}, [sessionId]);

	// Auto-save messages to localStorage for quick recovery (client-side only)
	useEffect(() => {
		if (typeof window === 'undefined') return;

		try {
			if (messages.length > 0) {
				const messagesData = {
					messages: messages.map((msg) => ({
						...msg,
						timestamp: msg.timestamp.toISOString(),
					})),
					lastUpdated: new Date().toISOString(),
				};
				localStorage.setItem('chat_messages_backup', JSON.stringify(messagesData));
				setHasBackup(true);
			} else {
				localStorage.removeItem('chat_messages_backup');
				setHasBackup(false);
			}
		} catch (error) {
			console.error('Failed to save messages backup to localStorage:', error);
			setHasBackup(false);
		}
	}, [messages]);

	return {
		isRecovered: !!sessionId,
		hasBackup,
		isClient,
	};
}
