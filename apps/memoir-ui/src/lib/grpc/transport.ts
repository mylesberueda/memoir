'use server';

import { cookies } from 'next/headers';

const COOKIE_NAME_ORGANIZATION_ID = 'x-organization-id';

export type OrgPidSource = { mode: 'cookie' } | { mode: 'none' } | { mode: 'explicit'; pid: string };

export async function setOrganizationContext(organizationId: string | null | undefined): Promise<void> {
	const cookieStore = await cookies();

	if (organizationId) {
		cookieStore.set(COOKIE_NAME_ORGANIZATION_ID, organizationId, {
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
			sameSite: 'strict',
			path: '/',
			maxAge: 365 * 24 * 60 * 60,
		});
	} else {
		cookieStore.delete(COOKIE_NAME_ORGANIZATION_ID);
	}
}

export async function getOrganizationContext(): Promise<string | undefined> {
	const cookieStore = await cookies();
	return cookieStore.get(COOKIE_NAME_ORGANIZATION_ID)?.value;
}
