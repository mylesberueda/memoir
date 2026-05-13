/**
 * Zitadel test utilities for integration tests.
 *
 * Provides helpers to create test users and get access tokens
 * for making authenticated API calls in integration tests.
 *
 * Creates human users with verified emails and uses the same
 * OIDC flow as real browser users to get tokens.
 */

import { randomUUID } from 'node:crypto';
import {
	checkPassword,
	createZitadelSession,
	exchangeCodeForTokens,
	finalizeAuthRequest,
	generateCodeChallenge,
	generateCodeVerifier,
	getApplicationKey,
	getServiceAccountToken,
	getZitadelUrl,
} from '@lib/zitadel';

interface TestUser {
	userId: string;
	email: string;
	accessToken: string;
	refreshToken: string;
	idToken: string;
}

/**
 * Create a test human user in Zitadel and get JWT tokens.
 *
 * Flow:
 * 1. Create human user with verified email
 * 2. Create user grant for project access
 * 3. Use ROPC grant to get access token and ID token
 *
 * @param email - Email address for the test user
 * @param password - Password for the test user
 * @param displayName - Display name for the test user
 * @returns TestUser with userId, email, accessToken, and idToken
 */
export async function createTestUser(email: string, password: string, displayName: string): Promise<TestUser> {
	const zitadelUrl = await getZitadelUrl();
	const saToken = await getServiceAccountToken();
	const projectId = process.env.ZITADEL_PROJECT_ID;

	if (!projectId) {
		throw new Error('ZITADEL_PROJECT_ID environment variable is not set');
	}

	// Split display name into given/family name
	const nameParts = displayName.trim().split(/\s+/);
	const givenName = nameParts[0] || displayName;
	const familyName = nameParts.length > 1 ? nameParts.slice(1).join(' ') : displayName;

	// Step 1: Create a human user with verified email
	const createResponse = await fetch(`${zitadelUrl}/v2/users/human`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${saToken}`,
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
				isVerified: true, // Pre-verify for tests
			},
			password: {
				password,
				changeRequired: false,
			},
		}),
	});

	if (!createResponse.ok) {
		const error = await createResponse.text();
		throw new Error(`Failed to create test user: ${createResponse.status} ${error}`);
	}

	const userData = await createResponse.json();
	const userId = userData.userId;

	// Step 2: Create user grant for project access
	const grantResponse = await fetch(`${zitadelUrl}/management/v1/users/${userId}/grants`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${saToken}`,
		},
		body: JSON.stringify({
			projectId,
			roleKeys: ['user'],
		}),
	});

	if (!grantResponse.ok) {
		const error = await grantResponse.text();
		throw new Error(`Failed to create user grant: ${grantResponse.status} ${error}`);
	}

	// Step 3: Start OIDC authorize flow (same as browser)
	const appKey = await getApplicationKey();
	const codeVerifier = await generateCodeVerifier();
	const codeChallenge = await generateCodeChallenge(codeVerifier);
	const state = await generateCodeVerifier();
	const redirectUri = 'http://localhost:3000/api/auth/callback';

	const authorizeParams = new URLSearchParams({
		client_id: appKey.clientId,
		redirect_uri: redirectUri,
		response_type: 'code',
		scope: `openid profile email offline_access urn:zitadel:iam:org:project:id:${projectId}:aud urn:zitadel:iam:user:metadata`,
		code_challenge: codeChallenge,
		code_challenge_method: 'S256',
		state,
	});

	const authorizeResponse = await fetch(`${zitadelUrl}/oauth/v2/authorize?${authorizeParams}`, {
		redirect: 'manual',
		headers: {
			'x-zitadel-login-client': appKey.clientId,
		},
	});

	const location = authorizeResponse.headers.get('location');
	if (!location) {
		throw new Error('No redirect from authorize endpoint');
	}

	const locationUrl = new URL(location, zitadelUrl);
	// V2 flow uses 'authRequest' param with 'V2_' prefix, V1 uses 'authRequestID'
	const authRequestId = locationUrl.searchParams.get('authRequest') || locationUrl.searchParams.get('authRequestID');
	if (!authRequestId) {
		throw new Error(`No authRequestID in redirect URL: ${location}`);
	}

	// Step 4: Create session and verify password (same as browser login)
	const session = await createZitadelSession(email);
	const verifiedSession = await checkPassword(session.sessionId, session.sessionToken, password);

	// Step 5: Finalize auth request to get authorization code
	const callback = await finalizeAuthRequest(authRequestId, verifiedSession.sessionId, verifiedSession.sessionToken);

	const callbackUrl = new URL(callback.callbackUrl);
	const code = callbackUrl.searchParams.get('code');
	if (!code) {
		throw new Error(`No code in callback URL: ${callback.callbackUrl}`);
	}

	// Step 6: Exchange code for tokens
	const tokens = await exchangeCodeForTokens(code, redirectUri, codeVerifier);

	if (!tokens.id_token) {
		throw new Error('No ID token received from Zitadel');
	}

	return {
		userId,
		email,
		accessToken: tokens.access_token,
		refreshToken: tokens.refresh_token,
		idToken: tokens.id_token,
	};
}

/**
 * Delete a test user from Zitadel.
 * Call this in afterAll to clean up test users.
 *
 * First queries the user to get their org ID, then deletes with proper org context.
 *
 * @param userId - The Zitadel user ID to delete
 */
export async function deleteTestUser(userId: string): Promise<void> {
	const zitadelUrl = await getZitadelUrl();
	const saToken = await getServiceAccountToken();

	// First, get the user to find their organization
	const getUserResponse = await fetch(`${zitadelUrl}/v2/users/${userId}`, {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${saToken}`,
		},
	});

	if (!getUserResponse.ok) {
		if (getUserResponse.status === 404) {
			return; // User already deleted
		}
		console.warn(`Failed to get test user ${userId} for deletion: ${await getUserResponse.text()}`);
		return;
	}

	const userData = await getUserResponse.json();
	const orgId = userData.user?.details?.resourceOwner;

	// Delete with org context
	const headers: Record<string, string> = {
		Authorization: `Bearer ${saToken}`,
	};

	if (orgId) {
		headers['x-zitadel-orgid'] = orgId;
	}

	const response = await fetch(`${zitadelUrl}/v2/users/${userId}`, {
		method: 'DELETE',
		headers,
	});

	if (!response.ok && response.status !== 404) {
		console.warn(`Failed to delete test user ${userId}: ${await response.text()}`);
	}
}

/**
 * Generate a unique test email address.
 * Uses a UUID to ensure uniqueness across test runs.
 */
export function generateTestEmail(prefix = 'test'): string {
	return `${prefix}-${randomUUID()}@integration-test.local`;
}
