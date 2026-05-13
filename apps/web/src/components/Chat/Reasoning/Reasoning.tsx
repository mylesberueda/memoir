'use client';

import useTimer from '@hooks/useTimer';
import { cn } from '@lib/utils';
import { BrainCircuitIcon } from 'lucide-react';
import type { ComponentProps } from 'react';
import React, { useEffect, useImperativeHandle, useRef, useState } from 'react';

export interface ReasoningRef {
	startTimer: () => void;
	stopTimer: () => number;
	setTimer: (seconds: number) => void;
}

export interface ReasoningProps extends Omit<ComponentProps<'div'>, 'content'> {
	/** The reasoning content to display */
	content: string | React.ReactNode;
	/** Custom title for the reasoning section */
	title?: string;
	/** Whether the reasoning section should be open by default */
	defaultOpen?: boolean;
	/** Duration of thinking in seconds (for completed reasoning) */
	thinkingDuration?: number;
	/** Rendering variant:
	 * - 'standalone' (default): full header with icon and toggle
	 * - 'inline': content only, no header (for use inside ChainOfThought.Step)
	 */
	variant?: 'standalone' | 'inline';
}

function getDisplayText(isTimerRunning: boolean, elapsed: number, thinkingDuration?: number, title = 'Reasoning') {
	if (isTimerRunning) {
		return `Thinking ${elapsed}s`;
	}
	if (thinkingDuration) {
		return `Thought for ${thinkingDuration}s`;
	}
	return title;
}

function Reasoning(
	{
		className,
		content,
		title = 'Reasoning',
		defaultOpen = false,
		thinkingDuration,
		variant = 'standalone',
		...props
	}: ReasoningProps,
	ref: React.Ref<ReasoningRef>,
) {
	const [isOpen, setIsOpen] = useState(defaultOpen);
	// If component mounts with defaultOpen=true, it was auto-opened
	const wasAutoOpenedRef = useRef(defaultOpen);
	const timer = useTimer();
	const previousDefaultOpenRef = useRef(defaultOpen);
	const isFirstMountRef = useRef(true);

	useImperativeHandle(
		ref,
		() => ({
			startTimer: () => {
				setIsOpen(true);
				wasAutoOpenedRef.current = true;
				timer.start();
			},

			stopTimer: () => {
				const finalDuration = timer.stop();

				if (wasAutoOpenedRef.current) {
					setTimeout(() => {
						setIsOpen(false);
						wasAutoOpenedRef.current = false;
					}, 1000);
				}

				return finalDuration;
			},

			setTimer: timer.setElapsed,
		}),
		[timer],
	);

	const toggleOpen = () => {
		setIsOpen(!isOpen);
		wasAutoOpenedRef.current = false;
	};

	// Handle auto-open/close based on defaultOpen prop changes
	useEffect(() => {
		const previousDefaultOpen = previousDefaultOpenRef.current;
		previousDefaultOpenRef.current = defaultOpen;
		const isFirstMount = isFirstMountRef.current;
		isFirstMountRef.current = false;

		// On first mount, start timer if defaultOpen is true
		if (isFirstMount && defaultOpen && wasAutoOpenedRef.current) {
			timer.start();
			return;
		}

		// When streaming starts (defaultOpen changes from false to true)
		if (!previousDefaultOpen && defaultOpen) {
			setIsOpen(true);
			wasAutoOpenedRef.current = true;
			timer.start();
		}

		// When streaming ends (defaultOpen changes from true to false)
		if (previousDefaultOpen && !defaultOpen) {
			timer.stop();
			// Auto-close after 1 second if it was auto-opened
			if (wasAutoOpenedRef.current) {
				setTimeout(() => {
					setIsOpen(false);
					wasAutoOpenedRef.current = false;
				}, 1000);
			}
		}
	}, [defaultOpen, timer]);

	const displayText = getDisplayText(timer.isRunning, timer.elapsed, thinkingDuration, title);

	const contentBody = (
		<div
			id="reasoning__body"
			className={cn(
				'text-muted-foreground text-sm leading-relaxed',
				'prose prose-sm max-w-none',
				'[&_p]:mb-2 [&_p:last-child]:mb-0',
				'[&_ul]:mb-2 [&_ol]:mb-2',
				'[&_li]:mb-1',
				'[&_code]:bg-muted [&_code]:px-1 [&_code]:py-0.5 [&_code]:rounded [&_code]:text-xs',
				'[&_pre]:bg-muted [&_pre]:p-3 [&_pre]:rounded-md [&_pre]:overflow-x-auto',
			)}>
			{typeof content === 'string' ? (
				<div id="reasoning__text" className="whitespace-pre-wrap">
					{content}
				</div>
			) : (
				content
			)}
		</div>
	);

	// Inline variant: just the collapsible content, no header chrome.
	// The parent ChainOfThought.Step provides the icon and toggle.
	if (variant === 'inline') {
		return (
			<div
				id="reasoning_inline__container"
				className={cn('not-prose w-full collapse border-0 bg-transparent p-0', isOpen && 'collapse-open', className)}
				{...props}>
				<button
					id="reasoning_inline__trigger"
					type="button"
					className={cn(
						'flex w-full items-center p-0 cursor-pointer collapse-title min-h-0',
						'hover:opacity-70 transition-opacity duration-200',
						'bg-transparent border-none text-left',
					)}
					onClick={toggleOpen}
					aria-expanded={isOpen}
					aria-label={`${isOpen ? 'Hide' : 'Show'} reasoning section`}>
					<span className="text-sm text-muted-foreground font-medium">{displayText}</span>
					<svg
						className={cn(
							'size-4 text-muted-foreground transition-transform duration-200 ml-1',
							isOpen ? 'rotate-180' : 'rotate-0',
						)}
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true">
						<title>Toggle reasoning</title>
						<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="reasoning_inline__content"
					className="overflow-hidden collapse-content transition-all transition-discrete duration-300 ease-in-out !p-0">
					{contentBody}
				</div>
			</div>
		);
	}

	return (
		<div
			id="reasoning__container"
			className={cn('not-prose w-full collapse border-0 bg-transparent p-0', isOpen && 'collapse-open', className)}
			{...props}>
			<button
				id="reasoning__trigger"
				type="button"
				className={cn(
					'flex w-full items-center justify-between p-0 cursor-pointer collapse-title min-h-0',
					'hover:opacity-70 transition-opacity duration-200',
					'bg-transparent border-none text-left',
				)}
				onClick={toggleOpen}
				aria-expanded={isOpen}
				aria-label={`${isOpen ? 'Hide' : 'Show'} reasoning section`}>
				<div id="reasoning__header" className="flex items-center gap-2">
					<BrainCircuitIcon
						className={cn('size-4 text-muted-foreground', timer.isRunning && 'animate-pulse text-primary')}
					/>
					<span className="text-sm text-muted-foreground font-medium">{displayText}</span>
					<svg
						className={cn(
							'size-4 text-muted-foreground transition-transform duration-200',
							isOpen ? 'rotate-180' : 'rotate-0',
						)}
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true">
						<title>Toggle reasoning</title>
						<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
					</svg>
				</div>
			</button>
			<div
				id="reasoning__content"
				className="overflow-hidden pl-6 collapse-content transition-all transition-discrete duration-300 ease-in-out">
				{contentBody}
			</div>
		</div>
	);
}

Reasoning.displayName = 'Reasoning';

export default React.forwardRef<ReasoningRef, ReasoningProps>(Reasoning);
