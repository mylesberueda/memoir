'use client';

import type { RefObject } from 'react';
import type { ChatProps } from '../Chat';
import Chat from '../Chat';
import type { PromptInputRef } from '../PromptInput';
import ChatSidebar from './ChatSidebar';
import type { ConversationListItem } from './ConversationList';

export interface ChatLayoutProps {
	chatRef?: RefObject<PromptInputRef | null>;
	chatProps: ChatProps;
	error?: string | null;
	sidebar: {
		title: string;
		conversations: ConversationListItem[];
		selectedConversation?: ConversationListItem | null;
		onNewConversation: () => void;
		onSelectConversation: (conversation: ConversationListItem) => void;
		onDeleteConversation: (conversationId: string) => void;
		onEditAgent?: () => void;
		onShareAgent?: () => void;
	};
}

export default function ChatLayout({ chatRef, chatProps, error, sidebar }: ChatLayoutProps) {
	return (
		<ChatSidebar
			title={sidebar.title}
			conversations={sidebar.conversations}
			selectedConversation={sidebar.selectedConversation}
			onNewConversation={sidebar.onNewConversation}
			onSelectConversation={sidebar.onSelectConversation}
			onDeleteConversation={sidebar.onDeleteConversation}
			onEditAgent={sidebar.onEditAgent}
			onShareAgent={sidebar.onShareAgent}>
			<div id="chat_layout__container" className="flex h-full flex-col">
				{error && (
					<div id="chat_layout__error" className="flex-shrink-0 p-4">
						<div className="alert alert-error">
							<span>{error}</span>
						</div>
					</div>
				)}
				<div id="chat_layout__chat" className="min-h-0 flex-1 p-4">
					<Chat ref={chatRef} {...chatProps} />
				</div>
			</div>
		</ChatSidebar>
	);
}
