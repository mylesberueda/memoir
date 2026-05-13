'use client';

import type { Provider } from '@actions/providers';
import { Button } from '@components';
import { ProviderSource } from '@polypixel/proto-ts/rig-service/rig/v1/provider_pb';
import cns from 'classnames';
import { Box, ExternalLink, Pencil, Server, Trash2 } from 'lucide-react';
import Link from 'next/link';
import { Badge } from 'rsc-daisyui';

interface ProviderRowProps {
	provider: Provider;
	onEdit: (provider: Provider) => void;
	onDelete: (provider: Provider) => void;
}

const PROVIDER_TYPE_LABELS: Record<string, string> = {
	openai: 'OpenAI',
	anthropic: 'Anthropic',
	ollama: 'Ollama',
	lmstudio: 'LM Studio',
};

function formatDate(timestamp: string): string {
	return new Date(timestamp).toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
	});
}

function getProviderTypeLabel(type: string): string {
	return PROVIDER_TYPE_LABELS[type.toLowerCase()] || type;
}

export default function ProviderRow({ provider, onEdit, onDelete }: ProviderRowProps) {
	const isSystemProvider = provider.source === ProviderSource.SYSTEM;

	return (
		<div
			className={cns(
				'card bg-base-100 shadow-sm',
				'border border-base-200 hover:border-base-300 transition-colors duration-150',
			)}>
			<div className="card-body p-4 gap-3">
				{/* Header: Name, Type, Status */}
				<div className="flex items-start justify-between gap-3">
					<div className="min-w-0 flex-1">
						<div className="flex items-center gap-2 flex-wrap">
							<h3 className="font-semibold text-base-content truncate">{provider.name}</h3>
							<Badge size="sm" className="badge-ghost">
								{getProviderTypeLabel(provider.providerType)}
							</Badge>
							{isSystemProvider && (
								<Badge size="sm" color="info">
									System
								</Badge>
							)}
						</div>
					</div>

					{/* Status dot */}
					<div className="flex items-center gap-2 flex-shrink-0">
						<span className={cns('text-xs', provider.isActive ? 'text-success' : 'text-base-content/40')}>
							{provider.isActive ? 'Active' : 'Inactive'}
						</span>
						<div className={cns('w-2 h-2 rounded-full', provider.isActive ? 'bg-success' : 'bg-base-content/30')} />
					</div>
				</div>

				{/* Details row */}
				<div className="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-base-content/60">
					{provider.endpointUrl && (
						<div className="flex items-center gap-1.5">
							<Server className="w-3.5 h-3.5" />
							<span className="font-mono text-xs truncate max-w-[200px]">{provider.endpointUrl}</span>
						</div>
					)}
					<div className="flex items-center gap-1.5">
						<span>Created {formatDate(provider.createdAt)}</span>
					</div>
				</div>

				{/* Actions row */}
				<div className="flex items-center justify-between pt-1 border-t border-base-200">
					{/* View models link */}
					<Link
						href={`/settings/models?provider=${provider.identifier.value}`}
						className="flex items-center gap-1.5 text-sm text-primary hover:underline">
						<Box className="w-3.5 h-3.5" />
						View models
						<ExternalLink className="w-3 h-3" />
					</Link>

					{/* Edit/Delete actions */}
					{!isSystemProvider ? (
						<div className="flex items-center gap-1">
							<Button size="sm" ghost onClick={() => onEdit(provider)} title="Edit provider">
								<Pencil className="h-4 w-4" />
								Edit
							</Button>
							<Button
								size="sm"
								ghost
								onClick={() => onDelete(provider)}
								title="Delete provider"
								className="text-error hover:bg-error hover:text-error-content">
								<Trash2 className="h-4 w-4" />
							</Button>
						</div>
					) : (
						<span className="text-xs text-base-content/40">Managed by system</span>
					)}
				</div>
			</div>
		</div>
	);
}
