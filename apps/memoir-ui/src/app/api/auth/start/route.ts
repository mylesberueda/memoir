import { startOidcFlow } from '@actions/auth';
import { redirect } from 'next/navigation';

/**
 * GET /api/auth/start
 *
 * Starts the OIDC flow by generating state/PKCE cookies and redirecting to Zitadel.
 * This is a Route Handler so it can set cookies (unlike Server Components).
 */
export async function GET() {
	const oidcUrl = await startOidcFlow();
	redirect(oidcUrl);
}
