import { cn } from '@lib/utils';
import type * as React from 'react';

export interface FilterBarProps {
	children: React.ReactNode;
	onSubmit: (event: React.FormEvent<HTMLFormElement>) => void;
	id?: string;
	className?: string;
}

export function FilterBar({ children, onSubmit, id, className }: FilterBarProps) {
	return (
		<form
			id={id}
			onSubmit={onSubmit}
			className={cn(
				'mb-6 flex flex-wrap items-end gap-x-4 gap-y-3 rounded-box border border-base-300 bg-base-100 p-4',
				className,
			)}>
			{children}
		</form>
	);
}

export interface FieldProps {
	label: string;
	htmlFor?: string;
	children: React.ReactNode;
	grow?: boolean;
	className?: string;
}

export function Field({ label, htmlFor, children, grow, className }: FieldProps) {
	return (
		<div className={cn('flex flex-col gap-1.5', grow && 'min-w-48 flex-1', className)}>
			<label htmlFor={htmlFor} className="font-medium text-base-content/70 text-xs">
				{label}
			</label>
			{children}
		</div>
	);
}
