'use client';

import Button from '@components/Button';
import type { Message } from '@lib/chat-state';
import type { ReactNode } from 'react';
import type { ChatMessageProps } from './Message';

export interface MessageStatusIndicatorProps {
	status: Message['status'];
	canRetry: boolean;
	onRetry: () => void;
}

export function MessageStatusIndicator({ status, canRetry, onRetry }: MessageStatusIndicatorProps): ReactNode {
	switch (status) {
		case 'sending':
			return (
				<span className="text-xs text-muted-foreground" data-testid="message-status-sending">
					<span className="loading loading-dots loading-xs" />
					Sending...
				</span>
			);
		case 'sent':
			return (
				<span className="text-xs text-green-600" data-testid="message-status-sent">
					Sent
				</span>
			);
		case 'failed':
			return (
				<div className="flex items-center gap-2">
					<span className="text-xs text-red-600" data-testid="message-error">
						Failed
					</span>
					{canRetry && (
						<Button
							type="button"
							onClick={onRetry}
							className="text-xs text-blue-600 hover:text-blue-800 underline"
							data-testid="retry-button">
							Retry
						</Button>
					)}
				</div>
			);
		case 'cancelled':
			return (
				<span className="text-xs text-amber-600" data-testid="message-status-cancelled">
					Cancelled
				</span>
			);
		default:
			return null;
	}
}

export interface TransformMessagesOptions {
	messages: Message[];
	assistantName: string;
	userAvatar?: string;
	canRetry: (msg: Message) => boolean;
	onRetry: (messageId: string) => void;
}

export type ChatMessage = ChatMessageProps;

export function transformMessagesToChatProps({
	messages,
	assistantName: _assistantName,
	userAvatar = 'https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp',
	canRetry,
	onRetry,
}: TransformMessagesOptions): ChatMessage[] {
	return messages.map((msg): ChatMessage => {
		const isUser = msg.role === 'user';
		const msgCanRetry = canRetry(msg);

		const statusIndicator = (
			<MessageStatusIndicator status={msg.status} canRetry={msgCanRetry} onRetry={() => onRetry(msg.id)} />
		);

		if (isUser) {
			return {
				id: msg.id,
				variant: 'rx' as const,
				timestamp: msg.timestamp,
				avatar: userAvatar,
				footer: statusIndicator,
				parts: msg.parts,
				status: msg.status,
				attachments: msg.attachments,
			};
		}

		return {
			id: msg.id,
			variant: 'tx' as const,
			timestamp: msg.timestamp,
			footer: statusIndicator,
			parts: msg.parts,
			thinkingDuration: msg.thinkingDuration,
			canRetry: msgCanRetry,
			status: msg.status,
		};
	});
}
