'use client';

import { cn } from '@lib/utils';
import type { ToolUIPart } from 'ai';
import { WrenchIcon } from 'lucide-react';
import type { ComponentProps } from 'react';
import { getStatusBadge } from './Tool.utils';

export interface ToolHeaderProps extends ComponentProps<'div'> {
	/** The tool display name (e.g., "Create Agent", "Database Query") */
	type: string;
	/** The current state of the tool execution */
	state: ToolUIPart['state'];
}

export default function ToolHeader({ className, type, state, ...props }: ToolHeaderProps) {
	return (
		<div className={cn('flex w-full items-center justify-between gap-4 p-3 cursor-pointer', className)} {...props}>
			<div className="flex items-center gap-2">
				<WrenchIcon className="size-4 text-muted-foreground" />
				<span className="font-medium text-sm">{type}</span>
			</div>
			{getStatusBadge(state)}
		</div>
	);
}
