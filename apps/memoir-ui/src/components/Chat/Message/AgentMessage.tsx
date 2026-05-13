'use client';

import { getTextContent, type MessagePart, MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { streamingStore } from '@lib/streaming-store';
import { cn } from '@lib/utils';
import { formatDistanceToNow } from 'date-fns';
import {
	BanIcon,
	BrainCircuitIcon,
	CopyIcon,
	MessageSquareIcon,
	RefreshCwIcon,
	ThumbsDownIcon,
	ThumbsUpIcon,
	WrenchIcon,
} from 'lucide-react';
import { useCallback, useMemo, useSyncExternalStore } from 'react';
import { Action, Actions } from '../Actions';
import ChainOfThought from '../ChainOfThought';
import Reasoning from '../Reasoning';
import Response from '../Response';
import Tool, { ToolInput, ToolOutput } from '../Tool';

export interface AgentMessageProps {
	id: string;
	variant: 'tx';
	timestamp: Date;
	footer?: React.ReactNode;
	thinkingDuration?: number;
	parts: MessagePart[];
	canRetry?: boolean;
	isStreaming?: boolean;
	status?: 'sending' | 'sent' | 'failed' | 'processing' | 'complete' | 'cancelled';
	onRetry?: () => void;
	onCopy?: () => void;
	onFeedback?: (type: 'like' | 'dislike') => void;
}

export default function AgentMessage({
	timestamp: timestamp_,
	footer,
	thinkingDuration,
	parts: propParts,
	isStreaming,
	status,
	onRetry,
	onCopy,
	onFeedback,
}: AgentMessageProps) {
	const streamingSnapshot = useSyncExternalStore(
		streamingStore.subscribe,
		streamingStore.getSnapshot,
		streamingStore.getSnapshot, // SSR: streaming never active server-side
	);
	const parts = isStreaming && streamingStore.isActive() ? streamingSnapshot.parts : propParts;

	const timestamp = useMemo(() => {
		return formatDistanceToNow(timestamp_, { addSuffix: true });
	}, [timestamp_]);

	const handleCopy = useCallback(() => {
		if (onCopy) {
			onCopy();
		} else {
			const textContent = getTextContent(parts);
			if (textContent) {
				navigator.clipboard.writeText(textContent);
			}
		}
	}, [onCopy, parts]);

	const toolResultsMap = useMemo(() => {
		if (parts.length === 0) return new Map();

		const map = new Map();
		for (const part of parts) {
			if (part.kind === MessagePartKind.TOOL_RESULT && part.toolResult) {
				map.set(part.toolResult.toolCallId, part);
			}
		}
		return map;
	}, [parts]);

	const metadata = useMemo(() => {
		const metaPart = parts.find((p) => p.kind === MessagePartKind.METADATA);
		if (!metaPart?.content) return null;
		try {
			return JSON.parse(metaPart.content) as {
				agentName?: string;
				modelId?: string;
				agent_name?: string;
				model_id?: string;
			};
		} catch {
			return null;
		}
	}, [parts]);

	const renderableParts = useMemo(() => {
		return parts.filter((p) => p.kind !== MessagePartKind.TOOL_RESULT && p.kind !== MessagePartKind.METADATA);
	}, [parts]);

	const lastTextPartIndex = useMemo(() => {
		if (renderableParts.length === 0) return -1;
		return (
			renderableParts
				.map((p, i) => ({ type: p.kind, index: i }))
				.filter((p) => p.type === MessagePartKind.TEXT)
				.pop()?.index ?? -1
		);
	}, [renderableParts]);

	const useTimeline = useMemo(() => {
		const kinds = new Set(renderableParts.map((p) => p.kind));
		return kinds.size > 1;
	}, [renderableParts]);

	const renderPartContent = (part: MessagePart, i: number) => {
		switch (part.kind) {
			case MessagePartKind.THINKING: {
				const isThisPartStreaming = part.status !== MessagePartStatus.COMPLETE;
				if (useTimeline) {
					return (
						<Reasoning
							key={`thinking-${part.id}`}
							content={part.content || ''}
							thinkingDuration={thinkingDuration}
							defaultOpen={isThisPartStreaming}
							variant="inline"
						/>
					);
				}
				return (
					<div key={`thinking-${part.id}`} className="w-full">
						<Reasoning
							content={part.content || ''}
							thinkingDuration={thinkingDuration}
							defaultOpen={isThisPartStreaming}
						/>
					</div>
				);
			}

			case MessagePartKind.TOOL_CALL: {
				const result = toolResultsMap.get(part.id);
				const finalStatus = result?.status || part.status;
				const toolCall = part.toolCall;

				const toolContent = (
					<Tool
						className="bg-base-100"
						type={toolCall?.name || 'Unknown Tool'}
						state={
							finalStatus === MessagePartStatus.STREAMING
								? 'input-streaming'
								: finalStatus === MessagePartStatus.COMPLETE
									? 'output-available'
									: 'output-error'
						}>
						{toolCall?.arguments && <ToolInput input={toolCall.arguments} />}
						{result?.toolResult?.result && (
							<ToolOutput
								output={result.toolResult.result}
								errorText={finalStatus === MessagePartStatus.FAILED ? 'Tool execution failed' : undefined}
							/>
						)}
					</Tool>
				);

				if (useTimeline) {
					return toolContent;
				}
				return (
					<div key={`tool-${part.id}`} className="w-full">
						{toolContent}
					</div>
				);
			}

			case MessagePartKind.TEXT: {
				if (!part.content?.trim()) return null;

				const isLastTextPart = i === lastTextPartIndex;

				if (useTimeline) {
					return (
						<div className="w-full min-w-0">
							<div
								className={cn(
									'flex flex-col gap-2 rounded-lg px-4 py-3 text-foreground text-sm',
									'bg-secondary text-secondary-content',
								)}>
								<div className="min-w-0">
									<Response className="text-primary-content">{part.content}</Response>
								</div>
							</div>
						</div>
					);
				}

				return (
					<div key={`text-${part.id}`} className="w-full min-w-0">
						<div
							className={cn(
								'flex flex-col gap-2 rounded-lg px-4 py-3 text-foreground text-sm',
								'bg-secondary text-secondary-content',
							)}>
							<div className="min-w-0">
								<Response className="text-primary-content">{part.content}</Response>
							</div>

							{isLastTextPart && (
								<>
									<div className="mt-2 text-xs text-muted-foreground">{timestamp}</div>
									<Actions className="mt-2">
										<Action tooltip="Copy" color="secondary" onClick={handleCopy}>
											<CopyIcon className="size-4" />
										</Action>
										{onRetry && (
											<Action tooltip="Retry" color="secondary" onClick={onRetry}>
												<RefreshCwIcon className="size-4" />
											</Action>
										)}
										{onFeedback && (
											<>
												<Action tooltip="Like" color="secondary" onClick={() => onFeedback('like')}>
													<ThumbsUpIcon className="size-4" />
												</Action>
												<Action tooltip="Dislike" color="secondary" onClick={() => onFeedback('dislike')}>
													<ThumbsDownIcon className="size-4" />
												</Action>
											</>
										)}
									</Actions>
								</>
							)}
						</div>
					</div>
				);
			}

			case MessagePartKind.TOOL_RESULT:
				return null;

			default:
				return null;
		}
	};

	const getStepIcon = (part: MessagePart) => {
		switch (part.kind) {
			case MessagePartKind.THINKING:
				return <BrainCircuitIcon />;
			case MessagePartKind.TOOL_CALL:
				return <WrenchIcon />;
			case MessagePartKind.TEXT:
				return <MessageSquareIcon />;
			default:
				return <MessageSquareIcon />;
		}
	};

	const getStepDotClassName = (part: MessagePart): string | undefined => {
		if (part.kind === MessagePartKind.TEXT) {
			return 'bg-secondary text-secondary-content border-secondary';
		}
		return undefined;
	};

	const getStepStatus = (part: MessagePart): 'streaming' | 'complete' => {
		return part.status === MessagePartStatus.COMPLETE ? 'complete' : 'streaming';
	};

	return (
		<div className="group is-assistant w-full py-3">
			<div className="flex flex-col gap-2 max-w-[80%] min-w-0">
				{useTimeline ? (
					<>
						<ChainOfThought>
							{renderableParts.map((part, i) => {
								const content = renderPartContent(part, i);
								if (content === null) return null;
								return (
									<ChainOfThought.Step
										key={`step-${part.id}`}
										icon={getStepIcon(part)}
										status={getStepStatus(part)}
										dotClassName={getStepDotClassName(part)}>
										{content}
									</ChainOfThought.Step>
								);
							})}
						</ChainOfThought>
						{lastTextPartIndex >= 0 && (
							<div className="pl-4 flex rounded-lg gap-1 justify-between items-center">
								{metadata && (metadata.agent_name || metadata.agentName) && (
									<div id="message__agent_info" className="flex gap-1 mt-2 mb-2 items-center">
										<div className="text-xs text-muted-foreground">{timestamp} by</div>
										<span className="text-xs text-foreground">{metadata.agent_name || metadata.agentName}</span>
										{(metadata.model_id || metadata.modelId) && (
											<span className="text-xs text-muted-foreground block">
												({metadata.model_id || metadata.modelId})
											</span>
										)}
									</div>
								)}
								<Actions className="">
									<Action tooltip="Copy" color="secondary" onClick={handleCopy}>
										<CopyIcon className="size-4" />
									</Action>
									{onRetry && (
										<Action tooltip="Retry" color="secondary" onClick={onRetry}>
											<RefreshCwIcon className="size-4" />
										</Action>
									)}
									{onFeedback && (
										<>
											<Action tooltip="Like" color="secondary" onClick={() => onFeedback('like')}>
												<ThumbsUpIcon className="size-4" />
											</Action>
											<Action tooltip="Dislike" color="secondary" onClick={() => onFeedback('dislike')}>
												<ThumbsDownIcon className="size-4" />
											</Action>
										</>
									)}
								</Actions>
							</div>
						)}
					</>
				) : (
					renderableParts.map((part, i) => renderPartContent(part, i))
				)}
				{isStreaming && (
					<div className="flex items-center gap-2 text-sm text-muted-foreground h-8">
						<span>Agent is responding</span>
						<span className="loading loading-dots loading-xs" />
					</div>
				)}
				{status === 'cancelled' && (
					<div className="flex items-center gap-2 text-sm text-warning h-8">
						<BanIcon className="size-4" />
						<span>Response interrupted by user</span>
					</div>
				)}
			</div>
			<div className="text-sm px-2">{footer}</div>
		</div>
	);
}
