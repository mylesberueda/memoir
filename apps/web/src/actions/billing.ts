'use server';

import { create } from '@bufbuild/protobuf';
import { billingServiceClient } from '@lib/grpc/clients';
import { actionLogger } from '@lib/logger';
import {
	type BillingInterval,
	CreateCheckoutSessionRequestSchema,
	type CreateCheckoutSessionResponse,
	CreatePortalSessionRequestSchema,
	type CreatePortalSessionResponse,
	GetCurrentPlanRequestSchema,
	type GetCurrentPlanResponse,
	GetPricingRequestSchema,
	type GetPricingResponse,
	type Tier,
} from '@startup/proto-ts/api-service/api/v1/billing_pb';

import type { ActionResult } from '.';

export async function getCurrentPlan(): Promise<ActionResult<GetCurrentPlanResponse>> {
	try {
		const client = await billingServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetCurrentPlanRequestSchema, {});
		const res = await client.getCurrentPlan(req);

		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('getCurrentPlan failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function getPricing(): Promise<ActionResult<GetPricingResponse>> {
	try {
		const client = await billingServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetPricingRequestSchema, {});
		const res = await client.getPricing(req);

		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('getPricing failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function createCheckoutSession(
	tier: Tier,
	interval: BillingInterval,
): Promise<ActionResult<CreateCheckoutSessionResponse>> {
	try {
		const client = await billingServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CreateCheckoutSessionRequestSchema, { tier, interval });
		const res = await client.createCheckoutSession(req);

		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('createCheckoutSession failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}

export async function createPortalSession(): Promise<ActionResult<CreatePortalSessionResponse>> {
	try {
		const client = await billingServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(CreatePortalSessionRequestSchema, {});
		const res = await client.createPortalSession(req);

		return { success: true, data: res };
	} catch (error) {
		actionLogger.error('createPortalSession failed', { error: error instanceof Error ? error.message : error });
		return { success: false, error: error instanceof Error ? error.message : 'An unexpected error occurred' };
	}
}
