import { createHmac } from 'node:crypto';
import { NextRequest } from 'next/server';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { proxy } from './proxy';

// Store original env
const originalEnv = process.env;

// Helper to create a valid signed session token
function createSignedToken(sessionId: string, secret: string): string {
	const signature = createHmac('sha256', secret).update(sessionId).digest('base64url');
	return `${sessionId}.${signature}`;
}

// Helper to create a mock NextRequest
function createMockRequest(
	pathname: string,
	options: {
		cookies?: Record<string, string>;
		searchParams?: Record<string, string>;
	} = {},
): NextRequest {
	const url = new URL(pathname, 'http://localhost:3000');
	if (options.searchParams) {
		for (const [key, value] of Object.entries(options.searchParams)) {
			url.searchParams.set(key, value);
		}
	}

	const request = new NextRequest(url);

	// Mock cookies
	if (options.cookies) {
		for (const [name, value] of Object.entries(options.cookies)) {
			request.cookies.set(name, value);
		}
	}

	return request;
}

describe('proxy', () => {
	beforeEach(() => {
		vi.resetModules();
		process.env = {
			...originalEnv,
			ZITADEL_URL: 'https://auth.example.com',
			ZITADEL_PUBLIC_HOST: 'app.example.com',
			SESSION_SECRET: 'test-secret-that-is-at-least-32-chars',
		};
	});

	afterEach(() => {
		process.env = originalEnv;
	});

	describe('Zitadel proxy paths', () => {
		it('should proxy /.well-known/ paths to Zitadel', () => {
			const request = createMockRequest('/.well-known/openid-configuration');
			const response = proxy(request);

			// NextResponse.rewrite returns a response with the rewritten URL
			expect(response.headers.get('x-middleware-rewrite')).toContain('auth.example.com');
		});

		it('should proxy /oauth/ paths to Zitadel', () => {
			const request = createMockRequest('/oauth/v2/authorize');
			const response = proxy(request);

			expect(response.headers.get('x-middleware-rewrite')).toContain('auth.example.com');
		});

		it('should proxy /oidc/ paths to Zitadel', () => {
			const request = createMockRequest('/oidc/v1/userinfo');
			const response = proxy(request);

			expect(response.headers.get('x-middleware-rewrite')).toContain('auth.example.com');
		});

		it('should proxy /idps/callback/ paths to Zitadel', () => {
			const request = createMockRequest('/idps/callback/google');
			const response = proxy(request);

			expect(response.headers.get('x-middleware-rewrite')).toContain('auth.example.com');
		});

		it('should proxy /saml/ paths to Zitadel', () => {
			const request = createMockRequest('/saml/v2/SSO');
			const response = proxy(request);

			expect(response.headers.get('x-middleware-rewrite')).toContain('auth.example.com');
		});
	});

	describe('Protected routes without session', () => {
		it('should redirect /dashboard to login when no session token', () => {
			const request = createMockRequest('/dashboard');
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/auth/login');
			expect(response.headers.get('location')).toContain('from=%2Fdashboard');
		});

		it('should redirect /settings to login when no session token', () => {
			const request = createMockRequest('/settings');
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/auth/login');
		});

		it('should redirect /org/members to login when no session token', () => {
			const request = createMockRequest('/org/members');
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/auth/login');
		});

		it('should preserve the original path in the redirect', () => {
			const request = createMockRequest('/dashboard/settings/profile');
			const response = proxy(request);

			expect(response.headers.get('location')).toContain('from=%2Fdashboard%2Fsettings%2Fprofile');
		});
	});

	describe('Protected routes with invalid session', () => {
		it('should redirect to login when session token has invalid format', () => {
			const request = createMockRequest('/dashboard', {
				cookies: { session_token: 'invalid-token-no-dot' },
			});
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/auth/login');
		});

		it('should redirect to login when session token has wrong signature', () => {
			const request = createMockRequest('/dashboard', {
				cookies: { session_token: 'session-id.wrong-signature' },
			});
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/auth/login');
		});

		it('should delete invalid session cookie on redirect', () => {
			const request = createMockRequest('/dashboard', {
				cookies: { session_token: 'invalid.signature' },
			});
			const response = proxy(request);

			const setCookie = response.headers.get('set-cookie');
			expect(setCookie).toContain('session_token=');
			// Cookie deletion uses Expires in the past
			expect(setCookie).toContain('Expires=Thu, 01 Jan 1970');
		});
	});

	describe('Protected routes with valid session', () => {
		it('should allow access to /dashboard with valid session token', () => {
			const validToken = createSignedToken('session-123', 'test-secret-that-is-at-least-32-chars');
			const request = createMockRequest('/dashboard', {
				cookies: { session_token: validToken },
			});
			const response = proxy(request);

			// NextResponse.next() doesn't set a redirect
			expect(response.status).toBe(200);
			expect(response.headers.get('location')).toBeNull();
		});

		it('should allow access to /settings with valid session token', () => {
			const validToken = createSignedToken('session-456', 'test-secret-that-is-at-least-32-chars');
			const request = createMockRequest('/settings', {
				cookies: { session_token: validToken },
			});
			const response = proxy(request);

			expect(response.status).toBe(200);
		});

		it('should support rotated secrets for validation', () => {
			// Set up rotated secrets (new,old)
			process.env.SESSION_SECRET = 'new-secret-that-is-at-least-32-chars,old-secret-that-is-at-least-32-chars';

			// Token signed with old secret should still be valid
			const validToken = createSignedToken('session-789', 'old-secret-that-is-at-least-32-chars');
			const request = createMockRequest('/dashboard', {
				cookies: { session_token: validToken },
			});
			const response = proxy(request);

			expect(response.status).toBe(200);
		});
	});

	describe('Auth routes', () => {
		it('should allow access to /auth/login without session', () => {
			const request = createMockRequest('/auth/login');
			const response = proxy(request);

			expect(response.status).toBe(200);
			expect(response.headers.get('location')).toBeNull();
		});

		it('should allow access to /auth/registration without session', () => {
			const request = createMockRequest('/auth/registration');
			const response = proxy(request);

			expect(response.status).toBe(200);
		});

		it('should redirect from /auth/login to /dashboard with valid session', () => {
			const validToken = createSignedToken('session-abc', 'test-secret-that-is-at-least-32-chars');
			const request = createMockRequest('/auth/login', {
				cookies: { session_token: validToken },
			});
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/dashboard');
		});

		it('should redirect from /auth/registration to /dashboard with valid session', () => {
			const validToken = createSignedToken('session-def', 'test-secret-that-is-at-least-32-chars');
			const request = createMockRequest('/auth/registration', {
				cookies: { session_token: validToken },
			});
			const response = proxy(request);

			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/dashboard');
		});

		it('should allow access to auth routes and clear invalid session cookie', () => {
			const request = createMockRequest('/auth/login', {
				cookies: { session_token: 'invalid.signature' },
			});
			const response = proxy(request);

			// Should allow access (not redirect)
			expect(response.status).toBe(200);

			// Should clear the invalid cookie
			const setCookie = response.headers.get('set-cookie');
			expect(setCookie).toContain('session_token=');
			// Cookie deletion uses Expires in the past
			expect(setCookie).toContain('Expires=Thu, 01 Jan 1970');
		});
	});

	describe('Public routes', () => {
		it('should allow access to root path without session', () => {
			const request = createMockRequest('/');
			const response = proxy(request);

			expect(response.status).toBe(200);
		});

		it('should allow access to unprotected paths without session', () => {
			const request = createMockRequest('/about');
			const response = proxy(request);

			expect(response.status).toBe(200);
		});

		it('should allow access to public paths with or without session', () => {
			const validToken = createSignedToken('session-xyz', 'test-secret-that-is-at-least-32-chars');
			const request = createMockRequest('/pricing', {
				cookies: { session_token: validToken },
			});
			const response = proxy(request);

			expect(response.status).toBe(200);
		});
	});

	describe('Environment configuration', () => {
		it('should handle missing SESSION_SECRET gracefully', () => {
			delete process.env.SESSION_SECRET;

			const request = createMockRequest('/dashboard', {
				cookies: { session_token: 'any.token' },
			});

			// Should redirect to login since validation fails
			const response = proxy(request);
			expect(response.status).toBe(307);
			expect(response.headers.get('location')).toContain('/auth/login');
		});

		it('should use default public host when ZITADEL_PUBLIC_HOST is not set', () => {
			delete process.env.ZITADEL_PUBLIC_HOST;

			const request = createMockRequest('/.well-known/openid-configuration');
			const response = proxy(request);

			// Should still proxy successfully
			expect(response.headers.get('x-middleware-rewrite')).toContain('auth.example.com');
		});
	});
});
