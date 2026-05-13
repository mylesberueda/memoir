import { AuthContext } from '@providers/AuthContextProvider';
import { renderHook } from '@testing-library/react';
import { useContext } from 'react';
import { describe, expect, it, vi } from 'vitest';
import useAuth from './useAuth';

// Mock useContext
vi.mock('react', async () => {
	const actual = await vi.importActual('react');
	return {
		...actual,
		useContext: vi.fn(),
	};
});

const mockUseContext = vi.mocked(useContext);

describe('useAuth', () => {
	it('should return default unauthenticated state when no context provider exists', () => {
		mockUseContext.mockReturnValue(null);

		const { result } = renderHook(() => useAuth());

		expect(result.current).toEqual({
			user: null,
			isLoading: false,
			error: null,
			isAuthenticated: false,
			refreshAuth: expect.any(Function),
		});
	});

	it('should return context value when AuthContextProvider exists', () => {
		const mockContextValue = {
			user: { id: '123', email: 'test@example.com', name: 'Test User' },
			isLoading: false,
			error: null,
			isAuthenticated: true,
			refreshAuth: vi.fn(),
		};

		mockUseContext.mockReturnValue(mockContextValue);

		const { result } = renderHook(() => useAuth());

		expect(result.current).toBe(mockContextValue);
	});

	it('should return context value when user is loading', () => {
		const mockContextValue = {
			user: null,
			isLoading: true,
			error: null,
			isAuthenticated: false,
			refreshAuth: vi.fn(),
		};

		mockUseContext.mockReturnValue(mockContextValue);

		const { result } = renderHook(() => useAuth());

		expect(result.current).toBe(mockContextValue);
	});

	it('should return context value when there is an error', () => {
		const mockContextValue = {
			user: null,
			isLoading: false,
			error: 'Authentication failed',
			isAuthenticated: false,
			refreshAuth: vi.fn(),
		};

		mockUseContext.mockReturnValue(mockContextValue);

		const { result } = renderHook(() => useAuth());

		expect(result.current).toBe(mockContextValue);
	});

	it('should return no-op refreshAuth function when no context exists', () => {
		mockUseContext.mockReturnValue(null);

		const { result } = renderHook(() => useAuth());

		// Should not throw when called
		expect(() => result.current.refreshAuth()).not.toThrow();
	});

	it('should call the correct useContext hook with AuthContext', () => {
		mockUseContext.mockReturnValue(null);

		renderHook(() => useAuth());

		expect(mockUseContext).toHaveBeenCalledWith(AuthContext);
	});
});
