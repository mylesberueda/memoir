'use client';

import React, { type ComponentProps } from 'react';
import { Modal as DaisyModal } from 'rsc-daisyui';

import { cn } from '@/lib/utils';

export interface ModalProps extends ComponentProps<typeof DaisyModal> {}

function Modal({ className, ...props }: ModalProps, ref: React.Ref<HTMLDialogElement>) {
	return <DaisyModal ref={ref} className={cn(className)} {...props} />;
}

Modal.displayName = 'Modal';

const ForwardedModal = React.forwardRef<HTMLDialogElement, ModalProps>(Modal);
ForwardedModal.displayName = 'Modal';

export default ForwardedModal;
