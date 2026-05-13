import type { Organization } from '@polypixel/proto-ts/api-service/api/v1/organizations_pb';

export function resolveOrgPid(orgs: Organization[], cookieOrgPid: string | undefined): string | undefined {
	if (cookieOrgPid && orgs.some((o) => o.pid === cookieOrgPid)) {
		return cookieOrgPid;
	}

	return orgs.at(0)?.pid;
}
