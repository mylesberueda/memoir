import { beforeEach, describe, expect, it } from 'vitest';
import type { TokenResponse } from '../types';
import { ZitadelAuthProvider } from './zitadel';

describe('ZitadelAuthProvider', () => {
	let provider: ZitadelAuthProvider;

	beforeEach(() => {
		provider = new ZitadelAuthProvider();
	});

	describe('extractUser', () => {
		it('should return user with id, email, and name when all claims present', () => {
			const tokens: TokenResponse = {
				accessToken: 'access-token',
				refreshToken: 'refresh-token',
				idToken: createMockJwt({
					sub: 'user-123',
					email: 'test@example.com',
					name: 'Test User',
				}),
				expiresIn: 3600,
				tokenType: 'Bearer',
			};

			const user = provider.extractUser(tokens);

			expect(user).toEqual({
				id: 'user-123',
				email: 'test@example.com',
				name: 'Test User',
			});
		});

		it('should use access token when ID token is not available', () => {
			const tokens: TokenResponse = {
				accessToken: createMockJwt({
					sub: 'user-456',
					email: 'access@example.com',
					name: 'Access User',
				}),
				refreshToken: 'refresh-token',
				idToken: null,
				expiresIn: 3600,
				tokenType: 'Bearer',
			};

			const user = provider.extractUser(tokens);

			expect(user).toEqual({
				id: 'user-456',
				email: 'access@example.com',
				name: 'Access User',
			});
		});

		it('should return null for email and name when claims are missing', () => {
			const tokens: TokenResponse = {
				accessToken: createMockJwt({
					sub: 'user-789',
				}),
				refreshToken: 'refresh-token',
				idToken: null,
				expiresIn: 3600,
				tokenType: 'Bearer',
			};

			const user = provider.extractUser(tokens);

			expect(user).toEqual({
				id: 'user-789',
				email: null,
				name: null,
			});
		});

		it('should return empty string for id when sub claim is missing', () => {
			const tokens: TokenResponse = {
				accessToken: createMockJwt({
					email: 'test@example.com',
				}),
				refreshToken: 'refresh-token',
				idToken: null,
				expiresIn: 3600,
				tokenType: 'Bearer',
			};

			const user = provider.extractUser(tokens);

			expect(user).toEqual({
				id: '',
				email: 'test@example.com',
				name: null,
			});
		});

		it('should prefer ID token claims over access token claims', () => {
			const tokens: TokenResponse = {
				accessToken: createMockJwt({
					sub: 'user-123',
					email: 'access@example.com',
					name: 'Access User',
				}),
				refreshToken: 'refresh-token',
				idToken: createMockJwt({
					sub: 'user-123',
					email: 'id@example.com',
					name: 'ID User',
				}),
				expiresIn: 3600,
				tokenType: 'Bearer',
			};

			const user = provider.extractUser(tokens);

			expect(user.email).toBe('id@example.com');
			expect(user.name).toBe('ID User');
		});
	});
});

/**
 * Create a mock JWT (base64url encoded JSON).
 * This is NOT cryptographically signed - only for testing decode logic.
 */
function createMockJwt(claims: Record<string, unknown>): string {
	const header = { alg: 'RS256', typ: 'JWT' };
	const encodedHeader = base64UrlEncode(JSON.stringify(header));
	const encodedPayload = base64UrlEncode(JSON.stringify(claims));
	const mockSignature = 'mock-signature';
	return `${encodedHeader}.${encodedPayload}.${mockSignature}`;
}

function base64UrlEncode(str: string): string {
	return Buffer.from(str).toString('base64').replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}
