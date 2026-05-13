'use client';

import { cn } from '@lib/utils';
import type { ToolUIPart } from 'ai';
import { WrenchIcon } from 'lucide-react';
import type { ComponentProps } from 'react';
import { useState } from 'react';
import { getBorderColor, getStatusBadge } from './Tool.utils';

export interface ToolProps extends ComponentProps<'div'> {
	/** The tool display name (e.g., "Create Agent", "Database Query") */
	type: string;
	/** The current state of the tool execution */
	state: ToolUIPart['state'];
	/** The tool content to display when opened */
	children?: React.ReactNode;
}

export default function Tool({ className, type, state, children, ...props }: ToolProps) {
	const [isOpen, setIsOpen] = useState(false);

	const toggleOpen = () => setIsOpen(!isOpen);

	const borderColor = getBorderColor(state);

	return (
		<div className={cn('not-prose w-full rounded-md border', borderColor, className)} {...props}>
			<button
				type="button"
				className="flex w-full items-center justify-between gap-4 p-3 cursor-pointer bg-transparent border-none"
				onClick={toggleOpen}
				aria-expanded={isOpen}>
				<div className="flex items-center gap-2">
					<WrenchIcon className="size-4 text-muted-foreground" />
					<span id="tool_call__name" className="font-medium text-sm">
						{type}
					</span>
				</div>
				{getStatusBadge(state)}
			</button>
			{isOpen && children && <div className="text-popover-foreground outline-none">{children}</div>}
		</div>
	);
}
