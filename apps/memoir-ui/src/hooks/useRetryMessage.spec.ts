import { create } from '@bufbuild/protobuf';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import useRetryMessage from './useRetryMessage';

type Message = {
	id: string;
	role: 'user' | 'assistant';
	status: 'sending' | 'sent' | 'failed' | 'complete';
	timestamp: Date;
	parts: ReturnType<typeof create<typeof MessagePartSchema>>[];
};

// Mock dependencies
const mockSendInferenceMessage = vi.fn();
const mockUpdateMessage = vi.fn();
const mockAddMessage = vi.fn();
const mockSetError = vi.fn();
const mockSetSessionId = vi.fn();

vi.mock('@actions/infer', () => ({
	sendInferenceMessage: (...args: unknown[]) => mockSendInferenceMessage(...args),
}));

vi.mock('@lib/chat-state', () => ({
	getTextContent: (parts?: { content?: string }[]) => {
		if (!parts || parts.length === 0) return '';
		return parts.map((p) => p.content || '').join('');
	},
}));

let mockMessages: Message[] = [];

// Create mock ChatStateActions object to pass via options.chat
const mockChatActions = {
	get state() {
		return { messages: mockMessages };
	},
	updateMessage: mockUpdateMessage,
	addMessage: mockAddMessage,
	setError: mockSetError,
	setSessionId: mockSetSessionId,
} as unknown as import('@lib/chat-state').ChatStateActions;

function createMockMessage(overrides: Partial<Message> = {}): Message {
	return {
		id: 'msg-1',
		role: 'user',
		status: 'failed',
		timestamp: new Date(),
		parts: [
			create(MessagePartSchema, {
				id: 'part-1',
				kind: MessagePartKind.TEXT,
				content: 'Test message',
				status: MessagePartStatus.COMPLETE,
			}),
		],
		...overrides,
	};
}

describe('useRetryMessage', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mockMessages = [createMockMessage()];
	});

	describe('canRetry', () => {
		it('should return true for failed user messages when assistant is available', () => {
			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			const message = createMockMessage({ role: 'user', status: 'failed' });
			expect(result.current.canRetry(message)).toBe(true);
		});

		it('should return false for assistant messages', () => {
			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			const message = createMockMessage({ role: 'assistant', status: 'failed' });
			expect(result.current.canRetry(message)).toBe(false);
		});

		it('should return false for sent user messages', () => {
			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			const message = createMockMessage({ role: 'user', status: 'sent' });
			expect(result.current.canRetry(message)).toBe(false);
		});

		it('should return false for sending user messages', () => {
			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			const message = createMockMessage({ role: 'user', status: 'sending' });
			expect(result.current.canRetry(message)).toBe(false);
		});

		it('should return false when no assistant is available', () => {
			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions }));

			const message = createMockMessage({ role: 'user', status: 'failed' });
			expect(result.current.canRetry(message)).toBe(false);
		});
	});

	describe('retryMessage', () => {
		it('should mark message as sending when retry starts', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response', conversationPid: 'thread-1' },
			});

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockUpdateMessage).toHaveBeenCalledWith('msg-1', { status: 'sending' });
		});

		it('should call sendChatMessage with correct parameters', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response', conversationPid: 'thread-1' },
			});

			const { result } = renderHook(() =>
				useRetryMessage({
					chat: mockChatActions,
					assistantId: 'assistant-123',
					sessionId: 'session-456',
				}),
			);

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockSendInferenceMessage).toHaveBeenCalledWith({
				agentPid: 'assistant-123',
				conversationPid: 'session-456',
				message: 'Test message',
			});
		});

		it('should mark message as sent on success', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response', conversationPid: 'thread-1' },
			});

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockUpdateMessage).toHaveBeenCalledWith('msg-1', { status: 'sent' });
		});

		it('should add assistant response on success', async () => {
			const mockParts = [
				create(MessagePartSchema, {
					id: 'part-1',
					kind: MessagePartKind.TEXT,
					content: 'Response text',
					status: MessagePartStatus.COMPLETE,
				}),
			];
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: {
					conversationPid: 'thread-1',
					message: { parts: mockParts },
				},
			});

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockAddMessage).toHaveBeenCalledWith({
				role: 'assistant',
				status: 'complete',
				parts: mockParts,
			});
		});

		it('should set session ID when new thread is created', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response', conversationPid: 'new-thread-123' },
			});

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockSetSessionId).toHaveBeenCalledWith('new-thread-123');
		});

		it('should not set session ID when session already exists', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response', conversationPid: 'thread-1' },
			});

			const { result } = renderHook(() =>
				useRetryMessage({
					chat: mockChatActions,
					assistantId: 'assistant-123',
					sessionId: 'existing-session',
				}),
			);

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockSetSessionId).not.toHaveBeenCalled();
		});

		it('should mark message as failed on error', async () => {
			mockSendInferenceMessage.mockRejectedValue(new Error('Network error'));

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockUpdateMessage).toHaveBeenCalledWith('msg-1', { status: 'failed' });
		});

		it('should set error message on failure', async () => {
			mockSendInferenceMessage.mockRejectedValue(new Error('Network error'));

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockSetError).toHaveBeenCalledWith('Network error');
		});

		it('should mark message as failed when API returns success: false', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: false,
				error: 'Server error',
			});

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockUpdateMessage).toHaveBeenCalledWith('msg-1', { status: 'failed' });
			expect(mockSetError).toHaveBeenCalledWith('Server error');
		});

		it('should not retry when assistant ID is missing', async () => {
			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockSendInferenceMessage).not.toHaveBeenCalled();
			expect(mockUpdateMessage).not.toHaveBeenCalled();
		});

		it('should not retry when message is not found', async () => {
			mockMessages = [];

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('nonexistent-msg');
			});

			expect(mockSendInferenceMessage).not.toHaveBeenCalled();
		});

		it('should not retry assistant messages', async () => {
			mockMessages = [createMockMessage({ id: 'msg-1', role: 'assistant' })];

			const { result } = renderHook(() => useRetryMessage({ chat: mockChatActions, assistantId: 'assistant-123' }));

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(mockSendInferenceMessage).not.toHaveBeenCalled();
		});

		it('should call onRetryStart callback when retry begins', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response' },
			});

			const onRetryStart = vi.fn();
			const { result } = renderHook(() =>
				useRetryMessage({
					chat: mockChatActions,
					assistantId: 'assistant-123',
					onRetryStart,
				}),
			);

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(onRetryStart).toHaveBeenCalledWith('msg-1');
		});

		it('should call onRetrySuccess callback on success', async () => {
			mockSendInferenceMessage.mockResolvedValue({
				success: true,
				data: { content: 'Response' },
			});

			const onRetrySuccess = vi.fn();
			const { result } = renderHook(() =>
				useRetryMessage({
					chat: mockChatActions,
					assistantId: 'assistant-123',
					onRetrySuccess,
				}),
			);

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(onRetrySuccess).toHaveBeenCalledWith('msg-1');
		});

		it('should call onRetryError callback on failure', async () => {
			mockSendInferenceMessage.mockRejectedValue(new Error('Network error'));

			const onRetryError = vi.fn();
			const { result } = renderHook(() =>
				useRetryMessage({
					chat: mockChatActions,
					assistantId: 'assistant-123',
					onRetryError,
				}),
			);

			await act(async () => {
				await result.current.retryMessage('msg-1');
			});

			expect(onRetryError).toHaveBeenCalledWith('msg-1', 'Network error');
		});
	});
});
