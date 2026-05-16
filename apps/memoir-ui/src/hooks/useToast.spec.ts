/**
 * @vitest-environment jsdom
 */

import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import useToast from './useToast';

// Mock setTimeout
vi.useFakeTimers();

describe('useToast', () => {
	beforeEach(() => {
		vi.clearAllTimers();
		// Clear global toast state between tests
		const { result } = renderHook(() => useToast());
		act(() => {
			result.current.clearAll();
		});
	});

	it('should initialize with empty toasts array', () => {
		const { result } = renderHook(() => useToast());
		expect(result.current.toasts).toEqual([]);
	});

	it('should add a toast', () => {
		const { result } = renderHook(() => useToast());

		act(() => {
			result.current.addToast('Test message', 'success');
		});

		expect(result.current.toasts).toHaveLength(1);
		expect(result.current.toasts[0]).toMatchObject({
			message: 'Test message',
			type: 'success',
			duration: 4000,
		});
		expect(result.current.toasts[0].id).toBeTruthy();
	});

	it('should add success toast using helper method', () => {
		const { result } = renderHook(() => useToast());

		act(() => {
			result.current.success('Success message');
		});

		expect(result.current.toasts).toHaveLength(1);
		expect(result.current.toasts[0].type).toBe('success');
		expect(result.current.toasts[0].message).toBe('Success message');
	});

	it('should add error toast using helper method', () => {
		const { result } = renderHook(() => useToast());

		act(() => {
			result.current.error('Error message');
		});

		expect(result.current.toasts).toHaveLength(1);
		expect(result.current.toasts[0].type).toBe('error');
		expect(result.current.toasts[0].message).toBe('Error message');
	});

	it('should remove toast manually', () => {
		const { result } = renderHook(() => useToast());

		act(() => {
			result.current.addToast('Test message', 'info');
		});

		const toastId = result.current.toasts[0].id;

		act(() => {
			result.current.removeToast(toastId);
		});

		expect(result.current.toasts).toHaveLength(0);
	});

	it('should auto-remove toast after duration', () => {
		const { result } = renderHook(() => useToast());

		act(() => {
			result.current.addToast('Test message', 'info', 1000);
		});

		expect(result.current.toasts).toHaveLength(1);

		// Fast-forward time
		act(() => {
			vi.advanceTimersByTime(1000);
		});

		expect(result.current.toasts).toHaveLength(0);
	});

	it('should clear all toasts', () => {
		const { result } = renderHook(() => useToast());

		act(() => {
			result.current.addToast('Message 1', 'success');
			result.current.addToast('Message 2', 'error');
		});

		expect(result.current.toasts).toHaveLength(2);

		act(() => {
			result.current.clearAll();
		});

		expect(result.current.toasts).toHaveLength(0);
	});

	it('should sync toasts across multiple hook instances', () => {
		const { result: result1 } = renderHook(() => useToast());
		const { result: result2 } = renderHook(() => useToast());

		act(() => {
			result1.current.addToast('Shared message', 'warning');
		});

		expect(result1.current.toasts).toHaveLength(1);
		expect(result2.current.toasts).toHaveLength(1);
		expect(result1.current.toasts[0].id).toBe(result2.current.toasts[0].id);
	});
});
