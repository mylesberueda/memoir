import { DocumentStatus, type MessageAttachment } from '@lib/chat-state';
import { streamingStore } from '@lib/streaming-store';
import { MessagePartKind } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, type Mock, vi } from 'vitest';
import { type StreamingChatOptions, useStreamingChat } from './useStreamingChat';

const mockUploadDocument = vi.fn();
const mockFetchDocument = vi.fn();
const mockDeleteDocument = vi.fn();

vi.mock('@actions/documents', () => ({
	uploadDocument: (...args: unknown[]) => mockUploadDocument(...args),
	fetchDocument: (...args: unknown[]) => mockFetchDocument(...args),
	deleteDocument: (...args: unknown[]) => mockDeleteDocument(...args),
}));

const mockCreateConversation = vi.fn();
const mockCancelInference = vi.fn();

vi.mock('@actions/infer', () => ({
	createConversation: (...args: unknown[]) => mockCreateConversation(...args),
	cancelInference: (...args: unknown[]) => mockCancelInference(...args),
}));

const mockStreamInferenceEvents = vi.fn();

vi.mock('@lib/streaming', () => ({
	streamInferenceEvents: (...args: unknown[]) => mockStreamInferenceEvents(...args),
}));

type MockedOptions<T> = {
	[K in keyof T]: T[K] extends (...args: infer A) => infer R ? Mock<(...args: A) => R> : T[K];
};

function createHookOptions(
	overrides?: Partial<Parameters<typeof useStreamingChat>[0]>,
): MockedOptions<StreamingChatOptions> {
	return {
		agentId: 'agent-123',
		sessionId: undefined as string | undefined,
		addMessage: vi.fn(() => `msg_${Date.now()}`),
		updateMessage: vi.fn(),
		setLoading: vi.fn(),
		setError: vi.fn(),
		setSessionId: vi.fn(),
		onSessionCreated: vi.fn(),
		...overrides,
	} as MockedOptions<StreamingChatOptions>;
}

function createTestFile(name = 'test.txt', content = 'file content') {
	return new File([content], name, { type: 'text/plain' });
}

/** Builds a mock uploadDocument response matching ActionResult shape */
function mockDocumentSuccess(pid = 'doc-123') {
	return {
		success: true,
		data: {
			document: { pid, status: DocumentStatus.PROCESSING },
		},
	};
}

// Mock streaming generator that acknowledges then completes immediately
async function* emptyStream(conversationPid = 'conv-ack') {
	yield {
		event: {
			case: 'acknowledged',
			value: {
				conversationPid,
				messagePid: 'msg-server-1',
			},
		},
	};
	yield {
		event: {
			case: 'complete',
			value: {
				conversationPid,
				message: { content: 'done', parts: [] },
			},
		},
	};
}

/**
 * Drain a submit/retry that involves polling (setTimeout loops).
 * Uses shouldAdvanceTime so fake timers auto-advance, allowing both
 * native async APIs (File.arrayBuffer) and setTimeout polls to resolve.
 */
async function drainWithFakeTimers(action: () => Promise<void>) {
	vi.useFakeTimers({ shouldAdvanceTime: true });
	const promise = action();
	for (let i = 0; i < 200; ++i) {
		await vi.advanceTimersByTimeAsync(500);
	}
	await promise;
	vi.useRealTimers();
}

describe('useStreamingChat — file upload failure handling', () => {
	beforeEach(() => {
		vi.clearAllMocks();

		// Default: streaming works
		mockStreamInferenceEvents.mockReturnValue(emptyStream());
	});

	afterEach(() => {
		vi.restoreAllMocks();
		vi.useRealTimers();
	});

	it('should set error and mark message failed when upload fails', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});
		mockUploadDocument.mockResolvedValue({ success: false, error: 'Auth required' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Tell me about the file',
					files: [createTestFile()],
				}),
			);
		});

		// User message should be marked failed
		expect(opts.updateMessage).toHaveBeenCalledWith(expect.any(String), expect.objectContaining({ status: 'failed' }));

		// Error should be set
		expect(opts.setError).toHaveBeenCalledWith(expect.stringContaining('failed'));
	});

	it('should not call inference when upload fails', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});
		mockUploadDocument.mockResolvedValue({ success: false, error: 'Server error' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Tell me about the file',
					files: [createTestFile()],
				}),
			);
		});

		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();
	});

	it('should not call inference when ingestion poll times out', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});
		mockUploadDocument.mockResolvedValue(mockDocumentSuccess());

		// Poll always returns PROCESSING (never reaches READY)
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-123', status: DocumentStatus.PROCESSING } },
		});

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Summarize this',
					files: [createTestFile()],
				}),
			);
		});

		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();
		expect(opts.setError).toHaveBeenCalledWith(expect.stringContaining('failed'));
	});

	it('should not call inference when one of multiple files fails', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});

		// First file succeeds, second fails at upload
		mockUploadDocument
			.mockResolvedValueOnce(mockDocumentSuccess('doc-1'))
			.mockResolvedValueOnce({ success: false, error: 'Quota exceeded' });

		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-1', status: DocumentStatus.READY } },
		});

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Compare these files',
					files: [createTestFile('good.txt'), createTestFile('bad.txt')],
				}),
			);
		});

		// Inference should NOT be called — one file failed
		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();
		expect(opts.setError).toHaveBeenCalledWith(expect.stringContaining('failed'));
	});

	// ────────────────────────────────────────────────────────────────────────
	// No assistant message created on failure
	// ────────────────────────────────────────────────────────────────────────

	it('should not add assistant message when file upload fails', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});
		mockUploadDocument.mockResolvedValue({ success: false, error: 'Upload failed' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({
				prompt: 'Summarize this',
				files: [createTestFile()],
			});
		});

		// addMessage should only be called once — for the user message
		// No assistant message should be created
		expect(opts.addMessage).toHaveBeenCalledTimes(1);
		expect(opts.addMessage).toHaveBeenCalledWith(expect.objectContaining({ role: 'user' }));
	});

	// ────────────────────────────────────────────────────────────────────────
	// Input remains usable after failure
	// ────────────────────────────────────────────────────────────────────────

	it('should not set loading when file upload fails', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});
		mockUploadDocument.mockResolvedValue({ success: false, error: 'Failed' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({
				prompt: 'Summarize',
				files: [createTestFile()],
			});
		});

		const loadingCalls = opts.setLoading.mock.calls.map((c: [boolean]) => c[0]);
		const lastLoadingCall = loadingCalls[loadingCalls.length - 1];
		expect(lastLoadingCall === undefined || lastLoadingCall === false).toBe(true);
	});

	// ────────────────────────────────────────────────────────────────────────
	// Happy path: successful upload reaches inference
	// ────────────────────────────────────────────────────────────────────────

	it('should call inference when all files upload successfully', async () => {
		const opts = createHookOptions();

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'conv-123' } },
		});
		mockUploadDocument.mockResolvedValue(mockDocumentSuccess());
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-123', status: DocumentStatus.READY } },
		});

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Summarize this',
					files: [createTestFile()],
				}),
			);
		});

		expect(mockStreamInferenceEvents).toHaveBeenCalled();
	});

	it('should skip file upload and call inference for text-only message', async () => {
		const opts = createHookOptions({ sessionId: 'existing-session' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({ prompt: 'Hello' });
		});

		// No document actions should be called
		expect(mockUploadDocument).not.toHaveBeenCalled();
		expect(mockFetchDocument).not.toHaveBeenCalled();
		expect(mockCreateConversation).not.toHaveBeenCalled();

		// Inference should proceed
		expect(mockStreamInferenceEvents).toHaveBeenCalled();
	});

	it('should create conversation before file upload when no session', async () => {
		const opts = createHookOptions({ sessionId: undefined });

		mockCreateConversation.mockResolvedValue({
			success: true,
			data: { conversation: { pid: 'new-conv' } },
		});
		mockUploadDocument.mockResolvedValue(mockDocumentSuccess());
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-123', status: DocumentStatus.READY } },
		});

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'About the file',
					files: [createTestFile()],
				}),
			);
		});

		expect(mockCreateConversation).toHaveBeenCalledWith('agent-123');
		expect(opts.setSessionId).toHaveBeenCalledWith('new-conv');
		expect(mockUploadDocument).toHaveBeenCalledWith(expect.objectContaining({ conversationPid: 'new-conv' }));
	});

	it('should not create conversation when session already exists', async () => {
		const opts = createHookOptions({ sessionId: 'existing-session' });

		mockUploadDocument.mockResolvedValue(mockDocumentSuccess());
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-123', status: DocumentStatus.READY } },
		});

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'About the file',
					files: [createTestFile()],
				}),
			);
		});

		expect(mockCreateConversation).not.toHaveBeenCalled();
	});
});

describe('useStreamingChat — retryAttachment', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mockStreamInferenceEvents.mockReturnValue(emptyStream());
	});

	afterEach(() => {
		vi.restoreAllMocks();
		vi.useRealTimers();
	});

	it('should retry failed attachment and trigger inference on success', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		// First submit: upload fails
		mockUploadDocument.mockResolvedValueOnce({ success: false, error: 'Temporary error' });

		const { result } = renderHook(() => useStreamingChat(opts));

		// Submit with a file that will fail — drainWithFakeTimers needed so
		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Analyze this file',
					files: [createTestFile('report.pdf')],
				}),
			);
		});

		// Should NOT have called inference
		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();

		// Get the attachment ID from the addMessage call (synchronous, reliable)
		const addMessageArgs = opts.addMessage.mock.calls[0][0] as { attachments?: MessageAttachment[] };
		const failedAttachmentId = addMessageArgs.attachments?.[0]?.id;
		const messageId = opts.addMessage.mock.results[0].value;

		// Now set up upload to succeed on retry
		mockUploadDocument.mockResolvedValueOnce(mockDocumentSuccess('doc-retry'));
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-retry', status: DocumentStatus.READY } },
		});
		mockStreamInferenceEvents.mockReturnValue(emptyStream());

		if (!failedAttachmentId) {
			fail('Missing failedAttachmentId');
		}

		// Build current attachments array as the caller would
		const currentAttachments: MessageAttachment[] = [
			{
				id: failedAttachmentId,
				name: 'report.pdf',
				type: 'text/plain',
				size: 12,
				status: DocumentStatus.FAILED,
			},
		];

		// console.log(failedAttachmentId);

		if (!failedAttachmentId) {
			fail('missing attachment id');
		}

		await act(async () => {
			await result.current.retryAttachment(messageId, failedAttachmentId, currentAttachments);
		});

		// Should have called uploadDocument again for the retry
		expect(mockUploadDocument).toHaveBeenCalledTimes(2);

		// Should have triggered inference after successful retry
		expect(mockStreamInferenceEvents).toHaveBeenCalled();
	});

	it('should not auto-trigger inference when failed attachments remain after retry', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		// Both files fail
		mockUploadDocument
			.mockResolvedValueOnce({ success: false, error: 'Error' })
			.mockResolvedValueOnce({ success: false, error: 'Error' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({
				prompt: 'Compare files',
				files: [createTestFile('a.txt'), createTestFile('b.txt')],
			});
		});

		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();

		const messageId = opts.addMessage.mock.results[0].value;

		// Retry first file — succeeds, but second is still FAILED
		mockUploadDocument.mockResolvedValueOnce(mockDocumentSuccess('doc-a'));
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'doc-a', status: DocumentStatus.READY } },
		});

		// Get attachment IDs from addMessage args (both uploads failed, IDs never replaced)
		const addMessageArgs = opts.addMessage.mock.calls[0][0] as { attachments?: MessageAttachment[] };
		const currentAttachments: MessageAttachment[] = (addMessageArgs.attachments ?? []).map((a) => ({
			...a,
			status: DocumentStatus.FAILED,
		}));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.retryAttachment(messageId, currentAttachments[0].id, currentAttachments),
			);
		});

		// Inference should NOT trigger — second file is still FAILED
		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();
	});
});

describe('useStreamingChat — deleteAttachment', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mockStreamInferenceEvents.mockReturnValue(emptyStream());
		mockDeleteDocument.mockResolvedValue({ success: true });
	});

	afterEach(() => {
		vi.restoreAllMocks();
		vi.useRealTimers();
	});

	it('should delete failed attachment and trigger inference when none remain', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		mockUploadDocument.mockResolvedValueOnce({ success: false, error: 'Failed' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({
				prompt: 'Summarize this',
				files: [createTestFile()],
			});
		});

		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();

		const messageId = opts.addMessage.mock.results[0].value;

		// Get the attachment ID from the addMessage call args (initial temp ID).
		// Upload failed, so the ID was never replaced with a server PID.
		const addMessageArgs = opts.addMessage.mock.calls[0][0] as { attachments?: MessageAttachment[] };
		const failedAttachmentId = addMessageArgs.attachments?.[0]?.id;
		expect(failedAttachmentId).toBeDefined();

		if (!failedAttachmentId) {
			fail('Missing failedAttachmentId');
		}

		const currentAttachments: MessageAttachment[] = [
			{
				id: failedAttachmentId,
				name: 'test.txt',
				type: 'text/plain',
				size: 12,
				status: DocumentStatus.FAILED,
			},
		];

		mockStreamInferenceEvents.mockReturnValue(emptyStream());

		await act(async () => {
			await result.current.deleteAttachment(messageId, failedAttachmentId, currentAttachments);
		});

		// Should trigger inference since no failed attachments remain
		expect(mockStreamInferenceEvents).toHaveBeenCalled();
	});

	it('should not auto-trigger inference when failed attachments remain after delete', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		// Both files fail
		mockUploadDocument
			.mockResolvedValueOnce({ success: false, error: 'Error' })
			.mockResolvedValueOnce({ success: false, error: 'Error' });

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({
				prompt: 'Compare files',
				files: [createTestFile('a.txt'), createTestFile('b.txt')],
			});
		});

		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();

		const messageId = opts.addMessage.mock.results[0].value;

		// Get attachment IDs from addMessage args (upload failed, so IDs were never replaced)
		const addMessageArgs = opts.addMessage.mock.calls[0][0] as { attachments?: MessageAttachment[] };
		const currentAttachments: MessageAttachment[] = (addMessageArgs.attachments ?? []).map((a) => ({
			...a,
			status: DocumentStatus.FAILED,
		}));
		expect(currentAttachments).toHaveLength(2);

		// Delete first file — second is still FAILED
		await act(async () => {
			await result.current.deleteAttachment(messageId, currentAttachments[0].id, currentAttachments);
		});

		// Inference should NOT trigger — one file is still FAILED
		expect(mockStreamInferenceEvents).not.toHaveBeenCalled();
	});

	it('should call deleteDocument for server-side attachment', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		// Upload succeeds but ingestion fails (poll returns FAILED)
		mockUploadDocument.mockResolvedValueOnce(mockDocumentSuccess('server-doc-pid'));
		mockFetchDocument.mockResolvedValue({
			success: true,
			data: { document: { pid: 'server-doc-pid', status: DocumentStatus.FAILED } },
		});

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await drainWithFakeTimers(() =>
				result.current.submit({
					prompt: 'Summarize',
					files: [createTestFile()],
				}),
			);
		});

		const messageId = opts.addMessage.mock.results[0].value;

		const currentAttachments: MessageAttachment[] = [
			{
				id: 'server-doc-pid',
				name: 'test.txt',
				type: 'text/plain',
				size: 12,
				status: DocumentStatus.FAILED,
			},
		];

		mockStreamInferenceEvents.mockReturnValue(emptyStream());

		await act(async () => {
			await result.current.deleteAttachment(messageId, 'server-doc-pid', currentAttachments);
		});

		// Should call deleteDocument for server-side documents
		expect(mockDeleteDocument).toHaveBeenCalledWith('server-doc-pid');
	});
});

describe('useStreamingChat — cancellation preserves streamed content', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		streamingStore.reset();
		mockCancelInference.mockResolvedValue(undefined);
	});

	afterEach(() => {
		vi.restoreAllMocks();
		streamingStore.reset();
	});

	it('should show agent message parts despite the user cancelling the output', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		// Stream yields two text parts with content, then the user cancels mid-stream.
		// The async generator pauses at a deferred promise so we can call stop() mid-flight.
		let resolveHang: () => void;
		const hangPromise = new Promise<void>((r) => {
			resolveHang = r;
		});

		async function* streamWithPauseBeforeComplete() {
			yield {
				event: {
					case: 'partStart' as const,
					value: {
						partId: 'part-1',
						kind: MessagePartKind.TEXT,
					},
				},
			};
			yield {
				event: {
					case: 'partDelta' as const,
					value: {
						partId: 'part-1',
						delta: { case: 'content' as const, value: 'Hello, I am responding to your' },
					},
				},
			};
			yield {
				event: {
					case: 'partDelta' as const,
					value: {
						partId: 'part-1',
						delta: { case: 'content' as const, value: ' message with useful information' },
					},
				},
			};
			// Pause here — user will cancel while we're "waiting for more tokens"
			await hangPromise;
			// This should never be reached after cancellation
			yield {
				event: {
					case: 'partDelta' as const,
					value: {
						partId: 'part-1',
						delta: { case: 'content' as const, value: ' that should not appear' },
					},
				},
			};
		}

		mockStreamInferenceEvents.mockReturnValue(streamWithPauseBeforeComplete());

		const { result } = renderHook(() => useStreamingChat(opts));

		// Start the stream (don't await — it will hang at the promise)
		let submitPromise: Promise<void>;
		act(() => {
			submitPromise = result.current.submit({ prompt: 'Tell me something' });
		});

		// Wait a tick for the stream to process the yielded events
		await act(async () => {
			await new Promise((r) => setTimeout(r, 0));
		});

		// User clicks stop
		await act(async () => {
			await result.current.stop();
		});

		// Unblock the generator so the submit promise resolves
		resolveHang?.();
		await act(async () => {
			await submitPromise;
		});

		// Find the updateMessage call that set status to 'cancelled'
		const cancelledCall = opts.updateMessage.mock.calls.find(
			([, updates]: [string, { status?: string }]) => updates.status === 'cancelled',
		);

		if (!cancelledCall) {
			fail('Expected a cancelledCall with status "cancelled"');
		}

		const [, cancelledUpdates] = cancelledCall;

		// The accumulated content should be preserved — not empty
		expect(cancelledUpdates.parts).toBeDefined();
		expect(cancelledUpdates.parts.length).toBeGreaterThan(0);
		expect(cancelledUpdates.parts[0].content).toBe('Hello, I am responding to your message with useful information');
	});

	it('should preserve parts when stream throws non-AbortError after cancellation', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		// Reproduces the real production bug: when the user cancels, the fetch abort
		// races with the backend closing the gRPC stream. The Next.js API route's
		// ReadableStream controller is already closed, so the NDJSON reader throws
		// "Invalid state: Controller is already closed" — NOT an AbortError.
		let resolveHang: () => void;

		const hangPromise = new Promise<void>((r) => {
			resolveHang = r;
		});

		async function* streamThatThrowsOnCancel() {
			yield {
				event: {
					case: 'partStart' as const,
					value: { partId: 'part-1', kind: MessagePartKind.TEXT },
				},
			};
			yield {
				event: {
					case: 'partDelta' as const,
					value: {
						partId: 'part-1',
						delta: { case: 'content' as const, value: 'Partial response before crash' },
					},
				},
			};
			// Pause — user cancels here
			await hangPromise;
			// Simulate the real error: controller closed due to abort race
			throw new Error('Invalid state: Controller is already closed');
		}

		mockStreamInferenceEvents.mockReturnValue(streamThatThrowsOnCancel());

		const { result } = renderHook(() => useStreamingChat(opts));

		let submitPromise: Promise<void>;
		act(() => {
			submitPromise = result.current.submit({ prompt: 'Tell me something' });
		});

		await act(async () => {
			await new Promise((r) => setTimeout(r, 0));
		});

		// User clicks stop — sets isCancelledRef, aborts fetch
		await act(async () => {
			await result.current.stop();
		});

		// Unblock generator — it will throw the non-AbortError
		resolveHang?.();
		await act(async () => {
			await submitPromise;
		});

		// The assistant message should be marked cancelled with parts preserved.
		// There may be multiple updateMessage calls with status 'cancelled' —
		// find the one that includes parts (the authoritative cancellation).
		const cancelledCall = opts.updateMessage.mock.calls.find(
			([, updates]: [string, { status?: string; parts?: unknown[] }]) =>
				updates.status === 'cancelled' && updates.parts && updates.parts.length > 0,
		);

		if (!cancelledCall) {
			fail('Expected a cancelledCall with status "cancelled" and non-empty parts');
		}

		const [, cancelledUpdates] = cancelledCall;
		expect(cancelledUpdates.parts[0].content).toBe('Partial response before crash');

		// Should NOT have set a real error — only the initial setError(null) clear is expected
		const errorCalls = opts.setError.mock.calls.filter(([val]: [string | null]) => val !== null);
		expect(errorCalls).toHaveLength(0);
	});
});

describe('useStreamingChat — message acknowledged event', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		streamingStore.reset();
	});

	afterEach(() => {
		vi.restoreAllMocks();
		streamingStore.reset();
	});

	it('should mark user message sent on acknowledged event', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		async function* streamWithAck() {
			yield {
				event: {
					case: 'acknowledged' as const,
					value: { conversationPid: 'conv-123', messagePid: 'msg-server-1' },
				},
			};
			yield {
				event: {
					case: 'complete' as const,
					value: { conversationPid: 'conv-123', message: { parts: [] } },
				},
			};
		}

		mockStreamInferenceEvents.mockReturnValue(streamWithAck());

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({ prompt: 'Hello' });
		});

		// User message should transition to 'sent' via acknowledged event
		const sentCall = opts.updateMessage.mock.calls.find(
			([, updates]: [string, { status?: string }]) => updates.status === 'sent',
		);
		expect(sentCall).toBeDefined();

		// The message ID in the sent call should match the user message
		const userMessageId = opts.addMessage.mock.results[0].value;
		expect(sentCall?.[0]).toBe(userMessageId);
	});

	it('should set session id from acknowledged on new conversation', async () => {
		const opts = createHookOptions({ sessionId: undefined });

		async function* streamWithNewConversationAck() {
			yield {
				event: {
					case: 'acknowledged' as const,
					value: { conversationPid: 'new-conv-from-ack', messagePid: 'msg-server-1' },
				},
			};
			yield {
				event: {
					case: 'complete' as const,
					value: { conversationPid: 'new-conv-from-ack', message: { parts: [] } },
				},
			};
		}

		mockStreamInferenceEvents.mockReturnValue(streamWithNewConversationAck());

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({ prompt: 'First message' });
		});

		// Session should be set from the acknowledged event, not just complete
		expect(opts.setSessionId).toHaveBeenCalledWith('new-conv-from-ack');
		expect(opts.onSessionCreated).toHaveBeenCalledWith('new-conv-from-ack');
	});

	it('should not override session id from acknowledged when session already exists', async () => {
		const opts = createHookOptions({ sessionId: 'existing-session' });

		async function* streamWithAck() {
			yield {
				event: {
					case: 'acknowledged' as const,
					value: { conversationPid: 'existing-session', messagePid: 'msg-server-1' },
				},
			};
			yield {
				event: {
					case: 'complete' as const,
					value: { conversationPid: 'existing-session', message: { parts: [] } },
				},
			};
		}

		mockStreamInferenceEvents.mockReturnValue(streamWithAck());

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({ prompt: 'Hello' });
		});

		// setSessionId should NOT be called — session already existed
		expect(opts.setSessionId).not.toHaveBeenCalled();
	});

	it('should mark user message failed when stream errors before acknowledged', async () => {
		const opts = createHookOptions({ sessionId: 'conv-123' });

		const failingStream: AsyncIterable<never> = {
			[Symbol.asyncIterator]: () => ({
				next: () => Promise.reject(new Error('gRPC stream failed: service unavailable')),
			}),
		};

		mockStreamInferenceEvents.mockReturnValue(failingStream);

		const { result } = renderHook(() => useStreamingChat(opts));

		await act(async () => {
			await result.current.submit({ prompt: 'Hello' });
		});

		// User message should be marked failed (not stuck at 'sending')
		const userMessageId = opts.addMessage.mock.results[0].value;
		const failedCall = opts.updateMessage.mock.calls.find(
			([id, updates]: [string, { status?: string }]) => id === userMessageId && updates.status === 'failed',
		);
		expect(failedCall).toBeDefined();

		// No 'sent' status should have been set
		const sentCall = opts.updateMessage.mock.calls.find(
			([id, updates]: [string, { status?: string }]) => id === userMessageId && updates.status === 'sent',
		);
		expect(sentCall).toBeUndefined();

		// Error should be reported
		expect(opts.setError).toHaveBeenCalledWith(expect.stringContaining('service unavailable'));
	});
});
