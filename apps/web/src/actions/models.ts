'use server';

import { create } from '@bufbuild/protobuf';
import { modelServiceClient } from '@lib/grpc/clients';
import {
	ListModelsRequestSchema,
	type ListModelsResponse as ProtoListModelsResponse,
	type Model as ProtoModel,
} from '@startup/proto-ts/rig-service/rig/v1/provider_pb';
import type { ActionResult } from '.';

type ModelIdentifier = ProtoModel['identifier'];
type PidIdentifier = Extract<ModelIdentifier, { case: 'pid' }>;
export type Model = Omit<ProtoModel, 'identifier'> & { identifier: PidIdentifier };

export type ListModelsResponse = Omit<ProtoListModelsResponse, 'models'> & { models: Model[] };

export async function getModels(): Promise<ActionResult<ListModelsResponse>> {
	try {
		const client = await modelServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListModelsRequestSchema, {
			isActive: true,
			includeDeprecated: false,
			page: 1,
			pageSize: 500,
		});

		const res = await client.listModels(req);

		const models = res.models.filter((m): m is Model => {
			if (m.identifier.case === 'id') {
				console.warn('[modelServiceClient::listModels]', 'Found id in public-facing response.');
				return false;
			}

			return m.identifier.case === 'pid';
		});

		return { success: true, data: { ...res, models } };
	} catch (error) {
		console.error('getModelsData error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function getModelsForProvider(providerPid: string): Promise<ActionResult<ListModelsResponse>> {
	try {
		const client = await modelServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListModelsRequestSchema, {
			providerPid,
			isActive: true,
			includeDeprecated: false,
			page: 1,
			pageSize: 500,
		});

		const res = await client.listModels(req);

		const models = res.models.filter((m): m is Model => {
			if (m.identifier.case === 'id') {
				console.warn('[modelServiceClient::listModels]', 'Found id in public-facing response.');
				return false;
			}

			return m.identifier.case === 'pid';
		});

		return { success: true, data: { ...res, models } };
	} catch (error) {
		console.error('getModelsForProvider error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
