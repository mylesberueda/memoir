'use client';

import { Badge as DaisyBadge } from 'rsc-daisyui';
import { cn } from '@/lib/utils';

export type BadgeProps = React.ComponentProps<typeof DaisyBadge>;

export default function Badge({ className, ...props }: BadgeProps) {
	return <DaisyBadge className={cn('gap-1', className)} {...props} />;
}
