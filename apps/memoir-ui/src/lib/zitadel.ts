'use server';

import { createPrivateKey, type KeyObject } from 'node:crypto';
import { SignJWT } from 'jose';

// Types for Zitadel API responses
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

// Cache for the imported private keys
let cachedPrivateKey: KeyObject | null = null;
let cachedApplicationKey: ApplicationKey | null = null;
let cachedServiceAccountKey: ServiceAccountKey | null = null;
let cachedServiceAccountPrivateKey: KeyObject | null = null;
let cachedServiceAccountToken: { token: string; expiresAt: number } | null = null;

export async function getZitadelUrl(): Promise<string> {
	const url = process.env.ZITADEL_URL;
	if (!url) {
		throw new Error('ZITADEL_URL environment variable is not set');
	}
	return url;
}

export async function getApplicationKey(): Promise<ApplicationKey> {
	if (cachedApplicationKey) {
		return cachedApplicationKey;
	}

	const keyBase64 = process.env.ZITADEL_APPLICATION_KEY;
	if (!keyBase64) {
		throw new Error('ZITADEL_APPLICATION_KEY environment variable is not set');
	}

	try {
		// Decode base64 to get JSON string
		const keyJson = Buffer.from(keyBase64, 'base64').toString('utf-8');
		cachedApplicationKey = JSON.parse(keyJson) as ApplicationKey;
		return cachedApplicationKey;
	} catch {
		throw new Error('ZITADEL_APPLICATION_KEY is not valid base64-encoded JSON');
	}
}

async function getPrivateKey(): Promise<KeyObject> {
	if (cachedPrivateKey) {
		return cachedPrivateKey;
	}

	const appKey = await getApplicationKey();
	cachedPrivateKey = createPrivateKey(appKey.key);
	return cachedPrivateKey;
}

function getServiceAccountKey(): ServiceAccountKey {
	if (cachedServiceAccountKey) {
		return cachedServiceAccountKey;
	}

	const keyBase64 = process.env.ZITADEL_SERVICE_KEY;
	if (!keyBase64) {
		throw new Error('ZITADEL_SERVICE_KEY environment variable is not set');
	}

	try {
		// Decode base64 to get JSON string
		const keyJson = Buffer.from(keyBase64, 'base64').toString('utf-8');
		cachedServiceAccountKey = JSON.parse(keyJson) as ServiceAccountKey;
		return cachedServiceAccountKey;
	} catch {
		throw new Error('ZITADEL_SERVICE_KEY is not valid base64-encoded JSON');
	}
}

function getServiceAccountPrivateKey(): KeyObject {
	if (cachedServiceAccountPrivateKey) {
		return cachedServiceAccountPrivateKey;
	}

	const saKey = getServiceAccountKey();
	cachedServiceAccountPrivateKey = createPrivateKey(saKey.key);
	return cachedServiceAccountPrivateKey;
}

/**
 * Get an access token for the service account.
 * Uses JWT bearer grant to exchange a signed JWT for an access token.
 * Token is cached until 60 seconds before expiry.
 */
export async function getServiceAccountToken(): Promise<string> {
	const now = Math.floor(Date.now() / 1000);

	if (cachedServiceAccountToken && cachedServiceAccountToken.expiresAt > now + 60) {
		return cachedServiceAccountToken.token;
	}

	const zitadelUrl = await getZitadelUrl();
	const saKey = getServiceAccountKey();
	const privateKey = getServiceAccountPrivateKey();

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
export async function createClientAssertion(): Promise<string> {
	const appKey = await getApplicationKey();
	const privateKey = await getPrivateKey();
	const zitadelUrl = await getZitadelUrl();

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

/**
 * Create a new Zitadel session with username check.
 * Step 1 of the login flow.
 */
export async function createZitadelSession(loginName: string): Promise<ZitadelSession> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
export async function checkPassword(
	sessionId: string,
	sessionToken: string,
	password: string,
): Promise<ZitadelSession> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
 * Step 4 of the login flow.
 * Returns the callback URL containing the authorization code.
 */
export async function finalizeAuthRequest(
	authRequestId: string,
	sessionId: string,
	sessionToken: string,
): Promise<ZitadelAuthRequestCallback> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
 * Exchange authorization code for tokens.
 * Step 5 of the login flow.
 */
export async function exchangeCodeForTokens(
	code: string,
	redirectUri: string,
	codeVerifier: string,
): Promise<ZitadelTokenResponse> {
	const zitadelUrl = await getZitadelUrl();
	const clientAssertion = await createClientAssertion();

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

	return response.json();
}

/**
 * Refresh an access token using a refresh token.
 */
export async function refreshTokens(refreshToken: string): Promise<ZitadelTokenResponse> {
	const zitadelUrl = await getZitadelUrl();
	const clientAssertion = await createClientAssertion();

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

	return response.json();
}

interface CreateUserResponse {
	userId: string;
}

/**
 * Create a new human user in Zitadel and grant them project access.
 * Uses service account token for user management API access.
 * @param email - User's email address
 * @param password - User's password
 * @param displayName - User's display name
 * @param roleKeys - Optional roles to assign (defaults to empty for basic access)
 */
export async function createHumanUser(
	email: string,
	password: string,
	displayName: string,
	roleKeys: string[] = ['user'],
): Promise<CreateUserResponse> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
	await createUserGrant(userId, roleKeys);

	return {
		userId,
	};
}

/**
 * Get the project ID for user grants.
 */
function getProjectId(): string {
	const projectId = process.env.ZITADEL_PROJECT_ID;
	if (!projectId) {
		throw new Error('ZITADEL_PROJECT_ID environment variable is not set');
	}
	return projectId;
}

/**
 * Get user ID from a Zitadel session.
 */
export async function getUserIdFromSession(sessionId: string): Promise<string | null> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
 * @param userId - The Zitadel user ID
 * @param roleKeys - Array of role keys to assign (defaults to ['user'])
 */
export async function createUserGrant(userId: string, roleKeys: string[] = ['user']): Promise<{ userGrantId: string }> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();
	const projectId = getProjectId();

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

/**
 * Delete a Zitadel session (logout from Zitadel).
 */
export async function deleteZitadelSession(sessionId: string, sessionToken: string): Promise<void> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
}

/**
 * Generate OIDC authorize URL for browser-based flow.
 * Stores PKCE code_verifier and state in cookies for callback validation.
 */
export async function getOidcAuthorizeUrl(
	redirectUri: string,
	scope = 'openid profile email offline_access urn:zitadel:iam:user:metadata',
): Promise<string> {
	const { cookies } = await import('next/headers');

	const appKey = await getApplicationKey();

	// Generate PKCE parameters
	const codeVerifier = await generateCodeVerifier();
	const codeChallenge = await generateCodeChallenge(codeVerifier);

	// Generate state parameter for CSRF protection
	const state = await generateCodeVerifier();

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

	const zitadelUrl = await getZitadelUrl();
	return `${zitadelUrl}/oauth/v2/authorize?${params}`;
}

// PKCE helpers
export async function generateCodeVerifier(): Promise<string> {
	const array = new Uint8Array(32);
	crypto.getRandomValues(array);
	return base64UrlEncode(array);
}

export async function generateCodeChallenge(verifier: string): Promise<string> {
	const encoder = new TextEncoder();
	const data = encoder.encode(verifier);
	const hash = await crypto.subtle.digest('SHA-256', data);
	return base64UrlEncode(new Uint8Array(hash));
}

export async function base64UrlEncode(buffer: Uint8Array): Promise<string> {
	let binary = '';
	for (const byte of buffer) {
		binary += String.fromCharCode(byte);
	}
	return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

/**
 * Change a user's password using current password verification.
 * Requires the user to provide their current password for security.
 * @param userId - Zitadel user ID
 * @param currentPassword - User's current password
 * @param newPassword - New password to set
 */
export async function changeUserPassword(userId: string, currentPassword: string, newPassword: string): Promise<void> {
	const zitadelUrl = await getZitadelUrl();
	const accessToken = await getServiceAccountToken();

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
			throw new Error('Current password is incorrect');
		}
		throw new Error(`Failed to change password: ${response.status}`);
	}
}
