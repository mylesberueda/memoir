'use client';

import { cn } from '@lib/utils';
import type { HTMLAttributes } from 'react';

export interface PromptInputToolsProps extends HTMLAttributes<HTMLDivElement> {}

export default function PromptInputTools({ className, ...props }: PromptInputToolsProps) {
	return (
		<div className={cn('flex items-center gap-1', '[&_button:first-child]:rounded-bl-xl', className)} {...props} />
	);
}
