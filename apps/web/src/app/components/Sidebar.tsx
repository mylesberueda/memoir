'use client';

import useAuth from '@hooks/useAuth';
import { useLayoutContext } from '@providers';
import { useOrganizationsOptional } from '@providers/OrganizationContextProvider';
import cns from 'classnames';
import {
	BotMessageSquare,
	BrainCircuit,
	Building2,
	CreditCard,
	FileText,
	Folder,
	HelpCircle,
	Home,
	Settings,
	Sparkles,
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
	const can = orgContext?.can ?? (() => false);

	interface NavItemProps {
		href: string;
		icon: React.ElementType;
		children: React.ReactNode;
	}

	function NavItem({ href, icon: Icon, children }: NavItemProps) {
		return (
			<Link
				href={href}
				onClick={layout.closeSidebar}
				className={cns(
					'flex items-center rounded-box px-3 py-2 text-base-content text-sm transition-colors',
					path === href ? 'bg-primary text-primary-content' : 'hover:bg-base-200',
				)}>
				<Icon className="mr-3 h-4 w-4 flex-shrink-0" suppressHydrationWarning />
				{children}
			</Link>
		);
	}

	function SectionHeader({ children }: { children: React.ReactNode }) {
		return (
			<div className="mb-2 px-3 font-semibold text-gray-500 text-xs uppercase tracking-wider dark:text-gray-400">
				{children}
			</div>
		);
	}

	return (
		<>
			<nav
				className={cns(
					'fixed inset-y-0 left-0 z-[70] w-64 transform bg-base-100 transition-transform duration-200 ease-in-out',
					'border-base-200 border-r lg:static lg:w-64 lg:translate-x-0',
					layout.isSidebarOpen ? 'translate-x-0' : '-translate-x-full',
				)}>
				<div className="flex h-full flex-col">
					<Link
						href="/"
						target="_blank"
						rel="noopener noreferrer"
						className="flex h-16 items-center border-base-200 border-b px-6">
						<div className="flex items-center gap-3">
							<BrainCircuit />
							<div className="font-semibold text-base-content text-lg hover:cursor-pointer">
								<span className="[font-variant:unicase]">STARTUP</span>
								<span>.ai</span>
							</div>
						</div>
					</Link>

					<div className="border-base-200 border-b px-4 py-3">
						<OrgSelectorWrapper />
					</div>

					<div className="flex-1 overflow-y-auto px-4 py-4">
						<div className="space-y-6">
							<div>
								<div className="space-y-1">
									<NavItem href="/assistant" icon={Sparkles}>
										Assistant
									</NavItem>
								</div>
							</div>

							<div>
								<SectionHeader>Overview</SectionHeader>
								<div className="space-y-1">
									<NavItem href="/dashboard" icon={Home}>
										Dashboard
									</NavItem>
									<NavItem href="/agents" icon={BotMessageSquare}>
										Agents
									</NavItem>
									<NavItem href="/conversations" icon={Building2}>
										Conversations
									</NavItem>
									<NavItem href="/files" icon={FileText}>
										Files
									</NavItem>
									<NavItem href="#" icon={Folder}>
										Projects
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
										{can('billing', 'read') && (
											<NavItem href="/org/billing" icon={CreditCard}>
												Billing
											</NavItem>
										)}
									</div>
								</div>
							)}
						</div>
					</div>

					<div className="border-gray-200 border-t px-4 py-4 dark:border-[#1F1F23]">
						{user ? (
							<div className="space-y-1">
								<NavItem href="/settings" icon={Settings}>
									Settings
								</NavItem>
								<NavItem href="#" icon={HelpCircle}>
									Help
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
