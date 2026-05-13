'use client';

import { cn } from '@lib/utils';
import React, { type ComponentProps } from 'react';
import { Tooltip as DaisyTooltip } from 'rsc-daisyui';

type DaisyTooltipProps = ComponentProps<typeof DaisyTooltip>;

export interface TooltipProps extends DaisyTooltipProps {}

function Tooltip({ className, ...props }: TooltipProps, ref: React.Ref<HTMLDivElement>) {
	return <DaisyTooltip ref={ref} className={cn(className)} {...props} />;
}

Tooltip.displayName = 'Tooltip';

export default React.forwardRef<HTMLDivElement, TooltipProps>(Tooltip);
