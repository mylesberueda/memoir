import type { ChatStateActions } from '@lib/chat-state';
import { forwardRef, useState } from 'react';
import Conversation, { type ConversationProps } from './Conversation';
import { ExportDialog } from './ExportDialog';
import PromptInput, { type PromptInputProps, type PromptInputRef } from './PromptInput';

export interface ChatProps extends PromptInputProps {
	chat?: ChatStateActions;
	messages?: ConversationProps['messages'];
	showLoading?: boolean;
	isLoading?: boolean;
	isStreaming?: boolean;
	onStopStreaming?: () => void;
	onRetry?: (messageId: string) => void;
	onCopy?: (messageId: string, content: string) => void;
	onFeedback?: (messageId: string, type: 'like' | 'dislike') => void;
	onRetryAttachment?: (messageId: string, attachmentId: string) => void;
	onDeleteAttachment?: (messageId: string, attachmentId: string) => void;
}

export default forwardRef<PromptInputRef, ChatProps>(function Chat(
	{
		chat,
		messages = [],
		showLoading = false,
		isLoading = false,
		isStreaming = false,
		onStopStreaming,
		onRetry,
		onCopy,
		onFeedback,
		onRetryAttachment,
		onDeleteAttachment,
		...promptBoxProps
	},
	ref,
) {
	const [showExportDialog, setShowExportDialog] = useState(false);

	return (
		<div id="chat" className="flex h-full flex-col" data-testid="conversation-history">
			<div id="chat__message-log" className="flex-1 min-h-0 relative">
				<div className="absolute inset-0">
					<Conversation
						messages={messages}
						isLoading={showLoading && isLoading}
						isStreaming={showLoading && isStreaming}
						onRetry={onRetry}
						onCopy={onCopy}
						onFeedback={onFeedback}
						onRetryAttachment={onRetryAttachment}
						onDeleteAttachment={onDeleteAttachment}
					/>
				</div>
			</div>
			<div id="chat__prompt-box" className="flex-shrink-0 w-full">
				<PromptInput
					ref={ref}
					disabled={promptBoxProps.disabled}
					isLoading={isLoading}
					isStreaming={isStreaming}
					onStopStreaming={onStopStreaming}
					enableMic={true}
					{...promptBoxProps}
				/>
			</div>
			{chat && <ExportDialog isOpen={showExportDialog} onClose={() => setShowExportDialog(false)} chat={chat} />}
		</div>
	);
});
