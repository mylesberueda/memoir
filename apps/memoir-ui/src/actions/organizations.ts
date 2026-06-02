'use server';

import type { Organization, OrganizationMember, ResourcePermission } from '@/lib/proto-shims';
import type { ActionResult } from '.';

type ListOrganizationsResponse = { organizations: Organization[] };
type GetOrganizationResponse = {
	organization?: Organization;
	userRole?: string;
	permissions?: Record<string, ResourcePermission>;
};
type CreateOrganizationResponse = { organization?: Organization };
type UpdateOrganizationResponse = { organization?: Organization };
type ListMembersResponse = { members: OrganizationMember[] };
type AddMemberResponse = { member?: OrganizationMember };
type UpdateMemberResponse = { member?: OrganizationMember };

const NOT_IMPLEMENTED: ActionResult<never> = { success: false, error: 'Not implemented' };

export async function getOrganizations(): Promise<ActionResult<ListOrganizationsResponse>> {
	return NOT_IMPLEMENTED;
}

export async function getOrganizationByPid(_pid: string): Promise<ActionResult<GetOrganizationResponse>> {
	return NOT_IMPLEMENTED;
}

export async function createOrg(
	_name: string,
	_slug: string,
	_revalidatePages: string[] = ['/settings/organization', '/organizations'],
): Promise<ActionResult<CreateOrganizationResponse>> {
	return NOT_IMPLEMENTED;
}

export async function updateOrg(
	_organizationPid: string,
	_updates: { name?: string; slug?: string },
	_revalidatePages: string[] = ['/settings/organization'],
): Promise<ActionResult<UpdateOrganizationResponse>> {
	return NOT_IMPLEMENTED;
}

export async function deleteOrg(
	_organizationPid: string,
	_revalidatePages: string[] = ['/settings/organization', '/organizations'],
): Promise<ActionResult<void>> {
	return NOT_IMPLEMENTED;
}

export async function getOrgMembers(_organizationPid: string): Promise<ActionResult<ListMembersResponse>> {
	return NOT_IMPLEMENTED;
}

export async function addOrgMember(
	_organizationPid: string,
	_userId: string,
	_role: string,
	_revalidatePages: string[] = ['/org/members', '/settings/organization'],
): Promise<ActionResult<AddMemberResponse>> {
	return NOT_IMPLEMENTED;
}

export async function addOrgMemberByEmail(
	_organizationPid: string,
	_email: string,
	_role: string,
	_revalidatePages: string[] = ['/org/members', '/settings/organization'],
): Promise<ActionResult<AddMemberResponse>> {
	return NOT_IMPLEMENTED;
}

export async function removeOrgMember(
	_organizationPid: string,
	_userId: string,
	_revalidatePages: string[] = ['/settings/organization'],
): Promise<ActionResult<void>> {
	return NOT_IMPLEMENTED;
}

export async function updateOrgMemberRole(
	_organizationPid: string,
	_userId: string,
	_role: string,
	_revalidatePages: string[] = ['/settings/organization'],
): Promise<ActionResult<UpdateMemberResponse>> {
	return NOT_IMPLEMENTED;
}
