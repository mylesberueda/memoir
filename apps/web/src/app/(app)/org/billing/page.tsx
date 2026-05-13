import { getCurrentPlan, getPricing } from '@actions/billing';
import { Tier } from '@startup/proto-ts/api-service/api/v1/billing_pb';
import BillingClient from '../../settings/billing/client';

export const metadata = {
	title: 'Organization Billing',
};

export default async function OrgBillingPage() {
	const [planResult, pricingResult] = await Promise.all([getCurrentPlan(), getPricing()]);

	const currentTier = planResult.success ? planResult.data.tier : Tier.FREE;
	const expiresAt = planResult.success ? planResult.data.expiresAt : undefined;
	const pricingTiers = pricingResult.success ? pricingResult.data.tiers : [];

	return <BillingClient initialTier={currentTier} expiresAt={expiresAt} pricingTiers={pricingTiers} />;
}
