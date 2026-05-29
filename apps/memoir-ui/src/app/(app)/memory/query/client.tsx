'use client';

import { type QueryHit, type Ranking, runQuery } from '@actions/query';
import type { KindFilter, Memory } from '@actions/timeline';

import { Select } from '@components';
import { Search } from 'lucide-react';
import { useState, useTransition } from 'react';

import EditMemoryModal from '../_components/EditMemoryModal';
import MemoryRow from '../_components/MemoryRow';

export default function QueryClient() {
	const [agentId, setAgentId] = useState('');
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
		<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div className="mb-8">
				<h1 className="text-3xl font-bold text-base-content">Query</h1>
				<p className="mt-2 text-base-content/70">
					Hybrid-ranked retrieval over a scope. Each hit shows its blended cosine-plus-recency score.
				</p>
			</div>

			<form
				id="query-filters"
				className="mb-6 flex flex-wrap items-end gap-4"
				onSubmit={(e) => {
					e.preventDefault();
					run();
				}}>
				<div className="flex-1 min-w-64">
					<label htmlFor="query-text" className="label">
						<span className="label-text">Query</span>
					</label>
					<input
						id="query-text"
						type="text"
						className="input input-bordered w-full"
						placeholder="what do you want to recall?"
						value={query}
						disabled={isPending}
						onChange={(e) => setQuery(e.target.value)}
					/>
				</div>

				<div className="min-w-48">
					<label htmlFor="query-agent-id" className="label">
						<span className="label-text">Agent ID</span>
					</label>
					<input
						id="query-agent-id"
						type="text"
						className="input input-bordered w-full"
						placeholder="agent persona id"
						value={agentId}
						disabled={isPending}
						onChange={(e) => setAgentId(e.target.value)}
					/>
				</div>

				<div>
					<label htmlFor="query-kind" className="label">
						<span className="label-text">Kind</span>
					</label>
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
				</div>

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
			</form>

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
		</div>
	);
}
