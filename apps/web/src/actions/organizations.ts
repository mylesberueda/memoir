'use server';

import { create } from '@bufbuild/protobuf';
import { organizationServiceClient } from '@lib/grpc/clients';
import { createChildLogger } from '@lib/logger';
import {
	AddMemberRequestSchema,
	type AddMemberResponse,
	CreateOrganizationRequestSchema,
	type CreateOrganizationResponse,
	DeleteOrganizationRequestSchema,
	GetOrganizationRequestSchema,
	type GetOrganizationResponse,
	ListMembersRequestSchema,
	type ListMembersResponse,
	ListOrganizationsRequestSchema,
	type ListOrganizationsResponse,
	RemoveMemberRequestSchema,
	UpdateMemberRequestSchema,
	type UpdateMemberResponse,
	UpdateOrganizationRequestSchema,
	type UpdateOrganizationResponse,
} from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { revalidatePath } from 'next/cache';
import type { ActionResult } from '.';

const log = createChildLogger({ action: 'organizations' });

export async function getOrganizations(): Promise<ActionResult<ListOrganizationsResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListOrganizationsRequestSchema, {});
		const data = await client.listOrganizations(req);
		return { success: true, data };
	} catch (error) {
		log.error('Failed to list organizations', { error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function getOrganizationByPid(pid: string): Promise<ActionResult<GetOrganizationResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetOrganizationRequestSchema, { organizationPid: pid });
		const data = await client.getOrganization(req);
		return { success: true, data };
	} catch (error) {
		log.error('Failed to get organization', { pid, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function createOrg(
	name: string,
	slug: string,
	revalidatePages: string[] = ['/settings/organization', '/organizations'],
): Promise<ActionResult<CreateOrganizationResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CreateOrganizationRequestSchema, { name, slug });
		const data = await client.createOrganization(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data };
	} catch (error) {
		log.error('Failed to create organization', { name, slug, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function updateOrg(
	organizationPid: string,
	updates: { name?: string; slug?: string },
	revalidatePages: string[] = ['/settings/organization'],
): Promise<ActionResult<UpdateOrganizationResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UpdateOrganizationRequestSchema, {
			organizationPid,
			...updates,
		});

		const data = await client.updateOrganization(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data };
	} catch (error) {
		log.error('Failed to update organization', { organizationPid, updates, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function deleteOrg(
	organizationPid: string,
	revalidatePages: string[] = ['/settings/organization', '/organizations'],
): Promise<ActionResult<void>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(DeleteOrganizationRequestSchema, { organizationPid });
		await client.deleteOrganization(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: undefined };
	} catch (error) {
		log.error('Failed to delete organization', { organizationPid, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function getOrgMembers(organizationPid: string): Promise<ActionResult<ListMembersResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListMembersRequestSchema, { organizationPid });
		const data = await client.listMembers(req);
		return { success: true, data };
	} catch (error) {
		log.error('Failed to list organization members', { organizationPid, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function addOrgMember(
	organizationPid: string,
	userId: string,
	role: string,
	revalidatePages: string[] = ['/org/members', '/settings/organization'],
): Promise<ActionResult<AddMemberResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(AddMemberRequestSchema, {
			organizationPid,
			userId,
			role,
		});

		const data = await client.addMember(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data };
	} catch (error) {
		log.error('Failed to add organization member', { organizationPid, userId, role, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function addOrgMemberByEmail(
	organizationPid: string,
	email: string,
	role: string,
	revalidatePages: string[] = ['/org/members', '/settings/organization'],
): Promise<ActionResult<AddMemberResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(AddMemberRequestSchema, {
			organizationPid,
			email,
			role,
		});

		const data = await client.addMember(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data };
	} catch (error) {
		log.error('Failed to add organization member by email', { organizationPid, email, role, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function removeOrgMember(
	organizationPid: string,
	userId: string,
	revalidatePages: string[] = ['/settings/organization'],
): Promise<ActionResult<void>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(RemoveMemberRequestSchema, { organizationPid, userId });
		await client.removeMember(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: undefined };
	} catch (error) {
		log.error('Failed to remove organization member', { organizationPid, userId, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function updateOrgMemberRole(
	organizationPid: string,
	userId: string,
	role: string,
	revalidatePages: string[] = ['/settings/organization'],
): Promise<ActionResult<UpdateMemberResponse>> {
	try {
		const client = await organizationServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UpdateMemberRequestSchema, {
			organizationPid,
			userId,
			role,
		});

		const data = await client.updateMember(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data };
	} catch (error) {
		log.error('Failed to update organization member role', { organizationPid, userId, role, error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
