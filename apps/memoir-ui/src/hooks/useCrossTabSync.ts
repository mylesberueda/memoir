'use client';

import { useEffect, useRef } from 'react';
import type { ChatStateActions, Message } from '@/lib/chat-state';

interface SyncData {
	// Use serialized messages for localStorage sync
	messages: Array<Omit<Message, 'timestamp'> & { timestamp: string }>;
	sessionId: string | null;
	timestamp: number;
	tabId: string;
}

export default function useCrossTabSync(chat: ChatStateActions) {
	const { state, setMessages, setSessionId } = chat;
	const { messages, sessionId } = state;
	const tabIdRef = useRef<string>('');
	const lastSyncTimestampRef = useRef<number>(0);

	// Generate unique tab ID
	useEffect(() => {
		tabIdRef.current = `tab_${Date.now()}_${Math.random().toString(36).slice(2)}`;
	}, []);

	// Listen for storage changes from other tabs
	useEffect(() => {
		const handleStorageChange = (e: StorageEvent) => {
			if (e.key === 'chat_sync_data' && e.newValue && tabIdRef.current) {
				try {
					const syncData: SyncData = JSON.parse(e.newValue);

					// Don't sync with our own updates
					if (syncData.tabId === tabIdRef.current) {
						return;
					}

					// Only sync if the other tab's data is newer
					if (syncData.timestamp > lastSyncTimestampRef.current) {
						// Convert timestamp strings back to Date objects
						const syncedMessages = syncData.messages.map((msg) => ({
							...msg,
							timestamp: new Date(msg.timestamp),
						}));

						setMessages(syncedMessages);
						if (syncData.sessionId !== sessionId) {
							setSessionId(syncData.sessionId || '');
						}

						lastSyncTimestampRef.current = syncData.timestamp;

						console.log(`Synced ${syncedMessages.length} messages from another tab`);
					}
				} catch (error) {
					console.error('Failed to sync messages from other tab:', error);
				}
			}
		};

		window.addEventListener('storage', handleStorageChange);
		return () => window.removeEventListener('storage', handleStorageChange);
	}, [setMessages, setSessionId, sessionId]);

	// Broadcast changes to other tabs
	useEffect(() => {
		if (messages.length > 0 && tabIdRef.current) {
			const syncData: SyncData = {
				messages: messages.map((msg) => ({
					...msg,
					timestamp: msg.timestamp.toISOString(),
				})),
				sessionId: sessionId ?? null,
				timestamp: Date.now(),
				tabId: tabIdRef.current,
			};

			lastSyncTimestampRef.current = syncData.timestamp;
			localStorage.setItem('chat_sync_data', JSON.stringify(syncData));
		}
	}, [messages, sessionId]);

	// Clean up on tab close/refresh
	useEffect(() => {
		const handleBeforeUnload = () => {
			// Remove our sync data if we're the last tab
			const syncDataStr = localStorage.getItem('chat_sync_data');
			if (syncDataStr && tabIdRef.current) {
				try {
					const syncData: SyncData = JSON.parse(syncDataStr);
					if (syncData.tabId === tabIdRef.current) {
						// We're the last tab that was syncing, keep the data for recovery
						localStorage.setItem('chat_sync_data_backup', syncDataStr);
					}
				} catch (error) {
					console.error('Error during tab cleanup:', error);
				}
			}
		};

		window.addEventListener('beforeunload', handleBeforeUnload);
		return () => window.removeEventListener('beforeunload', handleBeforeUnload);
	}, []);

	return {
		tabId: tabIdRef.current,
		isLeader: true, // For now, all tabs can sync. Could implement leader election later
	};
}

// Hook to detect if user is switching between tabs
export function useTabVisibility() {
	useEffect(() => {
		const handleVisibilityChange = () => {
			if (!document.hidden) {
				// Tab became visible, check for any missed updates
				const syncDataStr = localStorage.getItem('chat_sync_data');
				if (syncDataStr) {
					// Trigger a re-sync by simulating a storage event
					const event = new StorageEvent('storage', {
						key: 'chat_sync_data',
						newValue: syncDataStr,
					});
					window.dispatchEvent(event);
				}
			}
		};

		document.addEventListener('visibilitychange', handleVisibilityChange);
		return () => document.removeEventListener('visibilitychange', handleVisibilityChange);
	}, []);
}
