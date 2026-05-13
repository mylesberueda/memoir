'use client';

import { Button } from '@components';
import cns from 'classnames';
import Link from 'next/link';
import { type ReactNode, useCallback, useEffect, useRef, useState } from 'react';

export interface DockAction {
	id: string;
	icon: React.ElementType;
	label: string;
	onClick: () => void;
}

export interface DockViewItem {
	id: string;
	icon: React.ElementType;
	label: string;
	component: ReactNode;
}

export interface DockLinkItem {
	id: string;
	icon: React.ElementType;
	label: string;
	href: string;
	isActive?: boolean;
}

export interface ResponsiveDockProps {
	/** Action buttons shown at top (New Conversation, Edit Agent, etc.) */
	actions?: DockAction[];
	/** View items that render a component when active (History, Files) */
	views?: DockViewItem[];
	/** Link items for navigation (Settings pages) */
	links?: DockLinkItem[];
	/** Currently active view ID */
	activeViewId?: string;
	/** Callback when view changes */
	onViewChange?: (id: string) => void;
	/** Title shown in sidebar header on desktop */
	title?: string;
	/** Main content to render (chat area, page content) */
	children: ReactNode;
	/** Breakpoint for mobile/desktop switch */
	breakpoint?: 'sm' | 'md' | 'lg';
}

function DockItemButton({
	icon: Icon,
	label,
	isActive,
	onClick,
	className,
}: {
	icon: React.ElementType;
	label: string;
	isActive?: boolean;
	onClick?: () => void;
	className?: string;
}) {
	return (
		<button
			type="button"
			onClick={onClick}
			className={cns(
				'flex w-16 flex-col items-center gap-1 rounded-lg px-3 py-2 transition-colors',
				isActive ? 'bg-primary text-primary-content' : 'text-base-content hover:bg-primary/10 hover:text-primary',
				className,
			)}>
			<Icon className="h-5 w-5 shrink-0" suppressHydrationWarning />
			<span className="max-w-[4rem] truncate text-xs">{label}</span>
		</button>
	);
}

function DockItemLink({
	icon: Icon,
	label,
	href,
	isActive,
	className,
}: {
	icon: React.ElementType;
	label: string;
	href: string;
	isActive?: boolean;
	className?: string;
}) {
	return (
		<Link
			href={href}
			className={cns(
				'flex w-16 flex-col items-center gap-1 rounded-lg px-3 py-2 transition-colors',
				isActive ? 'bg-primary text-primary-content' : 'text-base-content hover:bg-primary/10 hover:text-primary',
				className,
			)}>
			<Icon className="h-5 w-5 shrink-0" suppressHydrationWarning />
			<span className="max-w-[4rem] truncate text-xs">{label}</span>
		</Link>
	);
}

function SidebarLink({
	icon: Icon,
	label,
	href,
	isActive,
}: {
	icon: React.ElementType;
	label: string;
	href: string;
	isActive?: boolean;
}) {
	return (
		<Link
			href={href}
			className={cns(
				'flex items-center rounded-box px-3 py-2 text-sm transition-colors',
				isActive ? 'bg-primary text-primary-content' : 'text-base-content hover:bg-base-200',
			)}>
			<Icon className="mr-3 h-4 w-4 flex-shrink-0" suppressHydrationWarning />
			{label}
		</Link>
	);
}

function SidebarActionButton({
	icon: Icon,
	label,
	onClick,
	primary,
}: {
	icon: React.ElementType;
	label: string;
	onClick: () => void;
	primary?: boolean;
}) {
	return (
		<Button type="button" onClick={onClick} color="primary" outline={!primary}>
			<Icon className="h-4 w-4" suppressHydrationWarning />
			{label}
		</Button>
	);
}

function MobileDock({
	actions,
	views,
	links,
	activeViewId,
	onViewChange,
}: Pick<ResponsiveDockProps, 'actions' | 'views' | 'links' | 'activeViewId' | 'onViewChange'>) {
	const scrollRef = useRef<HTMLDivElement>(null);
	const [canScrollLeft, setCanScrollLeft] = useState(false);
	const [canScrollRight, setCanScrollRight] = useState(false);

	const updateScrollState = useCallback(() => {
		const el = scrollRef.current;
		if (!el) return;
		setCanScrollLeft(el.scrollLeft > 0);
		setCanScrollRight(el.scrollLeft < el.scrollWidth - el.clientWidth - 1);
	}, []);

	const centerActiveItem = useCallback(() => {
		const el = scrollRef.current;
		if (!el) return;

		const activeItem = el.querySelector('[data-active="true"]') as HTMLElement;
		if (!activeItem) return;

		const containerWidth = el.clientWidth;
		const itemLeft = activeItem.offsetLeft;
		const itemWidth = activeItem.offsetWidth;
		const targetScroll = itemLeft - containerWidth / 2 + itemWidth / 2;

		el.scrollTo?.({ left: targetScroll, behavior: 'smooth' });
	}, []);

	useEffect(() => {
		updateScrollState();
		centerActiveItem();

		const el = scrollRef.current;
		if (!el) return;

		el.addEventListener('scroll', updateScrollState);
		window.addEventListener('resize', updateScrollState);

		return () => {
			el.removeEventListener('scroll', updateScrollState);
			window.removeEventListener('resize', updateScrollState);
		};
	}, [updateScrollState, centerActiveItem]);

	// Find the active item across all item types
	const getActiveItemId = () => {
		if (activeViewId) return activeViewId;
		const activeLink = links?.find((l) => l.isActive);
		return activeLink?.id;
	};

	const activeId = getActiveItemId();

	return (
		<div
			id="mobile_dock__container"
			className="fixed inset-x-0 bottom-0 z-50 border-t border-base-300 bg-base-100 md:hidden">
			<div className="relative">
				{/* Left fade */}
				<div
					className={cns(
						'pointer-events-none absolute left-0 top-0 bottom-0 z-10 w-8 bg-gradient-to-r from-base-100 to-transparent transition-opacity',
						canScrollLeft ? 'opacity-100' : 'opacity-0',
					)}
				/>
				{/* Right fade */}
				<div
					className={cns(
						'pointer-events-none absolute right-0 top-0 bottom-0 z-10 w-8 bg-gradient-to-l from-base-100 to-transparent transition-opacity',
						canScrollRight ? 'opacity-100' : 'opacity-0',
					)}
				/>

				<div
					ref={scrollRef}
					id="mobile_dock__scroll"
					className="flex justify-center gap-1 overflow-x-auto px-2 py-2 scrollbar-none">
					{/* Actions */}
					{actions?.map((action) => (
						<DockItemButton key={action.id} icon={action.icon} label={action.label} onClick={action.onClick} />
					))}

					{/* Separator if we have both actions and other items */}
					{actions && actions.length > 0 && ((views && views.length > 0) || (links && links.length > 0)) && (
						<div className="mx-1 w-px self-stretch bg-base-300" />
					)}

					{/* Views */}
					{views?.map((view) => (
						<div key={view.id} data-active={activeId === view.id}>
							<DockItemButton
								icon={view.icon}
								label={view.label}
								isActive={activeId === view.id}
								onClick={() => onViewChange?.(view.id)}
							/>
						</div>
					))}

					{/* Separator if we have both views and links */}
					{views && views.length > 0 && links && links.length > 0 && (
						<div className="mx-1 w-px self-stretch bg-base-300" />
					)}

					{/* Links */}
					{links?.map((link) => (
						<div key={link.id} data-active={activeId === link.id}>
							<DockItemLink icon={link.icon} label={link.label} href={link.href} isActive={link.isActive} />
						</div>
					))}
				</div>
			</div>
		</div>
	);
}

function DesktopSidebar({
	actions,
	views,
	links,
	activeViewId,
	onViewChange,
	title,
}: Pick<ResponsiveDockProps, 'actions' | 'views' | 'links' | 'activeViewId' | 'onViewChange' | 'title'>) {
	const activeView = views?.find((v) => v.id === activeViewId);

	return (
		<aside
			id="desktop_sidebar__container"
			className="hidden w-72 flex-shrink-0 flex-col border-r border-base-200 bg-base-100 md:flex">
			{title && (
				<div id="desktop_sidebar__header" className="border-b border-base-200 p-4">
					<h2 className="text-lg font-semibold text-base-content">{title}</h2>
				</div>
			)}
			{actions && actions.length > 0 && (
				<div id="desktop_sidebar__actions" className="flex flex-col gap-2 border-b border-base-200 p-4">
					{actions.map((action, idx) => (
						<SidebarActionButton
							key={action.id}
							icon={action.icon}
							label={action.label}
							onClick={action.onClick}
							primary={idx === 0}
						/>
					))}
				</div>
			)}

			{/* View tabs - styled like mobile dock */}
			{views && views.length > 0 && (
				<div id="desktop_sidebar__views" className="border-b border-base-200 px-2 py-3">
					<div className="flex justify-center gap-1">
						{views.map((view) => (
							<DockItemButton
								key={view.id}
								icon={view.icon}
								label={view.label}
								isActive={activeViewId === view.id}
								onClick={() => onViewChange?.(view.id)}
							/>
						))}
					</div>
				</div>
			)}

			{/* Active view component */}
			{activeView && (
				<div id="desktop_sidebar__view_content" className="flex-1 overflow-y-auto">
					{activeView.component}
				</div>
			)}

			{/* Links */}
			{links && links.length > 0 && (
				<div
					id="desktop_sidebar__links"
					className={cns(
						'p-4',
						(views && views.length > 0) || (actions && actions.length > 0) ? 'border-t border-base-200' : '',
						// If no views, links take up remaining space
						!views || views.length === 0 ? 'flex-1' : '',
					)}>
					<div className="space-y-1">
						{links.map((link) => (
							<SidebarLink
								key={link.id}
								icon={link.icon}
								label={link.label}
								href={link.href}
								isActive={link.isActive}
							/>
						))}
					</div>
				</div>
			)}
		</aside>
	);
}

export default function ResponsiveDock({
	actions,
	views,
	links,
	activeViewId,
	onViewChange,
	title,
	children,
}: ResponsiveDockProps) {
	const [mobileViewOpen, setMobileViewOpen] = useState(false);

	const handleMobileViewChange = useCallback(
		(id: string) => {
			if (id === activeViewId && mobileViewOpen) {
				setMobileViewOpen(false);
			} else {
				onViewChange?.(id);
				setMobileViewOpen(true);
			}
		},
		[activeViewId, mobileViewOpen, onViewChange],
	);

	const activeMobileView = views?.find((v) => v.id === activeViewId);

	return (
		<div id="responsive_dock__container" className="flex h-full">
			<DesktopSidebar
				actions={actions}
				views={views}
				links={links}
				activeViewId={activeViewId}
				onViewChange={onViewChange}
				title={title}
			/>
			<div
				id="responsive_dock__content"
				className="relative flex min-h-0 flex-1 flex-col overflow-y-auto pb-16 md:pb-0">
				{children}

				{activeMobileView && mobileViewOpen && (
					<div id="mobile_view_panel__container" className="absolute inset-0 z-40 flex flex-col bg-base-100 md:hidden">
						<div
							id="mobile_view_panel__header"
							className="flex items-center justify-between border-b border-base-200 p-4">
							<h2 className="text-lg font-semibold text-base-content">{activeMobileView.label}</h2>
							<button
								type="button"
								onClick={() => setMobileViewOpen(false)}
								className="btn btn-ghost btn-sm btn-circle">
								✕
							</button>
						</div>
						<div id="mobile_view_panel__content" className="flex-1 overflow-y-auto">
							{activeMobileView.component}
						</div>
					</div>
				)}
			</div>
			<MobileDock
				actions={actions}
				views={views}
				links={links}
				activeViewId={mobileViewOpen ? activeViewId : undefined}
				onViewChange={handleMobileViewChange}
			/>
		</div>
	);
}
