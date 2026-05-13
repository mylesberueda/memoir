'use client';

import type { Agent } from '@actions/agents';
import { MessageSquare, Pencil, Share2, Trash2 } from 'lucide-react';
import Link from 'next/link';

interface AgentCardProps {
	agent: Agent;
	onEdit?: (agent: Agent) => void;
	onDelete?: (agent: Agent) => void;
	onShare?: (agent: Agent) => void;
}

export default function AgentCard({ agent, onEdit, onDelete, onShare }: AgentCardProps) {
	const formatDate = (timestamp: string) => {
		return new Date(timestamp).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
		});
	};

	const truncateText = (text: string, maxLength: number) => {
		if (text.length <= maxLength) return text;
		return `${text.slice(0, maxLength).trim()}...`;
	};

	return (
		<div
			id="agent_card__container"
			className="card h-full bg-base-100 shadow transition-shadow duration-200 hover:shadow-lg">
			<div className="card-body flex flex-col justify-between">
				<div id="agent_card__content">
					<div id="agent_card__header" className="mb-4 flex items-start justify-between">
						<div className="flex-1">
							<h3 className="card-title text-base-content">{agent.name}</h3>
							<p className="text-sm text-base-content/70">{agent.model?.modelId}</p>
						</div>
						<div className={`badge ${agent.isActive ? 'badge-success' : 'badge-error'}`}>
							{agent.isActive ? 'Active' : 'Inactive'}
						</div>
					</div>

					<div id="agent_card__prompt" className="mb-4">
						<p className="text-sm text-base-content/80">{truncateText(agent.systemPrompt, 100)}</p>
					</div>

					<div id="agent_card__details" className="space-y-1 text-xs text-base-content/60">
						<div className="flex justify-between">
							<span>Temperature:</span>
							<span>{(agent.temperature / 100).toFixed(2)}</span>
						</div>
						<div className="flex justify-between">
							<span>Created:</span>
							<span>{formatDate(agent.createdAt)}</span>
						</div>
					</div>
				</div>

				<div id="agent_card__actions" className="card-actions justify-end pt-4">
					<Link
						href={`/conversations/${agent.identifier.value}`}
						className="btn btn-ghost btn-sm"
						title="Chat with agent">
						<MessageSquare className="h-4 w-4" />
					</Link>
					{onShare && (
						<button type="button" className="btn btn-ghost btn-sm" onClick={() => onShare(agent)} title="Share agent">
							<Share2 className="h-4 w-4" />
						</button>
					)}
					{onEdit && (
						<button type="button" className="btn btn-ghost btn-sm" onClick={() => onEdit(agent)} title="Edit agent">
							<Pencil className="h-4 w-4" />
						</button>
					)}
					{onDelete && (
						<button
							type="button"
							className="btn btn-ghost btn-sm text-error hover:bg-error hover:text-error-content"
							onClick={() => onDelete(agent)}
							title="Delete agent">
							<Trash2 className="h-4 w-4" />
						</button>
					)}
				</div>
			</div>
		</div>
	);
}
