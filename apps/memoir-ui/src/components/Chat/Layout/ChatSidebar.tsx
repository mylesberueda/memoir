'use client';

import ResponsiveDock, { type DockAction, type DockViewItem } from '@components/ResponsiveDock';
import { Clock, Files, Plus, Settings, Share2 } from 'lucide-react';
import { type ReactNode, useState } from 'react';
import ConversationList, { type ConversationListItem } from './ConversationList';
import FilesList from './FilesList';

interface ChatSidebarProps {
	title?: string;
	conversations: ConversationListItem[];
	selectedConversation?: ConversationListItem | null;
	onSelectConversation: (conversation: ConversationListItem) => void;
	onNewConversation: () => void;
	onDeleteConversation: (conversationId: string) => void;
	onEditAgent?: () => void;
	onShareAgent?: () => void;
	children: ReactNode;
}

type ViewId = 'history' | 'files';

export default function ChatSidebar({
	title = 'History',
	conversations,
	selectedConversation,
	onSelectConversation,
	onNewConversation,
	onDeleteConversation,
	onEditAgent,
	onShareAgent,
	children,
}: ChatSidebarProps) {
	const [activeViewId, setActiveViewId] = useState<ViewId>('history');

	const actions: DockAction[] = [
		{
			id: 'new',
			icon: Plus,
			label: 'New Chat',
			onClick: onNewConversation,
		},
		...(onShareAgent
			? [
					{
						id: 'share',
						icon: Share2,
						label: 'Share',
						onClick: onShareAgent,
					},
				]
			: []),
		...(onEditAgent
			? [
					{
						id: 'edit',
						icon: Settings,
						label: 'Edit Agent',
						onClick: onEditAgent,
					},
				]
			: []),
	];

	const views: DockViewItem[] = [
		{
			id: 'history',
			icon: Clock,
			label: 'History',
			component: (
				<ConversationList
					conversations={conversations}
					selectedConversation={selectedConversation}
					onSelectConversation={onSelectConversation}
					onDeleteConversation={onDeleteConversation}
				/>
			),
		},
		{
			id: 'files',
			icon: Files,
			label: 'Files',
			component: <FilesList conversationId={selectedConversation?.id} />,
		},
	];

	return (
		<ResponsiveDock
			title={title}
			actions={actions}
			views={views}
			activeViewId={activeViewId}
			onViewChange={(id) => setActiveViewId(id as ViewId)}>
			{children}
		</ResponsiveDock>
	);
}
