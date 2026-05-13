'use client';

import { cn } from '@lib/utils';
import * as React from 'react';
import { Select as DaisySelect } from 'rsc-daisyui';

type DaisySelectProps = React.ComponentProps<typeof DaisySelect>;

export interface SelectProps extends DaisySelectProps {}

function Select({ className, ...props }: SelectProps, ref: React.Ref<HTMLSelectElement>) {
	return <DaisySelect ref={ref} className={cn(className)} {...props} />;
}

Select.displayName = 'Select';

export default React.forwardRef<HTMLSelectElement, SelectProps>(Select);
