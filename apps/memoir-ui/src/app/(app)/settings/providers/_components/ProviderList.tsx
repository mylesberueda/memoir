'use client';

import type { ActionResult } from '@actions';
import type { ListProvidersResponse, Provider } from '@actions/providers';
import { Button } from '@components';
import { ProviderSource } from '@polypixel/proto-ts/rig-service/rig/v1/provider_pb';
import { AlertTriangle, Loader2 } from 'lucide-react';
import { useEffect, useMemo, useRef, useState } from 'react';
import ProviderRow from './ProviderRow';

interface ProviderListProps {
	providers: ActionResult<ListProvidersResponse> | undefined;
	isLoading: boolean;
	onRefresh: () => void;
	onEdit: (provider: Provider) => void;
	onDelete: (provider: Provider) => void;
}

const PROVIDERS_PER_BATCH = 20;

export default function ProviderList({
	providers: providersResult,
	isLoading,
	onRefresh,
	onEdit,
	onDelete,
}: ProviderListProps) {
	const [visibleUserCount, setVisibleUserCount] = useState(PROVIDERS_PER_BATCH);
	const [isLoadingMore, setIsLoadingMore] = useState(false);
	const loadMoreRef = useRef<HTMLDivElement>(null);

	const allProviders = providersResult?.success ? providersResult.data.providers : [];
	const error = providersResult && !providersResult.success ? providersResult.error : null;

	// Group providers by source
	const { systemProviders, userProviders } = useMemo(() => {
		const system: Provider[] = [];
		const user: Provider[] = [];

		for (const provider of allProviders) {
			if (provider.source === ProviderSource.SYSTEM) {
				system.push(provider);
			} else {
				user.push(provider);
			}
		}

		// Sort by name within each group
		system.sort((a, b) => a.name.localeCompare(b.name));
		user.sort((a, b) => a.name.localeCompare(b.name));

		return { systemProviders: system, userProviders: user };
	}, [allProviders]);

	// Visible user providers (for infinite scroll)
	const visibleUserProviders = useMemo(() => {
		return userProviders.slice(0, visibleUserCount);
	}, [userProviders, visibleUserCount]);

	const hasMoreUsers = visibleUserCount < userProviders.length;

	// Infinite scroll with Intersection Observer
	useEffect(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				const entry = entries[0];
				if (entry.isIntersecting && hasMoreUsers && !isLoadingMore) {
					setIsLoadingMore(true);
					setTimeout(() => {
						setVisibleUserCount((prev) => prev + PROVIDERS_PER_BATCH);
						setIsLoadingMore(false);
					}, 150);
				}
			},
			{
				rootMargin: '200px',
				threshold: 0,
			},
		);

		const currentRef = loadMoreRef.current;
		if (currentRef) {
			observer.observe(currentRef);
		}

		return () => {
			if (currentRef) {
				observer.unobserve(currentRef);
			}
		};
	}, [hasMoreUsers, isLoadingMore]);

	if (isLoading) {
		return (
			<div className="space-y-3">
				{[...Array(3)].map((_, i) => (
					// biome-ignore lint/suspicious/noArrayIndexKey: Static skeleton items
					<div key={`skeleton-${i}`} className="card bg-base-100 shadow-sm border border-base-200">
						<div className="card-body p-4">
							<div className="animate-pulse space-y-3">
								<div className="flex items-center gap-2">
									<div className="h-5 w-32 rounded bg-base-300" />
									<div className="h-4 w-16 rounded bg-base-300" />
								</div>
								<div className="h-3 w-48 rounded bg-base-300" />
								<div className="flex justify-between pt-2 border-t border-base-200">
									<div className="h-4 w-24 rounded bg-base-300" />
									<div className="h-8 w-20 rounded bg-base-300" />
								</div>
							</div>
						</div>
					</div>
				))}
			</div>
		);
	}

	if (error) {
		return (
			<div className="card bg-base-100 shadow">
				<div className="card-body flex flex-col items-center gap-4 py-12 text-center">
					<AlertTriangle className="h-12 w-12 text-error" />
					<h3 className="text-lg font-semibold text-base-content">Failed to Load Providers</h3>
					<p className="text-base-content/70">{error}</p>
					<Button color="primary" onClick={onRefresh}>
						Try Again
					</Button>
				</div>
			</div>
		);
	}

	if (allProviders.length === 0) {
		return (
			<div className="card bg-base-100 shadow">
				<div className="card-body flex flex-col items-center gap-2 py-12 text-center">
					<h3 className="text-lg font-semibold text-base-content">No Providers Yet</h3>
					<p className="text-base-content/70">Add your first LLM provider to start using AI models.</p>
				</div>
			</div>
		);
	}

	return (
		<div className="space-y-8">
			{/* User Providers Section */}
			{userProviders.length > 0 && (
				<section>
					<div className="mb-3 px-1">
						<h2 className="text-sm font-medium uppercase tracking-wider text-base-content/50">Your Providers</h2>
						<p className="text-xs text-base-content/40">
							{userProviders.length} provider{userProviders.length !== 1 ? 's' : ''}
						</p>
					</div>

					<div className="space-y-3">
						{visibleUserProviders.map((provider) => (
							<ProviderRow key={provider.identifier.value} provider={provider} onEdit={onEdit} onDelete={onDelete} />
						))}
					</div>

					{/* Infinite scroll trigger */}
					{hasMoreUsers && (
						<div ref={loadMoreRef} className="flex justify-center py-4">
							{isLoadingMore ? (
								<Loader2 className="w-5 h-5 text-base-content/50 animate-spin" />
							) : (
								<span className="text-sm text-base-content/50">Scroll for more</span>
							)}
						</div>
					)}
				</section>
			)}

			{/* System Providers Section */}
			{systemProviders.length > 0 && (
				<section>
					<div className="mb-3 px-1">
						<h2 className="text-sm font-medium uppercase tracking-wider text-base-content/50">System Providers</h2>
						<p className="text-xs text-base-content/40">
							Managed by the system &middot; {systemProviders.length} provider{systemProviders.length !== 1 ? 's' : ''}
						</p>
					</div>

					<div className="space-y-3">
						{systemProviders.map((provider) => (
							<ProviderRow key={provider.identifier.value} provider={provider} onEdit={onEdit} onDelete={onDelete} />
						))}
					</div>
				</section>
			)}
		</div>
	);
}
