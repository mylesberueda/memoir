import { AuthContext } from '@providers/AuthContextProvider';
import { useContext } from 'react';

/**
 * Hook to access the auth context
 * Returns default unauthenticated state if no provider exists
 */
export default function useAuth() {
	const context = useContext(AuthContext);

	// If no AuthContextProvider exists (e.g., in auth routes), return default unauthenticated state
	if (!context) {
		return {
			user: null,
			isLoading: false,
			error: null,
			isAuthenticated: false,
			refreshAuth: () => {},
		};
	}

	return context;
}
