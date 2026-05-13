'use client';

import type { Tool } from '@actions/tools';
import { Badge } from '@components';
import { cn } from '@lib/utils';
import cns from 'classnames';
import { AlertTriangle, Loader2, Wrench } from 'lucide-react';
import { useCallback } from 'react';
import { Checkbox } from 'rsc-daisyui';

interface ToolSelectorProps {
	tools: Tool[];
	selectedToolPids: string[];
	onChange: (toolPids: string[]) => void;
	disabled?: boolean;
	error?: string | null;
	loading?: boolean;
}

const TOOL_TYPE_BADGE_STYLES: Record<string, string> = {
	system: 'badge-info badge-outline',
	user_defined: 'badge-secondary badge-outline',
};

export default function ToolSelector({
	tools,
	selectedToolPids,
	onChange,
	disabled = false,
	error = null,
	loading = false,
}: ToolSelectorProps) {
	const handleToggle = useCallback(
		(toolPid: string) => {
			if (disabled) return;

			const isSelected = selectedToolPids.includes(toolPid);
			if (isSelected) {
				onChange(selectedToolPids.filter((pid) => pid !== toolPid));
			} else {
				onChange([...selectedToolPids, toolPid]);
			}
		},
		[selectedToolPids, onChange, disabled],
	);

	if (loading) {
		return (
			<div className="space-y-2">
				<div className="label">
					<span className="label-text flex items-center gap-2">
						<Wrench className="h-4 w-4" />
						Tools
					</span>
				</div>
				<div className="flex items-center justify-center py-6 border border-base-300 rounded-lg bg-base-200/30">
					<Loader2 className="w-4 h-4 text-base-content/50 animate-spin" />
					<span className="ml-2 text-sm text-base-content/50">Loading tools...</span>
				</div>
			</div>
		);
	}

	if (error) {
		return (
			<div className="space-y-2">
				<div className="label">
					<span className="label-text flex items-center gap-2">
						<Wrench className="h-4 w-4" />
						Tools
					</span>
				</div>
				<div className="alert alert-error py-3">
					<AlertTriangle className="h-4 w-4" />
					<span className="text-sm">{error}</span>
				</div>
			</div>
		);
	}

	if (tools.length === 0) {
		return (
			<div className="space-y-2">
				<div className="label">
					<span className="label-text flex items-center gap-2">
						<Wrench className="h-4 w-4" />
						Tools
					</span>
				</div>
				<div className="flex flex-col items-center justify-center py-6 border border-base-300 rounded-lg bg-base-200/30 text-center">
					<p className="text-sm text-base-content/60">No tools available</p>
					<p className="text-xs text-base-content/40 mt-1">Tools will appear here once configured.</p>
				</div>
			</div>
		);
	}

	return (
		<div className="space-y-2">
			<div className="label">
				<span className="label-text flex items-center gap-2">
					<Wrench className="h-4 w-4" />
					Tools
				</span>
				{selectedToolPids.length > 0 && (
					<span className="label-text-alt">
						<Badge color="primary" size="sm">
							{selectedToolPids.length} selected
						</Badge>
					</span>
				)}
			</div>

			<div className="border border-base-300 rounded-lg bg-base-100 overflow-hidden divide-y divide-base-200">
				{tools.map((tool) => {
					const isSelected = selectedToolPids.includes(tool.pid);
					const badgeStyle = TOOL_TYPE_BADGE_STYLES[tool.toolType] || TOOL_TYPE_BADGE_STYLES.system;

					return (
						// biome-ignore lint/a11y/noLabelWithoutControl: Checkbox from rsc-daisyui is an input element inside the label
						<label
							key={tool.pid}
							className={cns(
								'flex items-start gap-3 p-3 cursor-pointer transition-colors',
								'hover:bg-base-200/50',
								disabled && 'opacity-50 cursor-not-allowed',
								isSelected && 'bg-primary/5',
							)}>
							<Checkbox
								color="primary"
								size="sm"
								className="mt-0.5"
								checked={isSelected}
								onChange={() => handleToggle(tool.pid)}
								disabled={disabled}
							/>
							<div className="flex-1 min-w-0">
								<div className="flex items-center gap-2 flex-wrap">
									<span className="font-medium text-sm text-base-content">{tool.name}</span>
									<span className={cn('badge badge-xs', badgeStyle)}>{tool.toolType.replace('_', ' ')}</span>
								</div>
								{tool.description && <p className="text-xs text-base-content/60 mt-0.5">{tool.description}</p>}
							</div>
						</label>
					);
				})}
			</div>

			<p className="text-xs text-base-content/50">Select tools to give your agent additional capabilities.</p>
		</div>
	);
}
