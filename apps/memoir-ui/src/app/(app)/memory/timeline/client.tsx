'use client';

import { getTimeline, type KindFilter } from '@actions/timeline';
import { AgentIdInput, Field, FilterBar, PageContainer, PageHeader, Select } from '@components';
import useAgentIds from '@hooks/useAgentIds';
import type { Memory } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
import { Clock } from 'lucide-react';
import { useState, useTransition } from 'react';

import EditMemoryModal from '../_components/EditMemoryModal';
import MemoryRow from '../_components/MemoryRow';

export default function TimelineClient() {
	const [agentId, setAgentId] = useState('');
	const agents = useAgentIds();
	const [kind, setKind] = useState<KindFilter>('both');
	const [excludeSuperseded, setExcludeSuperseded] = useState(false);
	const [memories, setMemories] = useState<Memory[]>([]);
	const [editing, setEditing] = useState<Memory | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [loaded, setLoaded] = useState(false);
	const [isPending, startTransition] = useTransition();

	function load() {
		startTransition(async () => {
			setError(null);
			const res = await getTimeline({ agentId, kind, excludeSuperseded });
			if (!res.success) {
				setError(res.error);
				setMemories([]);
				setLoaded(true);
				return;
			}
			setMemories(res.data.memories);
			setLoaded(true);
		});
	}

	return (
		<PageContainer width="list">
			<PageHeader
				eyebrow="Memory"
				title="Timeline"
				description="The chronological memory event-log for a scope. Superseded rows are kept as an audit trail."
			/>

			<FilterBar
				id="timeline-filters"
				onSubmit={(e) => {
					e.preventDefault();
					load();
				}}>
				<Field label="Agent ID" htmlFor="timeline-agent-id" grow>
					<AgentIdInput
						id="timeline-agent-id"
						value={agentId}
						onChange={setAgentId}
						agents={agents}
						disabled={isPending}
					/>
				</Field>

				<Field label="Kind" htmlFor="timeline-kind">
					<Select
						id="timeline-kind"
						className="w-40"
						value={kind}
						disabled={isPending}
						onChange={(e) => setKind(e.target.value as KindFilter)}>
						<option value="both">All kinds</option>
						<option value="episodic">Episodic</option>
						<option value="semantic">Semantic</option>
					</Select>
				</Field>

				<label htmlFor="timeline-exclude-superseded" className="flex h-12 cursor-pointer items-center gap-2 text-sm">
					<input
						id="timeline-exclude-superseded"
						type="checkbox"
						className="checkbox checkbox-sm"
						checked={excludeSuperseded}
						disabled={isPending}
						onChange={(e) => setExcludeSuperseded(e.target.checked)}
					/>
					<span>Hide superseded</span>
				</label>

				<button type="submit" className="btn btn-primary" disabled={isPending}>
					{isPending ? (
						<>
							<span className="loading loading-spinner loading-sm" />
							Loading...
						</>
					) : (
						'Load timeline'
					)}
				</button>
			</FilterBar>

			{error && (
				<div className="alert alert-error mb-6">
					<span>{error}</span>
				</div>
			)}

			{loaded && !error && memories.length === 0 && (
				<div id="timeline-empty" className="flex flex-col items-center justify-center py-16 text-center">
					<Clock className="mb-4 h-16 w-16 text-base-content/30" />
					<h2 className="mb-2 font-semibold text-base-content text-xl">No memories</h2>
					<p className="max-w-md text-base-content/70">No memories in this scope match the current filters.</p>
				</div>
			)}

			<ul id="timeline-list" className="space-y-3">
				{memories.map((memory) => (
					<MemoryRow key={memory.pid} memory={memory} onEdit={setEditing} />
				))}
			</ul>

			{editing && (
				<EditMemoryModal
					memory={editing}
					open={true}
					onClose={() => setEditing(null)}
					onMemoryUpdated={(updated) => setMemories((prev) => prev.map((m) => (m.pid === updated.pid ? updated : m)))}
				/>
			)}
		</PageContainer>
	);
}
