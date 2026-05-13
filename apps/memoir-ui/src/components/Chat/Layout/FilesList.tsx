'use client';

import { detachDocuments, fetchConversationDocuments } from '@actions/documents';
import { FilePickerModal } from '@components/Modal';
import type { Document } from '@polypixel/memoir-sdk/rig-service/rig/v1/document_pb';
import { DocumentStatus } from '@polypixel/memoir-sdk/rig-service/rig/v1/document_pb';
import { AlertCircle, CheckCircle2, File, FileText, Image, Loader2, Paperclip, X } from 'lucide-react';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { Button } from '@/components';

function getFileIcon(contentType: string) {
	if (contentType.startsWith('image/')) return Image;
	if (contentType === 'application/pdf' || contentType.startsWith('text/')) return FileText;
	return File;
}

function formatFileSize(bytes: bigint): string {
	const n = Number(bytes);
	if (n < 1024) return `${n} B`;
	if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
	return `${(n / (1024 * 1024)).toFixed(1)} MB`;
}

function StatusBadge({ status }: { status: DocumentStatus }) {
	switch (status) {
		case DocumentStatus.PENDING:
		case DocumentStatus.PROCESSING:
			return <Loader2 className="size-4 shrink-0 animate-spin text-info" />;
		case DocumentStatus.READY:
			return <CheckCircle2 className="size-4 shrink-0 text-success" />;
		case DocumentStatus.FAILED:
			return <AlertCircle className="size-4 shrink-0 text-error" />;
		default:
			return null;
	}
}

interface FilesListProps {
	conversationId?: string;
}

export default function FilesList({ conversationId }: FilesListProps) {
	const [documents, setDocuments] = useState<Document[]>([]);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [pickerOpen, setPickerOpen] = useState(false);

	const loadDocuments = useCallback(async () => {
		if (!conversationId) return;
		setLoading(true);
		setError(null);

		const result = await fetchConversationDocuments(conversationId);
		if (result.success) {
			setDocuments(result.data.documents);
		} else {
			setError(result.error);
		}
		setLoading(false);
	}, [conversationId]);

	useEffect(() => {
		setDocuments([]);
		loadDocuments();
	}, [loadDocuments]);

	const handleDetach = useCallback(
		async (documentPid: string) => {
			if (!conversationId) return;
			const result = await detachDocuments(conversationId, [documentPid]);
			if (result.success) {
				setDocuments((prev) => prev.filter((d) => d.pid !== documentPid));
			}
		},
		[conversationId],
	);

	const attachedPids = useMemo(() => documents.map((d) => d.pid), [documents]);

	if (!conversationId) {
		return <div className="py-8 text-center text-sm text-base-content/60">Select a conversation to view files</div>;
	}

	if (loading) {
		return (
			<div className="flex items-center justify-center py-8">
				<Loader2 className="size-5 animate-spin text-base-content/40" />
			</div>
		);
	}

	if (error) {
		return <div className="py-8 text-center text-sm text-error">{error}</div>;
	}

	return (
		<div id="files_list__container" className="p-2">
			<Button
				type="button"
				onClick={() => setPickerOpen(true)}
				className="btn btn-ghost btn-sm btn-block justify-start gap-2 mb-2">
				<Paperclip className="size-4" />
				Attach files
			</Button>

			{documents.length === 0 ? (
				<div className="py-6 text-center text-sm text-base-content/60">No files in this conversation</div>
			) : (
				<div className="space-y-2">
					{documents.map((doc) => {
						const Icon = getFileIcon(doc.contentType);
						return (
							<div
								key={doc.pid}
								className="group/file flex items-center gap-3 rounded-box border border-transparent p-3 transition-colors hover:border-primary hover:bg-primary/10">
								<div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-base-200">
									<Icon className="h-5 w-5 text-base-content/70" />
								</div>
								<div className="min-w-0 flex-1">
									<p className="truncate text-sm font-medium text-base-content">{doc.filename}</p>
									<p className="text-xs text-base-content/60">{formatFileSize(doc.sizeBytes)}</p>
								</div>
								<StatusBadge status={doc.status} />
								<Button
									type="button"
									onClick={() => handleDetach(doc.pid)}
									className="opacity-0 group-hover/file:opacity-100 transition-opacity"
									title="Detach from conversation">
									<X className="size-4 text-base-content/40 hover:text-error" />
								</Button>
							</div>
						);
					})}
				</div>
			)}

			<FilePickerModal
				isOpen={pickerOpen}
				onClose={() => setPickerOpen(false)}
				conversationPid={conversationId}
				alreadyAttachedPids={attachedPids}
				onAttached={loadDocuments}
			/>
		</div>
	);
}
