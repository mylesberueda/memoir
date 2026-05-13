import type { UserAssistantResponse } from '@actions/agents';
import type { ListModelsResponse } from '@actions/models';
import { create } from '@bufbuild/protobuf';
import { type MessagePart, MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { MessagePartSchema } from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import AssistantPage from './client';

// ============================================================================
// Mock Setup
// ============================================================================

// Mock actions
const mockGetUserAssistantAction = vi.fn();
const mockUpdateAgentAction = vi.fn();
const mockCancelInference = vi.fn();
const mockDeleteConversation = vi.fn();
const mockFetchConversationMessages = vi.fn();
const mockFetchConversations = vi.fn();
const mockGetModelsData = vi.fn();

vi.mock('@actions/agents', () => ({
	getUserAssistant: () => mockGetUserAssistantAction(),
	updateAgent: (...args: unknown[]) => mockUpdateAgentAction(...args),
}));

vi.mock('@actions/infer', () => ({
	cancelInference: (...args: unknown[]) => mockCancelInference(...args),
	deleteConversation: (...args: unknown[]) => mockDeleteConversation(...args),
	fetchConversationMessages: (...args: unknown[]) => mockFetchConversationMessages(...args),
	fetchConversations: () => mockFetchConversations(),
}));

// Mock the streaming library - this is what the component actually uses
const mockStreamInferenceEvents = vi.fn();
vi.mock('@lib/streaming', () => ({
	streamInferenceEvents: (...args: unknown[]) => mockStreamInferenceEvents(...args),
}));

vi.mock('@actions/models', () => ({
	getModels: () => mockGetModelsData(),
}));

// Mock hooks
const mockToastSuccess = vi.fn();
const mockToastError = vi.fn();

vi.mock('@hooks/useCrossTabSync', () => ({
	default: vi.fn(),
}));

vi.mock('@hooks/useToast', () => ({
	default: () => ({
		success: mockToastSuccess,
		error: mockToastError,
	}),
}));

// Mock chat state - we need to track the state across renders
let mockChatState = {
	messages: [] as Array<{
		id: string;
		role: 'user' | 'assistant';
		status: string;
		timestamp: Date;
		parts: MessagePart[];
		hasThinking?: boolean;
		thinkingDuration?: number;
	}>,
	isLoading: false,
	error: undefined as string | undefined,
	sessionId: undefined as string | undefined,
	assistantId: null as string | null,
	pendingFiles: [] as File[],
};

const mockAddMessage = vi.fn((message: Omit<(typeof mockChatState.messages)[0], 'id' | 'timestamp'>) => {
	const id = `msg_${Date.now()}_${Math.random().toString(36).slice(2)}`;
	mockChatState.messages.push({
		...message,
		id,
		timestamp: new Date(),
	});
	return id;
});

const mockUpdateMessage = vi.fn((id: string, updates: Partial<(typeof mockChatState.messages)[0]>) => {
	const index = mockChatState.messages.findIndex((m) => m.id === id);
	if (index !== -1) {
		mockChatState.messages[index] = { ...mockChatState.messages[index], ...updates };
	}
});

const mockSetMessages = vi.fn((messages: typeof mockChatState.messages) => {
	mockChatState.messages = messages;
});

const mockSetLoading = vi.fn((loading: boolean) => {
	mockChatState.isLoading = loading;
});

const mockSetError = vi.fn((error: string | null) => {
	mockChatState.error = error ?? undefined;
});

const mockSetSessionId = vi.fn((sessionId: string) => {
	mockChatState.sessionId = sessionId;
});

const mockSetAssistantId = vi.fn((assistantId: string) => {
	mockChatState.assistantId = assistantId;
});

const mockClearChat = vi.fn(() => {
	mockChatState.messages = [];
	mockChatState.isLoading = false;
	mockChatState.pendingFiles = [];
	mockChatState.sessionId = undefined;
});

vi.mock('@lib/chat-state', async () => {
	const actual = await vi.importActual('@lib/chat-state');
	return {
		...actual,
		useAssistantChatState: () => ({
			state: mockChatState,
			addMessage: mockAddMessage,
			updateMessage: mockUpdateMessage,
			setMessages: mockSetMessages,
			setLoading: mockSetLoading,
			setError: mockSetError,
			setSessionId: mockSetSessionId,
			setAssistantId: mockSetAssistantId,
			setPendingFiles: vi.fn(),
			clearChat: mockClearChat,
			deleteMessage: vi.fn(),
			dispatch: vi.fn(),
		}),
	};
});

// Mock useRetryMessage hook
const mockRetryMessage = vi.fn();
const mockCanRetry = vi.fn();

vi.mock('@hooks/useRetryMessage', () => ({
	default: () => ({
		retryMessage: mockRetryMessage,
		canRetry: mockCanRetry,
	}),
}));

function createMockAssistant(overrides?: { model?: string; userId?: string }): UserAssistantResponse {
	const createdByUserId = overrides?.userId ? overrides.userId : 'mock-user-id';

	return {
		agent: {
			$typeName: 'rig.v1.Agent',
			identifier: { case: 'pid', value: 'agent-123' },
			name: 'Test Assistant',
			slug: 'test-assistant',
			model: {
				$typeName: 'rig.v1.AgentModel',
				pid: overrides?.model ?? 'gpt-4',
				modelId: 'gpt-4',
				provider: {
					$typeName: 'rig.v1.AgentProvider',
					pid: 'openai',
					name: 'OpenAI',
				},
			},
			temperature: 70,
			systemPrompt: 'You are a helpful assistant.',
			tools: [],
			isActive: true,
			createdAt: BigInt(Date.now()),
			updatedAt: BigInt(Date.now()),
			createdByUserId,
		},
	};
}

function createMockModels(): ListModelsResponse {
	return {
		$typeName: 'rig.v1.ListModelsResponse',
		models: [
			{
				$typeName: 'rig.v1.Model',
				identifier: { case: 'pid', value: 'gpt-4' },
				modelId: 'gpt-4',
				name: 'GPT-4',
				providerPid: 'openai',
				providerName: 'OpenAI',
				providerType: 'openai',
				isActive: true,
				createdAt: BigInt(Date.now()),
				updatedAt: BigInt(Date.now()),
				lastFetchedAt: BigInt(Date.now()),
			},
			{
				$typeName: 'rig.v1.Model',
				identifier: { case: 'pid', value: 'gpt-3.5-turbo' },
				modelId: 'gpt-3.5-turbo',
				name: 'GPT-3.5 Turbo',
				providerPid: 'openai',
				providerName: 'OpenAI',
				providerType: 'openai',
				isActive: true,
				createdAt: BigInt(Date.now()),
				updatedAt: BigInt(Date.now()),
				lastFetchedAt: BigInt(Date.now()),
			},
		],
		total: 2,
		page: 1,
		pageSize: 10,
	};
}

function createMockSessions() {
	return [
		{
			id: 'session-1',
			title: 'First conversation',
			last_message: 'Hello there',
			timestamp: new Date().toISOString(),
			agent_id: 'agent-123',
		},
	];
}

// Helper to create streaming events as plain objects (proto create() doesn't work in test env for these schemas)
interface MockInferResponse {
	event: {
		case: string;
		value: unknown;
	};
}

function createPartStartEvent(partId: string, type: MessagePartKind): MockInferResponse {
	return {
		event: {
			case: 'partStart',
			value: { partId, type },
		},
	};
}

function createPartDeltaEvent(partId: string, content: string): MockInferResponse {
	return {
		event: {
			case: 'partDelta',
			value: {
				partId,
				delta: { case: 'content', value: content },
			},
		},
	};
}

function createPartEndEvent(partId: string, status: MessagePartStatus): MockInferResponse {
	return {
		event: {
			case: 'partEnd',
			value: { partId, status },
		},
	};
}

function createAcknowledgedEvent(conversationPid: string, messagePid = 'msg-server-1'): MockInferResponse {
	return {
		event: {
			case: 'acknowledged',
			value: { conversationPid, messagePid },
		},
	};
}

function createCompleteEvent(threadPid: string, content: string): MockInferResponse {
	return {
		event: {
			case: 'complete',
			value: {
				threadPid,
				message: {
					content,
					parts: [
						{
							id: 'part-1',
							type: MessagePartKind.TEXT,
							content,
							status: MessagePartStatus.COMPLETE,
						},
					],
				},
			},
		},
	};
}

// Async generator helper for streaming mock
async function* mockStreamGenerator(events: MockInferResponse[]) {
	for (const event of events) {
		yield event;
	}
}

describe('AssistantPage', () => {
	beforeEach(() => {
		vi.clearAllMocks();

		// Reset mock chat state
		mockChatState = {
			messages: [],
			isLoading: false,
			error: undefined,
			sessionId: undefined,
			assistantId: null,
			pendingFiles: [],
		};

		// Default mock implementations
		mockGetUserAssistantAction.mockResolvedValue({
			success: true,
			data: createMockAssistant(),
		});

		mockGetModelsData.mockResolvedValue({
			success: true,
			data: createMockModels(),
		});

		mockFetchConversations.mockResolvedValue({
			success: true,
			data: { threads: [] },
		});

		mockCancelInference.mockResolvedValue({ success: true });

		mockCanRetry.mockReturnValue(false);
	});

	afterEach(() => {
		vi.resetAllMocks();
	});

	describe('Loading States', () => {
		it('should display a loading spinner when the assistant is loading', async () => {
			// Make the assistant action hang
			mockGetUserAssistantAction.mockImplementation(
				() => new Promise(() => {}), // Never resolves
			);

			render(<AssistantPage />);

			expect(screen.getByText('Loading assistant...')).toBeInTheDocument();
			expect(document.querySelector('.loading-spinner')).toBeInTheDocument();
		});

		it('should display an error message when the assistant fails to load', async () => {
			mockGetUserAssistantAction.mockResolvedValue({
				success: false,
				error: 'Network connection failed',
			});

			render(<AssistantPage />);

			await waitFor(() => {
				expect(screen.getByText('Failed to load assistant')).toBeInTheDocument();
				expect(screen.getByText('Network connection failed')).toBeInTheDocument();
			});
		});

		it('should display the chat interface when the assistant loads successfully', async () => {
			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// Should show the conversation history sidebar (History appears in header and as view tab)
			expect(screen.getAllByText('History').length).toBeGreaterThanOrEqual(1);
			// The button text is now "New Chat" in the ResponsiveDock (appears in both desktop sidebar and mobile dock)
			expect(screen.getAllByText('New Chat').length).toBeGreaterThanOrEqual(1);

			// Should show the chat container
			expect(document.getElementById('chat')).toBeInTheDocument();
		});

		it('should use initialAssistant when provided without fetching', async () => {
			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// Should not have called the fetch action
			expect(mockGetUserAssistantAction).not.toHaveBeenCalled();

			// Should display the chat interface immediately (History appears in header and as view tab)
			expect(screen.getAllByText('History').length).toBeGreaterThanOrEqual(1);
		});

		it('should fetch assistant if initialAssistant is not provided', async () => {
			render(<AssistantPage initialModels={createMockModels()} />);

			await waitFor(() => {
				expect(mockGetUserAssistantAction).toHaveBeenCalled();
			});
		});

		it('should use initialModels when provided without fetching', async () => {
			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			expect(mockGetModelsData).not.toHaveBeenCalled();
		});

		it('should fetch models if initialModels is not provided', async () => {
			render(<AssistantPage initialAssistant={createMockAssistant()} />);

			await waitFor(() => {
				expect(mockGetModelsData).toHaveBeenCalled();
			});
		});
	});

	describe('Sending Messages', () => {
		it('should add a user message to the chat when submitting a prompt', async () => {
			const user = userEvent.setup();

			// Set up streaming to complete immediately
			mockStreamInferenceEvents.mockReturnValue(mockStreamGenerator([createCompleteEvent('thread-1', 'Response')]));

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// Find the textarea and type a message
			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hello, assistant!');

			// Find and click the submit button (type="submit")
			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			// Verify addMessage was called with user message
			await waitFor(() => {
				expect(mockAddMessage).toHaveBeenCalledWith(
					expect.objectContaining({
						role: 'user',
						status: 'sending',
					}),
				);
			});
		});

		it('should add an assistant message placeholder when streaming begins', async () => {
			const user = userEvent.setup();

			mockStreamInferenceEvents.mockReturnValue(
				mockStreamGenerator([
					createPartStartEvent('part-1', MessagePartKind.TEXT),
					createPartDeltaEvent('part-1', 'Hello'),
					createCompleteEvent('thread-1', 'Hello'),
				]),
			);

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hi');

			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			await waitFor(() => {
				// Should have been called twice: once for user, once for assistant
				expect(mockAddMessage).toHaveBeenCalledTimes(2);

				// Second call should be the assistant placeholder
				expect(mockAddMessage).toHaveBeenNthCalledWith(
					2,
					expect.objectContaining({
						role: 'assistant',
						status: 'processing',
						parts: [],
					}),
				);
			});
		});

		it('should update the assistant message content as streaming data arrives', async () => {
			const user = userEvent.setup();

			mockStreamInferenceEvents.mockReturnValue(
				mockStreamGenerator([
					createPartStartEvent('part-1', MessagePartKind.TEXT),
					createPartDeltaEvent('part-1', 'Hello '),
					createPartDeltaEvent('part-1', 'world!'),
					createCompleteEvent('thread-1', 'Hello world!'),
				]),
			);

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hi');

			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			await waitFor(() => {
				// updateMessage should have been called to append content
				expect(mockUpdateMessage).toHaveBeenCalled();
			});
		});

		it('should update user message status to sent after successful delivery', async () => {
			const user = userEvent.setup();

			// Stream yields acknowledged (message persisted) then completes
			mockStreamInferenceEvents.mockReturnValue(
				mockStreamGenerator([createAcknowledgedEvent('thread-1'), createCompleteEvent('thread-1', 'Response')]),
			);

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hi');

			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			await waitFor(() => {
				expect(mockUpdateMessage).toHaveBeenCalledWith(expect.any(String), expect.objectContaining({ status: 'sent' }));
			});
		});

		it('should update user message status to failed when the request fails', async () => {
			const user = userEvent.setup();

			// When the stream generator throws, the component should catch the error
			// biome-ignore lint/correctness/useYield: generator must throw before yielding to simulate network error
			mockStreamInferenceEvents.mockImplementation(async function* () {
				throw new Error('Network error');
			});

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hi');

			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			await waitFor(() => {
				expect(mockUpdateMessage).toHaveBeenCalledWith(
					expect.any(String),
					expect.objectContaining({ status: 'failed' }),
				);
			});
		});

		it('should not send message when prompt is empty', async () => {
			const user = userEvent.setup();

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// Try to submit without typing anything - the button is disabled when empty
			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			expect(mockStreamInferenceEvents).not.toHaveBeenCalled();
			expect(mockAddMessage).not.toHaveBeenCalled();
		});
	});

	// ==========================================================================
	// Streaming Behavior
	// ==========================================================================

	describe('Streaming Behavior', () => {
		it('should stop streaming when the cancel button is clicked', async () => {
			const user = userEvent.setup();

			// Create a stream that yields content then waits
			let resolveStream: () => void = () => {};
			const streamPromise = new Promise<void>((resolve) => {
				resolveStream = resolve;
			});

			let _yieldCount = 0;
			async function* slowStream() {
				_yieldCount++;
				yield createPartStartEvent('part-1', MessagePartKind.TEXT);
				_yieldCount++;
				yield createPartDeltaEvent('part-1', 'Hello...');
				// Wait for external signal - this keeps streaming "active"
				await streamPromise;
				yield createCompleteEvent('thread-1', 'Hello...');
			}

			mockStreamInferenceEvents.mockReturnValue(slowStream());

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hi');

			const submitButton = document.getElementById('prompt_actions__submit') as HTMLButtonElement;
			await user.click(submitButton);

			// Wait for streaming to start — the send button becomes a cancel button
			await waitFor(
				() => {
					expect(document.getElementById('prompt_actions__cancel')).toBeInTheDocument();
				},
				{ timeout: 3000 },
			);

			const cancelButton = document.getElementById('prompt_actions__cancel') as HTMLButtonElement;
			await user.click(cancelButton);

			// Cancel inference should be called
			await waitFor(() => {
				expect(mockCancelInference).toHaveBeenCalled();
			});

			// Clean up the stream
			resolveStream?.();
		});

		it('should display partial content when streaming is in progress', async () => {
			const user = userEvent.setup();

			// Create stream that delivers content
			mockStreamInferenceEvents.mockReturnValue(
				mockStreamGenerator([
					createPartStartEvent('part-1', MessagePartKind.TEXT),
					createPartDeltaEvent('part-1', 'Streaming content here'),
					createPartEndEvent('part-1', MessagePartStatus.COMPLETE),
					createCompleteEvent('thread-1', 'Streaming content here'),
				]),
			);

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const textarea = screen.getByPlaceholderText(/what would you like to know/i);
			await user.type(textarea, 'Hi');

			const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
			await user.click(submitButton);

			// Wait for streaming to complete and verify partial content was received
			await waitFor(() => {
				expect(mockUpdateMessage).toHaveBeenCalledWith(
					expect.any(String),
					expect.objectContaining({
						parts: expect.arrayContaining([
							expect.objectContaining({
								content: 'Streaming content here',
							}),
						]),
					}),
				);
			});
		});
	});

	// ==========================================================================
	// Conversation Management
	// ==========================================================================

	describe('Conversation Management', () => {
		it('should display the conversation history sidebar', () => {
			render(
				<AssistantPage
					initialAssistant={createMockAssistant()}
					initialModels={createMockModels()}
					initialSessions={createMockSessions()}
				/>,
			);

			// History appears in header and as view tab
			expect(screen.getAllByText('History').length).toBeGreaterThanOrEqual(1);
			expect(screen.getByText('First conversation')).toBeInTheDocument();
		});

		it('should load messages when a conversation is selected', async () => {
			const user = userEvent.setup();

			const mockMessages = [
				{
					pid: 'msg-1',
					content: 'Hello',
					role: 'user',
					createdAt: BigInt(Date.now()),
					parts: [],
				},
			];

			mockFetchConversationMessages.mockResolvedValue({
				success: true,
				data: { thread: undefined, messages: mockMessages },
			});

			render(
				<AssistantPage
					initialAssistant={createMockAssistant()}
					initialModels={createMockModels()}
					initialSessions={createMockSessions()}
				/>,
			);

			// Click on the conversation
			const conversationItem = screen.getByText('First conversation');
			await user.click(conversationItem);

			await waitFor(() => {
				expect(mockFetchConversationMessages).toHaveBeenCalledWith('session-1');
			});
		});

		it('should clear the chat when "New Chat" is clicked', async () => {
			const user = userEvent.setup();

			render(
				<AssistantPage
					initialAssistant={createMockAssistant()}
					initialModels={createMockModels()}
					initialSessions={createMockSessions()}
				/>,
			);

			// The button text is now "New Chat" in the ResponsiveDock (appears in both desktop sidebar and mobile dock)
			// Use the desktop sidebar button (btn-primary) which is the first one
			const newConversationButtons = screen.getAllByText('New Chat');
			await user.click(newConversationButtons[0]);

			expect(mockClearChat).toHaveBeenCalled();
		});

		it('should delete a conversation when delete is confirmed', async () => {
			const user = userEvent.setup();

			mockDeleteConversation.mockResolvedValue({ success: true });

			render(
				<AssistantPage
					initialAssistant={createMockAssistant()}
					initialModels={createMockModels()}
					initialSessions={createMockSessions()}
				/>,
			);

			// Open the dropdown menu for the conversation
			// Note: rsc-daisyui's Dropdown.Button renders as <summary> element, not <button>
			// The ConversationList is rendered inside ResponsiveDock's view content area
			const moreButton = document.querySelector('#conversation_list__container summary');
			expect(moreButton).toBeInTheDocument();
			if (!moreButton) {
				return;
			}
			await user.click(moreButton);

			// Click delete
			const deleteButton = await screen.findByText('Delete');
			await user.click(deleteButton);

			await waitFor(() => {
				expect(mockDeleteConversation).toHaveBeenCalledWith('session-1');
			});
		});

		it('should display a success toast after conversation deletion', async () => {
			const user = userEvent.setup();

			mockDeleteConversation.mockResolvedValue({ success: true });

			render(
				<AssistantPage
					initialAssistant={createMockAssistant()}
					initialModels={createMockModels()}
					initialSessions={createMockSessions()}
				/>,
			);

			// Note: rsc-daisyui's Dropdown.Button renders as <summary> element, not <button>
			// The ConversationList is rendered inside ResponsiveDock's view content area
			const moreButton = document.querySelector('#conversation_list__container summary');
			if (!moreButton) {
				expect(moreButton).toBeInTheDocument();
				return;
			}
			await user.click(moreButton);

			const deleteButton = await screen.findByText('Delete');
			await user.click(deleteButton);

			await waitFor(() => {
				expect(mockToastSuccess).toHaveBeenCalledWith('Conversation deleted');
			});
		});

		it('should display an error toast if deletion fails', async () => {
			const user = userEvent.setup();

			mockDeleteConversation.mockResolvedValue({ success: false, error: 'Delete failed' });

			render(
				<AssistantPage
					initialAssistant={createMockAssistant()}
					initialModels={createMockModels()}
					initialSessions={createMockSessions()}
				/>,
			);

			// Note: rsc-daisyui's Dropdown.Button renders as <summary> element, not <button>
			// The ConversationList is rendered inside ResponsiveDock's view content area
			const moreButton = document.querySelector('#conversation_list__container summary');
			if (!moreButton) {
				expect(moreButton).toBeInTheDocument();
				return;
			}
			await user.click(moreButton);

			const deleteButton = await screen.findByText('Delete');
			await user.click(deleteButton);

			await waitFor(() => {
				expect(mockToastError).toHaveBeenCalledWith('Delete failed');
			});
		});
	});

	// ==========================================================================
	// Retry Behavior
	// ==========================================================================

	describe('Retry Behavior', () => {
		it('should display a retry button on failed user messages', async () => {
			// Set up a failed message in the state
			mockChatState.messages = [
				{
					id: 'msg-1',
					role: 'user',
					status: 'failed',
					timestamp: new Date(),
					parts: [
						create(MessagePartSchema, {
							id: 'part-1',
							kind: MessagePartKind.TEXT,
							content: 'Failed message',
							status: MessagePartStatus.FAILED,
						}),
					],
				},
			];

			mockCanRetry.mockReturnValue(true);

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// The retry button should be visible
			expect(screen.getByTestId('retry-button')).toBeInTheDocument();
		});

		it('should call retryMessage when retry is clicked', async () => {
			const user = userEvent.setup();

			mockChatState.messages = [
				{
					id: 'msg-1',
					role: 'user',
					status: 'failed',
					timestamp: new Date(),
					parts: [
						create(MessagePartSchema, {
							id: 'part-1',
							kind: MessagePartKind.TEXT,
							content: 'Failed message',
							status: MessagePartStatus.FAILED,
						}),
					],
				},
			];

			mockCanRetry.mockReturnValue(true);

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const retryButton = screen.getByTestId('retry-button');
			await user.click(retryButton);

			expect(mockRetryMessage).toHaveBeenCalledWith('msg-1');
		});
	});

	// ==========================================================================
	// Model Selection
	// ==========================================================================

	describe('Model Selection', () => {
		it('should update the assistant model when a new model is selected', async () => {
			const user = userEvent.setup();

			mockUpdateAgentAction.mockResolvedValue({ success: true });
			mockGetUserAssistantAction.mockResolvedValue({
				success: true,
				data: createMockAssistant({ model: 'gpt-3.5-turbo' }),
			});

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// Find the model selector button (shows current model name)
			const modelSelector = screen.getByRole('button', { name: 'GPT-4' });
			await user.click(modelSelector);

			// Select a different model from the popover (Menu.Item renders as <li>, not button)
			const newModelOption = screen.getByText('GPT-3.5 Turbo');
			await user.click(newModelOption);

			await waitFor(() => {
				// Targeted update: only pid and modelPid should be sent
				expect(mockUpdateAgentAction).toHaveBeenCalledWith(
					{
						pid: 'agent-123',
						modelPid: 'gpt-3.5-turbo',
					},
					[],
				);
			});
		});

		it('should display a success notification after model update', async () => {
			const user = userEvent.setup();

			mockUpdateAgentAction.mockResolvedValue({ success: true });
			mockGetUserAssistantAction.mockResolvedValue({
				success: true,
				data: createMockAssistant({ model: 'gpt-3.5-turbo' }),
			});

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const modelSelector = screen.getByRole('button', { name: 'GPT-4' });
			await user.click(modelSelector);

			// Menu.Item renders as <li>, not button
			const newModelOption = screen.getByText('GPT-3.5 Turbo');
			await user.click(newModelOption);

			await waitFor(() => {
				expect(mockToastSuccess).toHaveBeenCalledWith(expect.stringContaining('GPT-3.5 Turbo'));
			});
		});

		it('should display an error notification if model update fails', async () => {
			const user = userEvent.setup();

			mockUpdateAgentAction.mockRejectedValue(new Error('Update failed'));

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const modelSelector = screen.getByRole('button', { name: 'GPT-4' });
			await user.click(modelSelector);

			// Menu.Item renders as <li>, not button
			const newModelOption = screen.getByText('GPT-3.5 Turbo');
			await user.click(newModelOption);

			await waitFor(() => {
				expect(mockToastError).toHaveBeenCalledWith('Update failed');
			});
		});

		it('should not call updateAgentAction if the model is unchanged', async () => {
			const user = userEvent.setup();

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			// The current model is gpt-4, select the same model
			const modelSelector = screen.getByRole('button', { name: 'GPT-4' });
			await user.click(modelSelector);

			// Find the GPT-4 option in the popover (Menu.Item renders as <li>, not button)
			// Use getAllByText since "GPT-4" appears both in the button and as a menu item
			const modelOptions = screen.getAllByText('GPT-4');
			// The second match is the menu item (first is the button we already clicked)
			const sameModelOption = modelOptions[1];
			await user.click(sameModelOption);

			expect(mockUpdateAgentAction).not.toHaveBeenCalled();
		});
	});

	// ==========================================================================
	// Message Actions
	// ==========================================================================

	describe('Message Actions', () => {
		it('should copy message content to clipboard when copy is clicked', async () => {
			const user = userEvent.setup();

			const mockWriteText = vi.fn().mockResolvedValue(undefined);
			Object.defineProperty(navigator, 'clipboard', {
				value: { writeText: mockWriteText },
				writable: true,
				configurable: true,
			});

			mockChatState.messages = [
				{
					id: 'msg-1',
					role: 'assistant',
					status: 'complete',
					timestamp: new Date(),
					parts: [
						create(MessagePartSchema, {
							id: 'part-1',
							kind: MessagePartKind.TEXT,
							content: 'Message to copy',
							status: MessagePartStatus.COMPLETE,
						}),
					],
				},
			];

			render(<AssistantPage initialAssistant={createMockAssistant()} initialModels={createMockModels()} />);

			const copyButton = screen.getByRole('button', { name: 'Copy' });
			await user.click(copyButton);

			expect(mockWriteText).toHaveBeenCalledWith('Message to copy');
		});
	});
});
