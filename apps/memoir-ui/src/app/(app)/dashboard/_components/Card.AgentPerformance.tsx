import { Card } from '@components';
import cn from 'classnames';
import { Activity, AlertCircle, Bot, CheckCircle2, XCircle } from 'lucide-react';
import type React from 'react';

export interface AgentPerformanceItem {
	id: string;
	name: string;
	type: string;
	accuracy: number;
	status: 'online' | 'offline' | 'training';
	icon: React.ElementType;
}

const STATUS_ICONS = {
	online: <CheckCircle2 className="h-3.5 w-3.5 text-emerald-600 dark:text-emerald-400" />,
	offline: <XCircle className="h-3.5 w-3.5 text-red-600 dark:text-red-400" />,
	training: <AlertCircle className="h-3.5 w-3.5 text-amber-600 dark:text-amber-400" />,
};

const STATUS_TEXT = {
	online: 'Online',
	offline: 'Offline',
	training: 'Training',
};

export interface AgentPerformanceCardProps {
	agents: AgentPerformanceItem[];
}

export default function AgentPerformanceCard({ agents = [] }: AgentPerformanceCardProps) {
	return (
		<Card icon={Activity} title="Agent Performance" description="Your team's performance">
			{agents.length > 0 ? (
				<div className="grid grid-cols-1 gap-3 md:grid-cols-2 xl:grid-cols-4">
					<ItemList agents={agents} />
				</div>
			) : (
				<NonAgentsFound />
			)}
		</Card>
	);
}

function Item(item: AgentPerformanceItem) {
	return (
		<div key={item.id} className="rounded-box border border-base-200 bg-base-300 p-3">
			<div className="mb-3 flex items-center justify-between">
				<div className="flex min-w-0 items-center gap-2">
					<div className="flex-shrink-0 rounded-md bg-base-200 p-1.5">
						<item.icon className="h-3.5 w-3.5 text-base-content" />
					</div>
					<div className="min-w-0 self-start">
						<h3 className="truncate font-medium text-base-content text-sm">{item.name}</h3>
						<p className="text-base-content text-xs">{item.type}</p>
					</div>
				</div>
				<div className="ml-2 flex flex-shrink-0 items-center gap-1.5 self-start">
					{STATUS_ICONS[item.status]}
					<span
						className={cn('font-medium text-xs', {
							'text-success': item.status === 'online',
							'text-error': item.status === 'offline',
							'text-base-content': item.status === 'training',
						})}>
						{STATUS_TEXT[item.status]}
					</span>
				</div>
			</div>
			<div className="space-y-1.5">
				<div className="flex items-center justify-between text-xs">
					<span className="text-base-content">Accuracy</span>
					<span className="text-base-content">{item.accuracy}%</span>
				</div>
				<div className="h-1.5 overflow-hidden rounded-full bg-base-100">
					<div
						className={cn('h-full rounded-full', {
							'bg-success': item.accuracy >= 90,
							'bg-warning': item.accuracy >= 80 && item.accuracy < 90,
							'bg-error': item.accuracy < 80,
						})}
						style={{ width: `${item.accuracy}%` }}
					/>
				</div>
			</div>
		</div>
	);
}

interface AgentPerformanceListProps {
	agents: AgentPerformanceItem[];
}

function ItemList({ agents }: AgentPerformanceListProps) {
	return agents.map((agent) => <Item key={agent.id} {...agent} />);
}

function NonAgentsFound() {
	return (
		<div className="my-16 flex flex-col items-center justify-center">
			<Bot className="h-32 w-32" />
			<span>No agents found</span>
		</div>
	);
}
