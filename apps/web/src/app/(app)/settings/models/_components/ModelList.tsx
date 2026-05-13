'use client';

import type { ActionResult } from '@actions';
import type { ListModelsResponse } from '@actions/models';
import type { ListProvidersResponse } from '@actions/providers';
import { AlertTriangle, Loader2 } from 'lucide-react';
import { useSearchParams } from 'next/navigation';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import ModelRow from './ModelRow';

interface ModelListProps {
	models: ActionResult<ListModelsResponse> | undefined;
	providers: ActionResult<ListProvidersResponse> | undefined;
}

type SortField = 'name' | 'provider' | 'contextLength';
type SortDirection = 'asc' | 'desc';

const MODELS_PER_BATCH = 20;

export default function ModelList({ models: modelsResult, providers: providersResult }: ModelListProps) {
	const searchParams = useSearchParams();
	const [sortField, setSortField] = useState<SortField>('name');
	const [sortDirection, setSortDirection] = useState<SortDirection>('asc');
	const [visibleCount, setVisibleCount] = useState(MODELS_PER_BATCH);
	const [isLoadingMore, setIsLoadingMore] = useState(false);
	const loadMoreRef = useRef<HTMLDivElement>(null);

	const allModels = modelsResult?.success ? modelsResult.data.models : [];
	const allProviders = providersResult?.success ? providersResult.data.providers : [];
	const error = modelsResult && !modelsResult.success ? modelsResult.error : null;

	// Create provider lookup map
	const providerMap = useMemo(() => {
		return new Map(allProviders.map((p) => [p.identifier.value, p]));
	}, [allProviders]);

	// Filter models based on URL params
	const filteredModels = useMemo(() => {
		const providerFilter = searchParams.get('provider');
		const searchFilter = searchParams.get('search')?.toLowerCase();
		const capabilityFilter = searchParams.get('capability');
		const showDeprecated = searchParams.get('deprecated') === 'true';
		const showInactive = searchParams.get('inactive') === 'true';

		return allModels.filter((model) => {
			// Provider filter
			if (providerFilter && model.providerPid !== providerFilter) {
				return false;
			}

			// Search filter
			if (searchFilter) {
				const matchesName = model.name.toLowerCase().includes(searchFilter);
				const matchesModelId = model.modelId.toLowerCase().includes(searchFilter);
				const matchesProvider = model.providerName.toLowerCase().includes(searchFilter);
				if (!matchesName && !matchesModelId && !matchesProvider) {
					return false;
				}
			}

			// Capability filter
			if (capabilityFilter) {
				const caps = model.capabilities;
				switch (capabilityFilter) {
					case 'vision':
						if (!caps?.vision) return false;
						break;
					case 'functions':
						if (!caps?.functionCalling) return false;
						break;
					case 'json':
						if (!caps?.jsonMode) return false;
						break;
					case 'streaming':
						if (!caps?.streaming) return false;
						break;
				}
			}

			// Deprecated filter (only show deprecated if toggle is on)
			if (!showDeprecated && model.deprecationMessage) {
				return false;
			}

			// Inactive filter
			if (!showInactive && !model.isActive) {
				return false;
			}

			return true;
		});
	}, [allModels, searchParams]);

	// Sort models
	const sortedModels = useMemo(() => {
		return [...filteredModels].sort((a, b) => {
			let comparison = 0;

			switch (sortField) {
				case 'name':
					comparison = a.name.localeCompare(b.name);
					break;
				case 'provider':
					comparison = a.providerName.localeCompare(b.providerName);
					break;
				case 'contextLength':
					comparison = (a.contextLength ?? 0) - (b.contextLength ?? 0);
					break;
			}

			return sortDirection === 'asc' ? comparison : -comparison;
		});
	}, [filteredModels, sortField, sortDirection]);

	// Reset visible count when filters change
	useEffect(() => {
		setVisibleCount(MODELS_PER_BATCH);
	}, []);

	// Visible models (for infinite scroll)
	const visibleModels = useMemo(() => {
		return sortedModels.slice(0, visibleCount);
	}, [sortedModels, visibleCount]);

	const hasMore = visibleCount < sortedModels.length;

	// Infinite scroll with Intersection Observer
	useEffect(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				const entry = entries[0];
				if (entry.isIntersecting && hasMore && !isLoadingMore) {
					setIsLoadingMore(true);
					// Small delay to show loading state and prevent rapid firing
					setTimeout(() => {
						setVisibleCount((prev) => prev + MODELS_PER_BATCH);
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
	}, [hasMore, isLoadingMore]);

	const handleSort = useCallback(
		(field: SortField) => {
			if (sortField === field) {
				setSortDirection((prev) => (prev === 'asc' ? 'desc' : 'asc'));
			} else {
				setSortField(field);
				setSortDirection('asc');
			}
			setVisibleCount(MODELS_PER_BATCH);
		},
		[sortField],
	);

	const SortButton = ({
		field,
		children,
		className,
	}: {
		field: SortField;
		children: React.ReactNode;
		className?: string;
	}) => (
		<button
			type="button"
			className={`flex items-center gap-1 text-xs font-medium uppercase tracking-wider text-base-content/50 hover:text-base-content transition-colors ${className ?? ''}`}
			onClick={() => handleSort(field)}>
			{children}
			{sortField === field && <span className="text-primary">{sortDirection === 'asc' ? '↑' : '↓'}</span>}
		</button>
	);

	if (error) {
		return (
			<div className="card bg-base-100 shadow">
				<div className="card-body flex flex-col items-center gap-4 py-12 text-center">
					<AlertTriangle className="h-12 w-12 text-error" />
					<h3 className="text-lg font-semibold text-base-content">Failed to Load Models</h3>
					<p className="text-base-content/70">{error}</p>
					<button type="button" className="btn btn-primary" onClick={() => window.location.reload()}>
						Try Again
					</button>
				</div>
			</div>
		);
	}

	if (allModels.length === 0) {
		return (
			<div className="card bg-base-100 shadow">
				<div className="card-body flex flex-col items-center gap-2 py-12 text-center">
					<h3 className="text-lg font-semibold text-base-content">No Models Available</h3>
					<p className="text-base-content/70">
						Models will appear here once you add providers with valid API credentials.
					</p>
				</div>
			</div>
		);
	}

	if (filteredModels.length === 0) {
		return (
			<div className="card bg-base-100 shadow">
				<div className="card-body flex flex-col items-center gap-2 py-12 text-center">
					<h3 className="text-lg font-semibold text-base-content">No Models Match Filters</h3>
					<p className="text-base-content/70">Try adjusting your search or filter criteria.</p>
				</div>
			</div>
		);
	}

	return (
		<div className="card bg-base-100 shadow overflow-hidden">
			{/* Header row with sort controls */}
			<div
				className={[
					'px-4 py-3 border-b border-base-200 bg-base-200/30',
					'grid items-center gap-4',
					'grid-cols-[auto_1fr_auto]',
					'sm:grid-cols-[auto_1fr_8rem_auto]',
					'md:grid-cols-[auto_1fr_8rem_5rem_auto]',
					'lg:grid-cols-[auto_1fr_8rem_5rem_6rem_auto]',
				].join(' ')}>
				{/* Spacer for status dot */}
				<div className="w-2" />

				<SortButton field="name">Model</SortButton>

				<div className="hidden sm:block">
					<SortButton field="provider">Provider</SortButton>
				</div>

				<div className="hidden md:block text-right">
					<SortButton field="contextLength" className="justify-end">
						Context
					</SortButton>
				</div>

				<span className="hidden lg:block text-xs font-medium uppercase tracking-wider text-base-content/50">
					Capabilities
				</span>

				{/* Spacer for chevron */}
				<div className="w-4" />
			</div>

			{/* Model rows */}
			<div>
				{visibleModels.map((model) => (
					<ModelRow key={model.identifier.value} model={model} provider={providerMap.get(model.providerPid)} />
				))}
			</div>

			{/* Infinite scroll trigger / loading indicator */}
			{hasMore && (
				<div ref={loadMoreRef} className="flex justify-center py-4 border-t border-base-200">
					{isLoadingMore ? (
						<Loader2 className="w-5 h-5 text-base-content/50 animate-spin" />
					) : (
						<span className="text-sm text-base-content/50">Scroll for more</span>
					)}
				</div>
			)}

			{/* Footer with count */}
			<div className="px-4 py-3 border-t border-base-200 bg-base-200/30">
				<p className="text-xs text-base-content/50">
					Showing {visibleModels.length} of {sortedModels.length} models
					{sortedModels.length !== allModels.length && <span> ({allModels.length} total)</span>}
				</p>
			</div>
		</div>
	);
}
