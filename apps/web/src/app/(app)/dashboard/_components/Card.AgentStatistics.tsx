'use client';

import { Card } from '@components';
import cn from 'classnames';
import { BarChart2, Clock, Cpu, FileText, MessageSquare, Users, Zap } from 'lucide-react';
import type React from 'react';

export interface StatItem {
	id: string;
	title: string;
	value: string;
	change: string;
	trend: 'up' | 'down' | 'neutral';
	icon: React.ElementType;
	color: string;
}

const DEFAULT_STATS: StatItem[] = [
	{
		id: '1',
		title: 'Total Tokens Used',
		value: '0',
		change: '0%',
		trend: 'neutral',
		icon: Zap,
		color: 'text-amber-600 dark:text-amber-400',
	},
	{
		id: '2',
		title: 'Queries Processed',
		value: '0',
		change: '0%',
		trend: 'neutral',
		icon: MessageSquare,
		color: 'text-blue-600 dark:text-blue-400',
	},
	{
		id: '3',
		title: 'Reports Generated',
		value: '0',
		change: '0%',
		trend: 'neutral',
		icon: FileText,
		color: 'text-emerald-600 dark:text-emerald-400',
	},
	{
		id: '4',
		title: 'Avg. Response Time',
		value: '0',
		change: '0%',
		trend: 'neutral',
		icon: Clock,
		color: 'text-purple-600 dark:text-purple-400',
	},
	{
		id: '5',
		title: 'Active Agents',
		value: '0',
		change: '0',
		trend: 'neutral',
		icon: Cpu,
		color: 'text-red-600 dark:text-red-400',
	},
	{
		id: '6',
		title: 'Active Users',
		value: '0',
		change: '0%',
		trend: 'neutral',
		icon: Users,
		color: 'text-indigo-600 dark:text-indigo-400',
	},
];

export interface AgentStatisticsCardProps {
	stats?: StatItem[];
}

export default function AgentStatisticsCard({ stats = DEFAULT_STATS }: AgentStatisticsCardProps) {
	return (
		<Card icon={BarChart2} title="Agent Statistics" description="Your agent's stats, at your fingertips">
			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-3">
				{stats.map(({ icon: Icon, ...stat }) => (
					<div
						key={stat.id}
						className="bg-base-300 border border-base-100 rounded-box p-3 flex flex-col justify-between">
						<div className="flex items-center gap-2 mb-2">
							<div className={cn('p-1.5 rounded-md bg-base-100', stat.color)}>
								<Icon className="w-3.5 h-3.5" />
							</div>
							<span className="text-xs font-medium text-base-content line-clamp-1">{stat.title}</span>
						</div>
						<div className="flex items-end justify-between">
							<span className="text-lg sm:text-xl font-semibold text-base-content">{stat.value}</span>
							<span
								className={cn('text-xs font-medium', {
									'text-success': stat.trend === 'up',
									'text-error': stat.trend === 'down',
									'text-base-content': stat.trend === 'neutral',
								})}>
								{stat.change}
							</span>
						</div>
					</div>
				))}
			</div>
		</Card>
	);
}
