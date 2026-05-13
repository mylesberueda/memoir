import { create } from '@bufbuild/protobuf';
import type { Message } from '@lib/chat-state';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { act, renderHook, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import useCrossTabSync, { useTabVisibility } from './useCrossTabSync';

// Helper to create a properly typed mock message
function createMockMessage(overrides: Partial<Message> & { textContent?: string } = {}): Message {
	const { textContent = 'Test message', ...rest } = overrides;
	return {
		id: 'msg-1',
		role: 'user',
		status: 'complete',
		timestamp: new Date('2024-01-15T10:30:00Z'),
		parts: [
			create(MessagePartSchema, {
				id: 'part-1',
				kind: MessagePartKind.TEXT,
				content: textContent,
				status: MessagePartStatus.COMPLETE,
			}),
		],
		...rest,
	};
}

// Mock chat state actions (passed as argument to the hook)
const mockSetMessages = vi.fn();
const mockSetSessionId = vi.fn();
const mockChatActions = {
	state: {
		sessionId: null as null | string,
		messages: [] as Message[],
	},
	setMessages: mockSetMessages,
	setSessionId: mockSetSessionId,
} as unknown as import('@lib/chat-state').ChatStateActions;

// Mock localStorage
const mockLocalStorage = {
	getItem: vi.fn(),
	setItem: vi.fn(),
	removeItem: vi.fn(),
};
Object.defineProperty(window, 'localStorage', {
	value: mockLocalStorage,
	writable: true,
});

// Mock window events
const mockAddEventListener = vi.fn();
const mockRemoveEventListener = vi.fn();
const mockDispatchEvent = vi.fn();
Object.defineProperty(window, 'addEventListener', {
	value: mockAddEventListener,
	writable: true,
});
Object.defineProperty(window, 'removeEventListener', {
	value: mockRemoveEventListener,
	writable: true,
});
Object.defineProperty(window, 'dispatchEvent', {
	value: mockDispatchEvent,
	writable: true,
});

// Mock document
const mockDocumentAddEventListener = vi.fn();
const mockDocumentRemoveEventListener = vi.fn();
Object.defineProperty(document, 'addEventListener', {
	value: mockDocumentAddEventListener,
	writable: true,
});
Object.defineProperty(document, 'removeEventListener', {
	value: mockDocumentRemoveEventListener,
	writable: true,
});
Object.defineProperty(document, 'hidden', {
	value: false,
	writable: true,
});

describe('useCrossTabSync', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		// Reset mock state
		mockChatActions.state.sessionId = null;
		mockChatActions.state.messages = [] as Message[];
		// Reset mock functions
		mockSetMessages.mockClear();
		mockSetSessionId.mockClear();
		// Mock Date.now for consistent tab IDs
		vi.spyOn(Date, 'now').mockReturnValue(1642248000000);
		vi.spyOn(Math, 'random').mockReturnValue(0.123456789);
	});

	afterEach(() => {
		vi.resetAllMocks();
		vi.restoreAllMocks();
	});

	describe('initialization', () => {
		it('should generate unique tab ID on mount', async () => {
			const { result, rerender } = renderHook(() => useCrossTabSync(mockChatActions));

			// Initial render might have empty tabId, rerender to trigger useEffect
			rerender();

			// Tab ID should eventually be set
			await new Promise((resolve) => setTimeout(resolve, 0));
			rerender();

			expect(result.current.tabId).toMatch(/^tab_\d+_[a-z0-9]+$/);
		});

		it('should set up storage event listener', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			expect(mockAddEventListener).toHaveBeenCalledWith('storage', expect.any(Function));
		});

		it('should set up beforeunload event listener', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			expect(mockAddEventListener).toHaveBeenCalledWith('beforeunload', expect.any(Function));
		});

		it('should return isLeader as true', () => {
			const { result } = renderHook(() => useCrossTabSync(mockChatActions));

			expect(result.current.isLeader).toBe(true);
		});
	});

	describe('cleanup', () => {
		it('should remove event listeners on unmount', () => {
			const { unmount } = renderHook(() => useCrossTabSync(mockChatActions));

			unmount();

			expect(mockRemoveEventListener).toHaveBeenCalledWith('storage', expect.any(Function));
			expect(mockRemoveEventListener).toHaveBeenCalledWith('beforeunload', expect.any(Function));
		});
	});

	describe('message broadcasting', () => {
		it('should broadcast messages to localStorage when messages change', async () => {
			const testMessages: Message[] = [createMockMessage({ textContent: 'Test message' })];

			mockChatActions.state.messages = testMessages;
			mockChatActions.state.sessionId = 'session-123';

			renderHook(() => useCrossTabSync(mockChatActions));

			await waitFor(() => {
				expect(mockLocalStorage.setItem).toHaveBeenCalledWith(
					'chat_sync_data',
					expect.stringContaining('Test message'),
				);
			});

			// Verify the structure of saved data
			const [[, syncDataStr]] = mockLocalStorage.setItem.mock.calls;
			const syncData = JSON.parse(syncDataStr);
			expect(syncData.messages).toHaveLength(1);
			expect(syncData.messages[0].parts[0].content).toBe('Test message');
			expect(syncData.messages[0].timestamp).toBe('2024-01-15T10:30:00.000Z');
			expect(syncData.sessionId).toBe('session-123');
			expect(syncData.tabId).toMatch(/^tab_\d+_[a-z0-9]+$/);
			expect(syncData.timestamp).toBeTypeOf('number');
		});

		it('should not broadcast when messages are empty', () => {
			mockChatActions.state.messages = [];

			renderHook(() => useCrossTabSync(mockChatActions));

			expect(mockLocalStorage.setItem).not.toHaveBeenCalledWith('chat_sync_data', expect.anything());
		});
	});

	describe('message receiving', () => {
		it('should sync messages from other tabs', async () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			const otherTabData = {
				messages: [
					{
						id: 'msg-1',
						content: 'Message from other tab',
						role: 'user',
						timestamp: '2024-01-15T10:30:00Z',
						status: 'complete',
					},
				],
				sessionId: 'session-456',
				timestamp: Date.now() + 1000, // Newer timestamp
				tabId: 'other-tab-id',
			};

			// Simulate storage event from other tab
			const storageEvent = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: JSON.stringify(otherTabData),
			});

			// Get the storage event handler
			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			act(() => {
				storageHandler(storageEvent);
			});

			expect(mockSetMessages).toHaveBeenCalledWith([
				{
					id: 'msg-1',
					content: 'Message from other tab',
					role: 'user',
					timestamp: expect.any(Date),
					status: 'complete',
				},
			]);
			expect(mockSetSessionId).toHaveBeenCalledWith('session-456');
		});

		it('should not sync messages from same tab', async () => {
			const { result, rerender } = renderHook(() => useCrossTabSync(mockChatActions));

			// Wait for tab ID to be generated
			rerender();
			await new Promise((resolve) => setTimeout(resolve, 0));
			rerender();

			const sameTabData = {
				messages: [],
				sessionId: 'session-123',
				timestamp: Date.now(),
				tabId: result.current.tabId, // Same tab ID
			};

			const storageEvent = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: JSON.stringify(sameTabData),
			});

			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			act(() => {
				storageHandler(storageEvent);
			});

			expect(mockSetMessages).not.toHaveBeenCalled();
			expect(mockSetSessionId).not.toHaveBeenCalled();
		});

		it('should not sync older messages', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			// First, simulate receiving a message (sets lastSyncTimestamp)
			const newerData = {
				messages: [],
				sessionId: 'session-123',
				timestamp: Date.now(),
				tabId: 'other-tab-1',
			};

			const storageEvent1 = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: JSON.stringify(newerData),
			});

			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			act(() => {
				storageHandler(storageEvent1);
			});

			// Reset mocks
			mockSetMessages.mockClear();
			mockSetSessionId.mockClear();

			// Now simulate receiving older data
			const olderData = {
				messages: [{ id: 'old-msg', content: 'Old message' }],
				sessionId: 'old-session',
				timestamp: Date.now() - 10000, // Older timestamp
				tabId: 'other-tab-2',
			};

			const storageEvent2 = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: JSON.stringify(olderData),
			});

			act(() => {
				storageHandler(storageEvent2);
			});

			expect(mockSetMessages).not.toHaveBeenCalled();
			expect(mockSetSessionId).not.toHaveBeenCalled();
		});

		it('should handle malformed sync data gracefully', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			const storageEvent = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: 'invalid json',
			});

			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			expect(() => {
				act(() => {
					storageHandler(storageEvent);
				});
			}).not.toThrow();

			expect(mockSetMessages).not.toHaveBeenCalled();
		});

		it('should ignore non-sync storage events', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			const storageEvent = new StorageEvent('storage', {
				key: 'other_key',
				newValue: 'some value',
			});

			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			act(() => {
				storageHandler(storageEvent);
			});

			expect(mockSetMessages).not.toHaveBeenCalled();
		});

		it('should convert timestamp strings back to Date objects', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			const otherTabData = {
				messages: [
					{
						id: 'msg-1',
						content: 'Test',
						role: 'user',
						timestamp: '2024-01-15T10:30:00.000Z',
						status: 'complete',
					},
				],
				sessionId: 'session-123',
				timestamp: Date.now() + 1000,
				tabId: 'other-tab',
			};

			const storageEvent = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: JSON.stringify(otherTabData),
			});

			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			act(() => {
				storageHandler(storageEvent);
			});

			const [[messages]] = mockSetMessages.mock.calls;
			expect(messages[0].timestamp).toBeInstanceOf(Date);
			expect(messages[0].timestamp.toISOString()).toBe('2024-01-15T10:30:00.000Z');
		});

		it('should handle null sessionId in sync data', () => {
			// Set current sessionId to something non-null for comparison
			mockChatActions.state.sessionId = 'current-session';
			renderHook(() => useCrossTabSync(mockChatActions));

			const otherTabData = {
				messages: [
					{
						id: 'msg-1',
						content: 'Test message',
						role: 'user',
						timestamp: '2024-01-15T10:30:00.000Z',
						status: 'complete',
					},
				],
				sessionId: null, // This is the key: null sessionId
				timestamp: Date.now() + 1000,
				tabId: 'other-tab',
			};

			const storageEvent = new StorageEvent('storage', {
				key: 'chat_sync_data',
				newValue: JSON.stringify(otherTabData),
			});

			const [[, storageHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'storage');

			act(() => {
				storageHandler(storageEvent);
			});

			// Should call setSessionId with empty string (covers the || '' fallback)
			expect(mockSetSessionId).toHaveBeenCalledWith('');
		});
	});

	describe('tab cleanup', () => {
		it('should backup sync data on beforeunload if tab is current syncer', async () => {
			const { result, rerender } = renderHook(() => useCrossTabSync(mockChatActions));

			// Wait for tab ID to be generated
			rerender();
			await new Promise((resolve) => setTimeout(resolve, 0));
			rerender();

			const currentSyncData = {
				messages: [],
				sessionId: 'session-123',
				timestamp: Date.now(),
				tabId: result.current.tabId,
			};

			mockLocalStorage.getItem.mockReturnValue(JSON.stringify(currentSyncData));

			// Trigger beforeunload
			const [[, beforeUnloadHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'beforeunload');

			act(() => {
				beforeUnloadHandler();
			});

			expect(mockLocalStorage.setItem).toHaveBeenCalledWith('chat_sync_data_backup', JSON.stringify(currentSyncData));
		});

		it('should not backup sync data if not current tab', () => {
			renderHook(() => useCrossTabSync(mockChatActions));

			const otherTabSyncData = {
				messages: [],
				sessionId: 'session-123',
				timestamp: Date.now(),
				tabId: 'other-tab-id',
			};

			mockLocalStorage.getItem.mockReturnValue(JSON.stringify(otherTabSyncData));

			const [[, beforeUnloadHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'beforeunload');

			act(() => {
				beforeUnloadHandler();
			});

			expect(mockLocalStorage.setItem).not.toHaveBeenCalledWith('chat_sync_data_backup', expect.anything());
		});

		it('should handle errors during cleanup gracefully', async () => {
			const { result, rerender } = renderHook(() => useCrossTabSync(mockChatActions));

			// Wait for tab ID to be generated
			rerender();
			await new Promise((resolve) => setTimeout(resolve, 0));
			rerender();

			// Mock localStorage.getItem to return valid data with matching tabId, but JSON.parse to throw
			const tabId = result.current.tabId;
			mockLocalStorage.getItem.mockReturnValue(`{"messages":[],"sessionId":"test","timestamp":123,"tabId":"${tabId}"}`);
			// Mock JSON.parse to throw an error
			vi.spyOn(JSON, 'parse').mockImplementationOnce(() => {
				throw new Error('JSON parse error');
			});

			const [[, beforeUnloadHandler]] = mockAddEventListener.mock.calls.filter(([event]) => event === 'beforeunload');

			expect(() => {
				act(() => {
					beforeUnloadHandler();
				});
			}).not.toThrow();
		});
	});
});

describe('useTabVisibility', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mockChatActions.state = { sessionId: null, messages: [] };
	});

	it('should set up visibility change listener', () => {
		renderHook(() => useTabVisibility());

		expect(mockDocumentAddEventListener).toHaveBeenCalledWith('visibilitychange', expect.any(Function));
	});

	it('should remove visibility change listener on unmount', () => {
		const { unmount } = renderHook(() => useTabVisibility());

		unmount();

		expect(mockDocumentRemoveEventListener).toHaveBeenCalledWith('visibilitychange', expect.any(Function));
	});

	it('should trigger sync check when tab becomes visible', () => {
		renderHook(() => useTabVisibility());

		const syncData = JSON.stringify({
			messages: [],
			sessionId: 'session-123',
			timestamp: Date.now(),
			tabId: 'other-tab',
		});

		mockLocalStorage.getItem.mockReturnValue(syncData);
		Object.defineProperty(document, 'hidden', { value: false });

		// Get the visibility change handler
		const [[, visibilityHandler]] = mockDocumentAddEventListener.mock.calls;

		act(() => {
			visibilityHandler();
		});

		expect(mockDispatchEvent).toHaveBeenCalledWith(
			expect.objectContaining({
				type: 'storage',
				key: 'chat_sync_data',
				newValue: syncData,
			}),
		);
	});

	it('should not trigger sync when tab is hidden', () => {
		renderHook(() => useTabVisibility());

		Object.defineProperty(document, 'hidden', { value: true });

		const [[, visibilityHandler]] = mockDocumentAddEventListener.mock.calls;

		act(() => {
			visibilityHandler();
		});

		expect(mockDispatchEvent).not.toHaveBeenCalled();
	});

	it('should handle missing sync data gracefully', () => {
		renderHook(() => useTabVisibility());

		mockLocalStorage.getItem.mockReturnValue(null);
		Object.defineProperty(document, 'hidden', { value: false });

		const [[, visibilityHandler]] = mockDocumentAddEventListener.mock.calls;

		expect(() => {
			act(() => {
				visibilityHandler();
			});
		}).not.toThrow();

		expect(mockDispatchEvent).not.toHaveBeenCalled();
	});
});
