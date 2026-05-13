import 'server-only';

import type { ActionResult } from '@actions';
import type { SessionData } from '@lib/session';
import type { AuthContext, AuthUser, LoginResult, TokenResponse } from './types';

/**
 * Base class for authentication providers.
 *
 * Providers extend this class and implement abstract methods.
 * Default implementations are provided for common operations (storeSession, logout).
 */
export abstract class AuthProvider {
	// ========== Abstract Methods (provider must implement) ==========

	/**
	 * Authenticate user with email and password.
	 *
	 * @param email - User's email address
	 * @param password - User's password
	 * @param context - Optional provider-specific context (e.g., Zitadel authRequest)
	 * @returns LoginResult (either redirect URL or tokens directly)
	 *
	 * Example (Zitadel - redirect flow):
	 *   { success: true, data: { type: 'redirect', url: 'https://...' } }
	 *
	 * Example (Supabase - direct flow):
	 *   { success: true, data: { type: 'tokens', tokens: {...}, user: {...} } }
	 */
	abstract login(email: string, password: string, context?: AuthContext): Promise<ActionResult<LoginResult>>;

	/**
	 * Start OIDC authorization flow.
	 * Generates authorize URL and stores PKCE state.
	 *
	 * @param redirectUri - Callback URL after authorization
	 * @param scope - OAuth scopes to request (optional)
	 * @returns Authorize URL to redirect user's browser to
	 */
	abstract startOidcFlow(redirectUri: string, scope?: string): Promise<{ url: string }>;

	/**
	 * Exchange authorization code for tokens (OIDC callback).
	 *
	 * @param code - Authorization code from provider callback
	 * @param redirectUri - Same redirect URI used in authorize request
	 * @param codeVerifier - PKCE code verifier from stored state
	 * @returns Normalized token response
	 */
	abstract exchangeCodeForTokens(
		code: string,
		redirectUri: string,
		codeVerifier: string,
	): Promise<ActionResult<TokenResponse>>;

	/**
	 * Refresh access token using refresh token.
	 *
	 * @param refreshToken - Valid refresh token
	 * @returns New token response with refreshed access token
	 */
	abstract refreshTokens(refreshToken: string): Promise<ActionResult<TokenResponse>>;

	/**
	 * Register a new user.
	 *
	 * @param email - User's email address
	 * @param password - User's password
	 * @param displayName - User's display name
	 * @returns Created user ID
	 */
	abstract createUser(email: string, password: string, displayName: string): Promise<ActionResult<{ userId: string }>>;

	/**
	 * Change user's password.
	 *
	 * @param userId - User ID
	 * @param currentPassword - Current password for verification
	 * @param newPassword - New password to set
	 */
	abstract changePassword(userId: string, currentPassword: string, newPassword: string): Promise<ActionResult<void>>;

	/**
	 * Extract minimal user info from tokens for UI display.
	 * Services do full extraction using common-rs UserExtractor.
	 *
	 * @param tokens - Token response from provider
	 * @returns Minimal user info (id, email, name)
	 */
	abstract extractUser(tokens: TokenResponse): AuthUser;

	// ========== Default Implementation (providers can override) ==========

	/**
	 * Store tokens in Redis session.
	 * Default implementation writes to session storage.
	 * Providers can override if needed.
	 *
	 * @param sessionId - Session identifier
	 * @param tokens - Token response to store
	 * @param userId - User ID extracted from tokens
	 */
	async storeSession(sessionId: string, tokens: TokenResponse, userId: string): Promise<void> {
		// Dynamic import to avoid circular dependency
		const { setSessionById } = await import('@lib/session');

		const sessionData: SessionData = {
			accessToken: tokens.accessToken,
			refreshToken: tokens.refreshToken,
			idToken: tokens.idToken ?? '',
			userId,
			expiresAt: Math.floor(Date.now() / 1000) + tokens.expiresIn,
		};

		await setSessionById(sessionId, sessionData);
	}

	/**
	 * End provider session (optional).
	 * Default implementation does nothing.
	 * Providers can override to call provider-specific logout API.
	 *
	 * @param sessionId - Provider session ID (optional)
	 * @param sessionToken - Provider session token (optional)
	 */
	async logout(_sessionId?: string, _sessionToken?: string): Promise<ActionResult<void>> {
		return { success: true, data: undefined };
	}
}
