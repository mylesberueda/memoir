'use client';

import {
	autoUpdate,
	FloatingFocusManager,
	FloatingPortal,
	flip,
	offset,
	size,
	useDismiss,
	useFloating,
	useFocus,
	useInteractions,
	useListNavigation,
	useRole,
} from '@floating-ui/react';
import { cn } from '@lib/utils';
import { useEffect, useMemo, useRef, useState } from 'react';

export interface AgentIdInputProps {
	id?: string;
	value: string;
	onChange: (value: string) => void;
	agents: string[];
	disabled?: boolean;
	placeholder?: string;
	className?: string;
	/** Debounce window (ms) applied to the filter term. */
	debounceMs?: number;
}

export function AgentIdInput({
	id,
	value,
	onChange,
	agents,
	disabled,
	placeholder = 'agent persona id',
	className,
	debounceMs = 200,
}: AgentIdInputProps) {
	const [open, setOpen] = useState(false);
	const [activeIndex, setActiveIndex] = useState<number | null>(null);
	const [debouncedTerm, setDebouncedTerm] = useState(value);
	const listRef = useRef<Array<HTMLElement | null>>([]);

	useEffect(() => {
		const handle = setTimeout(() => setDebouncedTerm(value), debounceMs);
		return () => clearTimeout(handle);
	}, [value, debounceMs]);

	const matches = useMemo(() => {
		const term = debouncedTerm.trim().toLowerCase();
		// Substring filter, minus any agent that exactly equals the term — an
		// exact match is already in the input, so suggesting it is redundant.
		const pool = agents.filter((a) => {
			const lower = a.toLowerCase();
			return lower !== term && (term ? lower.includes(term) : true);
		});
		return pool.slice(0, 50);
	}, [agents, debouncedTerm]);

	const { refs, floatingStyles, context } = useFloating({
		open,
		onOpenChange: setOpen,
		whileElementsMounted: autoUpdate,
		placement: 'bottom-start',
		middleware: [
			offset(4),
			flip({ padding: 8 }),
			size({
				apply({ rects, elements, availableHeight }) {
					Object.assign(elements.floating.style, {
						width: `${rects.reference.width}px`,
						maxHeight: `${Math.min(availableHeight - 8, 288)}px`,
					});
				},
				padding: 8,
			}),
		],
	});

	const focus = useFocus(context);
	const dismiss = useDismiss(context);
	const role = useRole(context, { role: 'listbox' });
	const listNav = useListNavigation(context, {
		listRef,
		activeIndex,
		onNavigate: setActiveIndex,
		virtual: true,
		loop: true,
	});

	const { getReferenceProps, getFloatingProps, getItemProps } = useInteractions([focus, dismiss, role, listNav]);

	const showPopover = open && matches.length > 0;

	return (
		<>
			<input
				ref={refs.setReference}
				id={id}
				type="text"
				className={cn('input input-bordered w-full', className)}
				placeholder={placeholder}
				value={value}
				disabled={disabled}
				autoComplete="off"
				{...getReferenceProps({
					onChange(event) {
						const target = event.target as HTMLInputElement;
						onChange(target.value);
						setActiveIndex(null);
					},
					onKeyDown(event) {
						const active = activeIndex != null ? matches[activeIndex] : undefined;
						if (event.key === 'Enter' && active) {
							event.preventDefault();
							onChange(active);
							setOpen(false);
						}
					},
				})}
			/>
			{showPopover && (
				<FloatingPortal>
					<FloatingFocusManager context={context} initialFocus={-1} visuallyHiddenDismiss modal={false}>
						<ul
							{...getFloatingProps({
								ref: refs.setFloating,
								style: floatingStyles,
								className: 'z-[80] overflow-y-auto rounded-box border border-base-300 bg-base-100 p-1 shadow-lg',
							})}>
							{matches.map((agent, index) => (
								<li
									key={agent}
									{...getItemProps({
										ref(node) {
											listRef.current[index] = node;
										},
										onClick() {
											onChange(agent);
											setOpen(false);
										},
									})}
									className={cn(
										'cursor-pointer truncate rounded-field px-3 py-1.5 font-mono text-sm',
										activeIndex === index ? 'bg-primary text-primary-content' : 'text-base-content hover:bg-base-200',
									)}>
									{agent}
								</li>
							))}
						</ul>
					</FloatingFocusManager>
				</FloatingPortal>
			)}
		</>
	);
}
