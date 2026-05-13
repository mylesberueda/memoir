'use client';

import { cn } from '@lib/utils';
import type { HTMLAttributes } from 'react';

export interface PromptInputToolbarProps extends HTMLAttributes<HTMLDivElement> {}

export default function PromptInputToolbar({ className, ...props }: PromptInputToolbarProps) {
	return (
		<div className={cn('flex items-center justify-between p-2 bg-base-100 gap-2 max-w-full', className)} {...props} />
	);
}
