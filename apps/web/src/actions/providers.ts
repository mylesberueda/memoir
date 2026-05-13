'use server';

import { create, type JsonObject } from '@bufbuild/protobuf';
import { providerServiceClient } from '@lib/grpc/clients';
import {
	CreateProviderRequestSchema,
	DeleteProviderRequestSchema,
	GetProviderRequestSchema,
	ListProvidersRequestSchema,
	type ListProvidersResponse as ProtoListProvidersResponse,
	type Provider as ProtoProvider,
	ProviderSource,
	UpdateProviderRequestSchema,
} from '@startup/proto-ts/rig-service/rig/v1/provider_pb';
import { revalidatePath } from 'next/cache';

import type { ActionResult } from '.';

type ProviderIdentifier = ProtoProvider['identifier'];
type PidIdentifier = Extract<ProviderIdentifier, { case: 'pid' }>;
export type Provider = Omit<ProtoProvider, 'identifier'> & { identifier: PidIdentifier };

export type ListProvidersResponse = Omit<ProtoListProvidersResponse, 'providers'> & { providers: Provider[] };

export async function getProviders(): Promise<ActionResult<ListProvidersResponse>> {
	try {
		const client = await providerServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListProvidersRequestSchema, { isActive: true });
		const res = await client.listProviders(req);

		const providers = res.providers.filter((p): p is Provider => {
			if (p.identifier.case === 'id') {
				console.warn('[providerServiceClient::listProviders]', 'Found id in public-facing response.');
				return false;
			}

			return p.identifier.case === 'pid';
		});

		return { success: true, data: { ...res, providers } };
	} catch (error) {
		console.error('getProvidersData error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function getProvider(pid: string): Promise<ActionResult<Provider>> {
	try {
		const client = await providerServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetProviderRequestSchema, { pid });
		const res = await client.getProvider(req);

		if (!res.provider || res.provider.identifier.case !== 'pid') {
			return { success: false, error: 'Provider not found' };
		}

		return { success: true, data: res.provider as Provider };
	} catch (error) {
		console.error('getProviderAction error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export type CreateProviderInput = {
	name: string;
	providerType: string;
	credentials?: string;
	endpointUrl?: string;
	config?: JsonObject;
};

export async function createProvider(
	input: CreateProviderInput,
	revalidatePages: string[] = ['/settings/providers'],
): Promise<ActionResult<Provider>> {
	try {
		const client = await providerServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CreateProviderRequestSchema, {
			name: input.name,
			providerType: input.providerType,
			source: ProviderSource.USER,
			credentials: input.credentials ?? '',
			endpointUrl: input.endpointUrl ?? '',
			config: input.config,
		});

		const res = await client.createProvider(req);

		if (!res.provider || res.provider.identifier.case !== 'pid') {
			return { success: false, error: 'Failed to create provider' };
		}

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: res.provider as Provider };
	} catch (error) {
		console.error('createProviderAction error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export type UpdateProviderInput = {
	pid: string;
	name?: string;
	providerType?: string;
	credentials?: string;
	endpointUrl?: string;
	config?: JsonObject;
	isActive?: boolean;
};

export async function updateProvider(
	input: UpdateProviderInput,
	revalidatePages: string[] = ['/settings/providers'],
): Promise<ActionResult<Provider>> {
	try {
		const client = await providerServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UpdateProviderRequestSchema, {
			pid: input.pid,
			name: input.name,
			providerType: input.providerType,
			credentials: input.credentials,
			endpointUrl: input.endpointUrl,
			config: input.config,
			isActive: input.isActive,
		});

		const res = await client.updateProvider(req);

		if (!res.provider || res.provider.identifier.case !== 'pid') {
			return { success: false, error: 'Failed to update provider' };
		}

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: res.provider as Provider };
	} catch (error) {
		console.error('updateProviderAction error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function deleteProvider(
	pid: string,
	revalidatePages: string[] = ['/settings/providers'],
): Promise<ActionResult<void>> {
	try {
		const client = await providerServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(DeleteProviderRequestSchema, { pid });
		await client.deleteProvider(req);

		for (const path of revalidatePages) {
			revalidatePath(path);
		}

		return { success: true, data: undefined };
	} catch (error) {
		console.error('deleteProviderAction error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
