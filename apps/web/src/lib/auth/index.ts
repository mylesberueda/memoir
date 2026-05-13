import type { AuthProvider } from './base-provider';
import { ZitadelAuthProvider } from './providers/zitadel';

/**
 * Get the configured auth provider instance.
 *
 * Currently returns ZitadelAuthProvider.
 * To switch providers: change the import/instantiation here.
 * NOT a runtime environment variable check.
 */
export function getAuthProvider(): AuthProvider {
	return new ZitadelAuthProvider();
}

export { AuthProvider } from './base-provider';
// Re-export types for consumers
export type { AuthContext, AuthUser, LoginResult, TokenResponse } from './types';
