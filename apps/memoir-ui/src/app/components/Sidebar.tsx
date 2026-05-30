'use client';

import useAuth from '@hooks/useAuth';
import { useLayoutContext } from '@providers';
import { useOrganizationsOptional } from '@providers/OrganizationContextProvider';
import cns from 'classnames';
import {
	BrainCircuit,
	Building2,
	Clock,
	GitBranch,
	History,
	Home,
	MessageSquare,
	Search,
	Settings,
	Users2,
} from 'lucide-react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import OrgSelectorWrapper from './OrgSelectorWrapper';

export default function Sidebar() {
	const layout = useLayoutContext();
	const { user } = useAuth();
	const path = usePathname();
	const orgContext = useOrganizationsOptional();

	const currentOrg = orgContext?.currentOrg;
	const orgLabel = currentOrg?.name;

	interface NavItemProps {
		href: string;
		icon: React.ElementType;
		children: React.ReactNode;
	}

	function NavItem({ href, icon: Icon, children }: NavItemProps) {
		const active = path === href;
		return (
			<Link
				href={href}
				onClick={layout.closeSidebar}
				className={cns(
					'group relative flex items-center rounded-field px-3 py-2 text-sm transition-all duration-150',
					active
						? 'bg-base-200 font-medium text-base-content'
						: 'text-base-content/65 hover:bg-base-200/60 hover:text-base-content',
				)}>
				<span
					className={cns(
						'absolute left-0 top-1/2 h-4 w-0.5 -translate-y-1/2 rounded-full bg-primary transition-all duration-150',
						active ? 'opacity-100' : 'opacity-0 group-hover:opacity-40',
					)}
				/>
				<Icon
					className={cns(
						'mr-3 h-4 w-4 flex-shrink-0 transition-colors',
						active ? 'text-primary' : 'text-base-content/50 group-hover:text-base-content/80',
					)}
					suppressHydrationWarning
				/>
				{children}
			</Link>
		);
	}

	function SectionHeader({ children }: { children: React.ReactNode }) {
		return (
			<div className="mb-2 px-3 font-medium text-base-content/40 text-[0.6875rem] uppercase tracking-[0.14em]">
				{children}
			</div>
		);
	}

	return (
		<>
			<nav
				className={cns(
					'fixed inset-y-0 left-0 z-[70] w-64 transform bg-base-100 transition-transform duration-200 ease-in-out',
					'border-base-300 border-r lg:static lg:w-64 lg:translate-x-0',
					layout.isSidebarOpen ? 'translate-x-0' : '-translate-x-full',
				)}>
				<div className="flex h-full flex-col">
					<Link href="/dashboard" className="group flex h-16 items-center border-base-300 border-b px-6">
						<div className="flex items-center gap-2.5">
							<span className="flex h-8 w-8 items-center justify-center rounded-field bg-primary/10 text-primary transition-colors group-hover:bg-primary/15">
								<BrainCircuit className="h-[1.15rem] w-[1.15rem]" />
							</span>
							<span className="font-display font-semibold text-[1.35rem] leading-none tracking-[0.02em] text-base-content">
								Memoir
							</span>
						</div>
					</Link>

					<div className="border-base-300 border-b px-4 py-3">
						<OrgSelectorWrapper />
					</div>

					<div className="flex-1 overflow-y-auto px-4 py-4">
						<div className="space-y-6">
							<div>
								<SectionHeader>Overview</SectionHeader>
								<div className="space-y-1">
									<NavItem href="/dashboard" icon={Home}>
										Dashboard
									</NavItem>
								</div>
							</div>

							<div>
								<SectionHeader>Playground</SectionHeader>
								<div className="space-y-1">
									<NavItem href="/playground" icon={MessageSquare}>
										Chat
									</NavItem>
								</div>
							</div>

							<div>
								<SectionHeader>Memory</SectionHeader>
								<div className="space-y-1">
									<NavItem href="/memory/timeline" icon={Clock}>
										Timeline
									</NavItem>
									<NavItem href="/memory/query" icon={Search}>
										Query
									</NavItem>
									<NavItem href="/memory/as-of" icon={History}>
										Point-in-time
									</NavItem>
									<NavItem href="/memory/audit" icon={GitBranch}>
										Audit
									</NavItem>
								</div>
							</div>

							{orgLabel && (
								<div>
									<SectionHeader>
										<span className="truncate block max-w-[180px]" title={orgLabel}>
											{orgLabel}
										</span>
									</SectionHeader>
									<div className="space-y-1">
										<NavItem href="/org/details" icon={Building2}>
											Details
										</NavItem>
										<NavItem href="/org/members" icon={Users2}>
											Members
										</NavItem>
									</div>
								</div>
							)}
						</div>
					</div>

					<div className="border-base-300 border-t px-4 py-4">
						{user ? (
							<div className="space-y-1">
								<NavItem href="/settings" icon={Settings}>
									Settings
								</NavItem>
							</div>
						) : (
							<div className="space-y-2">
								<Link
									href="/auth/login"
									className="btn btn-primary btn-sm w-full"
									onClick={layout.closeSidebar}
									prefetch={false}>
									Sign In
								</Link>
								<Link
									href="/auth/register"
									className="btn btn-outline btn-sm w-full"
									onClick={layout.closeSidebar}
									prefetch={false}>
									Sign Up
								</Link>
							</div>
						)}
					</div>
				</div>
			</nav>

			{layout.isSidebarOpen && (
				// biome-ignore lint/a11y/noStaticElementInteractions: background
				<div
					className="fixed inset-0 z-[65] bg-black bg-opacity-50 lg:hidden"
					role="presentation"
					onClick={layout.closeSidebar}
				/>
			)}
		</>
	);
}
