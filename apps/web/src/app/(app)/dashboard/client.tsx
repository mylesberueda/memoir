'use client';

import { CheckoutModal } from '@components/Modal';
import { Bot, Brain, Clock, Cpu, FileText, MessageSquare, Users, Zap } from 'lucide-react';
import { Suspense } from 'react';

import {
	type ActivityItem,
	AgentPerformanceCard,
	type AgentPerformanceItem,
	AgentStatisticsCard,
	CreateAgentCard,
	RecentActivityCard,
	type StatItem,
} from './_components';

const STATS: StatItem[] = [
	{
		id: '1',
		title: 'Total Tokens Used',
		value: '1.2M',
		change: '+12.5%',
		trend: 'up',
		icon: Zap,
		color: 'text-amber-600 dark:text-amber-400',
	},
	{
		id: '2',
		title: 'Queries Processed',
		value: '8,459',
		change: '+23.1%',
		trend: 'up',
		icon: MessageSquare,
		color: 'text-blue-600 dark:text-blue-400',
	},
	{
		id: '3',
		title: 'Reports Generated',
		value: '1,245',
		change: '+5.3%',
		trend: 'up',
		icon: FileText,
		color: 'text-emerald-600 dark:text-emerald-400',
	},
	{
		id: '4',
		title: 'Avg. Response Time',
		value: '1.2s',
		change: '-8.7%',
		trend: 'down',
		icon: Clock,
		color: 'text-purple-600 dark:text-purple-400',
	},
	{
		id: '5',
		title: 'Active Agents',
		value: '12',
		change: '+2',
		trend: 'up',
		icon: Cpu,
		color: 'text-red-600 dark:text-red-400',
	},
	{
		id: '6',
		title: 'Active Users',
		value: '347',
		change: '+15.2%',
		trend: 'up',
		icon: Users,
		color: 'text-indigo-600 dark:text-indigo-400',
	},
];

const AGENTS: AgentPerformanceItem[] = [
	{
		id: '1',
		name: 'Customer Support',
		type: 'GPT-4',
		accuracy: 95,
		status: 'online',
		icon: Bot,
	},
	{
		id: '2',
		name: 'Data Analyzer',
		type: 'Claude 3',
		accuracy: 92,
		status: 'online',
		icon: Brain,
	},
	{
		id: '3',
		name: 'Content Generator',
		type: 'GPT-4',
		accuracy: 88,
		status: 'training',
		icon: Zap,
	},
	{
		id: '4',
		name: 'Research Assistant',
		type: 'Claude 3',
		accuracy: 90,
		status: 'offline',
		icon: Bot,
	},
];

const ACTIVITIES: ActivityItem[] = [
	{
		id: '1',
		agent: 'Customer Support',
		action: 'Answered query about product pricing',
		actionType: 'query',
		timestamp: '2 minutes ago',
		user: 'Sarah Johnson',
	},
	{
		id: '2',
		agent: 'Data Analyzer',
		action: 'Generated quarterly sales report',
		actionType: 'report',
		timestamp: '15 minutes ago',
		user: 'Michael Chen',
	},
	{
		id: '3',
		agent: 'Research Assistant',
		action: 'Performed market research on competitors',
		actionType: 'search',
		timestamp: '45 minutes ago',
		user: 'Alex Rodriguez',
	},
	{
		id: '4',
		agent: 'Content Generator',
		action: 'Created 5 social media posts',
		actionType: 'message',
		timestamp: '1 hour ago',
		user: 'Jamie Smith',
	},
	{
		id: '5',
		agent: 'Data Analyzer',
		action: 'Processed customer feedback data',
		actionType: 'data',
		timestamp: '2 hours ago',
	},
];

export default function DashboardClient() {
	return (
		<div className="flex flex-col gap-6 p-6">
			<Suspense fallback={null}>
				<CheckoutModal />
			</Suspense>
			<CreateAgentCard />
			<AgentStatisticsCard stats={STATS} />
			<AgentPerformanceCard agents={AGENTS} />
			<RecentActivityCard activities={ACTIVITIES} />
		</div>
	);
}
