'use client';

import { Button, Tooltip } from '@components';
import { cn } from '@lib/utils';
import type { ComponentProps } from 'react';

export interface ActionsProps extends ComponentProps<'div'> {}

export const Actions = ({ className, children, ...props }: ActionsProps) => (
	<div className={cn('flex items-center gap-1', className)} {...props}>
		{children}
	</div>
);

export interface ActionProps extends ComponentProps<typeof Button> {
	tooltip?: string;
	label?: string;
}

export const Action = ({ tooltip, children, label, className, size = 'sm', ...props }: ActionProps) => {
	const button = (
		<Button
			className={cn('size-9 p-1.5 text-muted-foreground hover:text-foreground relative', className)}
			size={size}
			type="button"
			{...props}>
			{children}
			<span className="sr-only">{label || tooltip}</span>
		</Button>
	);

	if (tooltip) {
		return <Tooltip tip={tooltip}>{button}</Tooltip>;
	}

	return button;
};
