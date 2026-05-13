'use client';

import type { ChatProps } from '@components/Chat';
import cns from 'classnames';
import { format, isThisWeek, isToday, isYesterday } from 'date-fns';
import { MoreHorizontal, Trash2 } from 'lucide-react';
import { Dropdown } from 'rsc-daisyui';

export interface ConversationListItem {
	id: string;
	title: string;
	lastMessage: string;
	timestamp: Date;
	messages: ChatProps['messages'];
}

interface ConversationListProps {
	conversations: ConversationListItem[];
	selectedConversation?: ConversationListItem | null;
	onSelectConversation: (conversation: ConversationListItem) => void;
	onDeleteConversation: (conversationId: string) => void;
}

function groupConversationsByDate(conversations: ConversationListItem[]) {
	const groups: { [key: string]: ConversationListItem[] } = {
		Today: [],
		Yesterday: [],
		'This Week': [],
		Older: [],
	};

	for (const conversation of conversations) {
		const date = new Date(conversation.timestamp);
		if (isToday(date)) {
			groups.Today.push(conversation);
		} else if (isYesterday(date)) {
			groups.Yesterday.push(conversation);
		} else if (isThisWeek(date)) {
			groups['This Week'].push(conversation);
		} else {
			groups.Older.push(conversation);
		}
	}

	return groups;
}

export default function ConversationList({
	conversations,
	selectedConversation,
	onSelectConversation,
	onDeleteConversation,
}: ConversationListProps) {
	const groupedConversations = groupConversationsByDate(conversations);

	return (
		<div id="conversation_list__container" className="p-2">
			{Object.entries(groupedConversations).map(([group, conversations]) => {
				if (conversations.length === 0) return null;

				return (
					<div key={group} className="mb-4">
						<h3 className="mb-2 px-2 text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
							{group}
						</h3>
						<div className="space-y-2">
							{conversations.map((conversation) => (
								// biome-ignore lint/a11y/noStaticElementInteractions: conversation div
								<div
									key={conversation.id}
									className={cns(
										'group relative cursor-pointer rounded-box border p-4 transition-colors',
										'hover:border-primary hover:bg-primary/10',
										selectedConversation?.id === conversation.id ? 'border-primary bg-base-300' : 'border-transparent',
									)}
									onClick={() => onSelectConversation(conversation)}
									onKeyDown={(e) => {
										if (e.code === 'Enter') {
											onSelectConversation(conversation);
										}
									}}>
									<Dropdown className="absolute right-4" align="end">
										<Dropdown.Button size="xs" ghost className="hover:bg-transparent">
											<MoreHorizontal size={14} suppressHydrationWarning />
										</Dropdown.Button>
										<Dropdown.Menu id="conversation-toggle">
											<Dropdown.Item className="text-error" onClick={() => onDeleteConversation(conversation.id)}>
												<Trash2 size={14} suppressHydrationWarning />
												<span>Delete</span>
											</Dropdown.Item>
										</Dropdown.Menu>
									</Dropdown>
									<div className="flex items-start justify-between">
										<div className="min-w-0 flex-1">
											<h4
												className={cns(
													'truncate text-sm font-medium',
													selectedConversation?.id === conversation.id ? 'text-primary' : 'text-base-content',
												)}>
												{conversation.title}
											</h4>
											<p className="mt-1 truncate text-xs text-base-content">{conversation.lastMessage}</p>
											<p className="mt-1 text-xs text-base-content/60">
												{format(conversation.timestamp, 'MMM d, h:mm a')}
											</p>
										</div>
									</div>
								</div>
							))}
						</div>
					</div>
				);
			})}

			{conversations.length === 0 && (
				<div className="py-8 text-center text-sm text-base-content/60">No conversations yet</div>
			)}
		</div>
	);
}
