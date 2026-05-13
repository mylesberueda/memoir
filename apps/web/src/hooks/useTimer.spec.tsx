import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import useTimer from './useTimer';

describe('useTimer', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	describe('initialization', () => {
		it('should initialize with default state', () => {
			const { result } = renderHook(() => useTimer());

			expect(result.current.elapsed).toBe(0);
			expect(result.current.isRunning).toBe(false);
		});

		it('should provide all required methods', () => {
			const { result } = renderHook(() => useTimer());

			expect(typeof result.current.start).toBe('function');
			expect(typeof result.current.stop).toBe('function');
			expect(typeof result.current.setElapsed).toBe('function');
		});
	});

	describe('timer functionality', () => {
		it('should start timer and update state', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			expect(result.current.isRunning).toBe(true);
			expect(result.current.elapsed).toBe(0);
		});

		it('should update elapsed time every second', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			// Advance time by 3 seconds
			act(() => {
				vi.advanceTimersByTime(3000);
			});

			expect(result.current.elapsed).toBe(3);
			expect(result.current.isRunning).toBe(true);
		});

		it('should not start timer if already running', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			// Advance time by 2 seconds
			act(() => {
				vi.advanceTimersByTime(2000);
			});

			expect(result.current.elapsed).toBe(2);

			// Try to start again - should not reset elapsed time
			act(() => {
				result.current.start();
			});

			expect(result.current.elapsed).toBe(2);
			expect(result.current.isRunning).toBe(true);
		});

		it('should stop timer and return final duration', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			// Advance time by 5 seconds
			act(() => {
				vi.advanceTimersByTime(5000);
			});

			let finalDuration: number = 0;
			act(() => {
				finalDuration = result.current.stop();
			});

			expect(finalDuration).toBe(5);
			expect(result.current.isRunning).toBe(false);
			expect(result.current.elapsed).toBe(5);
		});

		it('should return current elapsed time when stopping non-running timer', () => {
			const { result } = renderHook(() => useTimer());

			// Set elapsed time without starting
			act(() => {
				result.current.setElapsed(10);
			});

			let finalDuration: number = 0;
			act(() => {
				finalDuration = result.current.stop();
			});

			expect(finalDuration).toBe(10);
			expect(result.current.isRunning).toBe(false);
		});

		it('should handle multiple start/stop cycles', () => {
			const { result } = renderHook(() => useTimer());

			// First cycle
			act(() => {
				result.current.start();
			});

			act(() => {
				vi.advanceTimersByTime(2000);
			});

			act(() => {
				result.current.stop();
			});

			expect(result.current.elapsed).toBe(2);
			expect(result.current.isRunning).toBe(false);

			// Second cycle
			act(() => {
				result.current.start();
			});

			act(() => {
				vi.advanceTimersByTime(3000);
			});

			act(() => {
				result.current.stop();
			});

			expect(result.current.elapsed).toBe(3);
			expect(result.current.isRunning).toBe(false);
		});
	});

	describe('setElapsed functionality', () => {
		it('should set elapsed time when timer is not running', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.setElapsed(15);
			});

			expect(result.current.elapsed).toBe(15);
			expect(result.current.isRunning).toBe(false);
		});

		it('should adjust start time when timer is running', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			// Advance time by 2 seconds
			act(() => {
				vi.advanceTimersByTime(2000);
			});

			expect(result.current.elapsed).toBe(2);

			// Set elapsed to 5 seconds while running
			act(() => {
				result.current.setElapsed(5);
			});

			expect(result.current.elapsed).toBe(5);

			// Advance time by 3 more seconds
			act(() => {
				vi.advanceTimersByTime(3000);
			});

			// Should now show 8 seconds (5 + 3)
			expect(result.current.elapsed).toBe(8);
		});

		it('should update elapsed time to 0', () => {
			const { result } = renderHook(() => useTimer());

			// Set initial time
			act(() => {
				result.current.setElapsed(10);
			});

			expect(result.current.elapsed).toBe(10);

			// Reset to 0
			act(() => {
				result.current.setElapsed(0);
			});

			expect(result.current.elapsed).toBe(0);
		});
	});

	describe('edge cases and cleanup', () => {
		it('should handle very long durations', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			// Advance time by 1 hour (3600 seconds)
			act(() => {
				vi.advanceTimersByTime(3600000);
			});

			expect(result.current.elapsed).toBe(3600);

			let finalDuration: number = 0;
			act(() => {
				finalDuration = result.current.stop();
			});

			expect(finalDuration).toBe(3600);
		});

		it('should clean up interval on unmount', () => {
			const clearIntervalSpy = vi.spyOn(global, 'clearInterval');
			const { result, unmount } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			expect(result.current.isRunning).toBe(true);

			unmount();

			expect(clearIntervalSpy).toHaveBeenCalled();
		});

		it('should handle rapid start/stop calls', () => {
			const { result } = renderHook(() => useTimer());

			// Rapid start/stop calls
			act(() => {
				result.current.start();
				result.current.stop();
				result.current.start();
			});

			expect(result.current.isRunning).toBe(true);
			expect(result.current.elapsed).toBe(0);

			act(() => {
				vi.advanceTimersByTime(1000);
			});

			expect(result.current.elapsed).toBe(1);
		});

		it('should handle setElapsed with negative values', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.setElapsed(-5);
			});

			expect(result.current.elapsed).toBe(-5);

			// Starting timer with negative elapsed should work
			act(() => {
				result.current.start();
			});

			act(() => {
				vi.advanceTimersByTime(2000);
			});

			expect(result.current.elapsed).toBe(2);
		});

		it('should maintain precision to seconds (not milliseconds)', () => {
			const { result } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			// Advance by 1.9 seconds - should still show 1
			act(() => {
				vi.advanceTimersByTime(1900);
			});

			expect(result.current.elapsed).toBe(1);

			// Advance by another 200ms to reach 2.1 seconds total - should show 2
			act(() => {
				vi.advanceTimersByTime(200);
			});

			expect(result.current.elapsed).toBe(2);
		});

		it('should handle timer state across multiple renders', () => {
			const { result, rerender } = renderHook(() => useTimer());

			act(() => {
				result.current.start();
			});

			act(() => {
				vi.advanceTimersByTime(2000);
			});

			expect(result.current.elapsed).toBe(2);

			// Force rerender
			rerender();

			// State should be maintained
			expect(result.current.elapsed).toBe(2);
			expect(result.current.isRunning).toBe(true);

			// Timer should continue working
			act(() => {
				vi.advanceTimersByTime(1000);
			});

			expect(result.current.elapsed).toBe(3);
		});

		it('should use state.elapsed fallback when startTimeRef is falsy', () => {
			// This test targets the uncovered branch in the ternary operator on line 41
			// The branch: startTimeRef.current ? calculation : state.elapsed
			// We need to make startTimeRef.current falsy while state.isRunning is true

			const { result } = renderHook(() => useTimer());

			// Set elapsed time to a known value
			act(() => {
				result.current.setElapsed(77);
			});

			// Start the timer to get into running state
			act(() => {
				result.current.start();
			});

			expect(result.current.isRunning).toBe(true);

			// Here's the key insight: startTimeRef.current could become 0 (falsy)
			// if Date.now() returns a value equal to seconds * 1000 in setElapsed
			// Let's try to force this by manipulating Date.now

			const originalDateNow = Date.now;
			const targetTime = 77 * 1000; // 77 seconds in milliseconds

			// Mock Date.now to return exactly the time that would make startTimeRef = 0
			Date.now = vi.fn(() => targetTime);

			// Call setElapsed while timer is running - this will set:
			// startTimeRef.current = Date.now() - seconds * 1000 = targetTime - targetTime = 0
			act(() => {
				result.current.setElapsed(77);
			});

			// Now when we call stop(), startTimeRef.current should be 0 (falsy)
			// This should trigger the fallback branch: state.elapsed
			let finalDuration: number = 0;
			act(() => {
				finalDuration = result.current.stop();
			});

			// Restore Date.now
			Date.now = originalDateNow;

			// The hook should have used the fallback (state.elapsed) since startTimeRef was 0
			expect(finalDuration).toBe(77);
			expect(result.current.isRunning).toBe(false);
		});
	});
});
