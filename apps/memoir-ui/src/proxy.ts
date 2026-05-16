import { createHmac, timingSafeEqual } from 'node:crypto';
import type { NextRequest } from 'next/server';
import { NextResponse } from 'next/server';

const SESSION_COOKIE_NAME = 'session_token';
const ORGANIZATION_COOKIE_NAME = 'x-organization-id';

// Paths to proxy to Zitadel for OIDC
const ZITADEL_PROXY_PATHS = ['/.well-known/', '/oauth/', '/oidc/', '/idps/callback/', '/saml/'];

// Define protected routes that require authentication
const protectedRoutes = ['/dashboard', '/settings', '/org', '/organizations'];

// Define auth routes that should redirect to dashboard if already logged in
// Note: /login is included because Zitadel redirects there with authRequest parameter
const authRoutes = ['/login', '/auth/login', '/auth/registration'];

/**
 * Get Zitadel URL from environment.
 */
function getZitadelUrl(): string {
	const url = process.env.ZITADEL_URL;
	if (!url) {
		throw new Error('ZITADEL_URL environment variable is not set');
	}
	return url;
}

/**
 * Get the public host (this app's domain).
 * Zitadel uses this to redirect back to our login page.
 * Note: Zitadel trusted domains are hostnames only (no port).
 */
function getPublicHost(): string {
	return process.env.ZITADEL_PUBLIC_HOST || 'localhost';
}

/**
 * Get the Zitadel instance host from ZITADEL_URL.
 */
function getInstanceHost(): string {
	const url = new URL(getZitadelUrl());
	return url.host;
}

/**
 * Check if a path should be proxied to Zitadel.
 */
function isZitadelProxyPath(pathname: string): boolean {
	return ZITADEL_PROXY_PATHS.some((prefix) => pathname.startsWith(prefix));
}

/**
 * Proxy request to Zitadel with appropriate headers.
 */
function proxyToZitadel(request: NextRequest): NextResponse {
	const zitadelUrl = getZitadelUrl();
	const publicHost = getPublicHost();
	const instanceHost = getInstanceHost();

	// Clone headers and add Zitadel-specific ones
	const requestHeaders = new Headers(request.headers);
	requestHeaders.set('x-zitadel-login-client', publicHost);
	requestHeaders.set('x-zitadel-public-host', publicHost);
	requestHeaders.set('x-zitadel-instance-host', instanceHost);

	// Build the target URL
	const targetUrl = new URL(request.nextUrl.pathname + request.nextUrl.search, zitadelUrl);

	// CORS headers not needed for rewrites (browser sees our origin, not Zitadel's)
	const responseHeaders = new Headers();

	// Rewrite the request to Zitadel
	return NextResponse.rewrite(targetUrl, {
		request: {
			headers: requestHeaders,
		},
		headers: responseHeaders,
	});
}

/**
 * Get session secrets from environment (comma-separated for rotation support).
 * First secret is for signing, all are tried for validation.
 */
function getSessionSecrets(): string[] {
	const secretEnv = process.env.SESSION_SECRET;
	if (!secretEnv) {
		console.error('SESSION_SECRET environment variable is not set');
		return [];
	}

	return secretEnv
		.split(',')
		.map((s) => s.trim())
		.filter(Boolean);
}

/**
 * Verify the HMAC signature of a session token.
 * This is a lightweight check that doesn't require Redis.
 * Actual session validity is checked in server actions.
 */
function isSessionTokenValid(signedToken: string): boolean {
	try {
		const parts = signedToken.split('.');
		if (parts.length !== 2) return false;

		const [sessionId, signature] = parts;
		if (!sessionId || !signature) return false;

		const sigBuffer = Buffer.from(signature, 'base64url');
		const secrets = getSessionSecrets();

		// Try all secrets for validation (supports rotation)
		for (const secret of secrets) {
			const expectedSignature = createHmac('sha256', secret).update(sessionId).digest('base64url');

			const expectedBuffer = Buffer.from(expectedSignature, 'base64url');

			if (sigBuffer.length === expectedBuffer.length && timingSafeEqual(sigBuffer, expectedBuffer)) {
				return true;
			}
		}

		return false;
	} catch (error) {
		console.error('Session token validation error in middleware:', error);
		return false;
	}
}

export function proxy(request: NextRequest) {
	const { pathname } = request.nextUrl;

	// First, check if this is a Zitadel OIDC path that should be proxied
	if (isZitadelProxyPath(pathname)) {
		return proxyToZitadel(request);
	}

	const sessionToken = request.cookies.get(SESSION_COOKIE_NAME)?.value;

	// Check if the current path is a protected route
	const isProtectedRoute = protectedRoutes.some((route) => pathname.startsWith(route));

	if (pathname === '/' && sessionToken && isSessionTokenValid(sessionToken)) {
		return NextResponse.redirect(new URL('/dashboard', request.url));
	}

	// Check if the current path is an auth route
	const isAuthRoute = authRoutes.some((route) => pathname.startsWith(route));

	// Handle protected routes
	if (isProtectedRoute) {
		if (!sessionToken) {
			// No session token - redirect to login
			const loginUrl = new URL('/auth/login', request.url);
			loginUrl.searchParams.set('from', pathname);
			return NextResponse.redirect(loginUrl);
		}

		// Validate the session token signature
		const isValid = isSessionTokenValid(sessionToken);
		if (!isValid) {
			// Invalid signature - clear it and redirect to login
			const loginUrl = new URL('/auth/login', request.url);
			loginUrl.searchParams.set('from', pathname);
			const response = NextResponse.redirect(loginUrl);
			response.cookies.delete(SESSION_COOKIE_NAME);
			response.cookies.delete(ORGANIZATION_COOKIE_NAME);
			return response;
		}
	}

	// Handle auth routes - only redirect if session token is valid
	if (isAuthRoute && sessionToken) {
		// Check if this is a logout/session-cleanup request (session was invalid in Redis)
		const logoutParam = request.nextUrl.searchParams.get('logout');
		if (logoutParam === '1') {
			// Session was invalid - delete cookie and redirect to clean login page
			const response = NextResponse.redirect(new URL('/auth/login', request.url));
			response.cookies.delete(SESSION_COOKIE_NAME);
			response.cookies.delete(ORGANIZATION_COOKIE_NAME);
			return response;
		}

		// Validate the session token signature before redirecting
		const isValid = isSessionTokenValid(sessionToken);
		if (isValid) {
			// Valid signature - redirect to dashboard
			// Note: The session might be expired in Redis, but that will be handled
			// by the dashboard page when it tries to load user data
			return NextResponse.redirect(new URL('/dashboard', request.url));
		}
		// Invalid signature - clear it and allow access to auth page
		const response = NextResponse.next();
		response.cookies.delete(SESSION_COOKIE_NAME);
		response.cookies.delete(ORGANIZATION_COOKIE_NAME);
		return response;
	}

	return NextResponse.next();
}

// Configure which routes the middleware should run on
export const config = {
	matcher: [
		/*
		 * Match all request paths except for the ones starting with:
		 * - api (API routes)
		 * - _next/static (static files)
		 * - _next/image (image optimization files)
		 * - favicon.ico (favicon file)
		 * - public folder
		 */
		'/((?!api|_next/static|_next/image|favicon.ico|public).*)',
	],
};
