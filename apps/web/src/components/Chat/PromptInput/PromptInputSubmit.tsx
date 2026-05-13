'use client';

import { Button } from '@components';
import { cn } from '@lib/utils';
import type { ChatStatus } from 'ai';
import { Loader2Icon, SendIcon, SquareIcon, XIcon } from 'lucide-react';
import type { ComponentProps } from 'react';

export interface PromptInputSubmitProps extends ComponentProps<typeof Button> {
	status?: ChatStatus;
}

export default function PromptInputSubmit({
	className,
	color = 'primary',
	size = 'sm',
	status,
	type = 'submit',
	children,
	...props
}: PromptInputSubmitProps) {
	let Icon = <SendIcon className="size-4" />;

	if (status === 'submitted') {
		Icon = <Loader2Icon className="size-4 animate-spin" />;
	} else if (status === 'streaming') {
		Icon = <SquareIcon className="size-4" />;
	} else if (status === 'error') {
		Icon = <XIcon className="size-4" />;
	}

	return (
		<Button className={cn('gap-1.5 rounded-lg', className)} size={size} type={type} color={color} {...props}>
			{children ?? Icon}
		</Button>
	);
}
