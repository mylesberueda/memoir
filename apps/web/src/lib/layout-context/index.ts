import 'server-only';

import { create } from '@bufbuild/protobuf';
import { createAuthenticatedTransport, getOrganizationContext } from '@lib/grpc/transport';
import { createChildLogger } from '@lib/logger';
import { createClient } from '@connectrpc/connect';
import {
	GetOrganizationRequestSchema,
	ListOrganizationsRequestSchema,
	type Organization,
	OrganizationService,
	type ResourcePermission,
} from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { resolveOrgPid } from './resolve';

const log = createChildLogger({ module: 'layout-context' });

export type LayoutContext = {
	orgs: Organization[];
	resolvedPid: string | undefined;
	cookieOrgPid: string | undefined;
	permissions: Record<string, ResourcePermission>;
};

export async function loadLayoutContext(apiServiceUrl: string): Promise<LayoutContext> {
	const cookieOrgPid = await getOrganizationContext();

	const discoveryTransport = await createAuthenticatedTransport(apiServiceUrl, { mode: 'none' });
	if (!discoveryTransport) {
		return { orgs: [], resolvedPid: undefined, cookieOrgPid, permissions: {} };
	}

	const orgClient = createClient(OrganizationService, discoveryTransport);

	let orgs: Organization[] = [];
	try {
		const listRes = await orgClient.listOrganizations(create(ListOrganizationsRequestSchema, {}));
		orgs = listRes.organizations;
	} catch (error) {
		log.error('Failed to list organizations during layout-context load', { error });
		return { orgs: [], resolvedPid: undefined, cookieOrgPid, permissions: {} };
	}

	const resolvedPid = resolveOrgPid(orgs, cookieOrgPid);
	if (!resolvedPid) {
		return { orgs, resolvedPid: undefined, cookieOrgPid, permissions: {} };
	}

	const permTransport = await createAuthenticatedTransport(apiServiceUrl, { mode: 'explicit', pid: resolvedPid });
	if (!permTransport) {
		return { orgs, resolvedPid, cookieOrgPid, permissions: {} };
	}

	const permClient = createClient(OrganizationService, permTransport);
	try {
		const getRes = await permClient.getOrganization(
			create(GetOrganizationRequestSchema, { organizationPid: resolvedPid }),
		);
		return { orgs, resolvedPid, cookieOrgPid, permissions: getRes.permissions };
	} catch (error) {
		log.error('Failed to fetch permissions for resolved org', { resolvedPid, error });
		return { orgs, resolvedPid, cookieOrgPid, permissions: {} };
	}
}
