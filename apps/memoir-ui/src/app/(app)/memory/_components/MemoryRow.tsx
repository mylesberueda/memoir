'use client';

import type { Memory } from '@actions/timeline';
import { timestampDate } from '@bufbuild/protobuf/wkt';
import { MemoryStatus } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
import { format } from 'date-fns';
import { Pencil } from 'lucide-react';

interface MemoryRowProps {
	memory: Memory;
	score?: number;
	onEdit?: (memory: Memory) => void;
}

export default function MemoryRow({ memory, score, onEdit }: MemoryRowProps) {
	const superseded = memory.supersession !== undefined;
	const pending = memory.status === MemoryStatus.PENDING;
	return (
		<li className={`card border bg-base-100 ${superseded ? 'border-warning/40 opacity-70' : ''}`}>
			<div className="card-body gap-2 p-4">
				<div className="flex items-start justify-between gap-4">
					<p className="text-base-content">{memory.content}</p>
					<div className="flex shrink-0 items-center gap-2">
						{pending && <span className="badge badge-info badge-sm">pending</span>}
						{score !== undefined && <span className="badge badge-ghost badge-sm font-mono">{score.toFixed(3)}</span>}
						{superseded && <span className="badge badge-warning badge-sm">superseded</span>}
						{onEdit && !superseded && (
							<button
								type="button"
								className="btn btn-ghost btn-xs"
								aria-label="Edit memory"
								onClick={() => onEdit(memory)}>
								<Pencil className="h-3 w-3" />
							</button>
						)}
					</div>
				</div>
				<div className="flex flex-wrap gap-x-4 gap-y-1 text-base-content/60 text-xs">
					{memory.createdAt && <span>created {format(timestampDate(memory.createdAt), 'MMM d, yyyy h:mm a')}</span>}
					{memory.eventAt && <span>event {format(timestampDate(memory.eventAt), 'MMM d, yyyy h:mm a')}</span>}
					{superseded && memory.supersession?.winnerPid && <span>winner {memory.supersession.winnerPid}</span>}
				</div>
			</div>
		</li>
	);
}
