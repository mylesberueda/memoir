'use client';

import type { Provider } from '@actions/providers';
import { ProviderSource } from '@startup/proto-ts/rig-service/rig/v1/provider_pb';
import cns from 'classnames';
import { Pencil, Server, Trash2 } from 'lucide-react';

interface ProviderCardProps {
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

export default function ProviderCard({ provider, onEdit, onDelete }: ProviderCardProps) {
	const isSystemProvider = provider.source === ProviderSource.SYSTEM;

	const formatDate = (timestamp: string) => {
		return new Date(timestamp).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
		});
	};

	const getProviderTypeLabel = (type: string) => {
		return PROVIDER_TYPE_LABELS[type.toLowerCase()] || type;
	};

	return (
		<div className="card bg-base-100 shadow transition-shadow duration-200 hover:shadow-lg">
			<div className="card-body gap-4">
				{/* Header */}
				<div>
					<h3 className="card-title text-base-content">{provider.name}</h3>
					<p className="text-sm text-base-content/70 italic">{getProviderTypeLabel(provider.providerType)}</p>
				</div>

				<div className={cns('badge', provider.isActive ? 'badge-success' : 'badge-error')}>
					{provider.isActive ? 'Active' : 'Inactive'}
				</div>

				{/* Endpoint URL (for self-hosted providers) */}
				{provider.endpointUrl && (
					<div className="flex items-center gap-2 text-sm text-base-content/70">
						<Server className="h-4 w-4" />
						<span className="truncate">{provider.endpointUrl}</span>
					</div>
				)}

				{/* Configuration Details */}
				<div className="space-y-1 text-xs text-base-content/60">
					<div className="flex justify-between">
						<span>Source:</span>
						<span className={cns(isSystemProvider && 'text-info')}>{isSystemProvider ? 'System' : 'Personal'}</span>
					</div>
					<div className="flex justify-between">
						<span>Created:</span>
						<span>{formatDate(provider.createdAt)}</span>
					</div>
				</div>

				{/* Actions */}
				{!isSystemProvider && (
					<div className="card-actions justify-end">
						<button
							type="button"
							className="btn btn-ghost btn-sm"
							onClick={() => onEdit(provider)}
							title="Edit provider">
							<Pencil className="h-4 w-4" />
						</button>
						<button
							type="button"
							className="btn btn-ghost btn-sm text-error hover:bg-error hover:text-error-content"
							onClick={() => onDelete(provider)}
							title="Delete provider">
							<Trash2 className="h-4 w-4" />
						</button>
					</div>
				)}
			</div>
		</div>
	);
}
