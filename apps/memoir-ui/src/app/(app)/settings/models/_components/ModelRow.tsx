'use client';

import type { Model } from '@actions/models';
import type { Provider } from '@actions/providers';
import { ProviderSource } from '@polypixel/memoir-sdk/rig-service/rig/v1/provider_pb';
import cns from 'classnames';
import { ChevronDown, Eye, FileJson, MessageSquare, Sparkles, Zap } from 'lucide-react';
import { useCallback, useState } from 'react';

interface ModelRowProps {
	model: Model;
	provider: Provider | undefined;
}

function formatContextLength(length: number | undefined): string {
	if (!length) return '—';
	if (length >= 1000000) return `${(length / 1000000).toFixed(1)}M`;
	if (length >= 1000) return `${Math.round(length / 1000)}K`;
	return length.toString();
}

function formatCost(microdollars: bigint | undefined): string {
	if (!microdollars) return '—';
	const dollars = Number(microdollars) / 1000000;
	if (dollars < 0.001) return `$${dollars.toFixed(6)}`;
	if (dollars < 0.01) return `$${dollars.toFixed(4)}`;
	return `$${dollars.toFixed(2)}`;
}

interface CapabilityIconProps {
	enabled: boolean;
	icon: React.ElementType;
	label: string;
}

function CapabilityIcon({ enabled, icon: Icon, label }: CapabilityIconProps) {
	return (
		<div
			className={cns('tooltip tooltip-top', enabled ? 'text-success' : 'text-base-content/20')}
			data-tip={`${label}: ${enabled ? 'Yes' : 'No'}`}>
			<Icon className="w-4 h-4" />
		</div>
	);
}

export default function ModelRow({ model, provider }: ModelRowProps) {
	const [isExpanded, setIsExpanded] = useState(false);
	const isSystemProvider = provider?.source === ProviderSource.SYSTEM;

	const toggleExpanded = useCallback(() => {
		setIsExpanded((prev) => !prev);
	}, []);

	return (
		<div
			className={cns(
				'group border-b border-base-200 last:border-b-0',
				'bg-base-100 hover:bg-base-200/50 transition-colors duration-150',
			)}>
			{/* Main row - always visible */}
			<button
				type="button"
				onClick={toggleExpanded}
				className={cns(
					'w-full px-4 py-3 text-left',
					'grid items-center gap-4',
					'grid-cols-[auto_1fr_auto]',
					'sm:grid-cols-[auto_1fr_8rem_auto]',
					'md:grid-cols-[auto_1fr_8rem_5rem_auto]',
					'lg:grid-cols-[auto_1fr_8rem_5rem_6rem_auto]',
				)}
				aria-expanded={isExpanded}>
				{/* Status dot */}
				<div
					className={cns('w-2 h-2 rounded-full', model.isActive ? 'bg-success' : 'bg-base-content/30')}
					title={model.isActive ? 'Active' : 'Inactive'}
				/>

				{/* Model name and ID */}
				<div className="min-w-0">
					<div className="flex items-center gap-2">
						<span className="font-medium text-base-content truncate">{model.name}</span>
						{isSystemProvider && <span className="badge badge-info badge-xs flex-shrink-0">System</span>}
					</div>
					<code className="text-xs text-base-content/50 font-mono truncate block">{model.modelId}</code>
				</div>

				{/* Provider */}
				<span className="hidden sm:block text-sm text-base-content/70 truncate">{model.providerName}</span>

				{/* Context length */}
				<span className="hidden md:block text-sm font-mono text-base-content/70 text-right">
					{formatContextLength(model.contextLength)}
				</span>

				{/* Capability icons - compact inline display */}
				<div className="hidden lg:flex items-center gap-2">
					<CapabilityIcon enabled={model.capabilities?.vision ?? false} icon={Eye} label="Vision" />
					<CapabilityIcon enabled={model.capabilities?.functionCalling ?? false} icon={Zap} label="Functions" />
					<CapabilityIcon enabled={model.capabilities?.jsonMode ?? false} icon={FileJson} label="JSON Mode" />
					<CapabilityIcon enabled={model.capabilities?.streaming ?? false} icon={Sparkles} label="Streaming" />
				</div>

				{/* Expand chevron */}
				<ChevronDown
					className={cns('w-4 h-4 text-base-content/40 transition-transform duration-200', isExpanded && 'rotate-180')}
				/>
			</button>

			{/* Expanded details */}
			<div
				className={cns(
					'grid transition-all duration-200 ease-out',
					isExpanded ? 'grid-rows-[1fr]' : 'grid-rows-[0fr]',
				)}>
				<div className="overflow-hidden">
					<div className="px-4 pb-4 pt-1">
						<div className="pl-6 border-l-2 border-base-300 ml-0.5">
							{/* Mobile-only: show provider and context */}
							<div className="sm:hidden mb-3 flex gap-4 text-sm">
								<span className="text-base-content/70">{model.providerName}</span>
								<span className="font-mono text-base-content/70">
									{formatContextLength(model.contextLength)} tokens
								</span>
							</div>

							{/* Capabilities - full display on expand */}
							<div className="flex flex-wrap gap-2 mb-3">
								<div
									className={cns(
										'badge badge-sm gap-1',
										model.capabilities?.vision ? 'badge-success badge-outline' : 'badge-ghost',
									)}>
									<Eye className="w-3 h-3" /> Vision
								</div>
								<div
									className={cns(
										'badge badge-sm gap-1',
										model.capabilities?.functionCalling ? 'badge-success badge-outline' : 'badge-ghost',
									)}>
									<Zap className="w-3 h-3" /> Functions
								</div>
								<div
									className={cns(
										'badge badge-sm gap-1',
										model.capabilities?.jsonMode ? 'badge-success badge-outline' : 'badge-ghost',
									)}>
									<FileJson className="w-3 h-3" /> JSON
								</div>
								<div
									className={cns(
										'badge badge-sm gap-1',
										model.capabilities?.streaming ? 'badge-success badge-outline' : 'badge-ghost',
									)}>
									<Sparkles className="w-3 h-3" /> Streaming
								</div>
								<div
									className={cns(
										'badge badge-sm gap-1',
										model.capabilities?.systemPrompt ? 'badge-success badge-outline' : 'badge-ghost',
									)}>
									<MessageSquare className="w-3 h-3" /> System Prompt
								</div>
								<div
									className={cns(
										'badge badge-sm gap-1',
										model.capabilities?.multiTurn ? 'badge-success badge-outline' : 'badge-ghost',
									)}>
									<MessageSquare className="w-3 h-3" /> Multi-turn
								</div>
							</div>

							{/* Metadata - costs and details */}
							{(model.metadata?.maxOutputTokens ||
								model.metadata?.inputCostPerMtok ||
								model.metadata?.outputCostPerMtok ||
								model.metadata?.family ||
								model.metadata?.quantization) && (
								<div className="flex flex-wrap gap-x-6 gap-y-1 text-sm">
									{model.metadata?.maxOutputTokens && (
										<div className="flex gap-2">
											<span className="text-base-content/50">Max output:</span>
											<span className="font-mono">{formatContextLength(Number(model.metadata.maxOutputTokens))}</span>
										</div>
									)}
									{model.metadata?.inputCostPerMtok && (
										<div className="flex gap-2">
											<span className="text-base-content/50">Input:</span>
											<span className="font-mono">{formatCost(model.metadata.inputCostPerMtok)}/M</span>
										</div>
									)}
									{model.metadata?.outputCostPerMtok && (
										<div className="flex gap-2">
											<span className="text-base-content/50">Output:</span>
											<span className="font-mono">{formatCost(model.metadata.outputCostPerMtok)}/M</span>
										</div>
									)}
									{model.metadata?.family && (
										<div className="flex gap-2">
											<span className="text-base-content/50">Family:</span>
											<span>{model.metadata.family}</span>
										</div>
									)}
									{model.metadata?.quantization && (
										<div className="flex gap-2">
											<span className="text-base-content/50">Quantization:</span>
											<span className="font-mono">{model.metadata.quantization}</span>
										</div>
									)}
								</div>
							)}

							{/* Deprecation warning */}
							{model.deprecationMessage && (
								<div className="alert alert-warning mt-3 py-2">
									<span className="text-sm">
										<strong>Deprecated:</strong> {model.deprecationMessage}
									</span>
								</div>
							)}
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}
