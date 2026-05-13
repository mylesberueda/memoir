import { act, renderHook } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { useMessageHeights } from './useMessageHeights';

describe('useMessageHeights', () => {
	it('should initialize with default average height', () => {
		const { result } = renderHook(() => useMessageHeights());

		expect(result.current.averageHeight).toBe(80);
	});

	it('should store and retrieve message heights', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 120);
		});

		expect(result.current.getHeight('msg-1')).toBe(120);
	});

	it('should return average height for unknown messages', () => {
		const { result } = renderHook(() => useMessageHeights());

		expect(result.current.getHeight('unknown-msg')).toBe(80);
	});

	it('should update average height when adding messages', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 60);
		});

		// Average of 100 and 60 should be 80
		expect(result.current.averageHeight).toBe(80);
	});

	it('should calculate average height correctly with multiple messages', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 120);
			result.current.setHeight('msg-3', 90);
		});

		// Average of 100, 120, 90 should be 103.33... rounded to 103
		expect(result.current.averageHeight).toBe(103);
	});

	it('should update average when message heights change', () => {
		const { result } = renderHook(() => useMessageHeights());

		// Set initial heights
		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 100);
		});

		expect(result.current.averageHeight).toBe(100);

		// Update one height
		act(() => {
			result.current.setHeight('msg-2', 140);
		});

		// Average of 100 and 140 should be 120
		expect(result.current.averageHeight).toBe(120);
	});

	it('should calculate total height correctly', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 120);
			result.current.setHeight('msg-3', 90);
		});

		const totalHeight = result.current.getTotalHeight(['msg-1', 'msg-2', 'msg-3']);
		expect(totalHeight).toBe(310); // 100 + 120 + 90
	});

	it('should use average height for unknown messages in total calculation', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
		});

		const totalHeight = result.current.getTotalHeight(['msg-1', 'unknown-msg']);
		expect(totalHeight).toBe(200); // 100 + 100 (average since there's only one known height)
	});

	it('should calculate height up to specific index', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 120);
			result.current.setHeight('msg-3', 90);
		});

		const heightUpTo = result.current.getHeightUpTo(['msg-1', 'msg-2', 'msg-3'], 2);
		expect(heightUpTo).toBe(220); // 100 + 120 (first 2 messages)
	});

	it('should return 0 for height up to index 0', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
		});

		const heightUpTo = result.current.getHeightUpTo(['msg-1'], 0);
		expect(heightUpTo).toBe(0);
	});

	it('should handle empty message arrays', () => {
		const { result } = renderHook(() => useMessageHeights());

		const totalHeight = result.current.getTotalHeight([]);
		expect(totalHeight).toBe(0);

		const heightUpTo = result.current.getHeightUpTo([], 0);
		expect(heightUpTo).toBe(0);
	});

	it('should handle out-of-bounds index in getHeightUpTo', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 120);
		});

		// Index beyond array length should return total height
		const heightUpTo = result.current.getHeightUpTo(['msg-1', 'msg-2'], 10);
		expect(heightUpTo).toBe(220); // All messages
	});

	it('should maintain height values across multiple operations', () => {
		const { result } = renderHook(() => useMessageHeights());

		// Set initial heights
		act(() => {
			result.current.setHeight('msg-1', 100);
			result.current.setHeight('msg-2', 120);
		});

		// Get heights
		expect(result.current.getHeight('msg-1')).toBe(100);
		expect(result.current.getHeight('msg-2')).toBe(120);

		// Add more heights
		act(() => {
			result.current.setHeight('msg-3', 90);
		});

		// Previous heights should still be there
		expect(result.current.getHeight('msg-1')).toBe(100);
		expect(result.current.getHeight('msg-2')).toBe(120);
		expect(result.current.getHeight('msg-3')).toBe(90);
	});

	it('should round average height to nearest integer', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 101);
			result.current.setHeight('msg-2', 102);
		});

		// Average of 101 and 102 should be 101.5, rounded to 102
		expect(result.current.averageHeight).toBe(102);
	});

	it('should handle single message height', () => {
		const { result } = renderHook(() => useMessageHeights());

		act(() => {
			result.current.setHeight('msg-1', 150);
		});

		expect(result.current.averageHeight).toBe(150);
		expect(result.current.getHeight('msg-1')).toBe(150);
		expect(result.current.getTotalHeight(['msg-1'])).toBe(150);
	});
});
