'use client';

import { cn } from '@lib/utils';
import type { ToolUIPart } from 'ai';
import type { ComponentProps } from 'react';
import { CodeBlock } from './CodeBlock';

export interface ToolInputProps extends ComponentProps<'div'> {
	/** The input parameters for the tool */
	input: ToolUIPart['input'];
}

export default function ToolInput({ className, input, ...props }: ToolInputProps) {
	return (
		<div className={cn('space-y-2 overflow-hidden p-4', className)} {...props}>
			<h4 className="font-medium text-muted-foreground text-xs uppercase tracking-wide">Parameters</h4>
			<div className="rounded-md bg-muted/50">
				<CodeBlock code={JSON.stringify(input, null, 2)} language="json" />
			</div>
		</div>
	);
}
