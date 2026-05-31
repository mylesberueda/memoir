import 'server-only';

import type { ActionResult } from '@actions';
import { authClient } from '@/lib/grpc/client';
import { AuthProvider } from '../base-provider';
import type { AuthContext, AuthUser, LoginResult, TokenResponse } from '../types';

/**
 * Auth provider backed by memoir-service's AuthService gRPC surface.
 *
 * Login flow: username + password → access JWT (15min) + refresh JWT (7d).
 * Subsequent service calls attach the access JWT in
 * `authorization: Bearer <jwt>`. When it expires, the refresh JWT
 * exchanges for a fresh access JWT without re-prompting the user.
 *
 * The `AuthProvider` interface was originally shaped around an OIDC
 * provider with a redirect handoff. Methods that have no analog in
 * AuthService — startOidcFlow, exchangeCodeForTokens, changePassword —
 * throw "not supported" so any caller hitting them fails loudly rather
 * than silently misbehaving.
 */
export class AuthServiceAuthProvider extends AuthProvider {
	override async login(email: string, password: string, _context?: AuthContext): Promise<ActionResult<LoginResult>> {
		try {
			const response = await authClient().login({
				username: email,
				password,
			});

			if (!response.user) {
				return { success: false, error: 'login response did not include a user' };
			}

			const tokens: TokenResponse = {
				accessToken: response.accessToken,
				refreshToken: response.refreshToken,
				idToken: null,
				// AuthService's access TTL is 15min; mirror that here so the
				// session-refresh path triggers in time. If the backend
				// default changes, this is the place to thread the new value
				// through (or surface it in the proto response).
				expiresIn: 15 * 60,
				tokenType: 'Bearer',
			};

			const user: AuthUser = {
				id: response.user.pid,
				email: response.user.username,
				name: response.user.username,
			};

			return {
				success: true,
				data: { type: 'tokens', tokens, user },
			};
		} catch (err) {
			return { success: false, error: err instanceof Error ? err.message : 'login failed' };
		}
	}

	override async refreshTokens(refreshToken: string): Promise<ActionResult<TokenResponse>> {
		try {
			const response = await authClient().refreshToken({ refreshToken });
			return {
				success: true,
				data: {
					accessToken: response.accessToken,
					// The refresh token is unchanged — the client reuses the
					// one it already has until it expires.
					refreshToken,
					idToken: null,
					expiresIn: 15 * 60,
					tokenType: 'Bearer',
				},
			};
		} catch (err) {
			return { success: false, error: err instanceof Error ? err.message : 'refresh failed' };
		}
	}

	override async createUser(
		_email: string,
		_password: string,
		_displayName: string,
	): Promise<ActionResult<{ userId: string }>> {
		// CreateUser is admin-gated on the backend. memoir-ui's "create
		// user" flow is operator-only and must attach an admin JWT in
		// the authorization header. The caller is responsible for that;
		// this method assumes the cookie/context has already been
		// converted into a bearer header upstream.
		//
		// For v0.1, throwing here forces the caller to make that
		// upstream concern explicit (a self-serve registration flow is
		// out of scope; operators create users via `memoir auth create`
		// or via an authenticated admin UI page that calls authClient()
		// directly with the admin's JWT attached).
		return {
			success: false,
			error:
				'createUser from the AuthServiceAuthProvider is not yet implemented; use `memoir auth create` or call AuthService.CreateUser directly with an admin JWT',
		};
	}

	override async startOidcFlow(_redirectUri: string, _scope?: string): Promise<{ url: string }> {
		throw new Error('OIDC flow is not supported by AuthService; use login(username, password) instead');
	}

	override async exchangeCodeForTokens(
		_code: string,
		_redirectUri: string,
		_codeVerifier: string,
	): Promise<ActionResult<TokenResponse>> {
		return {
			success: false,
			error: 'OIDC code exchange is not supported by AuthService',
		};
	}

	override async changePassword(
		_userId: string,
		_currentPassword: string,
		_newPassword: string,
	): Promise<ActionResult<void>> {
		// AuthService has no password-change RPC in v0.1. Adding one
		// requires a proto change + handler that re-verifies the current
		// password before re-hashing the new one.
		return {
			success: false,
			error: 'changePassword is not yet supported by AuthService',
		};
	}

	override extractUser(_tokens: TokenResponse): AuthUser {
		// AuthService JWTs carry only `sub` (the user pid). The login
		// response includes the user separately, and the session storage
		// is keyed by user pid, so this method is effectively unused for
		// AuthService — but the interface requires it. Decode the sub
		// from the access token as a best-effort.
		//
		// Note: this does NOT verify the JWT signature. memoir-ui treats
		// its own session cookie as trusted; signature verification
		// happens server-side when the token is forwarded to
		// memoir-service.
		const sub = decodeJwtSub(_tokens.accessToken);
		return {
			id: sub ?? '',
			email: null,
			name: null,
		};
	}
}

/** Decode the `sub` claim from a JWT without signature verification. */
function decodeJwtSub(token: string): string | null {
	const parts = token.split('.');
	if (parts.length < 2) return null;
	try {
		const payload = JSON.parse(Buffer.from(parts[1], 'base64url').toString('utf8'));
		return typeof payload.sub === 'string' ? payload.sub : null;
	} catch {
		return null;
	}
}
