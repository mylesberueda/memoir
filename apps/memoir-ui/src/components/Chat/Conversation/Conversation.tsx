'use client';

import { getTextContent } from '@lib/chat-state';
import { cn } from '@lib/utils';
import { useCallback, useEffect, useRef, useState } from 'react';

import { AgentMessage, type ChatMessageProps, UserMessage } from '../Message';

export interface ConversationProps {
	messages: ChatMessageProps[];
	isLoading?: boolean;
	isStreaming?: boolean;
	className?: string;
	onRetry?: (messageId: string) => void;
	onCopy?: (messageId: string, content: string) => void;
	onFeedback?: (messageId: string, type: 'like' | 'dislike') => void;
	onRetryAttachment?: (messageId: string, attachmentId: string) => void;
	onDeleteAttachment?: (messageId: string, attachmentId: string) => void;
}

export default function Conversation({
	messages = [],
	isLoading = false,
	isStreaming = false,
	className,
	onRetry,
	onCopy,
	onFeedback,
	onRetryAttachment,
	onDeleteAttachment,
}: ConversationProps) {
	const scrollContainerRef = useRef<HTMLDivElement>(null);
	const [isAtBottom, setIsAtBottom] = useState(true);

	// Scroll to bottom when conversation loads initially
	useEffect(() => {
		if (messages.length > 0 && scrollContainerRef.current) {
			scrollContainerRef.current.scrollTop = scrollContainerRef.current.scrollHeight;
			setIsAtBottom(true);
		}
	}, [messages.length]); // Only trigger when messages first load

	// Keep scroll at bottom during responding/streaming
	useEffect(() => {
		if ((isStreaming || isLoading) && isAtBottom && scrollContainerRef.current) {
			// Auto-scroll when content changes during streaming
			const observer = new MutationObserver(() => {
				if (scrollContainerRef.current && isAtBottom) {
					scrollContainerRef.current.scrollTop = scrollContainerRef.current.scrollHeight;
				}
			});

			observer.observe(scrollContainerRef.current, {
				childList: true,
				subtree: true,
				characterData: true,
			});

			return () => observer.disconnect();
		}
		return undefined;
	}, [isStreaming, isLoading, isAtBottom]);

	const handleScroll = useCallback(() => {
		if (!scrollContainerRef.current) return;

		const { scrollTop: currentScrollTop, scrollHeight, clientHeight } = scrollContainerRef.current;
		const atBottom = scrollHeight - currentScrollTop - clientHeight < 50;
		setIsAtBottom(atBottom);
	}, []);

	const scrollToBottom = useCallback(() => {
		if (!scrollContainerRef.current) return;
		scrollContainerRef.current.scrollTop = scrollContainerRef.current.scrollHeight;
		setIsAtBottom(true);
	}, []);

	return (
		<div
			ref={scrollContainerRef}
			className={cn('relative flex-1 overflow-y-auto overflow-x-hidden h-full', className)}
			onScroll={handleScroll}
			data-testid="conversation-container"
			role="log">
			<div className="p-4 space-y-2">
				{messages.map((message, index) => {
					const isLastMessage = index === messages.length - 1;
					const isAssistantMessage = message.variant === 'tx';
					const messageIsStreaming = isLastMessage && isAssistantMessage && isStreaming;

					return (
						<div key={`${message.variant}-${message.timestamp.toString()}-${index}`} data-testid="chat-message">
							{message.variant === 'tx' ? (
								<AgentMessage
									{...message}
									isStreaming={messageIsStreaming}
									onRetry={message.canRetry && onRetry ? () => onRetry(message.id) : undefined}
									onCopy={onCopy ? () => onCopy(message.id, getTextContent(message.parts)) : undefined}
									onFeedback={onFeedback ? (type: 'like' | 'dislike') => onFeedback(message.id, type) : undefined}
								/>
							) : (
								<UserMessage
									{...message}
									onRetryAttachment={
										onRetryAttachment ? (attId: string) => onRetryAttachment(message.id, attId) : undefined
									}
									onDeleteAttachment={
										onDeleteAttachment ? (attId: string) => onDeleteAttachment(message.id, attId) : undefined
									}
								/>
							)}
						</div>
					);
				})}
				{messages.length === 0 && !isLoading && (
					<div className="flex h-full items-center justify-center text-center">
						<div className="space-y-2">
							<p className="text-lg font-medium text-muted-foreground">Start a conversation</p>
							<p className="text-sm text-muted-foreground">Send a message to begin chatting with the assistant</p>
						</div>
					</div>
				)}
			</div>
			{!isAtBottom && (
				<button
					type="button"
					onClick={scrollToBottom}
					className="absolute bottom-4 right-4 bg-primary text-primary-content rounded-full p-3 shadow-lg hover:bg-primary-focus transition-colors"
					data-testid="scroll-to-bottom-button"
					aria-label="Scroll to bottom">
					<svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<title>Scroll to bottom</title>
						<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
					</svg>
				</button>
			)}
		</div>
	);
}
