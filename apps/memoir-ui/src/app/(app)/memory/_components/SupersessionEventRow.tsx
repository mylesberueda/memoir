'use client';

import { timestampDate } from '@bufbuild/protobuf/wkt';
import type { SupersessionEvent } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
import { format } from 'date-fns';
import { ArrowLeftRight, RotateCcw } from 'lucide-react';

export default function SupersessionEventRow({ event }: { event: SupersessionEvent }) {
	const unsupersede = event.winnerPid === undefined;
	const Icon = unsupersede ? RotateCcw : ArrowLeftRight;
	return (
		<li className="card border border-base-300 bg-base-100">
			<div className="flex items-center gap-4 p-4">
				<Icon className={`h-5 w-5 shrink-0 ${unsupersede ? 'text-info' : 'text-warning'}`} />
				<div className="flex min-w-0 flex-1 flex-wrap items-center gap-x-3 gap-y-1.5">
					<span className={`badge badge-sm ${unsupersede ? 'badge-info' : 'badge-warning'}`}>
						{unsupersede ? 'unsupersede' : 'supersede'}
					</span>
					{event.winnerPid && (
						<span className="min-w-0 truncate text-base-content/80 text-sm">
							winner <span className="font-mono">{event.winnerPid}</span>
						</span>
					)}
				</div>
				{event.decidedAt && (
					<span className="shrink-0 whitespace-nowrap text-base-content/55 text-xs">
						{format(timestampDate(event.decidedAt), 'MMM d, yyyy h:mm a')}
					</span>
				)}
			</div>
		</li>
	);
}
