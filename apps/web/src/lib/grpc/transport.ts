'use server';

import { getAccessToken, getIdToken } from '@actions/auth';
import type { Interceptor, Transport } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import { cookies } from 'next/headers';

const COOKIE_NAME_ORGANIZATION_ID = 'x-organization-id';
const COOKIE_NAME_ID_TOKEN = 'x-id-token';

/**
 * Create an interceptor that adds auth, identity, and organization headers.
 */
function createAuthInterceptor(accessToken: string, idToken?: string, organizationId?: string): Interceptor {
	return (next) => async (req) => {
		req.header.set('Authorization', `Bearer ${accessToken}`);

		if (idToken) {
			req.header.set(COOKIE_NAME_ID_TOKEN, idToken);
		}

		if (organizationId) {
			req.header.set(COOKIE_NAME_ORGANIZATION_ID, organizationId);
		}
		return next(req);
	};
}

export type OrgPidSource = { mode: 'cookie' } | { mode: 'none' } | { mode: 'explicit'; pid: string };

export async function createAuthenticatedTransport(
	baseUrl: string,
	orgPid: OrgPidSource = { mode: 'cookie' },
): Promise<Transport | null> {
	const accessToken = await getAccessToken();
	if (!accessToken) {
		return null;
	}

	const idToken = await getIdToken();

	let organizationId: string | undefined;
	switch (orgPid.mode) {
		case 'cookie': {
			const cookieStore = await cookies();
			organizationId = cookieStore.get(COOKIE_NAME_ORGANIZATION_ID)?.value;
			break;
		}
		case 'explicit':
			organizationId = orgPid.pid;
			break;
		case 'none':
			organizationId = undefined;
			break;
	}

	return createGrpcTransport({
		baseUrl,
		interceptors: [createAuthInterceptor(accessToken, idToken ?? undefined, organizationId)],
	});
}

/**
 * Create an unauthenticated gRPC transport (for public endpoints).
 */
export async function createPublicTransport(baseUrl: string): Promise<Transport> {
	return createGrpcTransport({ baseUrl });
}

export async function createTransportWithToken(
	baseUrl: string,
	accessToken: string,
	idToken?: string,
	orgPid: OrgPidSource = { mode: 'cookie' },
): Promise<Transport> {
	let organizationId: string | undefined;
	switch (orgPid.mode) {
		case 'cookie': {
			const cookieStore = await cookies();
			organizationId = cookieStore.get(COOKIE_NAME_ORGANIZATION_ID)?.value;
			break;
		}
		case 'explicit':
			organizationId = orgPid.pid;
			break;
		case 'none':
			organizationId = undefined;
			break;
	}

	return createGrpcTransport({
		baseUrl,
		interceptors: [createAuthInterceptor(accessToken, idToken, organizationId)],
	});
}

/**
 * Set the current organization context.
 * Pass undefined/null to clear (personal context).
 */
export async function setOrganizationContext(organizationId: string | null | undefined): Promise<void> {
	const cookieStore = await cookies();

	if (organizationId) {
		cookieStore.set(COOKIE_NAME_ORGANIZATION_ID, organizationId, {
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
			sameSite: 'strict',
			path: '/',
			// No maxAge = session cookie (clears on browser close)
			// Or set a long duration if you want persistence:
			maxAge: 365 * 24 * 60 * 60, // 1 year
		});
	} else {
		cookieStore.delete(COOKIE_NAME_ORGANIZATION_ID);
	}
}

/**
 * Get the current organization context from cookie.
 * Returns undefined if no cookie is set.
 */
export async function getOrganizationContext(): Promise<string | undefined> {
	const cookieStore = await cookies();
	return cookieStore.get(COOKIE_NAME_ORGANIZATION_ID)?.value;
}
