import type { AuthProvider } from './base-provider';
import { AuthServiceAuthProvider } from './providers/auth-service';

/**
 * Get the configured auth provider instance.
 *
 * Returns AuthServiceAuthProvider — memoir-service's username/password
 * → JWT flow. To switch providers: change the import/instantiation here.
 * NOT a runtime environment variable check.
 */
export function getAuthProvider(): AuthProvider {
	return new AuthServiceAuthProvider();
}

export { AuthProvider } from './base-provider';
// Re-export types for consumers
export type { AuthContext, AuthUser, LoginResult, TokenResponse } from './types';
