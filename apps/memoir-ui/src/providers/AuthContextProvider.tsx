'use client';

import { getCurrentUser } from '@actions/auth';
import type React from 'react';
import { createContext, useCallback, useEffect, useState, useTransition } from 'react';

export interface User {
	id: string;
	email: string | null;
	name: string | null;
}

export interface AuthContextValue {
	user: User | null;
	isLoading: boolean;
	error: string | null;
	isAuthenticated: boolean;
	refreshAuth: () => void;
}

export const AuthContext = createContext<AuthContextValue | null>(null);

interface AuthContextProviderProps {
	children: React.ReactNode;
	user?: User | null;
}

/**
 * AuthContextProvider that uses server actions for authentication
 * This provides auth state across the app without API round trips
 */
export function AuthContextProvider({ children, user = null }: AuthContextProviderProps) {
	const [user_, setUser] = useState<User | null>(user);
	const [isLoading, setIsLoading] = useState(!user);
	const [error, setError] = useState<string | null>(null);
	const [isPending, startTransition] = useTransition();

	const fetchUser = useCallback(async () => {
		try {
			setError(null);
			const userData = await getCurrentUser();
			// Only set user if they have required fields
			if (userData?.email && userData.name) {
				setUser({
					id: userData.id,
					email: userData.email,
					name: userData.name,
				});
			} else {
				setUser(null);
			}
		} catch (err) {
			setError(err instanceof Error ? err.message : 'Failed to get user');
			setUser(null);
		} finally {
			setIsLoading(false);
		}
	}, []);

	const refreshAuth = () => {
		startTransition(() => {
			setIsLoading(true);
			fetchUser();
		});
	};

	// Initialize auth state on mount only if no initial user provided
	useEffect(() => {
		// Skip fetching if we already have the user from server
		if (!user) {
			fetchUser();
		}
	}, [fetchUser, user]);

	const value: AuthContextValue = {
		user: user_,
		isLoading: isLoading || isPending,
		error,
		isAuthenticated: !!user_ && !error,
		refreshAuth,
	};

	return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
