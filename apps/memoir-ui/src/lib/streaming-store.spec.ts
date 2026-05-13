import { create } from '@bufbuild/protobuf';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { streamingStore } from './streaming-store';

function createTextPart(id: string, content: string) {
	return create(MessagePartSchema, {
		id,
		kind: MessagePartKind.TEXT,
		content,
		status: MessagePartStatus.STREAMING,
	});
}

function createThinkingPart(id: string, content: string) {
	return create(MessagePartSchema, {
		id,
		kind: MessagePartKind.THINKING,
		content,
		status: MessagePartStatus.STREAMING,
	});
}

afterEach(() => {
	streamingStore.reset();
});

describe('StreamingPartsStore', () => {
	it('should start with an empty snapshot', () => {
		const snapshot = streamingStore.getSnapshot();

		expect(snapshot.parts).toEqual([]);
		expect(snapshot.hasThinking).toBe(false);
	});

	it('should not be active before start is called', () => {
		expect(streamingStore.isActive()).toBe(false);
	});

	it('should be active after start and inactive after reset', () => {
		streamingStore.start();
		expect(streamingStore.isActive()).toBe(true);

		streamingStore.reset();
		expect(streamingStore.isActive()).toBe(false);
	});

	it('should accumulate parts via addPart', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));
		streamingStore.addPart(createTextPart('p2', 'World'));

		const parts = streamingStore.flush();

		expect(parts).toHaveLength(2);
		expect(parts[0].content).toBe('Hello');
		expect(parts[1].content).toBe('World');
	});

	it('should track hasThinking when a thinking part is added', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));

		const _beforeThinking = streamingStore.flush();
		expect(streamingStore.getSnapshot().hasThinking).toBe(false);

		streamingStore.start();
		streamingStore.addPart(createThinkingPart('p2', 'Hmm...'));
		streamingStore.flush();

		expect(streamingStore.getSnapshot().hasThinking).toBe(true);
	});

	it('should apply deltas to existing parts via updateDelta', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hel'));

		streamingStore.updateDelta('p1', (part) => {
			part.content = `${part.content || ''}lo World`;
		});

		const parts = streamingStore.flush();

		expect(parts[0].content).toBe('Hello World');
	});

	it('should update part status via endPart', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Done'));

		streamingStore.endPart('p1', MessagePartStatus.COMPLETE, () => {});

		const parts = streamingStore.flush();

		expect(parts[0].status).toBe(MessagePartStatus.COMPLETE);
	});

	it('should flush pending updates and return final parts', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));

		// Flush before rAF fires — should still return parts
		const parts = streamingStore.flush();

		expect(parts).toHaveLength(1);
		expect(parts[0].content).toBe('Hello');
		// Snapshot should also be updated after flush
		expect(streamingStore.getSnapshot().parts).toHaveLength(1);
	});

	it('should preserve parts in snapshot after user cancels the stream', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'The beginning of a'));
		streamingStore.updateDelta('p1', (part) => {
			part.content = `${part.content || ''} response that gets`;
		});
		streamingStore.addPart(createThinkingPart('p2', 'Let me think about'));

		// Simulate cancellation: flush mid-stream
		const parts = streamingStore.flush();

		expect(parts).toHaveLength(2);
		expect(parts[0].content).toBe('The beginning of a response that gets');
		expect(parts[1].content).toBe('Let me think about');
		expect(streamingStore.getSnapshot().hasThinking).toBe(true);
	});

	it('should reset to empty state', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));
		streamingStore.flush();

		streamingStore.reset();

		expect(streamingStore.getSnapshot().parts).toEqual([]);
		expect(streamingStore.getSnapshot().hasThinking).toBe(false);
		expect(streamingStore.isActive()).toBe(false);
	});

	it('should notify subscribers when snapshot changes', async () => {
		const listener = vi.fn();
		streamingStore.subscribe(listener);

		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));

		// rAF-based: wait for the frame to fire
		await vi.waitFor(() => {
			expect(listener).toHaveBeenCalled();
		});

		expect(streamingStore.getSnapshot().parts).toHaveLength(1);
	});

	it('should not update snapshot until rAF fires or flush is called', () => {
		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));

		// Snapshot is still stale — rAF hasn't fired yet
		expect(streamingStore.getSnapshot().parts).toEqual([]);

		// But flush forces it through
		streamingStore.flush();
		expect(streamingStore.getSnapshot().parts).toHaveLength(1);
	});

	it('should stop notifying after unsubscribe', () => {
		const listener = vi.fn();
		const unsubscribe = streamingStore.subscribe(listener);

		unsubscribe();

		streamingStore.start();
		streamingStore.addPart(createTextPart('p1', 'Hello'));

		// Force flush to trigger emit synchronously
		streamingStore.flush();

		expect(listener).not.toHaveBeenCalled();
	});
});
