'use client';

import type { ActionResult } from '@actions';
import type { Agent, ListAgentsResponse } from '@actions/agents';
import { Button } from '@components';
import { AlertTriangle } from 'lucide-react';
import { useEffect, useMemo, useState } from 'react';
import AgentCard from './AgentCard';

interface AgentGridProps {
	agents: ActionResult<ListAgentsResponse> | undefined;
	isLoading: boolean;
	onRefresh: () => void;
	onEdit?: (agent: Agent) => void;
	onCreate?: () => void;
	onDelete?: (agent: Agent) => void;
	onShare?: (agent: Agent) => void;
}

const AGENTS_PER_PAGE = 12;

export default function AgentGrid({
	agents: agentsResult,
	isLoading,
	onRefresh,
	onEdit,
	onCreate,
	onDelete,
	onShare,
}: AgentGridProps) {
	const [currentPage, setCurrentPage] = useState(1);

	const agents = agentsResult?.success ? agentsResult.data.agents : [];
	const error = agentsResult && !agentsResult.success ? agentsResult.error : null;

	const paginatedAgents = useMemo(() => {
		const startIndex = (currentPage - 1) * AGENTS_PER_PAGE;
		const endIndex = startIndex + AGENTS_PER_PAGE;
		return agents.slice(startIndex, endIndex);
	}, [agents, currentPage]);

	const totalPages = Math.ceil(agents.length / AGENTS_PER_PAGE);

	useEffect(() => {
		if (currentPage > 1) {
			// Only scroll on page change, not initial mount
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}, [currentPage]);

	const handlePageChange = (page: number) => {
		setCurrentPage(page);
	};

	const handleAgentEdit = (agent: Agent) => {
		if (onEdit) {
			onEdit(agent);
		}
	};

	const handleAgentDelete = (agent: Agent) => {
		if (!onDelete) return;
		onDelete(agent);
	};

	// Loading skeleton
	if (isLoading) {
		return (
			<div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{[...Array(8)].map((_, i) => (
					// biome-ignore lint/suspicious/noArrayIndexKey: Static skeleton items with fixed order
					<div key={`skeleton-${i}`} className="card bg-base-100 shadow">
						<div className="card-body">
							<div className="animate-pulse">
								<div className="mb-2 h-4 w-3/4 rounded bg-base-300" />
								<div className="mb-4 h-3 w-1/2 rounded bg-base-300" />
								<div className="mb-4 h-16 rounded bg-base-300" />
								<div className="flex gap-2">
									<div className="h-8 w-16 rounded bg-base-300" />
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
				<div className="card-body py-12 text-center">
					<div className="flex justify-center mb-4">
						<AlertTriangle className="h-12 w-12 text-error" />
					</div>
					<h3 className="mb-2 text-lg font-semibold text-base-content">Failed to Load Agents</h3>
					<p className="mb-6 text-base-content/70">{error}</p>
					<Button color="primary" onClick={onRefresh}>
						Try Again
					</Button>
				</div>
			</div>
		);
	}

	if (agents.length === 0) {
		return (
			<div className="card bg-base-100 shadow-xl">
				<div className="card-body py-12 text-center">
					<h3 className="mb-2 text-lg font-semibold text-base-content">No Agents Yet</h3>
					<p className="mb-6 text-base-content/70">
						{onCreate
							? 'Create your first AI agent to get started with intelligent automation.'
							: 'No agents have been created for this organization yet.'}
					</p>
					{onCreate && (
						<Button color="primary" onClick={onCreate}>
							Create Your First Agent
						</Button>
					)}
				</div>
			</div>
		);
	}

	return (
		<div className="space-y-6">
			<div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{paginatedAgents.map((agent) => (
					<AgentCard
						key={agent.identifier.value}
						agent={agent}
						onEdit={handleAgentEdit}
						onDelete={handleAgentDelete}
						onShare={onShare}
					/>
				))}
			</div>

			{totalPages > 1 && (
				<div className="flex justify-center">
					<div className="join">
						<Button className="join-item" onClick={() => handlePageChange(currentPage - 1)} disabled={currentPage <= 1}>
							« Prev
						</Button>

						{Array.from({ length: totalPages }, (_, i) => i + 1).map((page) => (
							<Button
								key={page}
								className="join-item"
								active={currentPage === page}
								onClick={() => handlePageChange(page)}>
								{page}
							</Button>
						))}

						<Button
							className="join-item"
							onClick={() => handlePageChange(currentPage + 1)}
							disabled={currentPage >= totalPages}>
							Next »
						</Button>
					</div>
				</div>
			)}
		</div>
	);
}
