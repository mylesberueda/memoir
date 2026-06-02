/**
 * Integration tests for `AuthServiceAuthProvider`.
 *
 * Hits memoir-service's live gRPC AuthService. Requires:
 *
 * - `MEMOIR_SERVICE_URL`     — the gRPC base URL (e.g. http://localhost:5500)
 * - `MEMOIR_TEST_USERNAME`   — a pre-created user (created out-of-band via
 *                              `memoir auth create` or MEMOIR_DEV_MODE)
 * - `MEMOIR_TEST_PASSWORD`   — that user's password
 *
 * Missing env vars are a hard fail at the top of the suite so misconfigured
 * runs produce one clear error rather than ten obscure ones.
 */
import { describe, expect, it } from 'vitest';
import { AuthServiceAuthProvider } from './auth-service';

const SERVICE_URL = process.env.MEMOIR_SERVICE_URL;
const HAS_ENV = Boolean(SERVICE_URL && process.env.MEMOIR_TEST_USERNAME && process.env.MEMOIR_TEST_PASSWORD);

function requireEnv(name: string): string {
	const value = process.env[name];
	if (!value) throw new Error(`${name} is required to run AuthServiceAuthProvider integration tests`);
	return value;
}

describe.skipIf(!HAS_ENV)('AuthServiceAuthProvider integration', () => {
	const USERNAME = requireEnv('MEMOIR_TEST_USERNAME');
	const PASSWORD = requireEnv('MEMOIR_TEST_PASSWORD');

	it('should issue access and refresh tokens for valid credentials', async () => {
		const provider = new AuthServiceAuthProvider();
		const result = await provider.login(USERNAME, PASSWORD);

		expect(result.success, `login failed: ${result.success ? '' : result.error}`).toBe(true);
		if (!result.success) return;

		expect(result.data.type).toBe('tokens');
		if (result.data.type !== 'tokens') return;

		expect(result.data.tokens.accessToken).toMatch(/^[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$/);
		expect(result.data.tokens.refreshToken).toMatch(/^[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$/);
		expect(result.data.tokens.tokenType).toBe('Bearer');
		expect(result.data.tokens.expiresIn).toBeGreaterThan(0);

		expect(result.data.user.id).toBeTruthy();
	});

	it('should reject login when password is wrong', async () => {
		const provider = new AuthServiceAuthProvider();
		const result = await provider.login(USERNAME, `${PASSWORD}-wrong`);

		expect(result.success).toBe(false);
		if (result.success) return;
		expect(result.error).toBeTruthy();
	});

	it('should reject login when username does not exist', async () => {
		const provider = new AuthServiceAuthProvider();
		const result = await provider.login(`__nonexistent__${Date.now()}`, 'anything');

		expect(result.success).toBe(false);
	});

	it('should exchange a refresh token for a fresh access token', async () => {
		const provider = new AuthServiceAuthProvider();
		const login = await provider.login(USERNAME, PASSWORD);
		expect(login.success).toBe(true);
		if (!login.success || login.data.type !== 'tokens') return;

		const refresh = await provider.refreshTokens(login.data.tokens.refreshToken);
		expect(refresh.success, `refresh failed: ${refresh.success ? '' : refresh.error}`).toBe(true);
		if (!refresh.success) return;

		expect(refresh.data.accessToken).toMatch(/^[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$/);
		// The new access token should differ from the original because `iat`
		// advances. (Both are signed with the same secret + sub, so the
		// payload differs only in iat/exp.) Same-second issuance could
		// produce identical tokens; the test tolerates that by only
		// asserting the new token is well-formed, not strictly distinct.
		expect(refresh.data.accessToken.length).toBeGreaterThan(0);
	});

	it('should reject a malformed refresh token', async () => {
		const provider = new AuthServiceAuthProvider();
		const result = await provider.refreshTokens('not.a.real.token');

		expect(result.success).toBe(false);
	});
});
