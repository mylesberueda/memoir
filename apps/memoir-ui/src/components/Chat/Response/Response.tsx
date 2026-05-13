'use client';

import { cn } from '@lib/utils';
import type { ComponentProps, ReactElement } from 'react';
import { Streamdown, type StreamdownProps } from 'streamdown';
import { CodeBlock, CodeBlockCopyButton } from '../Tool/CodeBlock';

export interface ResponseProps extends ComponentProps<typeof Streamdown> {}

const customComponents: StreamdownProps['components'] = {
	ul: ({ className, ...props }) => <ul className={cn('ml-6 list-inside list-disc space-y-1', className)} {...props} />,
	ol: ({ className, ...props }) => (
		<ol className={cn('ml-6 list-inside list-decimal space-y-1', className)} {...props} />
	),
	li: ({ className, ...props }) => <li className={cn('[&>p]:inline [&>p:first-child]:inline', className)} {...props} />,
	pre: ({ children, className, ...props }) => {
		// Extract code content and language from the nested code element
		// Streamdown renders: <pre><code class="language-xxx">content</code></pre>
		const codeChild = children as ReactElement<{ className?: string; children?: string }> | undefined;
		if (codeChild?.props?.className?.startsWith('language-')) {
			const language = codeChild.props.className.replace('language-', '');
			const code = typeof codeChild.props.children === 'string' ? codeChild.props.children : '';
			return (
				<CodeBlock code={code} language={language} className="my-3">
					<CodeBlockCopyButton size="sm" />
				</CodeBlock>
			);
		}
		// Fallback for non-language code blocks
		return (
			<pre className={cn('overflow-x-auto max-w-full rounded-md bg-base-200 p-3 my-3', className)} {...props}>
				{children}
			</pre>
		);
	},
};

export default function Response({ className, components, ...props }: ResponseProps) {
	return (
		<Streamdown
			className={cn('size-full min-w-0 [&>*:first-child]:mt-0 [&>*:last-child]:mb-0', className)}
			components={{ ...customComponents, ...components }}
			{...props}
		/>
	);
}
