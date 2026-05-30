'use client';

import type { SupersessionEvent } from '@actions/supersession-history';
import { timestampDate } from '@bufbuild/protobuf/wkt';
import { format } from 'date-fns';
import { ArrowLeftRight, RotateCcw } from 'lucide-react';

export default function SupersessionEventRow({ event }: { event: SupersessionEvent }) {
	const unsupersede = event.winnerPid === undefined;
	const Icon = unsupersede ? RotateCcw : ArrowLeftRight;
	return (
		<li className="card border bg-base-100">
			<div className="card-body flex-row items-center gap-4 p-4">
				<Icon className={`h-5 w-5 shrink-0 ${unsupersede ? 'text-info' : 'text-warning'}`} />
				<div className="flex flex-1 flex-wrap items-baseline gap-x-4 gap-y-1">
					<span className={`badge badge-sm ${unsupersede ? 'badge-info' : 'badge-warning'}`}>
						{unsupersede ? 'unsupersede' : 'supersede'}
					</span>
					{event.winnerPid && (
						<span className="text-base-content/80 text-sm">
							winner <span className="font-mono">{event.winnerPid}</span>
						</span>
					)}
					{event.decidedAt && (
						<span className="ml-auto text-base-content/60 text-xs">
							{format(timestampDate(event.decidedAt), 'MMM d, yyyy h:mm a')}
						</span>
					)}
				</div>
			</div>
		</li>
	);
}
