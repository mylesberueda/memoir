'use client';

import type { ActionResult } from '@actions';
import type { ListProvidersResponse, Provider } from '@actions/providers';
import { ProviderSource } from '@startup/proto-ts/rig-service/rig/v1/provider_pb';
import cns from 'classnames';
import { AlertTriangle } from 'lucide-react';
import { useEffect, useMemo, useState } from 'react';
import ProviderCard from './ProviderCard';

interface ProviderGridProps {
	providers: ActionResult<ListProvidersResponse> | undefined;
	isLoading: boolean;
	onRefresh: () => void;
	onEdit: (provider: Provider) => void;
	onDelete: (provider: Provider) => void;
}

const PROVIDERS_PER_PAGE = 12;

export default function ProviderGrid({
	providers: providersResult,
	isLoading,
	onRefresh,
	onEdit,
	onDelete,
}: ProviderGridProps) {
	const [currentPage, setCurrentPage] = useState(1);

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

		return { systemProviders: system, userProviders: user };
	}, [allProviders]);

	// Client-side pagination for user providers only
	const paginatedUserProviders = useMemo(() => {
		const startIndex = (currentPage - 1) * PROVIDERS_PER_PAGE;
		const endIndex = startIndex + PROVIDERS_PER_PAGE;
		return userProviders.slice(startIndex, endIndex);
	}, [userProviders, currentPage]);

	const totalPages = Math.ceil(userProviders.length / PROVIDERS_PER_PAGE);

	useEffect(() => {
		if (currentPage > 1) {
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}, [currentPage]);

	const handlePageChange = (page: number) => {
		setCurrentPage(page);
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
				{[...Array(6)].map((_, i) => (
					// biome-ignore lint/suspicious/noArrayIndexKey: Static skeleton items with fixed order
					<div key={`skeleton-${i}`} className="card bg-base-100 shadow">
						<div className="card-body">
							<div className="animate-pulse">
								<div className="mb-2 h-4 w-3/4 rounded bg-base-300" />
								<div className="mb-4 h-3 w-1/2 rounded bg-base-300" />
								<div className="mb-4 h-12 rounded bg-base-300" />
								<div className="flex gap-2">
									<div className="h-8 w-16 rounded bg-base-300" />
									<div className="h-8 w-16 rounded bg-base-300" />
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
			<div className="card bg-base-100 shadow-xl">
				<div className="card-body flex flex-col items-center gap-4 py-12 text-center">
					<AlertTriangle className="h-12 w-12 text-error" />
					<h3 className="text-lg font-semibold text-base-content">Failed to Load Providers</h3>
					<p className="text-base-content/70">{error}</p>
					<button type="button" className="btn btn-primary" onClick={onRefresh}>
						Try Again
					</button>
				</div>
			</div>
		);
	}

	if (allProviders.length === 0) {
		return (
			<div className="card bg-base-100 shadow-xl">
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
				<section className="flex flex-col gap-4">
					<h2 className="text-xl font-semibold text-base-content">Your Providers</h2>
					<div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
						{paginatedUserProviders.map((provider) => (
							<ProviderCard key={provider.identifier.value} provider={provider} onEdit={onEdit} onDelete={onDelete} />
						))}
					</div>

					{/* Pagination */}
					{totalPages > 1 && (
						<div className="flex justify-center">
							<div className="join">
								<button
									type="button"
									className="btn join-item"
									onClick={() => handlePageChange(currentPage - 1)}
									disabled={currentPage <= 1}>
									« Prev
								</button>
								{Array.from({ length: totalPages }, (_, i) => i + 1).map((page) => (
									<button
										key={page}
										type="button"
										className={cns('btn join-item', currentPage === page && 'btn-active')}
										onClick={() => handlePageChange(page)}>
										{page}
									</button>
								))}
								<button
									type="button"
									className="btn join-item"
									onClick={() => handlePageChange(currentPage + 1)}
									disabled={currentPage >= totalPages}>
									Next »
								</button>
							</div>
						</div>
					)}
				</section>
			)}

			{/* System Providers Section */}
			{systemProviders.length > 0 && (
				<section className="flex flex-col gap-4">
					<div>
						<h2 className="text-xl font-semibold text-base-content">System Providers</h2>
						<p className="text-sm text-base-content/70">
							These providers are managed by the system and are available to all users.
						</p>
					</div>
					<div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
						{systemProviders.map((provider) => (
							<ProviderCard key={provider.identifier.value} provider={provider} onEdit={onEdit} onDelete={onDelete} />
						))}
					</div>
				</section>
			)}
		</div>
	);
}
