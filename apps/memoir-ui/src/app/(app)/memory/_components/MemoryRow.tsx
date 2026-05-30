'use client';

import { timestampDate } from '@bufbuild/protobuf/wkt';
import { type Memory, MemoryKind, MemoryStatus } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
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
	const failed = memory.status === MemoryStatus.FAILED;
	const semantic = memory.kind === MemoryKind.SEMANTIC;
	return (
		<li className={`card border border-base-300 bg-base-100 ${superseded ? 'border-warning/40 opacity-70' : ''}`}>
			<div className="flex flex-col gap-2.5 p-4">
				<div className="flex items-start justify-between gap-4">
					<p className="min-w-0 break-words text-base-content leading-relaxed">{memory.content}</p>
					<div className="flex shrink-0 items-center gap-2">
						<span className={`badge badge-sm ${semantic ? 'badge-accent' : 'badge-secondary'}`}>
							{semantic ? 'semantic' : 'episodic'}
						</span>
						{pending && <span className="badge badge-info badge-sm">pending</span>}
						{failed && <span className="badge badge-error badge-sm">failed</span>}
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
				<div className="flex flex-wrap gap-x-4 gap-y-1.5 text-base-content/55 text-xs">
					{memory.createdAt && <span>created {format(timestampDate(memory.createdAt), 'MMM d, yyyy h:mm a')}</span>}
					{memory.eventAt && <span>event {format(timestampDate(memory.eventAt), 'MMM d, yyyy h:mm a')}</span>}
					{superseded && memory.supersession?.winnerPid && (
						<span className="font-mono">winner {memory.supersession.winnerPid}</span>
					)}
				</div>
			</div>
		</li>
	);
}
