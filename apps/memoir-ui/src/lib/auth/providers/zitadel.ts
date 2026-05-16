import 'server-only';

import { createPrivateKey, type KeyObject } from 'node:crypto';
import type { ActionResult } from '@actions';
import { decodeJwt, SignJWT } from 'jose';
import { AuthProvider } from '../base-provider';
import type { AuthContext, AuthUser, LoginResult, TokenResponse } from '../types';

// ========== Types for Zitadel API responses ==========

interface ZitadelSession {
	sessionId: string;
	sessionToken: string;
}

interface ZitadelAuthRequestCallback {
	callbackUrl: string;
}

interface ZitadelTokenResponse {
	access_token: string;
	refresh_token: string;
	token_type: string;
	expires_in: number;
	id_token?: string;
}

interface ApplicationKey {
	type: string;
	keyId: string;
	key: string;
	appId: string;
	clientId: string;
}

interface ServiceAccountKey {
	type: string;
	keyId: string;
	key: string;
	userId: string;
}

// ========== Cache for imported private keys ==========

let cachedPrivateKey: KeyObject | null = null;
let cachedApplicationKey: ApplicationKey | null = null;
let cachedServiceAccountKey: ServiceAccountKey | null = null;
let cachedServiceAccountPrivateKey: KeyObject | null = null;
let cachedServiceAccountToken: { token: string; expiresAt: number } | null = null;

/**
 * Zitadel authentication provider implementation.
 *
 * Implements multi-step OIDC flow with session-based authentication:
 * 1. Create session with username check
 * 2. Verify password
 * 3. Finalize auth request
 * 4. Exchange code for tokens
 *
 * Encapsulates Zitadel-specific details:
 * - Service account JWT bearer grants
 * - Application key management
 * - URN-namespaced claims (handled in services via common-rs)
 * - Grant auto-creation on GrantRequired errors
 */
export class ZitadelAuthProvider extends AuthProvider {
	// ========== Environment Configuration ==========

	private async getZitadelUrl(): Promise<string> {
		const url = process.env.ZITADEL_URL;
		if (!url) {
			throw new Error('ZITADEL_URL environment variable is not set');
		}
		return url;
	}

	private getProjectId(): string {
		const projectId = process.env.ZITADEL_PROJECT_ID;
		if (!projectId) {
			throw new Error('ZITADEL_PROJECT_ID environment variable is not set');
		}
		return projectId;
	}

	private async getApplicationKey(): Promise<ApplicationKey> {
		if (cachedApplicationKey) {
			return cachedApplicationKey;
		}

		const keyBase64 = process.env.ZITADEL_APPLICATION_KEY;
		if (!keyBase64) {
			throw new Error('ZITADEL_APPLICATION_KEY environment variable is not set');
		}

		try {
			const keyJson = Buffer.from(keyBase64, 'base64').toString('utf-8');
			cachedApplicationKey = JSON.parse(keyJson) as ApplicationKey;
			return cachedApplicationKey;
		} catch {
			throw new Error('ZITADEL_APPLICATION_KEY is not valid base64-encoded JSON');
		}
	}

	private async getPrivateKey(): Promise<KeyObject> {
		if (cachedPrivateKey) {
			return cachedPrivateKey;
		}

		const appKey = await this.getApplicationKey();
		cachedPrivateKey = createPrivateKey(appKey.key);
		return cachedPrivateKey;
	}

	private getServiceAccountKey(): ServiceAccountKey {
		if (cachedServiceAccountKey) {
			return cachedServiceAccountKey;
		}

		const keyBase64 = process.env.ZITADEL_SERVICE_KEY;
		if (!keyBase64) {
			throw new Error('ZITADEL_SERVICE_KEY environment variable is not set');
		}

		try {
			const keyJson = Buffer.from(keyBase64, 'base64').toString('utf-8');
			cachedServiceAccountKey = JSON.parse(keyJson) as ServiceAccountKey;
			return cachedServiceAccountKey;
		} catch {
			throw new Error('ZITADEL_SERVICE_KEY is not valid base64-encoded JSON');
		}
	}

	private getServiceAccountPrivateKey(): KeyObject {
		if (cachedServiceAccountPrivateKey) {
			return cachedServiceAccountPrivateKey;
		}

		const saKey = this.getServiceAccountKey();
		cachedServiceAccountPrivateKey = createPrivateKey(saKey.key);
		return cachedServiceAccountPrivateKey;
	}

	// ========== Service Account Token Management ==========

	/**
	 * Get an access token for the service account.
	 * Uses JWT bearer grant to exchange a signed JWT for an access token.
	 * Token is cached until 60 seconds before expiry.
	 */
	private async getServiceAccountToken(): Promise<string> {
		const now = Math.floor(Date.now() / 1000);

		if (cachedServiceAccountToken && cachedServiceAccountToken.expiresAt > now + 60) {
			return cachedServiceAccountToken.token;
		}

		const zitadelUrl = await this.getZitadelUrl();
		const saKey = this.getServiceAccountKey();
		const privateKey = this.getServiceAccountPrivateKey();

		// Create JWT assertion for the service account
		const assertion = await new SignJWT({})
			.setProtectedHeader({ alg: 'RS256', kid: saKey.keyId })
			.setIssuedAt(now)
			.setExpirationTime(now + 3600)
			.setIssuer(saKey.userId)
			.setSubject(saKey.userId)
			.setAudience(zitadelUrl)
			.sign(privateKey);

		// Exchange JWT for access token
		const params = new URLSearchParams({
			grant_type: 'urn:ietf:params:oauth:grant-type:jwt-bearer',
			assertion,
			scope: 'openid urn:zitadel:iam:org:project:id:zitadel:aud',
		});

		const response = await fetch(`${zitadelUrl}/oauth/v2/token`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded',
			},
			body: params,
		});

		if (!response.ok) {
			const error = await response.text();
			throw new Error(`Failed to get service account token: ${response.status} ${error}`);
		}

		const data = await response.json();
		cachedServiceAccountToken = {
			token: data.access_token,
			expiresAt: now + data.expires_in,
		};

		return cachedServiceAccountToken.token;
	}

	/**
	 * Create a client assertion JWT for authenticating with Zitadel APIs.
	 * Used for private_key_jwt authentication method.
	 */
	private async createClientAssertion(): Promise<string> {
		const appKey = await this.getApplicationKey();
		const privateKey = await this.getPrivateKey();
		const zitadelUrl = await this.getZitadelUrl();

		const now = Math.floor(Date.now() / 1000);

		return new SignJWT({})
			.setProtectedHeader({ alg: 'RS256', kid: appKey.keyId })
			.setIssuedAt(now)
			.setExpirationTime(now + 3600) // 1 hour
			.setIssuer(appKey.clientId)
			.setSubject(appKey.clientId)
			.setAudience(zitadelUrl)
			.sign(privateKey);
	}

	// ========== Zitadel Session Management (Multi-step Flow) ==========

	/**
	 * Create a new Zitadel session with username check.
	 * Step 1 of the login flow.
	 */
	private async createZitadelSession(loginName: string): Promise<ZitadelSession> {
		const zitadelUrl = await this.getZitadelUrl();
		const accessToken = await this.getServiceAccountToken();

		const response = await fetch(`${zitadelUrl}/v2/sessions`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`,
			},
			body: JSON.stringify({
				checks: {
					user: {
						loginName,
					},
				},
			}),
		});

		if (!response.ok) {
			const error = await response.text();
			throw new Error(`Failed to create Zitadel session: ${response.status} ${error}`);
		}

		const data = await response.json();
		return {
			sessionId: data.sessionId,
			sessionToken: data.sessionToken,
		};
	}

	/**
	 * Update Zitadel session with password check.
	 * Step 2 of the login flow.
	 */
	private async checkPassword(sessionId: string, sessionToken: string, password: string): Promise<ZitadelSession> {
		const zitadelUrl = await this.getZitadelUrl();
		const accessToken = await this.getServiceAccountToken();

		const response = await fetch(`${zitadelUrl}/v2/sessions/${sessionId}`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`,
			},
			body: JSON.stringify({
				sessionToken,
				checks: {
					password: {
						password,
					},
				},
			}),
		});

		if (!response.ok) {
			const error = await response.text();
			throw new Error(`Password check failed: ${response.status} ${error}`);
		}

		const data = await response.json();
		// PATCH response only returns sessionToken, not sessionId (per Zitadel docs)
		return {
			sessionId,
			sessionToken: data.sessionToken,
		};
	}

	/**
	 * Finalize the auth request with a verified session.
	 * Step 3 of the login flow.
	 * Returns the callback URL containing the authorization code.
	 */
	private async finalizeAuthRequest(
		authRequestId: string,
		sessionId: string,
		sessionToken: string,
	): Promise<ZitadelAuthRequestCallback> {
		const zitadelUrl = await this.getZitadelUrl();
		const accessToken = await this.getServiceAccountToken();

		const response = await fetch(`${zitadelUrl}/v2/oidc/auth_requests/${authRequestId}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`,
			},
			body: JSON.stringify({
				session: {
					sessionId,
					sessionToken,
				},
			}),
		});

		if (!response.ok) {
			const error = await response.text();
			throw new Error(`Failed to finalize auth request: ${response.status} ${error}`);
		}

		const data = await response.json();
		return {
			callbackUrl: data.callbackUrl,
		};
	}

	/**
	 * Get user ID from a Zitadel session.
	 */
	private async getUserIdFromSession(sessionId: string): Promise<string | null> {
		const zitadelUrl = await this.getZitadelUrl();
		const accessToken = await this.getServiceAccountToken();

		const response = await fetch(`${zitadelUrl}/v2/sessions/${sessionId}`, {
			headers: {
				Authorization: `Bearer ${accessToken}`,
			},
		});

		if (!response.ok) {
			return null;
		}

		const data = await response.json();
		return data.session?.factors?.user?.id || null;
	}

	/**
	 * Create a user grant for the given user.
	 * This allows the user to authenticate to the OIDC application.
	 */
	private async createUserGrant(userId: string, roleKeys: string[] = ['user']): Promise<{ userGrantId: string }> {
		const zitadelUrl = await this.getZitadelUrl();
		const accessToken = await this.getServiceAccountToken();
		const projectId = this.getProjectId();

		const response = await fetch(`${zitadelUrl}/management/v1/users/${userId}/grants`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`,
			},
			body: JSON.stringify({
				projectId,
				roleKeys,
			}),
		});

		if (!response.ok) {
			const error = await response.text();
			// Check if grant already exists
			if (response.status === 409 || error.includes('AlreadyExists')) {
				return { userGrantId: 'existing' };
			}
			throw new Error(`Failed to create user grant: ${response.status} ${error}`);
		}

		const data = await response.json();
		return { userGrantId: data.userGrantId };
	}

	// ========== PKCE Helpers ==========

	private async generateCodeVerifier(): Promise<string> {
		const array = new Uint8Array(32);
		crypto.getRandomValues(array);
		return this.base64UrlEncode(array);
	}

	private async generateCodeChallenge(verifier: string): Promise<string> {
		const encoder = new TextEncoder();
		const data = encoder.encode(verifier);
		const hash = await crypto.subtle.digest('SHA-256', data);
		return this.base64UrlEncode(new Uint8Array(hash));
	}

	private async base64UrlEncode(buffer: Uint8Array): Promise<string> {
		let binary = '';
		for (const byte of buffer) {
			binary += String.fromCharCode(byte);
		}
		return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
	}

	// ========== AuthProvider Implementation ==========

	async login(email: string, password: string, context?: AuthContext): Promise<ActionResult<LoginResult>> {
		try {
			// Step 1: Create session with username check
			const session = await this.createZitadelSession(email);

			// Step 2: Verify password
			const verifiedSession = await this.checkPassword(session.sessionId, session.sessionToken, password);

			// Step 3: Finalize auth request (requires authRequest from context)
			const authRequest = context?.authRequest;
			if (!authRequest) {
				return {
					success: false,
					error: 'Missing auth request. Please start the login flow from the authorize URL.',
				};
			}

			try {
				const callback = await this.finalizeAuthRequest(
					authRequest,
					verifiedSession.sessionId,
					verifiedSession.sessionToken,
				);

				return {
					success: true,
					data: {
						type: 'redirect',
						url: callback.callbackUrl,
					},
				};
			} catch (finalizeError) {
				// Check if user needs a grant (first-time login for existing user)
				if (finalizeError instanceof Error && finalizeError.message.includes('GrantRequired')) {
					const userId = await this.getUserIdFromSession(verifiedSession.sessionId);

					if (userId) {
						await this.createUserGrant(userId);
						const callback = await this.finalizeAuthRequest(
							authRequest,
							verifiedSession.sessionId,
							verifiedSession.sessionToken,
						);

						return {
							success: true,
							data: {
								type: 'redirect',
								url: callback.callbackUrl,
							},
						};
					}
				}

				throw finalizeError;
			}
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'An unexpected error occurred';

			// Map Zitadel errors to user-friendly messages (security-conscious)
			if (errorMessage.includes('Password check failed') || errorMessage.includes('credentials')) {
				return {
					success: false,
					error: 'Invalid email or password',
				};
			}

			if (errorMessage.includes('session') && errorMessage.includes('404')) {
				// Return same error as invalid password to prevent account enumeration
				return {
					success: false,
					error: 'Invalid email or password',
				};
			}

			return {
				success: false,
				error: 'Login failed. Please try again.',
			};
		}
	}

	async startOidcFlow(
		redirectUri: string,
		scope = 'openid profile email offline_access urn:zitadel:iam:user:metadata',
	): Promise<{ url: string }> {
		const { cookies } = await import('next/headers');

		const appKey = await this.getApplicationKey();

		// Generate PKCE parameters
		const codeVerifier = await this.generateCodeVerifier();
		const codeChallenge = await this.generateCodeChallenge(codeVerifier);

		// Generate state parameter for CSRF protection
		const state = await this.generateCodeVerifier();

		const cookieStore = await cookies();
		cookieStore.set('pkce_verifier', codeVerifier, {
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
			sameSite: 'lax',
			maxAge: 600, // 10 minutes
			path: '/',
		});
		cookieStore.set('oauth_state', state, {
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
			sameSite: 'lax',
			maxAge: 600, // 10 minutes
			path: '/',
		});

		const params = new URLSearchParams({
			client_id: appKey.clientId,
			redirect_uri: redirectUri,
			response_type: 'code',
			scope,
			code_challenge: codeChallenge,
			code_challenge_method: 'S256',
			state,
		});

		const zitadelUrl = await this.getZitadelUrl();
		return { url: `${zitadelUrl}/oauth/v2/authorize?${params}` };
	}

	async exchangeCodeForTokens(
		code: string,
		redirectUri: string,
		codeVerifier: string,
	): Promise<ActionResult<TokenResponse>> {
		try {
			const zitadelUrl = await this.getZitadelUrl();
			const clientAssertion = await this.createClientAssertion();

			const params = new URLSearchParams({
				grant_type: 'authorization_code',
				code,
				redirect_uri: redirectUri,
				code_verifier: codeVerifier,
				client_assertion_type: 'urn:ietf:params:oauth:client-assertion-type:jwt-bearer',
				client_assertion: clientAssertion,
			});

			const response = await fetch(`${zitadelUrl}/oauth/v2/token`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/x-www-form-urlencoded',
				},
				body: params,
			});

			if (!response.ok) {
				const error = await response.text();
				throw new Error(`Token exchange failed: ${response.status} ${error}`);
			}

			const data: ZitadelTokenResponse = await response.json();

			return {
				success: true,
				data: {
					accessToken: data.access_token,
					refreshToken: data.refresh_token,
					idToken: data.id_token ?? null,
					expiresIn: data.expires_in,
					tokenType: data.token_type,
				},
			};
		} catch (error) {
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Token exchange failed',
			};
		}
	}

	async refreshTokens(refreshToken: string): Promise<ActionResult<TokenResponse>> {
		try {
			const zitadelUrl = await this.getZitadelUrl();
			const clientAssertion = await this.createClientAssertion();

			const params = new URLSearchParams({
				grant_type: 'refresh_token',
				refresh_token: refreshToken,
				client_assertion_type: 'urn:ietf:params:oauth:client-assertion-type:jwt-bearer',
				client_assertion: clientAssertion,
			});

			const response = await fetch(`${zitadelUrl}/oauth/v2/token`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/x-www-form-urlencoded',
				},
				body: params,
			});

			if (!response.ok) {
				const error = await response.text();
				throw new Error(`Token refresh failed: ${response.status} ${error}`);
			}

			const data: ZitadelTokenResponse = await response.json();

			return {
				success: true,
				data: {
					accessToken: data.access_token,
					refreshToken: data.refresh_token,
					idToken: data.id_token ?? null,
					expiresIn: data.expires_in,
					tokenType: data.token_type,
				},
			};
		} catch (error) {
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Token refresh failed',
			};
		}
	}

	async createUser(email: string, password: string, displayName: string): Promise<ActionResult<{ userId: string }>> {
		try {
			const zitadelUrl = await this.getZitadelUrl();
			const accessToken = await this.getServiceAccountToken();

			// Split display name into given/family name (simple heuristic)
			const nameParts = displayName.trim().split(/\s+/);
			const givenName = nameParts[0] || displayName;
			const familyName = nameParts.length > 1 ? nameParts.slice(1).join(' ') : displayName;

			const response = await fetch(`${zitadelUrl}/v2/users/human`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${accessToken}`,
				},
				body: JSON.stringify({
					username: email,
					profile: {
						givenName,
						familyName,
						displayName,
					},
					email: {
						email,
						sendCode: {}, // Triggers Zitadel to send verification email
					},
					password: {
						password,
						changeRequired: false,
					},
				}),
			});

			if (!response.ok) {
				const error = await response.text();
				throw new Error(`Failed to create user: ${response.status} ${error}`);
			}

			const data = await response.json();
			const userId = data.userId;

			// Create user grant so they can access the OIDC application
			await this.createUserGrant(userId, ['user']);

			return {
				success: true,
				data: { userId },
			};
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'An unexpected error occurred';

			// Map common Zitadel errors to user-friendly messages
			if (errorMessage.includes('409') || errorMessage.includes('already exists')) {
				return {
					success: false,
					error: 'An account with this email already exists',
				};
			}

			if (errorMessage.includes('password') && errorMessage.includes('policy')) {
				return {
					success: false,
					error: 'Password does not meet security requirements',
				};
			}

			return {
				success: false,
				error: 'Registration failed. Please try again.',
			};
		}
	}

	async changePassword(userId: string, currentPassword: string, newPassword: string): Promise<ActionResult<void>> {
		try {
			const zitadelUrl = await this.getZitadelUrl();
			const accessToken = await this.getServiceAccountToken();

			const response = await fetch(`${zitadelUrl}/v2/users/${userId}/password`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${accessToken}`,
				},
				body: JSON.stringify({
					newPassword: {
						password: newPassword,
						changeRequired: false,
					},
					currentPassword,
				}),
			});

			if (!response.ok) {
				const error = await response.text();
				// Parse common Zitadel errors for user-friendly messages
				if (response.status === 400 && error.includes('password')) {
					return {
						success: false,
						error: 'Current password is incorrect',
					};
				}
				throw new Error(`Failed to change password: ${response.status}`);
			}

			return {
				success: true,
				data: undefined,
			};
		} catch (error) {
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Failed to change password',
			};
		}
	}

	extractUser(tokens: TokenResponse): AuthUser {
		// Prefer ID token for user claims, fallback to access token
		const tokenToDecode = tokens.idToken ?? tokens.accessToken;

		try {
			const claims = decodeJwt(tokenToDecode);

			return {
				id: (claims.sub as string) ?? '',
				email: (claims.email as string) ?? null,
				name: (claims.name as string) ?? null,
			};
		} catch (error) {
			throw new Error(`Failed to decode JWT: ${error instanceof Error ? error.message : 'Unknown error'}`);
		}
	}

	override async logout(sessionId?: string, sessionToken?: string): Promise<ActionResult<void>> {
		if (!sessionId || !sessionToken) {
			// No Zitadel session to delete
			return { success: true, data: undefined };
		}

		try {
			const zitadelUrl = await this.getZitadelUrl();
			const accessToken = await this.getServiceAccountToken();

			const response = await fetch(`${zitadelUrl}/v2/sessions/${sessionId}`, {
				method: 'DELETE',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${accessToken}`,
				},
				body: JSON.stringify({ sessionToken }),
			});

			if (!response.ok) {
				const error = await response.text();
				throw new Error(`Failed to delete Zitadel session: ${response.status} ${error}`);
			}

			return {
				success: true,
				data: undefined,
			};
		} catch (error) {
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Failed to logout from provider',
			};
		}
	}
}
