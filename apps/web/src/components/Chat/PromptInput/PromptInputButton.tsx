'use client';

import { Button } from '@components';
import { cn } from '@lib/utils';
import type { ComponentProps } from 'react';

type ButtonProps = Omit<ComponentProps<typeof Button>, 'size'>;

export interface PromptInputButtonProps extends ButtonProps {}

export default function PromptInputButton({ className, ...props }: PromptInputButtonProps) {
	return <Button className={cn('shrink-0 gap-1.5 rounded-lg', className)} size="sm" type="button" ghost {...props} />;
}
