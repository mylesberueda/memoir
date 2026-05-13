'use client';

import { Button, Card } from '@components';
import { ArrowRight, Bot, Database, FileText, MessageSquare, Search, User } from 'lucide-react';

export interface ActivityItem {
	id: string;
	agent: string;
	action: string;
	actionType: 'query' | 'report' | 'search' | 'data' | 'message';
	timestamp: string;
	user?: string;
}

const ACTION_ICONS = {
	query: <MessageSquare className="w-3.5 h-3.5" />,
	report: <FileText className="w-3.5 h-3.5" />,
	search: <Search className="w-3.5 h-3.5" />,
	data: <Database className="w-3.5 h-3.5" />,
	message: <MessageSquare className="w-3.5 h-3.5" />,
};

interface RecentActivityCardProps {
	activities?: ActivityItem[];
}

export default function RecentActivityCard({ activities = [] }: RecentActivityCardProps) {
	return (
		<Card icon={MessageSquare} title="Recent Agent Activity" description="What your agents have been up to">
			<div className="space-y-4">
				<div className="space-y-1">
					{activities.length > 0 ? <ItemList activities={activities} /> : <NoActivitiesFound />}
				</div>

				<div className="pt-2">
					{activities.length > 0 ? (
						<Button outline color="primary" className="w-full text-xs font-medium">
							View All Activity
							<ArrowRight className="w-3.5 h-3.5 ml-2" />
						</Button>
					) : (
						<Button outline color="primary" className="w-full text-xs font-medium">
							Configure your agents
							<ArrowRight className="w-3.5 h-3.5 ml-2" />
						</Button>
					)}
				</div>
			</div>
		</Card>
	);
}

function Item(item: ActivityItem) {
	return (
		<div className="group rounded-box flex items-start sm:items-center gap-3 p-3 hover:bg-base-200 transition-all duration-200">
			<div className="p-2 rounded-box bg-base-300 border border-base-100 flex-shrink-0 mt-0.5 sm:mt-0">
				<Bot className="w-4 h-4 text-base-content" />
			</div>
			<div className="flex-1 min-w-0">
				<div className="flex flex-wrap items-center gap-2">
					<h3 className="text-sm font-medium text-base-content truncate">{item.agent}</h3>
					<div className="px-2 py-1 rounded-box bg-base-300 border border-base-100 flex items-center gap-1 flex-shrink-0">
						{ACTION_ICONS[item.actionType]}
						<span className="text-xs text-muted">
							{item.actionType.charAt(0).toUpperCase() + item.actionType.slice(1)}
						</span>
					</div>
				</div>
				<p className="text-xs text-muted mt-1 line-clamp-2 sm:line-clamp-1">{item.action}</p>
				<div className="flex flex-wrap items-center gap-2 mt-1">
					<span className="text-xs text-muted">{item.timestamp}</span>
					{item.user && (
						<>
							<span className="text-xs text-muted hidden sm:inline">•</span>
							<div className="flex items-center gap-1 text-xs text-muted">
								<User className="w-3 h-3" />
								<span className="truncate max-w-[100px] sm:max-w-none">{item.user}</span>
							</div>
						</>
					)}
				</div>
			</div>
		</div>
	);
}

interface ActivityListProps {
	activities: ActivityItem[];
}

function ItemList({ activities }: ActivityListProps) {
	return activities.map((activity) => <Item key={activity.id} {...activity} />);
}

function NoActivitiesFound() {
	return (
		<div className="flex flex-col justify-center items-center my-16">
			<Bot className="w-32 h-32" />
			<span>No recent activity</span>
		</div>
	);
}
