/**
 * Authentication types for auth provider abstraction.
 *
 * Web app performs minimal user extraction for UI display.
 * Services (using common-rs) do full user extraction with UserExtractor trait.
 */

/**
 * Normalized token response from all auth providers.
 * All providers must normalize their token responses to this format.
 */
export interface TokenResponse {
	/** Access token (JWT) for API authentication */
	accessToken: string;
	/** Refresh token for obtaining new access tokens */
	refreshToken: string;
	/** Optional ID token (some providers don't issue) */
	idToken: string | null;
	/** Seconds until access token expires */
	expiresIn: number;
	/** Token type (usually "Bearer") */
	tokenType: string;
}

/**
 * Minimal user info for UI display.
 * Services extract full User with tier, org_roles, metadata via common-rs.
 */
export interface AuthUser {
	/** User identifier from 'sub' claim */
	id: string;
	/** Email address (optional) */
	email: string | null;
	/** Display name (optional) */
	name: string | null;
}

/**
 * Login result - discriminated union for different authentication flow types.
 */
export type LoginResult =
	/** OIDC redirect flow - browser must redirect to returned URL */
	| { type: 'redirect'; url: string }
	/** Direct token flow - tokens returned immediately */
	| { type: 'tokens'; tokens: TokenResponse; user: AuthUser };

/**
 * Optional context for provider-specific login parameters.
 * Allows Zitadel to pass authRequest without polluting other providers.
 */
export interface AuthContext {
	/** Zitadel auth request ID from authorize redirect */
	authRequest?: string;
	/** Additional provider-specific parameters */
	[key: string]: unknown;
}
