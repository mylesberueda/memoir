'use client';

import { getTimeline, type KindFilter, type Memory } from '@actions/timeline';
import { Select } from '@components';
import { Clock } from 'lucide-react';
import { useState, useTransition } from 'react';

import EditMemoryModal from '../_components/EditMemoryModal';
import MemoryRow from '../_components/MemoryRow';

export default function TimelineClient() {
	const [agentId, setAgentId] = useState('');
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
		<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div className="mb-8">
				<h1 className="text-3xl font-bold text-base-content">Timeline</h1>
				<p className="mt-2 text-base-content/70">
					The chronological memory event-log for a scope. Superseded rows are kept as an audit trail.
				</p>
			</div>

			<form
				id="timeline-filters"
				className="mb-6 flex flex-wrap items-end gap-4"
				onSubmit={(e) => {
					e.preventDefault();
					load();
				}}>
				<div className="flex-1 min-w-48">
					<label htmlFor="timeline-agent-id" className="label">
						<span className="label-text">Agent ID</span>
					</label>
					<input
						id="timeline-agent-id"
						type="text"
						className="input input-bordered w-full"
						placeholder="agent persona id"
						value={agentId}
						disabled={isPending}
						onChange={(e) => setAgentId(e.target.value)}
					/>
				</div>

				<div>
					<label htmlFor="timeline-kind" className="label">
						<span className="label-text">Kind</span>
					</label>
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
				</div>

				<label htmlFor="timeline-exclude-superseded" className="label cursor-pointer gap-2">
					<input
						id="timeline-exclude-superseded"
						type="checkbox"
						className="checkbox checkbox-sm"
						checked={excludeSuperseded}
						disabled={isPending}
						onChange={(e) => setExcludeSuperseded(e.target.checked)}
					/>
					<span className="label-text">Hide superseded</span>
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
			</form>

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
		</div>
	);
}
