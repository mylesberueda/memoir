'use client';

import type { Model } from '@actions/models';
import { cn } from '@lib/utils';
import { ChevronUp, Search, X } from 'lucide-react';
import { useCallback, useMemo, useRef, useState } from 'react';
import { Dropdown, Input, Menu } from 'rsc-daisyui';

import PromptInputButton from './PromptInputButton';

export interface PromptInputModelSelectProps {
	value: string;
	onChange: (value: string) => void;
	models?: Model[];
	isLoading?: boolean;
	error?: Error | null;
	disabled?: boolean;
}

export default function PromptInputModelSelect({
	value,
	onChange,
	models = [],
	isLoading = false,
	error = null,
	disabled,
}: PromptInputModelSelectProps) {
	const [searchQuery, setSearchQuery] = useState('');
	const searchInputRef = useRef<HTMLInputElement>(null);

	const selectedModel = models.find((model) => model.identifier.value === value);

	let displayText = 'Select Model';
	let isDisabled = disabled || isLoading;

	if (isLoading) {
		displayText = 'Loading models...';
	} else if (error) {
		displayText = 'Error loading models';
		isDisabled = true;
	} else if (selectedModel) {
		displayText = selectedModel.name;
	} else if (models.length === 0) {
		displayText = 'No models available';
		isDisabled = true;
	}

	// Filter models based on search query
	const filteredModels = useMemo(() => {
		if (!searchQuery.trim()) return models;

		const query = searchQuery.toLowerCase();
		return models.filter(
			(model) => model.name.toLowerCase().includes(query) || model.providerName.toLowerCase().includes(query),
		);
	}, [models, searchQuery]);

	// Group models by provider for better organization
	const groupedModels = useMemo(() => {
		const groups = new Map<string, Model[]>();
		for (const model of filteredModels) {
			const existing = groups.get(model.providerName) || [];
			existing.push(model);
			groups.set(model.providerName, existing);
		}
		return groups;
	}, [filteredModels]);

	const handleModelSelect = useCallback(
		(modelPid: string) => {
			onChange(modelPid);
			setSearchQuery('');
			// Close the popover
			const popover = document.getElementById('model-select-popover');
			if (popover) {
				popover.hidePopover();
			}
		},
		[onChange],
	);

	// Handle popover toggle event to focus search and scroll selected item into view
	const handlePopoverToggle = useCallback(
		(event: React.ToggleEvent) => {
			const popover = event.currentTarget as HTMLElement;
			if (popover.matches(':popover-open')) {
				// Focus search input when popover opens
				setTimeout(() => {
					searchInputRef.current?.focus();
				}, 0);

				// Scroll selected item into view
				if (value) {
					const selectedElement = popover.querySelector(`[data-selected="true"]`);
					if (selectedElement) {
						selectedElement.scrollIntoView({ block: 'center', behavior: 'instant' });
					}
				}
			} else {
				// Clear search when popover closes
				setSearchQuery('');
			}
		},
		[value],
	);

	return (
		<>
			{models.length > 0 && (
				<Dropdown.Popover
					className="max-h-[24rem] w-72 overflow-y-auto mr-2 mb-1 p-0"
					id="model-select-popover"
					onToggle={handlePopoverToggle}
					style={
						{
							positionAnchor: '--model-select-anchor',
							positionArea: 'block-start span-inline-end',
						} as React.CSSProperties
					}>
					<div id="model-select__search" className="sticky top-0 z-10 p-2 bg-base-200 border-b border-base-300">
						<div className="relative">
							<Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-base-content/40 pointer-events-none" />
							<Input
								ref={searchInputRef}
								type="text"
								placeholder="Search models..."
								size="sm"
								className="pl-9 pr-8"
								value={searchQuery}
								onChange={(e) => setSearchQuery(e.target.value)}
							/>
							{searchQuery && (
								<button
									type="button"
									className="absolute right-2 top-1/2 -translate-y-1/2 text-base-content/40 hover:text-base-content transition-colors"
									onClick={() => setSearchQuery('')}
									title="Clear search">
									<X className="h-4 w-4" />
								</button>
							)}
						</div>
					</div>
					<div id="model-select__list">
						{filteredModels.length === 0 ? (
							<div className="px-4 py-8 text-center text-sm text-base-content/60">
								No models found for "{searchQuery}"
							</div>
						) : (
							Array.from(groupedModels.entries()).map(([providerName, providerModels]) => (
								<div key={providerName}>
									<div className="px-3 py-2 text-xs font-semibold text-base-content/50 uppercase tracking-wider bg-base-200/50 sticky top-0">
										{providerName}
									</div>
									{providerModels.map((model) => (
										<Menu.Item
											key={model.identifier.value}
											data-selected={value === model.identifier.value}
											className={cn(
												'hover:bg-base-300 px-3 py-2',
												value === model.identifier.value && '!bg-primary !text-primary-content',
											)}
											onClick={() => handleModelSelect(model.identifier.value)}>
											<span className="font-medium">{model.name}</span>
										</Menu.Item>
									))}
								</div>
							))
						)}
					</div>
				</Dropdown.Popover>
			)}
			<PromptInputButton
				type="button"
				disabled={isDisabled}
				className="min-w-0 max-w-[160px] flex-shrink-0 flex items-center justify-between"
				popoverTarget="model-select-popover"
				style={{ anchorName: '--model-select-anchor' } as React.CSSProperties}>
				<span className="truncate flex-1 min-w-0">{displayText}</span>
				<ChevronUp className="size-4 ml-2 flex-shrink-0" />
			</PromptInputButton>
		</>
	);
}
