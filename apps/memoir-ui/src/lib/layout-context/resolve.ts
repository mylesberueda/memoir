import type { Organization } from '@/lib/proto-shims';

export function resolveOrgPid(orgs: Organization[], cookieOrgPid: string | undefined): string | undefined {
	if (cookieOrgPid && orgs.some((o) => o.pid === cookieOrgPid)) {
		return cookieOrgPid;
	}

	return orgs.at(0)?.pid;
}
