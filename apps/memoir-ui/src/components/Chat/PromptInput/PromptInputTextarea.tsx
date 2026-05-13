'use client';

import { Textarea } from '@components';
import { cn } from '@lib/utils';
import type { ComponentProps, KeyboardEventHandler } from 'react';
import { forwardRef } from 'react';

export interface PromptInputTextareaProps extends ComponentProps<typeof Textarea> {
	minHeight?: number;
	maxHeight?: number;
}

export default forwardRef<HTMLTextAreaElement, PromptInputTextareaProps>(function PromptInputTextarea(
	{ onChange, className, placeholder = 'What would you like to know?', minHeight = 48, maxHeight = 164, ...props },
	ref,
) {
	const handleKeyDown: KeyboardEventHandler<HTMLTextAreaElement> = (e) => {
		if (e.key === 'Enter') {
			// Don't submit if IME composition is in progress
			if (e.nativeEvent.isComposing) {
				return;
			}

			if (e.shiftKey) {
				// Allow newline
				return;
			}

			// Submit on Enter (without Shift)
			e.preventDefault();
			const form = e.currentTarget.form;
			if (form) {
				form.requestSubmit();
			}
		}
	};

	return (
		<Textarea
			ref={ref}
			className={cn(
				'w-full resize-none rounded-none !border-none p-3 !shadow-none !outline-none',
				'field-sizing-content max-h-[6lh] bg-base-100',
				className,
			)}
			name="message"
			data-testid="chat-input"
			onChange={(e) => {
				onChange?.(e);
			}}
			onKeyDown={handleKeyDown}
			placeholder={placeholder}
			{...props}
		/>
	);
});
