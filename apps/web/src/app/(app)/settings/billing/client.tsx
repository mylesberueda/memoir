'use client';

import { createCheckoutSession, createPortalSession } from '@actions/billing';
import { Button } from '@components';
import {
	BillingInterval,
	type PricingTier as ProtoPricingTier,
	Tier,
} from '@polypixel/proto-ts/api-service/api/v1/billing_pb';
import cns from 'classnames';
import { Check, ExternalLink, Loader2, Sparkles } from 'lucide-react';
import { useMemo, useState } from 'react';

interface DisplayPricingTier {
	id: Tier;
	name: string;
	monthlyPriceCents: number;
	annualPriceCents: number;
	currency: string;
	description: string;
	features: string[];
	isPopular: boolean;
	buttonText: string;
	discountPercent?: number;
}

const TIER_DESCRIPTIONS: Record<Tier, string> = {
	[Tier.UNSPECIFIED]: '',
	[Tier.FREE]: 'Get started for free',
	[Tier.PLUS]: 'For growing teams',
	[Tier.PRO]: 'For professionals',
	[Tier.ENTERPRISE]: 'Custom solutions',
};

function formatPrice(priceCents: number, currency: string): string {
	if (priceCents === 0) return '$0';
	if (priceCents === -1) return 'Custom';
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: currency.toUpperCase(),
		minimumFractionDigits: priceCents % 100 === 0 ? 0 : 2,
		maximumFractionDigits: 2,
	}).format(priceCents / 100);
}

function calculateDiscount(monthlyPriceCents: number, annualPriceCents: number): number | undefined {
	if (monthlyPriceCents <= 0 || annualPriceCents <= 0) return undefined;
	const yearlyIfMonthly = monthlyPriceCents * 12;
	const saved = yearlyIfMonthly - annualPriceCents;
	if (saved <= 0) return undefined;
	return Math.round((saved / yearlyIfMonthly) * 100);
}

function transformPricingTier(proto: ProtoPricingTier, currentTier: Tier): DisplayPricingTier {
	const isCurrentPlan = proto.tier === currentTier;
	const isEnterprise = proto.tier === Tier.ENTERPRISE;
	const isHigherTier = proto.tier > currentTier;

	let buttonText: string;
	if (isEnterprise) {
		buttonText = 'Contact Sales';
	} else if (isCurrentPlan) {
		buttonText = 'Current Plan';
	} else if (isHigherTier) {
		buttonText = 'Upgrade';
	} else {
		buttonText = 'Subscribe';
	}

	return {
		id: proto.tier,
		name: proto.name,
		monthlyPriceCents: proto.priceCents,
		annualPriceCents: proto.annualPriceCents,
		currency: proto.currency,
		description: TIER_DESCRIPTIONS[proto.tier] || '',
		features: [...proto.features],
		isPopular: proto.tier === Tier.PRO,
		buttonText,
		discountPercent: calculateDiscount(proto.priceCents, proto.annualPriceCents),
	};
}

function IntervalToggle({
	interval,
	onChange,
}: {
	interval: BillingInterval;
	onChange: (interval: BillingInterval) => void;
}) {
	return (
		<div id="interval_toggle__container" className="join">
			<button
				id="interval_toggle__monthly"
				type="button"
				className={cns('join-item btn btn-primary', interval === BillingInterval.MONTHLY ? 'btn-active' : 'btn-soft')}
				onClick={() => onChange(BillingInterval.MONTHLY)}>
				1 month
			</button>
			<button
				id="interval_toggle__annual"
				type="button"
				className={cns('join-item btn btn-primary', interval === BillingInterval.ANNUAL ? 'btn-active' : 'btn-soft')}
				onClick={() => onChange(BillingInterval.ANNUAL)}>
				12 months
			</button>
		</div>
	);
}

function PricingCard({
	tier,
	interval,
	onAction,
	isLoading,
	isCurrentPlan,
}: {
	tier: DisplayPricingTier;
	interval: BillingInterval;
	onAction: (tier: Tier) => void;
	isLoading: boolean;
	isCurrentPlan: boolean;
}) {
	const isEnterprise = tier.id === Tier.ENTERPRISE;
	const isCustom = tier.monthlyPriceCents === -1;
	const isFree = tier.monthlyPriceCents === 0;
	const tierSlug = tier.name.toLowerCase();

	const monthlyEquivalent =
		interval === BillingInterval.ANNUAL && !isCustom && !isFree
			? Math.round(tier.annualPriceCents / 12)
			: tier.monthlyPriceCents;

	const displayPrice = isCustom ? 'Custom' : formatPrice(monthlyEquivalent, tier.currency);

	let billingText: string | null = null;
	if (!isCustom && !isFree) {
		if (interval === BillingInterval.ANNUAL) {
			billingText = `Billed at ${formatPrice(tier.annualPriceCents, tier.currency)} every 12 months`;
		} else {
			billingText = `Billed at ${formatPrice(tier.monthlyPriceCents, tier.currency)} every month`;
		}
	}

	const showStrikethrough = interval === BillingInterval.ANNUAL && tier.discountPercent;

	return (
		<article
			id={`pricing_card__${tierSlug}`}
			className={cns(
				'card relative bg-base-100 border p-6 flex flex-col',
				tier.isPopular ? 'border-primary ring-1 ring-primary/20' : 'border-base-300',
				isCurrentPlan && 'bg-base-200',
			)}>
			{tier.isPopular && (
				<div id={`pricing_card__${tierSlug}_badge`} className="absolute -top-3 left-1/2 -translate-x-1/2 z-10">
					<div className="badge badge-primary gap-1 shadow-lg">
						<Sparkles className="h-3 w-3" />
						Recommended
					</div>
				</div>
			)}

			<div id={`pricing_card__${tierSlug}_header`} className="mb-4">
				<div className="flex items-center gap-2">
					<h3 className="text-xl font-bold tracking-tight">{tier.name}</h3>
					{interval === BillingInterval.ANNUAL && tier.discountPercent && (
						<span className="badge badge-success badge-sm">{tier.discountPercent}% off</span>
					)}
				</div>
				<p className="text-sm text-base-content/50">{tier.description}</p>
			</div>

			<div id={`pricing_card__${tierSlug}_price`} className="mb-6 pb-6 border-b border-base-300">
				<p className="text-base text-base-content/40 line-through min-h-6">
					{showStrikethrough && formatPrice(tier.monthlyPriceCents, tier.currency)}
				</p>
				<div className="flex items-baseline gap-1">
					<span className="text-4xl 2xl:text-3xl font-extrabold tracking-tight">{displayPrice}</span>
					{!isCustom && <span className="text-base text-base-content/50">/month</span>}
				</div>
				<p className="text-xs text-base-content/40 mt-1 min-h-4">{billingText}</p>
			</div>

			<ul id={`pricing_card__${tierSlug}_features`} className="flex flex-col gap-3 flex-1">
				{tier.features.map((feature) => (
					<li key={feature} className="flex items-start gap-3 text-sm">
						<Check className="h-4 w-4 shrink-0 text-success mt-0.5" />
						<span>{feature}</span>
					</li>
				))}
			</ul>

			<div id={`pricing_card__${tierSlug}_action`} className="mt-6">
				<Button
					id={`pricing_card__${tierSlug}_button`}
					color={tier.isPopular ? 'primary' : undefined}
					outline={!tier.isPopular}
					className="w-full"
					disabled={isCurrentPlan || isLoading}
					onClick={() => onAction(tier.id)}>
					{isLoading ? (
						<Loader2 className="h-4 w-4 animate-spin" />
					) : (
						<>
							{tier.buttonText}
							{isEnterprise && <ExternalLink className="ml-2 h-4 w-4" />}
						</>
					)}
				</Button>
			</div>
		</article>
	);
}

interface BillingClientProps {
	initialTier: Tier;
	expiresAt?: string;
	pricingTiers: ProtoPricingTier[];
}

export default function BillingClient({ initialTier, expiresAt, pricingTiers }: BillingClientProps) {
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [currentTier] = useState<Tier>(initialTier);
	const [interval, setInterval] = useState<BillingInterval>(BillingInterval.MONTHLY);

	const displayTiers = useMemo(
		() => pricingTiers.map((t) => transformPricingTier(t, currentTier)),
		[pricingTiers, currentTier],
	);

	const currentPlanName = displayTiers.find((t) => t.id === currentTier)?.name ?? 'Free';

	const handleAction = async (tier: Tier) => {
		setIsLoading(true);
		setError(null);

		try {
			const result = await createCheckoutSession(tier, interval);
			if (result.success) {
				window.location.href = result.data.redirectUrl;
			} else {
				setError(result.error);
			}
		} catch {
			setError('Failed to create checkout session');
		} finally {
			setIsLoading(false);
		}
	};

	const handleManageBilling = async () => {
		setIsLoading(true);
		setError(null);

		try {
			const result = await createPortalSession();
			if (result.success) {
				window.location.href = result.data.portalUrl;
			} else {
				setError(result.error);
			}
		} catch {
			setError('Failed to open billing portal');
		} finally {
			setIsLoading(false);
		}
	};

	return (
		<div id="billing_page__container" className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<header id="billing_page__header" className="mb-8">
				<h1 className="text-2xl font-bold tracking-tight">Billing</h1>
				<p className="mt-1 text-base-content/60">Manage your subscription and billing details</p>
			</header>

			{error && (
				<div id="billing_page__error" className="alert alert-error mb-6">
					<span>{error}</span>
				</div>
			)}

			<section id="billing_page__current_plan" className="card bg-base-100 border border-base-300 p-6 mb-10">
				<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
					<div id="current_plan__info">
						<p className="text-xs font-medium uppercase tracking-wide text-base-content/50">Current Plan</p>
						<p className="text-lg font-semibold">{currentPlanName}</p>
						{expiresAt && currentTier !== Tier.FREE && (
							<p className="text-sm text-base-content/50 mt-1">
								Renews{' '}
								{new Date(expiresAt).toLocaleDateString('en-US', {
									month: 'long',
									day: 'numeric',
									year: 'numeric',
								})}
							</p>
						)}
					</div>
					<Button id="current_plan__manage_button" ghost onClick={handleManageBilling} disabled={isLoading}>
						Manage Billing
						<ExternalLink className="ml-2 h-4 w-4" />
					</Button>
				</div>
			</section>

			<section id="billing_page__plans">
				<div id="plans__header" className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
					<h2 className="text-lg font-semibold">Choose your plan</h2>
					<IntervalToggle interval={interval} onChange={setInterval} />
				</div>

				<div id="plans__grid" className="grid grid-cols-1 gap-6 sm:grid-cols-2 2xl:grid-cols-4">
					{displayTiers.map((tier) => (
						<PricingCard
							key={tier.id}
							tier={tier}
							interval={interval}
							onAction={handleAction}
							isLoading={isLoading}
							isCurrentPlan={tier.id === currentTier}
						/>
					))}
				</div>
			</section>
		</div>
	);
}
