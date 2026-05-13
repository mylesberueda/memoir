import { timingSafeEqual } from 'node:crypto';
import { getAuthProvider } from '@lib/auth';
import { createSession } from '@lib/session';
import { cookies } from 'next/headers';
import { type NextRequest, NextResponse } from 'next/server';

const REDIRECT_URI = process.env.NEXT_PUBLIC_APP_URL
	? `${process.env.NEXT_PUBLIC_APP_URL}/api/auth/callback`
	: 'http://localhost:3000/api/auth/callback';

/**
 * OIDC callback handler.
 * Receives authorization code from auth provider, exchanges for tokens, creates session.
 *
 * Org-resolution after sign-in is intentionally absent: the OrganizationService
 * gRPC client that previously populated the session's initial org pid was
 * removed alongside the rest of the deleted api-service surface. The future
 * memoir-server scaffold reintroduces whatever org bootstrap shape Memoir
 * actually wants.
 */
export async function GET(request: NextRequest) {
	const searchParams = request.nextUrl.searchParams;
	const code = searchParams.get('code');
	const state = searchParams.get('state');
	const error = searchParams.get('error');
	const errorDescription = searchParams.get('error_description');

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

		await createSession({
			accessToken: tokens.accessToken,
			refreshToken: tokens.refreshToken,
			idToken: tokens.idToken,
			userId: user.id,
			expiresAt: Math.floor(Date.now() / 1000) + tokens.expiresIn,
		});

		cookieStore.delete('pkce_verifier');
		cookieStore.delete('oauth_state');

		return NextResponse.redirect(new URL('/dashboard', request.url));
	} catch (err) {
		console.error('OIDC callback error:', err);
		const loginUrl = new URL('/auth/login', request.url);
		loginUrl.searchParams.set('error', err instanceof Error ? err.message : 'Authentication failed');
		return NextResponse.redirect(loginUrl);
	}
}
