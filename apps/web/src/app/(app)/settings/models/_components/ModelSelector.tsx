'use client';

import type { Model } from '@actions/models';
import { Select } from '@components';
import cns from 'classnames';
import { useMemo } from 'react';

interface ModelSelectorProps {
	models: Model[];
	value: string;
	onChange: (modelId: string) => void;
	providerPid?: string;
	disabled?: boolean;
	error?: boolean;
	className?: string;
	placeholder?: string;
	id?: string;
	name?: string;
}

function formatContextLength(length: number | undefined): string {
	if (!length) return '';
	if (length >= 1000000) return `${(length / 1000000).toFixed(1)}M ctx`;
	if (length >= 1000) return `${(length / 1000).toFixed(0)}K ctx`;
	return `${length} ctx`;
}

export default function ModelSelector({
	models,
	value,
	onChange,
	providerPid,
	disabled = false,
	error = false,
	className,
	placeholder = 'Select a model',
	id,
	name,
}: ModelSelectorProps) {
	// Filter models by provider if specified
	const filteredModels = useMemo(() => {
		if (!providerPid) return models;
		return models.filter((model) => model.providerPid === providerPid);
	}, [models, providerPid]);

	// Group models by provider for hierarchical display
	const modelsByProvider = useMemo(() => {
		const groups: Record<string, Model[]> = {};

		for (const model of filteredModels) {
			const providerName = model.providerName || 'Unknown';
			if (!groups[providerName]) {
				groups[providerName] = [];
			}
			groups[providerName].push(model);
		}

		// Sort models within each group by name
		for (const models of Object.values(groups)) {
			models.sort((a, b) => a.name.localeCompare(b.name));
		}

		// Sort provider groups alphabetically
		return Object.entries(groups).sort(([a], [b]) => a.localeCompare(b));
	}, [filteredModels]);

	const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
		onChange(e.target.value);
	};

	// If filtering by provider, show flat list; otherwise show grouped
	const showGroups = !providerPid && modelsByProvider.length > 1;

	return (
		<Select
			id={id}
			name={name}
			className={cns('w-full', error && 'select-error', className)}
			value={value}
			onChange={handleChange}
			disabled={disabled || filteredModels.length === 0}>
			{filteredModels.length === 0 ? (
				<option value="" disabled>
					{providerPid ? 'No models for this provider' : 'No models available'}
				</option>
			) : (
				<>
					<option value="" disabled>
						{placeholder}
					</option>
					{showGroups
						? modelsByProvider.map(([providerName, providerModels]) => (
								<optgroup key={providerName} label={providerName.toUpperCase()}>
									{providerModels.map((model) => (
										<option key={model.modelId} value={model.modelId}>
											{model.name}
											{model.contextLength ? ` (${formatContextLength(model.contextLength)})` : ''}
										</option>
									))}
								</optgroup>
							))
						: filteredModels.map((model) => (
								<option key={model.modelId} value={model.modelId}>
									{model.name}
									{model.contextLength ? ` (${formatContextLength(model.contextLength)})` : ''}
								</option>
							))}
				</>
			)}
		</Select>
	);
}
