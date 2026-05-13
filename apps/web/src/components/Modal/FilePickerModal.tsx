'use client';

import { attachDocuments, fetchDocuments } from '@actions/documents';
import { Button, Modal } from '@components';
import type { Document } from '@polypixel/proto-ts/rig-service/rig/v1/document_pb';
import { DocumentStatus } from '@polypixel/proto-ts/rig-service/rig/v1/document_pb';
import { CheckCircle2, FileIcon, FileText, Image, Loader2, Search, X } from 'lucide-react';
import { useCallback, useEffect, useMemo, useState } from 'react';

function getFileIcon(contentType: string) {
	if (contentType.startsWith('image/')) return Image;
	if (contentType === 'application/pdf' || contentType.startsWith('text/')) return FileText;
	return FileIcon;
}

function formatFileSize(bytes: bigint): string {
	const n = Number(bytes);
	if (n < 1024) return `${n} B`;
	if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
	return `${(n / (1024 * 1024)).toFixed(1)} MB`;
}

interface FilePickerModalProps {
	isOpen: boolean;
	onClose: () => void;
	conversationPid: string;
	alreadyAttachedPids?: string[];
	onAttached?: () => void;
}

export default function FilePickerModal({
	isOpen,
	onClose,
	conversationPid,
	alreadyAttachedPids = [],
	onAttached,
}: FilePickerModalProps) {
	const [documents, setDocuments] = useState<Document[]>([]);
	const [loading, setLoading] = useState(false);
	const [search, setSearch] = useState('');
	const [selected, setSelected] = useState<Set<string>>(new Set());
	const [attaching, setAttaching] = useState(false);

	useEffect(() => {
		if (!isOpen) return;
		setSelected(new Set());
		setSearch('');
		setLoading(true);

		fetchDocuments({ pageSize: 100, status: DocumentStatus.READY }).then((result) => {
			if (result.success) {
				setDocuments(result.data.documents);
			}
			setLoading(false);
		});
	}, [isOpen]);

	const alreadyAttached = useMemo(() => new Set(alreadyAttachedPids), [alreadyAttachedPids]);

	const filtered = useMemo(() => {
		if (!search.trim()) return documents;
		const q = search.toLowerCase();
		return documents.filter((d) => d.filename.toLowerCase().includes(q));
	}, [documents, search]);

	const toggleSelect = useCallback((pid: string) => {
		setSelected((prev) => {
			const next = new Set(prev);
			if (next.has(pid)) {
				next.delete(pid);
			} else {
				next.add(pid);
			}
			return next;
		});
	}, []);

	const handleAttach = useCallback(async () => {
		if (selected.size === 0) return;
		setAttaching(true);
		const result = await attachDocuments(conversationPid, Array.from(selected));
		setAttaching(false);
		if (result.success) {
			onAttached?.();
			onClose();
		}
	}, [selected, conversationPid, onAttached, onClose]);

	return (
		<Modal open={isOpen}>
			<div className="modal-box max-w-lg">
				<div id="file_picker__header" className="flex items-center justify-between mb-4">
					<h3 className="font-bold text-lg">Attach Files</h3>
					<Button ghost size="sm" shape="circle" onClick={onClose}>
						<X className="w-4 h-4" />
					</Button>
				</div>

				<div id="file_picker__search" className="mb-3">
					<label className="input input-bordered input-sm flex items-center gap-2 w-full">
						<Search className="size-4 text-base-content/40" />
						<input
							type="text"
							className="grow"
							placeholder="Search files..."
							value={search}
							onChange={(e) => setSearch(e.target.value)}
						/>
					</label>
				</div>

				<div id="file_picker__list" className="max-h-64 overflow-y-auto">
					{loading ? (
						<div className="flex items-center justify-center py-8">
							<Loader2 className="size-5 animate-spin text-base-content/40" />
						</div>
					) : filtered.length === 0 ? (
						<div className="py-8 text-center text-sm text-base-content/60">
							{search ? 'No files match your search' : 'No files available'}
						</div>
					) : (
						<div className="space-y-1">
							{filtered.map((doc) => {
								const Icon = getFileIcon(doc.contentType);
								const isAttached = alreadyAttached.has(doc.pid);
								const isSelected = selected.has(doc.pid);

								return (
									<button
										key={doc.pid}
										type="button"
										disabled={isAttached}
										onClick={() => toggleSelect(doc.pid)}
										className={`flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors ${
											isAttached
												? 'opacity-50 cursor-not-allowed'
												: isSelected
													? 'bg-primary/10 border border-primary'
													: 'hover:bg-base-200 border border-transparent'
										}`}>
										<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded bg-base-200">
											<Icon className="size-4 text-base-content/60" />
										</div>
										<div className="min-w-0 flex-1">
											<p className="truncate text-sm font-medium">{doc.filename}</p>
											<p className="text-xs text-base-content/50">{formatFileSize(doc.sizeBytes)}</p>
										</div>
										{isAttached ? (
											<CheckCircle2 className="size-4 shrink-0 text-success" />
										) : isSelected ? (
											<div className="size-4 shrink-0 rounded-sm bg-primary flex items-center justify-center">
												<CheckCircle2 className="size-3 text-primary-content" />
											</div>
										) : (
											<div className="size-4 shrink-0 rounded-sm border border-base-content/20" />
										)}
									</button>
								);
							})}
						</div>
					)}
				</div>

				<div id="file_picker__actions" className="mt-4 flex justify-end gap-2">
					<Button size="sm" ghost onClick={onClose}>
						Cancel
					</Button>
					<Button size="sm" color="primary" onClick={handleAttach} disabled={selected.size === 0 || attaching}>
						{attaching ? (
							<>
								<Loader2 className="size-4 animate-spin" />
								Attaching...
							</>
						) : (
							`Attach ${selected.size > 0 ? `(${selected.size})` : ''}`
						)}
					</Button>
				</div>
			</div>
		</Modal>
	);
}
