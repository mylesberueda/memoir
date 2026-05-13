'use client';

import { cn } from '@lib/utils';
import type { ComponentProps, ReactNode } from 'react';

export interface ChainOfThoughtProps extends ComponentProps<'div'> {
	children: ReactNode;
}

export interface StepProps extends ComponentProps<'div'> {
	/** Icon rendered in the timeline dot */
	icon: ReactNode;
	/** Optional label displayed next to the icon (e.g. "Thinking 3s", tool name) */
	label?: string;
	/** Step status controls dot styling */
	status?: 'streaming' | 'complete';
	/** Additional classes merged onto the timeline dot */
	dotClassName?: string;
	children: ReactNode;
}

function Step({ icon, label: _label, status = 'complete', dotClassName, children, className, ...props }: StepProps) {
	const isStreaming = status === 'streaming';

	return (
		<div id="timeline_step__container" className={cn('relative flex gap-3', className)} {...props}>
			<div id="timeline_step__gutter" className="flex flex-col items-center shrink-0">
				<div
					id="timeline_step__dot"
					className={cn(
						'flex items-center justify-center size-8 rounded-full border-2 bg-base-100 z-10',
						isStreaming ? 'border-primary animate-pulse text-primary' : 'border-base-content/20 text-base-content/50',
						dotClassName,
					)}>
					<div id="timeline_step__icon" className="size-4 [&>svg]:size-4">
						{icon}
					</div>
				</div>
				<div id="timeline_step__connector" className="w-0.5 grow bg-base-content/10" />
			</div>
			<div id="timeline_step__content" className="min-w-0 flex-1 pb-6 mt-1">
				{children}
			</div>
		</div>
	);
}

export default function ChainOfThought({ children, className, ...props }: ChainOfThoughtProps) {
	return (
		<div
			id="chain_of_thought__container"
			className={cn('flex flex-col [&>*:last-child_#timeline_step__connector]:hidden', className)}
			{...props}>
			{children}
		</div>
	);
}

ChainOfThought.Step = Step;
