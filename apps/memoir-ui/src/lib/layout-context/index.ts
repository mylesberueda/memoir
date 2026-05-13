import 'server-only';

import { getOrganizationContext } from '@lib/grpc/transport';
import type { Organization, ResourcePermission } from '@/lib/proto-shims';

export type LayoutContext = {
	orgs: Organization[];
	resolvedPid: string | undefined;
	cookieOrgPid: string | undefined;
	permissions: Record<string, ResourcePermission>;
};

export async function loadLayoutContext(_apiServiceUrl: string): Promise<LayoutContext> {
	const cookieOrgPid = await getOrganizationContext();
	return { orgs: [], resolvedPid: undefined, cookieOrgPid, permissions: {} };
}
