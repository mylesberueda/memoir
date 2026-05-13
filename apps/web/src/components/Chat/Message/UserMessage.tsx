'use client';

import { Avatar } from '@components';
import { DocumentStatus, type MessageAttachment, type MessagePart, MessagePartKind } from '@lib/chat-state';
import { cn } from '@lib/utils';
import { AlertCircleIcon, CheckCircle2Icon, FileIcon, Loader2Icon, RefreshCwIcon, XIcon } from 'lucide-react';
import { useMemo } from 'react';

export interface UserMessageProps {
	id: string;
	variant: 'rx';
	timestamp: Date;
	footer?: React.ReactNode;
	parts: MessagePart[];
	avatar?: string;
	attachments?: MessageAttachment[];
	status?: 'sending' | 'sent' | 'failed' | 'processing' | 'complete' | 'cancelled';
	onRetryAttachment?: (attachmentId: string) => void;
	onDeleteAttachment?: (attachmentId: string) => void;
}

export default function UserMessage({
	footer,
	parts,
	avatar,
	attachments,
	onRetryAttachment,
	onDeleteAttachment,
}: UserMessageProps) {
	const renderableParts = useMemo(() => {
		return parts.filter((p) => p.kind !== MessagePartKind.TOOL_RESULT && p.kind !== MessagePartKind.METADATA);
	}, [parts]);

	return (
		<div
			className="group is-user grid w-full gap-x-2 gap-y-1 py-3 grid-cols-[1fr_auto] justify-items-end"
			style={{ gridTemplateRows: 'auto auto' }}>
			<div className="flex flex-col gap-2 max-w-[80%] min-w-0 col-start-1 row-start-1">
				{attachments && attachments.length > 0 && (
					<div id="message_attachments__container" className="flex flex-wrap gap-1.5">
						{attachments.map((att) => (
							<div
								key={att.id}
								className={cn(
									'flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs',
									'bg-base-200 text-base-content/80',
								)}>
								<FileIcon className="size-3.5 shrink-0" />
								<span className="max-w-[120px] truncate">{att.name}</span>
								{att.status === DocumentStatus.PENDING || att.status === DocumentStatus.PROCESSING ? (
									<Loader2Icon className="size-3.5 shrink-0 animate-spin text-info" />
								) : att.status === DocumentStatus.READY ? (
									<CheckCircle2Icon className="size-3.5 shrink-0 text-success" />
								) : att.status === DocumentStatus.FAILED ? (
									<>
										<AlertCircleIcon className="size-3.5 shrink-0 text-error" />
										{onRetryAttachment && (
											<button
												type="button"
												onClick={() => onRetryAttachment(att.id)}
												className="p-0.5 rounded hover:bg-base-300 cursor-pointer"
												title="Retry upload">
												<RefreshCwIcon className="size-3 shrink-0 text-info" />
											</button>
										)}
										{onDeleteAttachment && (
											<button
												type="button"
												onClick={() => onDeleteAttachment(att.id)}
												className="p-0.5 rounded hover:bg-base-300 cursor-pointer"
												title="Remove file">
												<XIcon className="size-3 shrink-0 text-error" />
											</button>
										)}
									</>
								) : null}
							</div>
						))}
					</div>
				)}
				{renderableParts.map((part) => {
					if (part.kind !== MessagePartKind.TEXT) return null;
					if (!part.content?.trim()) return null;

					return (
						<div key={`text-${part.id}`} className="w-full min-w-0">
							<div className="flex flex-col gap-2 rounded-lg px-4 py-3 text-foreground text-sm bg-primary text-primary-content">
								<div className="is-user:dark min-w-0">
									<div className="prose prose-sm max-w-none whitespace-pre-wrap">{part.content}</div>
								</div>
							</div>
						</div>
					);
				})}
			</div>

			<div className="flex items-end self-end col-start-2 row-start-1">
				<Avatar className="shrink-0">
					<div className="w-8 rounded-full">
						{/** biome-ignore lint/performance/noImgElement: Image seems to crash this for some reason */}
						<img
							alt="avatar"
							src={avatar || 'https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp'}
							width={32}
							height={32}
						/>
					</div>
				</Avatar>
			</div>

			<div className="text-sm px-2 col-start-1 row-start-2">{footer}</div>
		</div>
	);
}
