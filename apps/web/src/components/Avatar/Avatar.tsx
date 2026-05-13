'use client';

import * as React from 'react';
import { Avatar as DaisyAvatar } from 'rsc-daisyui';
import { cn } from '@/lib/utils';

export type AvatarProps = React.ComponentProps<typeof DaisyAvatar>;

function Avatar({ className, ...props }: AvatarProps, ref: React.Ref<HTMLDivElement>) {
	return (
		<DaisyAvatar ref={ref} className={cn('ring-primary ring-offset-base-100 ring-offset-2', className)} {...props} />
	);
}

Avatar.displayName = 'Avatar';

export default React.forwardRef<HTMLDivElement, AvatarProps>(Avatar);
