'use client';

import { cn } from '@lib/utils';
import React, { type ComponentProps } from 'react';
import { Button as DaisyButton, Loading } from 'rsc-daisyui';

export interface ButtonProps extends Omit<ComponentProps<typeof DaisyButton>, 'disabled'> {
	/** Shows a loading spinner and disables the button */
	loading?: boolean;
	/** Disables the button */
	disabled?: boolean;
}

function Button({ className, loading, disabled, children, ...props }: ButtonProps, ref: React.Ref<HTMLButtonElement>) {
	const isDisabled = disabled || loading;
	return (
		<DaisyButton
			ref={ref}
			className={cn('gap-2', isDisabled && 'btn-disabled', className)}
			aria-disabled={isDisabled || undefined}
			// Spread native disabled attribute explicitly since rsc-daisyui consumes it for styling only
			{...(isDisabled ? { disabled: true } : {})}
			{...props}>
			{loading && <Loading variant="spinner" size="sm" />}
			{children}
		</DaisyButton>
	);
}

Button.displayName = 'Button';

const ForwardedButton = React.forwardRef<HTMLButtonElement, ButtonProps>(Button);
ForwardedButton.displayName = 'Button';

export default ForwardedButton;
