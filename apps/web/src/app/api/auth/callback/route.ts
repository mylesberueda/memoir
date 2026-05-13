import { timingSafeEqual } from 'node:crypto';
import { create } from '@bufbuild/protobuf';
import { createClient } from '@connectrpc/connect';
import { getAuthProvider } from '@lib/auth';
import { createTransportWithToken } from '@lib/grpc/transport';
import { createSession } from '@lib/session';
import {
	ListOrganizationsRequestSchema,
	OrganizationService,
} from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { cookies } from 'next/headers';
import { type NextRequest, NextResponse } from 'next/server';

const COOKIE_NAME_ORGANIZATION_ID = 'x-organization-id';

// Redirect URI must match what was used in authorize request
const REDIRECT_URI = process.env.NEXT_PUBLIC_APP_URL
	? `${process.env.NEXT_PUBLIC_APP_URL}/api/auth/callback`
	: 'http://localhost:3000/api/auth/callback';

/**
 * OIDC callback handler.
 * Receives authorization code from auth provider, exchanges for tokens, creates session.
 */
export async function GET(request: NextRequest) {
	const searchParams = request.nextUrl.searchParams;
	const code = searchParams.get('code');
	const state = searchParams.get('state');
	const error = searchParams.get('error');
	const errorDescription = searchParams.get('error_description');

	// Handle OIDC errors
	if (error) {
		console.error('OIDC callback error:', error, errorDescription);
		const loginUrl = new URL('/auth/login', request.url);
		loginUrl.searchParams.set('error', errorDescription || error);
		return NextResponse.redirect(loginUrl);
	}

	if (!code) {
		console.error('No authorization code in callback');
		const loginUrl = new URL('/auth/login', request.url);
		loginUrl.searchParams.set('error', 'No authorization code received');
		return NextResponse.redirect(loginUrl);
	}

	try {
		const cookieStore = await cookies();

		const storedState = cookieStore.get('oauth_state')?.value;
		if (!state || !storedState) {
			throw new Error('OAuth state missing. Please try logging in again.');
		}
		const stateBuffer = Buffer.from(state);
		const storedBuffer = Buffer.from(storedState);
		if (stateBuffer.length !== storedBuffer.length || !timingSafeEqual(stateBuffer, storedBuffer)) {
			throw new Error('OAuth state mismatch. Please try logging in again.');
		}

		const codeVerifier = cookieStore.get('pkce_verifier')?.value;
		if (!codeVerifier) {
			throw new Error('PKCE code verifier not found. Please try logging in again.');
		}

		const provider = getAuthProvider();
		const result = await provider.exchangeCodeForTokens(code, REDIRECT_URI, codeVerifier);

		if (!result.success) {
			throw new Error(result.error);
		}

		const tokens = result.data;
		const user = provider.extractUser(tokens);

		if (!tokens.idToken) {
			throw new Error('No ID token received. Ensure openid scope is requested.');
		}

		// Create our session in Redis
		await createSession({
			accessToken: tokens.accessToken,
			refreshToken: tokens.refreshToken,
			idToken: tokens.idToken,
			userId: user.id,
			expiresAt: Math.floor(Date.now() / 1000) + tokens.expiresIn,
		});

		cookieStore.delete('pkce_verifier');
		cookieStore.delete('oauth_state');

		// Resolve organization context: ?oid param > existing cookie > first org
		const apiServiceUrl = process.env.API_SERVICE_URL;
		if (!apiServiceUrl) {
			throw new Error('API_SERVICE_URL is not set');
		}
		const transport = await createTransportWithToken(apiServiceUrl, tokens.accessToken, tokens.idToken ?? undefined, {
			mode: 'none',
		});
		const orgClient = createClient(OrganizationService, transport);
		const orgsResponse = await orgClient.listOrganizations(create(ListOrganizationsRequestSchema, {}));

		const orgs = orgsResponse.organizations;
		const oidParam = request.nextUrl.searchParams.get('oid');
		const existingCookie = cookieStore.get(COOKIE_NAME_ORGANIZATION_ID)?.value;

		const resolvedOrgPid =
			(oidParam && orgs.some((o) => o.pid === oidParam) ? oidParam : undefined) ??
			(existingCookie && orgs.some((o) => o.pid === existingCookie) ? existingCookie : undefined) ??
			orgs.at(0)?.pid;

		if (resolvedOrgPid) {
			cookieStore.set(COOKIE_NAME_ORGANIZATION_ID, resolvedOrgPid, {
				httpOnly: true,
				secure: process.env.NODE_ENV === 'production',
				sameSite: 'strict',
				path: '/',
				maxAge: 365 * 24 * 60 * 60,
			});
		}

		return NextResponse.redirect(new URL('/dashboard', request.url));
	} catch (err) {
		console.error('OIDC callback error:', err);
		const loginUrl = new URL('/auth/login', request.url);
		loginUrl.searchParams.set('error', err instanceof Error ? err.message : 'Authentication failed');
		return NextResponse.redirect(loginUrl);
	}
}
