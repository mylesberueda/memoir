import {
	type MessagePart,
	MessagePartKind,
	type MessagePartStatus,
} from '@startup/proto-ts/rig-service/rig/v1/inference_pb';

type Listener = () => void;

export interface StreamingSnapshot {
	parts: MessagePart[];
	hasThinking: boolean;
}

const EMPTY_SNAPSHOT: StreamingSnapshot = { parts: [], hasThinking: false };

/**
 * External store for streaming message parts. Accumulates mutations in-place
 * and batches React notifications to once per animation frame via rAF.
 */
class StreamingPartsStore {
	private parts: MessagePart[] = [];
	private hasThinking = false;
	private snapshot: StreamingSnapshot = EMPTY_SNAPSHOT;
	private listeners = new Set<Listener>();
	private rafId: number | null = null;
	private active = false;

	start() {
		this.parts = [];
		this.hasThinking = false;
		this.snapshot = EMPTY_SNAPSHOT;
		this.active = true;
	}

	addPart(part: MessagePart) {
		this.parts.push(part);
		if (part.kind === MessagePartKind.THINKING) {
			this.hasThinking = true;
		}
		this.scheduleEmit();
	}

	updateDelta(partId: string, apply: (part: MessagePart) => void) {
		const part = this.parts.find((p) => p.id === partId);
		if (part) {
			apply(part);
			this.scheduleEmit();
		}
	}

	endPart(partId: string, status: MessagePartStatus, apply: (part: MessagePart) => void) {
		const part = this.parts.find((p) => p.id === partId);
		if (part) {
			part.status = status;
			apply(part);
			this.scheduleEmit();
		}
	}

	/** Flush pending rAF and return final parts for committing to the reducer. */
	flush(): MessagePart[] {
		if (this.rafId !== null) {
			cancelAnimationFrame(this.rafId);
			this.rafId = null;
		}
		// Create the final snapshot so any subscribed components see the last state
		this.snapshot = { parts: [...this.parts], hasThinking: this.hasThinking };
		this.emit();
		return this.parts;
	}

	reset() {
		if (this.rafId !== null) {
			cancelAnimationFrame(this.rafId);
			this.rafId = null;
		}
		this.parts = [];
		this.hasThinking = false;
		this.snapshot = EMPTY_SNAPSHOT;
		this.active = false;
	}

	isActive() {
		return this.active;
	}

	subscribe = (listener: Listener): (() => void) => {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	};

	getSnapshot = (): StreamingSnapshot => {
		return this.snapshot;
	};

	private scheduleEmit() {
		if (this.rafId === null) {
			this.rafId = requestAnimationFrame(() => {
				this.rafId = null;
				// New array ref so React sees a change via Object.is
				this.snapshot = { parts: [...this.parts], hasThinking: this.hasThinking };
				this.emit();
			});
		}
	}

	private emit() {
		for (const listener of this.listeners) {
			listener();
		}
	}
}

export const streamingStore = new StreamingPartsStore();
