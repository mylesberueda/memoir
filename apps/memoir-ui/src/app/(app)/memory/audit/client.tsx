'use client';

import { getSupersessionHistory, type SupersessionHistoryResult } from '@actions/supersession-history';
import { GitBranch } from 'lucide-react';
import { useState, useTransition } from 'react';

import SupersessionEventRow from '../_components/SupersessionEventRow';

export default function AuditClient() {
	const [pid, setPid] = useState('');
	const [result, setResult] = useState<SupersessionHistoryResult | null>(null);
	const [resolvedPid, setResolvedPid] = useState<string | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [isPending, startTransition] = useTransition();

	function load() {
		startTransition(async () => {
			setError(null);
			const res = await getSupersessionHistory({ pid });
			if (!res.success) {
				setError(res.error);
				setResult(null);
				setResolvedPid(pid.trim());
				return;
			}
			setResult(res.data);
			setResolvedPid(pid.trim());
		});
	}

	return (
		<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div className="mb-8">
				<h1 className="text-3xl font-bold text-base-content">Supersession audit</h1>
				<p className="mt-2 text-base-content/70">
					The chronological supersede/unsupersede trail behind a memory's current truth status.
				</p>
			</div>

			<form
				id="audit-filters"
				className="mb-6 flex flex-wrap items-end gap-4"
				onSubmit={(e) => {
					e.preventDefault();
					load();
				}}>
				<div className="flex-1 min-w-64">
					<label htmlFor="audit-pid" className="label">
						<span className="label-text">Memory pid</span>
					</label>
					<input
						id="audit-pid"
						type="text"
						className="input input-bordered w-full font-mono"
						placeholder="paste a memory pid"
						value={pid}
						disabled={isPending}
						onChange={(e) => setPid(e.target.value)}
					/>
				</div>

				<button type="submit" className="btn btn-primary" disabled={isPending}>
					{isPending ? (
						<>
							<span className="loading loading-spinner loading-sm" />
							Loading...
						</>
					) : (
						'Load history'
					)}
				</button>
			</form>

			{error && (
				<div className="alert alert-error mb-6">
					<span>{error}</span>
				</div>
			)}

			{result && resolvedPid && (
				<p id="audit-caption" className="mb-4 text-base-content/60 text-sm">
					History for <span className="font-mono">{resolvedPid}</span>
				</p>
			)}

			{result && result.events.length === 0 && (
				<div id="audit-empty" className="flex flex-col items-center justify-center py-16 text-center">
					<GitBranch className="mb-4 h-16 w-16 text-base-content/30" />
					<h2 className="mb-2 font-semibold text-base-content text-xl">No supersession history</h2>
					<p className="max-w-md text-base-content/70">
						This memory was never superseded — or no memory with that pid exists. The events table is empty for it.
					</p>
				</div>
			)}

			<ul id="audit-list" className="space-y-3">
				{result?.events.map((event, idx) => (
					<SupersessionEventRow
						key={event.decidedAt ? `${event.decidedAt.seconds}-${event.decidedAt.nanos}-${idx}` : idx}
						event={event}
					/>
				))}
			</ul>
		</div>
	);
}
