'use client';

import { cn } from '@lib/utils';
import type { ComponentProps } from 'react';

export interface ToolContentProps extends ComponentProps<'div'> {
	/** Whether the content is currently open (accordion state) */
	isOpen?: boolean;
}

export default function ToolContent({ className, isOpen = true, ...props }: ToolContentProps) {
	if (!isOpen) {
		return null;
	}

	return <div className={cn('text-popover-foreground outline-none', className)} {...props} />;
}
