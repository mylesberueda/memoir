import { getCurrentUser } from '@actions/auth';
import { ArrowUpRight, Clock, GitBranch, History, MessageSquare, Search } from 'lucide-react';
import type { Metadata } from 'next';
import Link from 'next/link';

export const metadata: Metadata = {
	title: 'Dashboard | Memoir',
};

const SURFACES = [
	{
		href: '/playground',
		icon: MessageSquare,
		eyebrow: 'Playground',
		title: 'Chat',
		body: 'Talk to an agent backed by memoir. Each turn reads context and records the conversation.',
	},
	{
		href: '/memory/timeline',
		icon: Clock,
		eyebrow: 'Memory',
		title: 'Timeline',
		body: 'The chronological event-log for a scope. Superseded rows stay as an audit trail.',
	},
	{
		href: '/memory/query',
		icon: Search,
		eyebrow: 'Memory',
		title: 'Query',
		body: 'Hybrid-ranked retrieval across a scope. Inspect what an agent would recall.',
	},
	{
		href: '/memory/as-of',
		icon: History,
		eyebrow: 'Memory',
		title: 'Point-in-time',
		body: 'Reconstruct what was known at a past instant. Time-travel the memory store.',
	},
	{
		href: '/memory/audit',
		icon: GitBranch,
		eyebrow: 'Memory',
		title: 'Audit',
		body: 'Trace supersession events — how a fact was edited, replaced, or revised over time.',
	},
];

export default async function DashboardPage() {
	const user = await getCurrentUser().catch(() => null);

	return (
		<div id="dashboard__container" className="mx-auto max-w-6xl px-4 py-10 sm:px-6 lg:px-8">
			<header id="dashboard__masthead" className="mb-10 border-base-300 border-b pb-8">
				<p className="mb-3 font-medium text-[0.6875rem] text-primary uppercase tracking-[0.16em]">The Archive</p>
				<h1 className="font-display text-4xl text-base-content sm:text-5xl">Dashboard</h1>
				<p className="mt-3 max-w-xl text-base-content/60 leading-relaxed">
					A self-hosted memory service for AI agents. Seed it from the playground, then inspect what gets remembered
					across every read surface below.
				</p>
				<div id="dashboard__session" className="mt-5 inline-flex items-center gap-2 text-sm">
					<span className="inline-block h-1.5 w-1.5 rounded-full bg-success" />
					<span className="text-base-content/50">Signed in as</span>
					<span className="font-mono text-base-content/80">{user?.id ?? 'unknown'}</span>
				</div>
			</header>

			<ul
				id="dashboard__surfaces"
				className="grid gap-px overflow-hidden rounded-box border border-base-300 bg-base-300 sm:grid-cols-2 lg:grid-cols-3">
				{SURFACES.map((surface) => {
					const Icon = surface.icon;
					return (
						<li key={surface.href} className="bg-base-100">
							<Link
								href={surface.href}
								className="group flex h-full flex-col p-6 transition-colors hover:bg-base-200/50">
								<div className="mb-4 flex items-start justify-between">
									<span className="flex h-10 w-10 items-center justify-center rounded-field bg-primary/10 text-primary">
										<Icon className="h-5 w-5" />
									</span>
									<ArrowUpRight className="h-4 w-4 text-base-content/30 transition-all group-hover:translate-x-0.5 group-hover:-translate-y-0.5 group-hover:text-primary" />
								</div>
								<p className="mb-1 font-medium text-[0.6875rem] text-base-content/40 uppercase tracking-[0.14em]">
									{surface.eyebrow}
								</p>
								<h2 className="font-display font-semibold text-xl text-base-content">{surface.title}</h2>
								<p className="mt-2 text-sm text-base-content/55 leading-relaxed">{surface.body}</p>
							</Link>
						</li>
					);
				})}
			</ul>
		</div>
	);
}
