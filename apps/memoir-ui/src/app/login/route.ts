import { type NextRequest, NextResponse } from 'next/server';

/**
 * Redirect route for Zitadel OIDC flow.
 *
 * When using custom login UI, Zitadel redirects to /login?authRequest=V2_...
 * This route redirects to our actual login page at /auth/login, preserving
 * the authRequest query parameter.
 */
export function GET(request: NextRequest) {
	const searchParams = request.nextUrl.searchParams;
	const authRequest = searchParams.get('authRequest');

	const loginUrl = new URL('/auth/login', request.url);
	if (authRequest) {
		loginUrl.searchParams.set('authRequest', authRequest);
	}

	return NextResponse.redirect(loginUrl);
}
