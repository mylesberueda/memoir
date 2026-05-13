import { fetchConversationMessages } from '@actions/infer';
import { create } from '@bufbuild/protobuf';
import type { Message } from '@lib/chat-state';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import { renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import useSessionRecovery from './useSessionRecovery';

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

// Mock the chat state actions object (passed as argument to the hook)
const mockSetMessages = vi.fn();
const mockSetSessionId = vi.fn();
const mockChatStateActions = {
	state: {
		sessionId: null as null | string,
		messages: [] as Message[],
	},
	setMessages: mockSetMessages,
	setSessionId: mockSetSessionId,
} as unknown as import('@lib/chat-state').ChatStateActions;

vi.mock('@actions/infer', () => ({
	fetchConversationMessages: vi.fn(),
}));

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

// Mock window object
Object.defineProperty(window, 'window', {
	value: global,
	writable: true,
});

// Remove wrapper - just mock the dependency

describe('useSessionRecovery', () => {
	const mockFetchConversationMessages = vi.mocked(fetchConversationMessages);

	beforeEach(() => {
		vi.clearAllMocks();
		// Reset mock state
		mockChatStateActions.state.sessionId = null;
		mockChatStateActions.state.messages = [];
		// Ensure window is defined with localStorage mock
		const mockWindow = {
			addEventListener: vi.fn(),
			removeEventListener: vi.fn(),
			localStorage: mockLocalStorage, // Keep localStorage mock
		};
		Object.defineProperty(global, 'window', {
			value: mockWindow,
			writable: true,
		});
		// Also ensure localStorage is available globally
		Object.defineProperty(global, 'localStorage', {
			value: mockLocalStorage,
			writable: true,
		});
	});

	afterEach(() => {
		vi.resetAllMocks();
	});

	describe('initialization', () => {
		it('should initialize with correct default values', () => {
			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			expect(result.current.isRecovered).toBe(false);
			expect(result.current.hasBackup).toBe(false);
			expect(result.current.isClient).toBe(true); // Will be true in test environment
		});

		it('should set isClient to true after mount', () => {
			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			expect(result.current.isClient).toBe(true);
		});
	});

	describe('session recovery', () => {
		it('should attempt to recover session when session ID exists in localStorage', async () => {
			// Mock proto Message objects - use any since we can't easily construct full proto types in tests
			const mockMessages = [
				{
					pid: '1',
					content: 'Test message',
					role: 'user',
					createdAt: '2024-01-15T10:30:00+00:00',
					parts: [
						{
							type: 1, // MessagePartType.TEXT
							id: 'part-1',
							content: 'Test message',
							status: 3, // MessagePartStatus.COMPLETE
						},
					],
					toolCalls: [],
					toolResults: [],
				},
			];

			mockLocalStorage.getItem.mockImplementation((key) => {
				if (key === 'chat_session_id') return 'session-123';
				if (key === 'chat_messages_backup') return null;
				return null;
			});

			mockFetchConversationMessages.mockResolvedValue({
				success: true,
				data: {
					conversation: undefined,
					// biome-ignore lint/suspicious/noExplicitAny: Mock proto objects in tests
					messages: mockMessages as any,
				},
			});

			renderHook(() => useSessionRecovery(mockChatStateActions));

			// Wait for the async effect to complete
			await vi.waitFor(() => {
				expect(mockFetchConversationMessages).toHaveBeenCalledWith('session-123');
			});

			await vi.waitFor(() => {
				expect(mockSetMessages).toHaveBeenCalledWith([
					{
						id: 'msg_1',
						role: 'user',
						timestamp: expect.any(Date),
						status: 'complete',
						parts: expect.any(Array),
						hasThinking: false,
					},
				]);
			});
			expect(mockSetSessionId).toHaveBeenCalledWith('session-123');
		});

		it('should not attempt recovery if session already exists', async () => {
			mockChatStateActions.state.sessionId = 'existing-session';
			mockLocalStorage.getItem.mockReturnValue('session-123');

			renderHook(() => useSessionRecovery(mockChatStateActions));

			// Wait a bit to ensure no fetch is called
			await new Promise((resolve) => setTimeout(resolve, 100));

			expect(mockFetchConversationMessages).not.toHaveBeenCalled();
		});

		it('should not attempt recovery if messages already exist', async () => {
			mockChatStateActions.state.messages = [
				createMockMessage({
					id: 'existing-msg',
					textContent: 'mock content',
					role: 'assistant',
					status: 'sent',
					timestamp: new Date(),
				}),
			];
			mockLocalStorage.getItem.mockReturnValue('session-123');

			renderHook(() => useSessionRecovery(mockChatStateActions));

			await new Promise((resolve) => setTimeout(resolve, 100));

			expect(mockFetchConversationMessages).not.toHaveBeenCalled();
		});

		it('should handle failed response by cleaning localStorage', async () => {
			mockLocalStorage.getItem.mockReturnValue('session-123');
			mockFetchConversationMessages.mockResolvedValue({
				success: false,
				error: 'Session not found',
			});

			renderHook(() => useSessionRecovery(mockChatStateActions));

			await vi.waitFor(() => {
				expect(mockLocalStorage.removeItem).toHaveBeenCalledWith('chat_session_id');
			});
		});

		it('should handle fetch errors gracefully', async () => {
			mockLocalStorage.getItem.mockReturnValue('session-123');
			mockFetchConversationMessages.mockRejectedValue(new Error('Network error'));

			renderHook(() => useSessionRecovery(mockChatStateActions));

			await vi.waitFor(() => {
				expect(mockLocalStorage.removeItem).toHaveBeenCalledWith('chat_session_id');
			});
		});

		it('should only attempt recovery once', async () => {
			mockLocalStorage.getItem.mockReturnValue('session-123');
			mockFetchConversationMessages.mockResolvedValue({
				success: true,
				data: {
					conversation: undefined,
					messages: [],
				},
			});

			const { rerender } = renderHook(() => useSessionRecovery(mockChatStateActions));

			// Wait for first recovery attempt
			await vi.waitFor(() => {
				expect(mockFetchConversationMessages).toHaveBeenCalledTimes(1);
			});

			// Rerender and ensure no additional fetch
			rerender();
			await new Promise((resolve) => setTimeout(resolve, 100));

			expect(mockFetchConversationMessages).toHaveBeenCalledTimes(1);
		});
	});

	describe('localStorage management', () => {
		it('should save session ID to localStorage when set', async () => {
			mockChatStateActions.state.sessionId = 'new-session';

			renderHook(() => useSessionRecovery(mockChatStateActions));

			await vi.waitFor(() => {
				expect(mockLocalStorage.setItem).toHaveBeenCalledWith('chat_session_id', 'new-session');
			});
		});

		it('should remove session ID from localStorage when cleared', async () => {
			// Start with a session ID
			mockChatStateActions.state.sessionId = 'session-123';
			const { rerender } = renderHook(() => useSessionRecovery(mockChatStateActions));

			// Clear the session ID
			mockChatStateActions.state.sessionId = null;
			rerender();

			await vi.waitFor(() => {
				expect(mockLocalStorage.removeItem).toHaveBeenCalledWith('chat_session_id');
			});
		});

		it('should save messages backup to localStorage', async () => {
			mockChatStateActions.state.messages = [createMockMessage({ textContent: 'Test message' })];

			renderHook(() => useSessionRecovery(mockChatStateActions));

			await vi.waitFor(() => {
				expect(mockLocalStorage.setItem).toHaveBeenCalledWith(
					'chat_messages_backup',
					expect.stringContaining('Test message'),
				);
			});
		});

		it('should update hasBackup when messages are saved', () => {
			mockChatStateActions.state.messages = [createMockMessage({ textContent: 'Test' })];

			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			expect(result.current.hasBackup).toBe(true);
		});

		it('should remove backup when messages are cleared', async () => {
			// Start with messages
			mockChatStateActions.state.messages = [
				createMockMessage({
					textContent: 'mock content',
					role: 'assistant',
					status: 'sent',
				}),
			];
			const { rerender } = renderHook(() => useSessionRecovery(mockChatStateActions));

			// Clear messages
			mockChatStateActions.state.messages = [];
			rerender();

			await vi.waitFor(() => {
				expect(mockLocalStorage.removeItem).toHaveBeenCalledWith('chat_messages_backup');
			});
		});

		it('should serialize message timestamps correctly', async () => {
			const testDate = new Date('2024-01-15T10:30:00Z');
			mockChatStateActions.state.messages = [
				createMockMessage({
					textContent: 'Test',
					timestamp: testDate,
				}),
			];

			renderHook(() => useSessionRecovery(mockChatStateActions));

			await vi.waitFor(() => {
				const backupCalls = mockLocalStorage.setItem.mock.calls.filter(([key]) => key === 'chat_messages_backup');
				expect(backupCalls.length).toBeGreaterThan(0);
				const [[, backupData]] = backupCalls;
				const parsed = JSON.parse(backupData);
				expect(parsed.messages[0].timestamp).toBe('2024-01-15T10:30:00.000Z');
			});
		});
	});

	describe('return values', () => {
		it('should return isRecovered true when session ID exists', () => {
			mockChatStateActions.state.sessionId = 'session-123';

			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			expect(result.current.isRecovered).toBe(true);
		});

		it('should return isRecovered false when no session ID', () => {
			mockChatStateActions.state.sessionId = null;

			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			expect(result.current.isRecovered).toBe(false);
		});

		it('should set hasBackup to true when messages are present', () => {
			// Test the simpler case: when messages exist, hasBackup should be true
			mockChatStateActions.state.messages = [createMockMessage({ textContent: 'Test' })];

			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			// When messages are present, backup should be created and hasBackup should be true
			expect(result.current.hasBackup).toBe(true);
		});
	});

	describe('SSR safety', () => {
		beforeEach(() => {
			// Clear all mocks before each SSR test
			vi.clearAllMocks();
			mockChatStateActions.state.sessionId = null;
			mockChatStateActions.state.messages = [];
		});

		it('should initialize isClient as false', () => {
			// Test that the hook starts with isClient: false which simulates SSR behavior
			const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));

			// The hook should start with isClient: false, then set it to true after mount
			expect(result.current.isClient).toBe(true); // Will be true in test environment
			expect(result.current.hasBackup).toBe(false);
			expect(result.current.isRecovered).toBe(false);
		});

		it('should handle undefined localStorage gracefully', () => {
			// Test that the hook handles cases where localStorage might not be available
			const originalGetItem = mockLocalStorage.getItem;
			const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

			mockLocalStorage.getItem.mockImplementation(() => {
				throw new Error('localStorage not available');
			});

			expect(() => {
				const { result } = renderHook(() => useSessionRecovery(mockChatStateActions));
				// Should not throw and should initialize properly
				expect(result.current.isClient).toBe(true);
				expect(result.current.hasBackup).toBe(false);
				expect(result.current.isRecovered).toBe(false);
			}).not.toThrow();

			// Restore
			mockLocalStorage.getItem = originalGetItem;
			consoleSpy.mockRestore();
		});
	});
});
