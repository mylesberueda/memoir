import { create } from '@bufbuild/protobuf';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { act, renderHook } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { AssistantChatProvider, hasThinkingParts, type Message, useAssistantChatState } from './chat-state';

// Helper to create MessagePart objects
function createTextPart(id: string, content: string): ReturnType<typeof create<typeof MessagePartSchema>> {
	return create(MessagePartSchema, {
		id,
		kind: MessagePartKind.TEXT,
		content,
		status: MessagePartStatus.COMPLETE,
	});
}

function createThinkingPart(id: string, content: string): ReturnType<typeof create<typeof MessagePartSchema>> {
	return create(MessagePartSchema, {
		id,
		kind: MessagePartKind.THINKING,
		content,
		status: MessagePartStatus.COMPLETE,
	});
}

// Wrapper for rendering hooks with AssistantChatProvider
function wrapper({ children }: { children: React.ReactNode }) {
	return <AssistantChatProvider>{children}</AssistantChatProvider>;
}

describe('hasThinkingParts', () => {
	it('should return true when parts contain thinking type', () => {
		const parts = [createTextPart('1', 'Hello'), createThinkingPart('2', 'Thinking...')];
		expect(hasThinkingParts(parts)).toBe(true);
	});

	it('should return false when no thinking parts exist', () => {
		const parts = [createTextPart('1', 'Hello'), createTextPart('2', 'World')];
		expect(hasThinkingParts(parts)).toBe(false);
	});

	it('should return false for empty array', () => {
		expect(hasThinkingParts([])).toBe(false);
	});

	it('should return false for undefined', () => {
		expect(hasThinkingParts(undefined)).toBe(false);
	});
});

describe('useAssistantChatState', () => {
	it('should throw error when used outside ChatProvider', () => {
		// Suppress console.error for this test
		const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

		expect(() => {
			renderHook(() => useAssistantChatState());
		}).toThrow('useAssistantChatState must be used within an AssistantChatProvider');

		consoleSpy.mockRestore();
	});

	it('should provide initial state', () => {
		const { result } = renderHook(() => useAssistantChatState(), { wrapper });

		expect(result.current.state).toEqual({
			messages: [],
			isLoading: false,
			assistantId: null,
			pendingFiles: [],
		});
	});

	describe('addMessage', () => {
		it('should add a user message', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sending',
				});
			});

			expect(result.current.state.messages).toHaveLength(1);
			expect(result.current.state.messages[0].parts[0].content).toBe('Hello');
			expect(result.current.state.messages[0].role).toBe('user');
			expect(result.current.state.messages[0].status).toBe('sending');
		});

		it('should return the generated message ID', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			let messageId: string | undefined;
			act(() => {
				messageId = result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sending',
				});
			});

			if (!messageId) {
				expect(true).toBe(false);
			}

			expect(messageId).toMatch(/^msg_\d+_[a-z0-9]+$/);
			expect(result.current.state.messages[0].id).toBe(messageId);
		});

		it('should set timestamp on message', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			const before = new Date();
			act(() => {
				result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sending',
				});
			});
			const after = new Date();

			const timestamp = result.current.state.messages[0].timestamp;
			expect(timestamp.getTime()).toBeGreaterThanOrEqual(before.getTime());
			expect(timestamp.getTime()).toBeLessThanOrEqual(after.getTime());
		});

		it('should enrich assistant messages with hasThinking', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.addMessage({
					parts: [createThinkingPart('1', 'Thinking about this...')],
					role: 'assistant',
					status: 'complete',
				});
			});

			expect(result.current.state.messages[0].hasThinking).toBe(true);
		});

		it('should not set hasThinking for user messages', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sent',
				});
			});

			expect(result.current.state.messages[0].hasThinking).toBeUndefined();
		});

		it('should calculate thinkingDuration for completed assistant messages with thinking', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			// Create a long thinking content to test duration calculation
			const longThinking = 'A'.repeat(600); // Should result in ~3 seconds (600/200)

			act(() => {
				result.current.addMessage({
					parts: [createThinkingPart('1', longThinking)],
					role: 'assistant',
					status: 'complete',
				});
			});

			expect(result.current.state.messages[0].thinkingDuration).toBe(3);
		});
	});

	describe('updateMessage', () => {
		it('should update an existing message', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			let messageId: string | undefined;
			act(() => {
				messageId = result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sending',
				});
			});

			act(() => {
				if (!messageId) {
					expect(messageId).toBeDefined();
					return;
				}
				result.current.updateMessage(messageId, { status: 'sent' });
			});

			expect(result.current.state.messages[0].status).toBe('sent');
		});

		it('should update message parts and re-enrich', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			let messageId: string;
			act(() => {
				messageId = result.current.addMessage({
					parts: [],
					role: 'assistant',
					status: 'processing',
				});
			});

			act(() => {
				if (!messageId) {
					expect(messageId).toBeDefined();
					return;
				}

				result.current.updateMessage(messageId, {
					parts: [createThinkingPart('1', 'Thinking...')],
					status: 'complete',
				});
			});

			expect(result.current.state.messages[0].hasThinking).toBe(true);
		});

		it('should not update if message does not exist', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sent',
				});
			});

			const messagesBefore = result.current.state.messages;

			act(() => {
				result.current.updateMessage('nonexistent-id', { status: 'failed' });
			});

			// State should be unchanged (same reference)
			expect(result.current.state.messages).toBe(messagesBefore);
		});
	});

	describe('deleteMessage', () => {
		it('should remove a message by ID', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			let messageId: string;
			act(() => {
				messageId = result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sent',
				});
			});

			expect(result.current.state.messages).toHaveLength(1);

			act(() => {
				if (!messageId) {
					expect(messageId).toBeDefined();
					return;
				}

				result.current.deleteMessage(messageId);
			});

			expect(result.current.state.messages).toHaveLength(0);
		});

		it('should not affect other messages', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			let messageId1: string;
			let messageId2: string | undefined;
			act(() => {
				messageId1 = result.current.addMessage({
					parts: [createTextPart('1', 'First')],
					role: 'user',
					status: 'sent',
				});
				messageId2 = result.current.addMessage({
					parts: [createTextPart('2', 'Second')],
					role: 'user',
					status: 'sent',
				});
			});

			act(() => {
				if (!messageId1) {
					expect(messageId1).toBeDefined();
					return;
				}

				result.current.deleteMessage(messageId1);
			});

			if (!messageId2) {
				expect(messageId2).toBeDefined();
				return;
			}

			expect(result.current.state.messages).toHaveLength(1);
			expect(result.current.state.messages[0].id).toBe(messageId2);
		});
	});

	describe('setMessages', () => {
		it('should replace all messages', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			const newMessages: Message[] = [
				{
					id: 'msg-1',
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sent',
					timestamp: new Date(),
				},
				{
					id: 'msg-2',
					parts: [createTextPart('2', 'Hi there')],
					role: 'assistant',
					status: 'complete',
					timestamp: new Date(),
				},
			];

			act(() => {
				result.current.setMessages(newMessages);
			});

			expect(result.current.state.messages).toHaveLength(2);
			expect(result.current.state.messages[0].parts[0].content).toBe('Hello');
			expect(result.current.state.messages[1].parts[0].content).toBe('Hi there');
		});

		it('should enrich all messages', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			const newMessages: Message[] = [
				{
					id: 'msg-1',
					parts: [createThinkingPart('1', 'Thinking...')],
					role: 'assistant',
					status: 'complete',
					timestamp: new Date(),
				},
			];

			act(() => {
				result.current.setMessages(newMessages);
			});

			expect(result.current.state.messages[0].hasThinking).toBe(true);
		});
	});

	describe('setLoading', () => {
		it('should set loading state to true', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setLoading(true);
			});

			expect(result.current.state.isLoading).toBe(true);
		});

		it('should set loading state to false', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setLoading(true);
			});

			act(() => {
				result.current.setLoading(false);
			});

			expect(result.current.state.isLoading).toBe(false);
		});
	});

	describe('setError', () => {
		it('should set error message', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setError('Something went wrong');
			});

			expect(result.current.state.error).toBe('Something went wrong');
		});

		it('should clear error when set to null', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setError('Error');
			});

			act(() => {
				result.current.setError(null);
			});

			expect(result.current.state.error).toBeUndefined();
		});
	});

	describe('setSessionId', () => {
		it('should set session ID', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setSessionId('session-123');
			});

			expect(result.current.state.sessionId).toBe('session-123');
		});
	});

	describe('setAssistantId', () => {
		it('should set assistant ID', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setAssistantId('assistant-456');
			});

			expect(result.current.state.assistantId).toBe('assistant-456');
		});
	});

	describe('setPendingFiles', () => {
		it('should set pending files', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			const files = [new File(['content'], 'test.txt', { type: 'text/plain' })];

			act(() => {
				result.current.setPendingFiles(files);
			});

			expect(result.current.state.pendingFiles).toHaveLength(1);
			expect(result.current.state.pendingFiles[0].name).toBe('test.txt');
		});
	});

	describe('clearChat', () => {
		it('should clear all messages', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.addMessage({
					parts: [createTextPart('1', 'Hello')],
					role: 'user',
					status: 'sent',
				});
				result.current.addMessage({
					parts: [createTextPart('2', 'Hi')],
					role: 'assistant',
					status: 'complete',
				});
			});

			expect(result.current.state.messages).toHaveLength(2);

			act(() => {
				result.current.clearChat();
			});

			expect(result.current.state.messages).toHaveLength(0);
		});

		it('should reset loading state', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setLoading(true);
			});

			act(() => {
				result.current.clearChat();
			});

			expect(result.current.state.isLoading).toBe(false);
		});

		it('should clear pending files', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			const files = [new File(['content'], 'test.txt', { type: 'text/plain' })];

			act(() => {
				result.current.setPendingFiles(files);
			});

			act(() => {
				result.current.clearChat();
			});

			expect(result.current.state.pendingFiles).toHaveLength(0);
		});

		it('should clear sessionId to start a new session', () => {
			const { result } = renderHook(() => useAssistantChatState(), { wrapper });

			act(() => {
				result.current.setSessionId('session-123');
				result.current.setAssistantId('assistant-456');
			});

			act(() => {
				result.current.clearChat();
			});

			expect(result.current.state.sessionId).toBeUndefined();
			expect(result.current.state.assistantId).toBe('assistant-456');
		});
	});
});
