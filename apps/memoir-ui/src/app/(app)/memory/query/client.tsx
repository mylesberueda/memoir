'use client';

import { runQuery } from '@actions/query';
import type { KindFilter } from '@actions/timeline';
import { AgentIdInput, Field, FilterBar, PageContainer, PageHeader, Select } from '@components';
import useAgentIds from '@hooks/useAgentIds';
import type { Memory, QueryHit, Ranking } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
import { Search } from 'lucide-react';
import { useState, useTransition } from 'react';

import EditMemoryModal from '../_components/EditMemoryModal';
import MemoryRow from '../_components/MemoryRow';

export default function QueryClient() {
	const [agentId, setAgentId] = useState('');
	const agents = useAgentIds();
	const [query, setQuery] = useState('');
	const [kind, setKind] = useState<KindFilter>('both');
	const [hits, setHits] = useState<QueryHit[]>([]);
	const [rankingUsed, setRankingUsed] = useState<Ranking | undefined>(undefined);
	const [editing, setEditing] = useState<Memory | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [loaded, setLoaded] = useState(false);
	const [isPending, startTransition] = useTransition();

	function run() {
		startTransition(async () => {
			setError(null);
			const res = await runQuery({ agentId, query, kind });
			if (!res.success) {
				setError(res.error);
				setHits([]);
				setRankingUsed(undefined);
				setLoaded(true);
				return;
			}
			setHits(res.data.hits);
			setRankingUsed(res.data.rankingUsed);
			setLoaded(true);
		});
	}

	const hybrid = rankingUsed?.strategy.case === 'hybrid' ? rankingUsed.strategy.value : undefined;

	return (
		<PageContainer width="list">
			<PageHeader
				eyebrow="Memory"
				title="Query"
				description="Hybrid-ranked retrieval over a scope. Each hit shows its blended cosine-plus-recency score."
			/>

			<FilterBar
				id="query-filters"
				onSubmit={(e) => {
					e.preventDefault();
					run();
				}}>
				<Field label="Query" htmlFor="query-text" grow className="min-w-64">
					<input
						id="query-text"
						type="text"
						className="input input-bordered w-full"
						placeholder="what do you want to recall?"
						value={query}
						disabled={isPending}
						onChange={(e) => setQuery(e.target.value)}
					/>
				</Field>

				<Field label="Agent ID" htmlFor="query-agent-id">
					<AgentIdInput
						id="query-agent-id"
						value={agentId}
						onChange={setAgentId}
						agents={agents}
						disabled={isPending}
					/>
				</Field>

				<Field label="Kind" htmlFor="query-kind">
					<Select
						id="query-kind"
						className="w-40"
						value={kind}
						disabled={isPending}
						onChange={(e) => setKind(e.target.value as KindFilter)}>
						<option value="both">All kinds</option>
						<option value="episodic">Episodic</option>
						<option value="semantic">Semantic</option>
					</Select>
				</Field>

				<button type="submit" className="btn btn-primary" disabled={isPending}>
					{isPending ? (
						<>
							<span className="loading loading-spinner loading-sm" />
							Querying...
						</>
					) : (
						'Run query'
					)}
				</button>
			</FilterBar>

			{error && (
				<div className="alert alert-error mb-6">
					<span>{error}</span>
				</div>
			)}

			{loaded && !error && hybrid && (
				<p id="query-ranking-used" className="mb-4 text-base-content/60 text-sm">
					Ranking:{' '}
					<span className="font-mono">
						hybrid · alpha {hybrid.alpha.toFixed(2)} · {hybrid.decay?.function.case ?? 'unset'} decay
					</span>
				</p>
			)}

			{loaded && !error && hits.length === 0 && (
				<div id="query-empty" className="flex flex-col items-center justify-center py-16 text-center">
					<Search className="mb-4 h-16 w-16 text-base-content/30" />
					<h2 className="mb-2 font-semibold text-base-content text-xl">No results</h2>
					<p className="max-w-md text-base-content/70">No memories in this scope matched the query.</p>
				</div>
			)}

			<ul id="query-list" className="space-y-3">
				{hits.map(
					(hit) =>
						hit.memory && <MemoryRow key={hit.memory.pid} memory={hit.memory} score={hit.score} onEdit={setEditing} />,
				)}
			</ul>

			{editing && (
				<EditMemoryModal
					memory={editing}
					open={true}
					onClose={() => setEditing(null)}
					onMemoryUpdated={(updated) =>
						setHits((prev) => prev.map((h) => (h.memory?.pid === updated.pid ? { ...h, memory: updated } : h)))
					}
				/>
			)}
		</PageContainer>
	);
}
