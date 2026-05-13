'use client';

import type { ActionResult } from '@actions';
import type { ListProvidersResponse } from '@actions/providers';
import { Button } from '@components';
import { ProviderSource } from '@polypixel/proto-ts/rig-service/rig/v1/provider_pb';
import cns from 'classnames';
import { AlertTriangle, Eye, FileJson, Search, Sparkles, X, Zap } from 'lucide-react';
import { usePathname, useRouter, useSearchParams } from 'next/navigation';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { Badge, Input, Select, Toggle } from 'rsc-daisyui';

interface ModelFiltersProps {
	providers: ActionResult<ListProvidersResponse> | undefined;
}

const DEBOUNCE_MS = 300;

const CAPABILITIES = [
	{ key: 'vision', label: 'Vision', icon: Eye },
	{ key: 'functions', label: 'Functions', icon: Zap },
	{ key: 'json', label: 'JSON', icon: FileJson },
	{ key: 'streaming', label: 'Streaming', icon: Sparkles },
] as const;

type CapabilityKey = (typeof CAPABILITIES)[number]['key'];

export default function ModelFilters({ providers: providersResult }: ModelFiltersProps) {
	const router = useRouter();
	const pathname = usePathname();
	const searchParams = useSearchParams();

	const providers = providersResult?.success ? providersResult.data.providers : [];

	// Local state for search input (debounced)
	const [searchInput, setSearchInput] = useState(searchParams.get('search') ?? '');

	// Sync search input with URL params on mount and param changes
	useEffect(() => {
		setSearchInput(searchParams.get('search') ?? '');
	}, [searchParams]);

	// Update URL params
	const updateParams = useCallback(
		(updates: Record<string, string | null>) => {
			const params = new URLSearchParams(searchParams.toString());

			for (const [key, value] of Object.entries(updates)) {
				if (value === null || value === '') {
					params.delete(key);
				} else {
					params.set(key, value);
				}
			}

			const queryString = params.toString();
			router.push(queryString ? `${pathname}?${queryString}` : pathname);
		},
		[searchParams, pathname, router],
	);

	// Debounced search update
	useEffect(() => {
		const timer = setTimeout(() => {
			const currentSearch = searchParams.get('search') ?? '';
			if (searchInput !== currentSearch) {
				updateParams({ search: searchInput || null });
			}
		}, DEBOUNCE_MS);

		return () => clearTimeout(timer);
	}, [searchInput, searchParams, updateParams]);

	const handleProviderChange = useCallback(
		(e: React.ChangeEvent<HTMLSelectElement>) => {
			updateParams({ provider: e.target.value || null });
		},
		[updateParams],
	);

	const handleCapabilityToggle = useCallback(
		(capability: CapabilityKey) => {
			const current = searchParams.get('capability');
			updateParams({ capability: current === capability ? null : capability });
		},
		[searchParams, updateParams],
	);

	const handleDeprecatedToggle = useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			updateParams({ deprecated: e.target.checked ? 'true' : null });
		},
		[updateParams],
	);

	const handleInactiveToggle = useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			updateParams({ inactive: e.target.checked ? 'true' : null });
		},
		[updateParams],
	);

	const handleClearFilters = useCallback(() => {
		setSearchInput('');
		router.push(pathname);
	}, [pathname, router]);

	// Check if any filters are active
	const hasActiveFilters = useMemo(() => {
		return (
			searchParams.has('provider') ||
			searchParams.has('search') ||
			searchParams.has('capability') ||
			searchParams.has('deprecated') ||
			searchParams.has('inactive')
		);
	}, [searchParams]);

	// Group providers by source for the dropdown
	const groupedProviders = useMemo(() => {
		const system = providers.filter((p) => p.source === ProviderSource.SYSTEM);
		const user = providers.filter((p) => p.source === ProviderSource.USER);
		return { system, user };
	}, [providers]);

	const currentProvider = searchParams.get('provider') ?? '';
	const currentCapability = searchParams.get('capability') as CapabilityKey | null;
	const showDeprecated = searchParams.get('deprecated') === 'true';
	const showInactive = searchParams.get('inactive') === 'true';

	return (
		<div className="space-y-3">
			<div className="flex flex-col gap-3 sm:flex-row sm:items-center">
				<div className="relative flex-1">
					<Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-base-content/40 pointer-events-none z-40" />
					<Input
						type="text"
						placeholder="Search by name or ID..."
						size="sm"
						className="w-full pl-9 pr-8"
						value={searchInput}
						onChange={(e) => setSearchInput(e.target.value)}
					/>
					{searchInput && (
						<button
							type="button"
							className="absolute right-2 top-1/2 -translate-y-1/2 text-base-content/40 hover:text-base-content transition-colors"
							onClick={() => setSearchInput('')}
							title="Clear search">
							<X className="h-4 w-4" />
						</button>
					)}
				</div>

				<Select size="sm" className="w-auto flex-shrink-0" value={currentProvider} onChange={handleProviderChange}>
					<option value="">All Providers</option>
					{groupedProviders.user.length > 0 && (
						<optgroup label="Your Providers">
							{groupedProviders.user.map((provider) => (
								<option key={provider.identifier.value} value={provider.identifier.value}>
									{provider.name}
								</option>
							))}
						</optgroup>
					)}
					{groupedProviders.system.length > 0 && (
						<optgroup label="System Providers">
							{groupedProviders.system.map((provider) => (
								<option key={provider.identifier.value} value={provider.identifier.value}>
									{provider.name}
								</option>
							))}
						</optgroup>
					)}
				</Select>

				{hasActiveFilters && (
					<Button size="sm" ghost className="hover:btn-error hover:text-error-content" onClick={handleClearFilters}>
						<X className="h-3.5 w-3.5" />
						Clear all
					</Button>
				)}
			</div>

			<div className="flex flex-wrap items-center gap-2">
				<button type="button" className="cursor-default focus:outline-none">
					<Badge size="sm" className="badge-ghost">
						Capabilities
					</Badge>
				</button>
				{CAPABILITIES.map(({ key, label, icon: Icon }) => (
					<button key={key} type="button" onClick={() => handleCapabilityToggle(key)} className="focus:outline-none">
						<Badge
							size="sm"
							color={currentCapability === key ? 'primary' : undefined}
							className={cns(
								'gap-1 cursor-pointer transition-colors',
								currentCapability === key ? '' : 'badge-ghost hover:bg-base-300',
							)}>
							<Icon className="w-3 h-3" />
							{label}
						</Badge>
					</button>
				))}

				<div className="w-px h-4 bg-base-300 mx-1" />

				{/** biome-ignore lint/a11y/noLabelWithoutControl: Toggle doesn't count as input */}
				<label className="flex items-center gap-1.5 cursor-pointer">
					<Toggle size="xs" color="warning" checked={showDeprecated} onChange={handleDeprecatedToggle} />
					<span className="text-xs text-base-content/70 flex items-center gap-1">
						<AlertTriangle className="w-3 h-3" />
						Deprecated
					</span>
				</label>

				{/** biome-ignore lint/a11y/noLabelWithoutControl: Toggle doesn't count as input */}
				<label className="flex items-center gap-1.5 cursor-pointer">
					<Toggle size="xs" checked={showInactive} onChange={handleInactiveToggle} />
					<span className="text-xs text-base-content/70">Inactive</span>
				</label>
			</div>
		</div>
	);
}
