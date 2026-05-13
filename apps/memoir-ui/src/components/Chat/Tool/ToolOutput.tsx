'use client';

import { cn } from '@lib/utils';
import type { ToolUIPart } from 'ai';
import type { ComponentProps, ReactNode } from 'react';

export interface ToolOutputProps extends ComponentProps<'div'> {
	/** The output content from the tool execution */
	output: ReactNode;
	/** Error text if the tool execution failed */
	errorText: ToolUIPart['errorText'];
}

export default function ToolOutput({ className, output, errorText, ...props }: ToolOutputProps) {
	if (!(output || errorText)) {
		return null;
	}

	return (
		<div className={cn('space-y-2 p-4', className)} {...props}>
			<h4 className="font-medium text-muted-foreground text-xs uppercase tracking-wide">
				{errorText ? 'Error' : 'Result'}
			</h4>
			<div
				className={cn(
					'overflow-x-auto rounded-md text-xs [&_table]:w-full',
					errorText ? 'bg-destructive/10 text-destructive' : 'bg-muted/50 text-foreground',
				)}>
				{errorText && <div>{errorText}</div>}
				{output && <div>{output}</div>}
			</div>
		</div>
	);
}
