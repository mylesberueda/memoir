import { cn } from '@lib/utils';
import React, { type ComponentProps } from 'react';
import { Textarea as DaisyTextarea } from 'rsc-daisyui';

type DaisyTextareaProps = ComponentProps<typeof DaisyTextarea>;

export interface TextareaProps extends DaisyTextareaProps {}

function Textarea({ className, ...props }: TextareaProps, ref: React.Ref<HTMLTextAreaElement>) {
	return <DaisyTextarea ref={ref} className={cn(className)} {...props} />;
}

Textarea.displayName = 'Textarea';

export default React.forwardRef<HTMLTextAreaElement, TextareaProps>(Textarea);
