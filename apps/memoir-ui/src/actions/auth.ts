'use server';

import { getAuthProvider } from '@lib/auth';
import { setOrganizationContext } from '@lib/grpc/transport';
import { authLogger } from '@lib/logger';
import { createSession, deleteSession, getSession, updateSession } from '@lib/session';
import { redirect } from 'next/navigation';

interface LoginResult {
	success: boolean;
	error?: string;
	errorCode?: string;
}

/**
 * Server action to handle user login via the configured AuthProvider.
 *
 * Flow: provider verifies username + password against memoir-service and
 * returns a JWT access/refresh pair. We persist the tokens in a server-side
 * Redis session keyed by an HttpOnly cookie, then signal success so the
 * client can navigate to the post-login destination.
 *
 * The `_authRequest` parameter is preserved for signature compatibility
 * with the previous Zitadel OIDC flow but is unused — AuthService has no
 * concept of an authorization-request handoff.
 */
export async function login(email: string, password: string, _authRequest?: string): Promise<LoginResult> {
	const provider = getAuthProvider();

	const result = await provider.login(email, password);

	if (!result.success) {
		authLogger.error('Login failed', { error: result.error, email });
		return {
			success: false,
			error: result.error,
			errorCode: 'LOGIN_FAILED',
		};
	}

	if (result.data.type !== 'tokens') {
		// AuthService only returns the direct-token flow; a redirect here
		// would mean a misconfigured provider.
		authLogger.error('Unexpected login flow shape from provider', {
			email,
			flow: result.data.type,
		});
		return {
			success: false,
			error: 'Unexpected authentication flow.',
			errorCode: 'LOGIN_FAILED',
		};
	}

	const { tokens, user } = result.data;

	await createSession({
		accessToken: tokens.accessToken,
		refreshToken: tokens.refreshToken,
		// AuthService doesn't issue an ID token. SessionData requires the
		// field; empty string is the documented "absent" signal here.
		idToken: tokens.idToken ?? '',
		userId: user.id,
		expiresAt: Math.floor(Date.now() / 1000) + tokens.expiresIn,
	});

	return { success: true };
}

/**
 * Server action to handle user logout. Clears the Redis session + cookie.
 */
export async function logout(): Promise<void> {
	await deleteSession();
	await setOrganizationContext(null);
	redirect('/');
}

// Refresh buffer: refresh tokens 5 minutes before expiry.
const REFRESH_BUFFER_SECONDS = 5 * 60;

/**
 * Refresh the session tokens if expired, expiring soon, or force refresh is requested.
 * Returns the (possibly refreshed) access token, or null if refresh failed.
 *
 * Force refresh remains in the session shape for back-compat with code that
 * sets it; AuthService has no equivalent metadata-refresh signal so it's
 * effectively only triggered by expiry today.
 */
async function ensureValidToken(): Promise<{ accessToken: string; userId: string } | null> {
	const session = await getSession();
	if (!session) {
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
			...(tokens.idToken && { idToken: tokens.idToken }),
			expiresAt: Math.floor(Date.now() / 1000) + tokens.expiresIn,
			forceRefresh: false,
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
 *
 * Returns the user's pid if a valid session exists, or null otherwise.
 * The AuthService JWT carries only `sub` (the user pid); email/name
 * aren't in the claims and aren't stored in the session, so the
 * email/name fields are always `null` here.
 *
 * Consumers that need the richer user shape should fetch it via
 * AuthService.GetUser using the access token. The UI layer at
 * `providers/AuthContextProvider.tsx` and `app/(app)/layout.tsx` currently
 * gates on `email && name` being non-null; with this return shape they'll
 * treat any AuthService-authed user as "incomplete profile" until those
 * consumers are updated to fetch the full user record.
 */
export async function getCurrentUser(): Promise<{ id: string; email: string | null; name: string | null } | null> {
	try {
		const result = await ensureValidToken();
		if (!result) {
			return null;
		}
		return { id: result.userId, email: null, name: null };
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
 * Server action to register a new user.
 *
 * NOTE: AuthService's CreateUser RPC is admin-gated. memoir-ui's
 * self-serve registration is therefore not supported in v0.1; this
 * action returns an error directing operators to use `memoir auth create`
 * or an admin UI page that calls AuthService.CreateUser directly with an
 * admin JWT attached.
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
		message: 'Account created.',
		userId: result.data.userId,
	};
}
