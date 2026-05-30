'use client';

import { type RecallAsOfResult, recallAsOf } from '@actions/recall-as-of';
import type { KindFilter } from '@actions/timeline';

import { AgentIdInput, Field, FilterBar, PageContainer, PageHeader, Select } from '@components';
import useAgentIds from '@hooks/useAgentIds';
import { History } from 'lucide-react';
import { useState, useTransition } from 'react';

import MemoryRow from '../_components/MemoryRow';

export default function AsOfClient() {
	const [agentId, setAgentId] = useState('');
	const agents = useAgentIds();
	const [asOf, setAsOf] = useState('');
	const [kind, setKind] = useState<KindFilter>('both');
	const [result, setResult] = useState<RecallAsOfResult | null>(null);
	const [resolvedAsOf, setResolvedAsOf] = useState<Date | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [isPending, startTransition] = useTransition();

	function reconstruct() {
		const instant = new Date(asOf);
		if (Number.isNaN(instant.getTime())) {
			setError('Pick a valid date and time');
			return;
		}
		startTransition(async () => {
			setError(null);
			const res = await recallAsOf({ agentId, asOf: instant, kind });
			if (!res.success) {
				setError(res.error);
				setResult(null);
				setResolvedAsOf(instant);
				return;
			}
			setResult(res.data);
			setResolvedAsOf(instant);
		});
	}

	return (
		<PageContainer width="list">
			<PageHeader
				eyebrow="Memory"
				title="Point-in-time"
				description="Reconstruct memoir's active knowledge as of an instant. A future time shows current state; a time before anything existed shows nothing."
			/>

			<FilterBar
				id="as-of-filters"
				onSubmit={(e) => {
					e.preventDefault();
					reconstruct();
				}}>
				<Field label="As of" htmlFor="as-of-instant">
					<input
						id="as-of-instant"
						type="datetime-local"
						className="input input-bordered w-full"
						value={asOf}
						disabled={isPending}
						onChange={(e) => setAsOf(e.target.value)}
					/>
				</Field>

				<Field label="Agent ID" htmlFor="as-of-agent-id" grow>
					<AgentIdInput
						id="as-of-agent-id"
						value={agentId}
						onChange={setAgentId}
						agents={agents}
						disabled={isPending}
					/>
				</Field>

				<Field label="Kind" htmlFor="as-of-kind">
					<Select
						id="as-of-kind"
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
							Reconstructing...
						</>
					) : (
						'Reconstruct'
					)}
				</button>
			</FilterBar>

			{error && (
				<div className="alert alert-error mb-6">
					<span>{error}</span>
				</div>
			)}

			{result && resolvedAsOf && (
				<p id="as-of-caption" className="mb-4 text-base-content/60 text-sm">
					Active as of <span className="font-mono">{resolvedAsOf.toLocaleString()}</span>
				</p>
			)}

			{result && result.memories.length === 0 && (
				<div id="as-of-empty" className="flex flex-col items-center justify-center py-16 text-center">
					<History className="mb-4 h-16 w-16 text-base-content/30" />
					<h2 className="mb-2 font-semibold text-base-content text-xl">Nothing active</h2>
					<p className="max-w-md text-base-content/70">No memories were active in this scope as of that instant.</p>
				</div>
			)}

			<ul id="as-of-list" className="space-y-3">
				{result?.memories.map((memory) => (
					<MemoryRow key={memory.pid} memory={memory} />
				))}
			</ul>
		</PageContainer>
	);
}
