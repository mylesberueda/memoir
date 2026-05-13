'use server';

import { getAuthProvider } from '@lib/auth';
import { setOrganizationContext } from '@lib/grpc/transport';
import { authLogger } from '@lib/logger';
import { deleteSession, getSession, updateSession } from '@lib/session';
import { decodeJwt } from 'jose';
import { redirect } from 'next/navigation';

const REDIRECT_URI = process.env.NEXT_PUBLIC_APP_URL
	? `${process.env.NEXT_PUBLIC_APP_URL}/api/auth/callback`
	: 'http://localhost:3000/api/auth/callback';

interface LoginResult {
	success: boolean;
	error?: string;
	errorCode?: string;
	callbackUrl?: string;
}

/**
 * Server action to handle user login via auth provider.
 *
 * The browser starts the flow via /oauth/v2/authorize, which redirects to our login page
 * with an authRequest. We authenticate the user and finalize the auth request, returning
 * a callback URL containing the authorization code.
 */
export async function login(email: string, password: string, authRequest: string): Promise<LoginResult> {
	const provider = getAuthProvider();

	const result = await provider.login(email, password, { authRequest });

	if (!result.success) {
		authLogger.error('Login failed', { error: result.error, email });
		return {
			success: false,
			error: result.error,
			errorCode: 'LOGIN_FAILED',
		};
	}

	// Provider returns either redirect URL or tokens directly
	if (result.data.type === 'redirect') {
		return {
			success: true,
			callbackUrl: result.data.url,
		};
	}

	// Direct token flow - should not happen with Zitadel, but handle it
	authLogger.warn('Unexpected direct token flow from provider', { email });
	return {
		success: false,
		error: 'Unexpected authentication flow. Please try again.',
		errorCode: 'LOGIN_FAILED',
	};
}

/**
 * Server action to handle user logout.
 * Removes session from Redis and optionally from Zitadel.
 */
export async function logout(): Promise<void> {
	// Note: We could also call deleteZitadelSession here if we stored
	// the Zitadel session ID, but for now we just clear our Redis session.
	await deleteSession();
	await setOrganizationContext(null);
	redirect('/');
}

// Refresh buffer: refresh tokens 5 minutes before expiry
const REFRESH_BUFFER_SECONDS = 5 * 60;

/**
 * Refresh the session tokens if expired, expiring soon, or force refresh is requested.
 * Returns the (possibly refreshed) access token, or null if refresh failed.
 *
 * Force refresh is triggered by webhook when user metadata changes (e.g., tier update).
 * This ensures the user gets a new JWT with updated claims without logging out.
 */
async function ensureValidToken(): Promise<{ accessToken: string; userId: string } | null> {
	const session = await getSession();
	if (!session) {
		// Session not in Redis (expired/deleted) but cookie still exists - clean it up
		authLogger.warn('Session not found in Redis, cleaning up orphaned cookie');
		await deleteSession();
		return null;
	}

	const now = Math.floor(Date.now() / 1000);
	const tokenExpiring = session.expiresAt <= now + REFRESH_BUFFER_SECONDS;
	const needsRefresh = tokenExpiring || session.forceRefresh === true;

	if (!needsRefresh) {
		return { accessToken: session.accessToken, userId: session.userId };
	}

	// Token expired/expiring or force refresh requested - attempt refresh
	if (!session.refreshToken) {
		authLogger.warn('Token refresh needed but no refresh token available', {
			userId: session.userId,
			forceRefresh: session.forceRefresh,
		});
		await deleteSession();
		return null;
	}

	try {
		if (session.forceRefresh) {
			authLogger.info('Force refreshing tokens due to metadata change', { userId: session.userId });
		}

		const provider = getAuthProvider();
		const result = await provider.refreshTokens(session.refreshToken);

		if (!result.success) {
			throw new Error(result.error);
		}

		const tokens = result.data;

		await updateSession({
			accessToken: tokens.accessToken,
			refreshToken: tokens.refreshToken,
			// id_token may not be returned on refresh, keep existing if not present
			...(tokens.idToken && { idToken: tokens.idToken }),
			expiresAt: Math.floor(Date.now() / 1000) + tokens.expiresIn,
			forceRefresh: false, // Clear the flag after successful refresh
		});

		return { accessToken: tokens.accessToken, userId: session.userId };
	} catch (error) {
		authLogger.error('Token refresh failed', { error, userId: session.userId });
		await deleteSession();
		return null;
	}
}

/**
 * Server action to get current authenticated user.
 * Returns user data or null if not authenticated.
 * Automatically refreshes tokens if expired.
 *
 * Self-healing: Clears invalid sessions to prevent redirect loops between
 * proxy middleware (which validates cookie signature) and app layout
 * (which validates session data and claims).
 */
export async function getCurrentUser() {
	try {
		const result = await ensureValidToken();
		if (!result) {
			return null;
		}

		const session = await getSession();
		if (!session?.idToken) {
			authLogger.warn('Session missing idToken', { userId: result.userId });
			await deleteSession();
			return null;
		}

		try {
			const decoded = decodeJwt(session.idToken);
			const email = decoded.email as string | undefined;
			const name = decoded.name as string | undefined;

			if (!email || !name) {
				authLogger.warn('idToken missing required claims', {
					userId: result.userId,
					hasEmail: !!email,
					hasName: !!name,
				});
				await deleteSession();
				return null;
			}

			return {
				id: result.userId,
				email,
				name,
			};
		} catch (error) {
			authLogger.error('Failed to decode idToken', { error, userId: result.userId });
			await deleteSession();
			return null;
		}
	} catch (error) {
		authLogger.error('getCurrentUser failed unexpectedly', { error });
		await deleteSession();
		return null;
	}
}

/**
 * Get the current session's access token for making authenticated API calls.
 * Automatically refreshes tokens if expired.
 */
export async function getAccessToken(): Promise<string | null> {
	try {
		const result = await ensureValidToken();
		return result?.accessToken ?? null;
	} catch (error) {
		authLogger.error('getAccessToken failed', { error });
		return null;
	}
}

/**
 * Get the current session's ID token for identity propagation to microservices.
 * Contains user profile claims (email, name, roles, etc.).
 * Automatically refreshes tokens if expired.
 */
export async function getIdToken(): Promise<string | null> {
	try {
		// Ensure tokens are valid (may trigger refresh)
		const result = await ensureValidToken();
		if (!result) {
			return null;
		}

		// Get the (possibly refreshed) session to retrieve id_token
		const session = await getSession();
		return session?.idToken ?? null;
	} catch (error) {
		authLogger.error('getIdToken failed', { error });
		return null;
	}
}

/**
 * Server action to register a new user.
 * Creates the user via auth provider and triggers email verification.
 */
export async function register(
	email: string,
	password: string,
	name: string,
): Promise<{ success: boolean; error?: string; message?: string; userId?: string }> {
	const provider = getAuthProvider();
	const result = await provider.createUser(email, password, name);

	if (!result.success) {
		authLogger.error('Registration failed', { error: result.error, email });
		return {
			success: false,
			error: result.error,
		};
	}

	return {
		success: true,
		message: 'Account created. Please check your email to verify your account.',
		userId: result.data.userId,
	};
}

/**
 * Server action to verify user email.
 * Note: With Zitadel, email verification is typically handled through Zitadel's flows.
 *
 * This is a placeholder that will need to be implemented based on your Zitadel setup.
 */
export async function verifyEmail(_token: string): Promise<{ success: boolean; error?: string }> {
	// TODO: Implement using Zitadel User Management API
	return {
		success: false,
		error: 'Email verification is handled through Zitadel. Please check your email for the verification link.',
	};
}

/**
 * Get the OIDC authorize URL to start the authentication flow.
 * This should be used when the login page is accessed directly (no authRequest).
 * Stores PKCE code_verifier in cookie for callback handler.
 */
export async function startOidcFlow(): Promise<string> {
	const provider = getAuthProvider();
	const result = await provider.startOidcFlow(REDIRECT_URI);
	return result.url;
}
