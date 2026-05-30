import { cn } from '@lib/utils';
import type * as React from 'react';

type PageWidth = 'list' | 'form' | 'wide';

const WIDTH_CLASS: Record<PageWidth, string> = {
	form: 'max-w-2xl',
	list: 'max-w-5xl',
	wide: 'max-w-7xl',
};

export interface PageContainerProps {
	children: React.ReactNode;
	width?: PageWidth;
	className?: string;
}

export function PageContainer({ children, width = 'list', className }: PageContainerProps) {
	return (
		<div id="page__container" className={cn('mx-auto w-full px-4 py-8 sm:px-6 lg:px-8', WIDTH_CLASS[width], className)}>
			{children}
		</div>
	);
}

export interface PageHeaderProps {
	title: string;
	description?: React.ReactNode;
	eyebrow?: string;
	actions?: React.ReactNode;
	className?: string;
}

export function PageHeader({ title, description, eyebrow, actions, className }: PageHeaderProps) {
	return (
		<header id="page__header" className={cn('mb-8 flex items-start justify-between gap-4', className)}>
			<div className="min-w-0">
				{eyebrow && (
					<p className="mb-2 font-medium text-[0.6875rem] text-primary uppercase tracking-[0.16em]">{eyebrow}</p>
				)}
				<h1 className="font-display text-3xl text-base-content">{title}</h1>
				{description && <p className="mt-2 max-w-2xl text-base-content/60 leading-relaxed">{description}</p>}
			</div>
			{actions && <div className="flex shrink-0 items-center gap-2">{actions}</div>}
		</header>
	);
}
