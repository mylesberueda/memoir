import '@testing-library/jest-dom/vitest';

import { getCurrentUser } from '@actions/auth';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React, { useContext } from 'react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { AuthContext, AuthContextProvider, type AuthContextValue, type User } from './AuthContextProvider';

// Mock the server action
vi.mock('@actions/auth', () => ({
	getCurrentUser: vi.fn(),
}));

const mockGetCurrentUser = vi.mocked(getCurrentUser);

// Test component that consumes the AuthContext
const TestConsumer = ({ onUpdate }: { onUpdate?: (authState: AuthContextValue) => void }) => {
	const authState = useContext(AuthContext);

	// Call onUpdate when authState changes (for testing purposes)
	React.useEffect(() => {
		if (onUpdate && authState) {
			onUpdate(authState);
		}
	}, [authState, onUpdate]);

	if (!authState) {
		return <div data-testid="no-context">No auth context</div>;
	}

	return (
		<div data-testid="auth-consumer">
			<div data-testid="user-id">{authState.user?.id || 'null'}</div>
			<div data-testid="user-email">{authState.user?.email || 'null'}</div>
			<div data-testid="user-name">{authState.user?.name || 'null'}</div>
			<div data-testid="is-loading">{authState.isLoading.toString()}</div>
			<div data-testid="error">{authState.error || 'null'}</div>
			<div data-testid="is-authenticated">{authState.isAuthenticated.toString()}</div>
			<button data-testid="refresh-auth" type="button" onClick={authState.refreshAuth}>
				Refresh Auth
			</button>
		</div>
	);
};

const renderWithProvider = (children: React.ReactNode) => {
	return render(<AuthContextProvider>{children}</AuthContextProvider>);
};

describe('AuthContextProvider', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	describe('initialization', () => {
		it('should provide initial loading state', () => {
			// Mock getCurrentUser to return a promise that never resolves (simulates loading)
			mockGetCurrentUser.mockImplementation(
				() => new Promise(() => {}), // Never resolves
			);

			renderWithProvider(<TestConsumer />);

			expect(screen.getByTestId('is-loading')).toHaveTextContent('true');
			expect(screen.getByTestId('user-id')).toHaveTextContent('null');
			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
			expect(screen.getByTestId('error')).toHaveTextContent('null');
		});

		it('should call getCurrentUser on mount', () => {
			mockGetCurrentUser.mockResolvedValue(null);

			renderWithProvider(<TestConsumer />);

			expect(mockGetCurrentUser).toHaveBeenCalledTimes(1);
		});
	});

	describe('successful authentication', () => {
		const mockUser: User = {
			id: 'test-user-123',
			email: 'test@example.com',
			name: 'Test User',
		};

		it('should set user data when getCurrentUser returns user', async () => {
			mockGetCurrentUser.mockResolvedValue(mockUser);

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});

			expect(screen.getByTestId('user-id')).toHaveTextContent(mockUser.id);
			expect(screen.getByTestId('user-email')).toHaveTextContent(mockUser.email);
			expect(screen.getByTestId('user-name')).toHaveTextContent(mockUser.name);
			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('true');
			expect(screen.getByTestId('error')).toHaveTextContent('null');
		});

		it('should clear error state when authentication succeeds', async () => {
			// First call fails, second succeeds
			mockGetCurrentUser.mockRejectedValueOnce(new Error('Network error')).mockResolvedValueOnce(mockUser);

			renderWithProvider(<TestConsumer />);

			// Wait for first call to complete (error state)
			await waitFor(() => {
				expect(screen.getByTestId('error')).toHaveTextContent('Network error');
			});

			// Trigger refresh
			const refreshButton = screen.getByTestId('refresh-auth');
			await userEvent.click(refreshButton);

			// Wait for second call to complete (success state)
			await waitFor(() => {
				expect(screen.getByTestId('is-authenticated')).toHaveTextContent('true');
			});

			expect(screen.getByTestId('error')).toHaveTextContent('null');
			expect(screen.getByTestId('user-id')).toHaveTextContent(mockUser.id);
		});
	});

	describe('authentication failure', () => {
		it('should handle null response from getCurrentUser', async () => {
			mockGetCurrentUser.mockResolvedValue(null);

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});

			expect(screen.getByTestId('user-id')).toHaveTextContent('null');
			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
			expect(screen.getByTestId('error')).toHaveTextContent('null');
		});

		it('should handle Error objects from getCurrentUser', async () => {
			const errorMessage = 'Authentication failed';
			mockGetCurrentUser.mockRejectedValue(new Error(errorMessage));

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('error')).toHaveTextContent(errorMessage);
			});

			expect(screen.getByTestId('user-id')).toHaveTextContent('null');
			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
			expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
		});

		it('should handle non-Error exceptions from getCurrentUser', async () => {
			mockGetCurrentUser.mockRejectedValue('String error');

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('error')).toHaveTextContent('Failed to get user');
			});

			expect(screen.getByTestId('user-id')).toHaveTextContent('null');
			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
		});

		it('should clear user state when error occurs', async () => {
			const mockUser: User = {
				id: 'test-user-123',
				email: 'test@example.com',
				name: 'Test User',
			};

			// First call succeeds, second fails
			mockGetCurrentUser.mockResolvedValueOnce(mockUser).mockRejectedValueOnce(new Error('Token expired'));

			renderWithProvider(<TestConsumer />);

			// Wait for first call to complete (success state)
			await waitFor(() => {
				expect(screen.getByTestId('is-authenticated')).toHaveTextContent('true');
			});

			// Trigger refresh
			const refreshButton = screen.getByTestId('refresh-auth');
			await userEvent.click(refreshButton);

			// Wait for second call to complete (error state)
			await waitFor(() => {
				expect(screen.getByTestId('error')).toHaveTextContent('Token expired');
			});

			expect(screen.getByTestId('user-id')).toHaveTextContent('null');
			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
		});
	});

	describe('refreshAuth functionality', () => {
		it('should refresh auth state when refreshAuth is called', async () => {
			const mockUser: User = {
				id: 'refreshed-user-456',
				email: 'refreshed@example.com',
				name: 'Refreshed User',
			};

			mockGetCurrentUser.mockResolvedValueOnce(null).mockResolvedValueOnce(mockUser);

			renderWithProvider(<TestConsumer />);

			// Wait for initial call to complete
			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});

			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');

			// Call refreshAuth
			const refreshButton = screen.getByTestId('refresh-auth');
			await userEvent.click(refreshButton);

			// Wait for refresh to complete and check final state
			await waitFor(() => {
				expect(screen.getByTestId('user-id')).toHaveTextContent(mockUser.id);
			});

			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('true');
			expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			expect(mockGetCurrentUser).toHaveBeenCalledTimes(2);
		});

		it('should handle errors during refresh', async () => {
			mockGetCurrentUser.mockResolvedValueOnce(null).mockRejectedValueOnce(new Error('Refresh failed'));

			renderWithProvider(<TestConsumer />);

			// Wait for initial call to complete
			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});

			// Call refreshAuth
			const refreshButton = screen.getByTestId('refresh-auth');
			await userEvent.click(refreshButton);

			// Wait for refresh to complete with error
			await waitFor(() => {
				expect(screen.getByTestId('error')).toHaveTextContent('Refresh failed');
			});

			expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
			expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
		});

		it('should show loading state during transition', async () => {
			let resolvePromise: ((value: User | null) => void) | undefined;
			const promise = new Promise<User | null>((resolve) => {
				resolvePromise = resolve;
			});

			mockGetCurrentUser.mockReturnValue(promise);

			renderWithProvider(<TestConsumer />);

			// Should be loading initially
			expect(screen.getByTestId('is-loading')).toHaveTextContent('true');

			// Resolve the promise
			resolvePromise?.(null);

			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});
		});
	});

	describe('isAuthenticated logic', () => {
		it('should return false when user is null', async () => {
			mockGetCurrentUser.mockResolvedValue(null);

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
			});
		});

		it('should return false when there is an error', async () => {
			mockGetCurrentUser.mockRejectedValue(new Error('Auth error'));

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('is-authenticated')).toHaveTextContent('false');
			});
		});

		it('should return true when user exists and no error', async () => {
			const mockUser: User = {
				id: 'auth-user-789',
				email: 'auth@example.com',
				name: 'Auth User',
			};

			mockGetCurrentUser.mockResolvedValue(mockUser);

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('is-authenticated')).toHaveTextContent('true');
			});
		});
	});

	describe('loading state management', () => {
		it('should combine isLoading and isPending states', async () => {
			const mockUser: User = {
				id: 'loading-user-101',
				email: 'loading@example.com',
				name: 'Loading User',
			};

			// Create a controlled promise for the refresh call
			let resolveRefresh: ((value: User) => void) | undefined;
			const refreshPromise = new Promise<User>((resolve) => {
				resolveRefresh = resolve;
			});

			mockGetCurrentUser
				.mockResolvedValueOnce(mockUser) // Initial load
				.mockReturnValueOnce(refreshPromise); // Refresh call

			renderWithProvider(<TestConsumer />);

			// Wait for initial load to complete
			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});

			// Trigger refresh (this will use transition and show loading)
			const refreshButton = screen.getByTestId('refresh-auth');
			await userEvent.click(refreshButton);

			// Should show loading during transition
			expect(screen.getByTestId('is-loading')).toHaveTextContent('true');

			// Resolve the refresh
			resolveRefresh?.(mockUser);

			await waitFor(() => {
				expect(screen.getByTestId('is-loading')).toHaveTextContent('false');
			});
		});
	});

	describe('context consumption', () => {
		it('should provide null when no provider exists', () => {
			render(<TestConsumer />);

			expect(screen.getByTestId('no-context')).toBeInTheDocument();
		});

		it('should provide auth context when provider exists', async () => {
			mockGetCurrentUser.mockResolvedValue(null);

			renderWithProvider(<TestConsumer />);

			await waitFor(() => {
				expect(screen.getByTestId('auth-consumer')).toBeInTheDocument();
			});

			expect(screen.queryByTestId('no-context')).not.toBeInTheDocument();
		});
	});

	describe('multiple consumers', () => {
		it('should provide same context to multiple consumers', async () => {
			const mockUser: User = {
				id: 'multi-user-202',
				email: 'multi@example.com',
				name: 'Multi User',
			};

			mockGetCurrentUser.mockResolvedValue(mockUser);

			renderWithProvider(
				<div>
					<TestConsumer />
					<div data-testid="second-consumer">
						<TestConsumer />
					</div>
				</div>,
			);

			await waitFor(() => {
				expect(screen.getAllByTestId('is-authenticated')).toHaveLength(2);
			});

			const authenticatedElements = screen.getAllByTestId('is-authenticated');
			authenticatedElements.forEach((element) => {
				expect(element).toHaveTextContent('true');
			});

			const userIdElements = screen.getAllByTestId('user-id');
			userIdElements.forEach((element) => {
				expect(element).toHaveTextContent(mockUser.id);
			});
		});
	});
});
