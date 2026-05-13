'use client';

import { deleteDocument, fetchDocuments, getDownloadUrl, uploadDocument } from '@actions/documents';
import FileUpload, { type FileUploadRef } from '@components/Chat/FileUpload';
import { ACCEPTED_MIME_TYPES, MAX_FILE_SIZE_MB, MAX_FILES_PER_UPLOAD } from '@lib/documents';
import type { Document } from '@polypixel/memoir-sdk/rig-service/rig/v1/document_pb';
import { DocumentStatus } from '@polypixel/memoir-sdk/rig-service/rig/v1/document_pb';
import { AlertCircle, CheckCircle2, Download, FileIcon, FileText, Image, Loader2, Trash2, Upload } from 'lucide-react';
import { useCallback, useEffect, useRef, useState } from 'react';

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

function StatusBadge({ status }: { status: DocumentStatus }) {
	switch (status) {
		case DocumentStatus.PENDING:
			return (
				<span className="badge badge-sm badge-warning gap-1">
					<Loader2 className="size-3 animate-spin" />
					Pending
				</span>
			);
		case DocumentStatus.PROCESSING:
			return (
				<span className="badge badge-sm badge-info gap-1">
					<Loader2 className="size-3 animate-spin" />
					Processing
				</span>
			);
		case DocumentStatus.READY:
			return (
				<span className="badge badge-sm badge-success gap-1">
					<CheckCircle2 className="size-3" />
					Ready
				</span>
			);
		case DocumentStatus.FAILED:
			return (
				<span className="badge badge-sm badge-error gap-1">
					<AlertCircle className="size-3" />
					Failed
				</span>
			);
		default:
			return null;
	}
}

export default function FilesClient() {
	const [documents, setDocuments] = useState<Document[]>([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [showUpload, setShowUpload] = useState(false);
	const [uploading, setUploading] = useState(false);
	const fileUploadRef = useRef<FileUploadRef>(null);

	const loadDocuments = useCallback(async () => {
		setLoading(true);
		setError(null);
		const result = await fetchDocuments({ pageSize: 100 });
		if (result.success) {
			setDocuments(result.data.documents);
		} else {
			setError(result.error);
		}
		setLoading(false);
	}, []);

	useEffect(() => {
		loadDocuments();
	}, [loadDocuments]);

	const handleUpload = useCallback(
		async (files: File[]) => {
			setUploading(true);
			try {
				for (const file of files) {
					const content = new Uint8Array(await file.arrayBuffer());
					await uploadDocument({
						filename: file.name,
						contentType: file.type,
						content,
					});
				}

				await loadDocuments();
				setShowUpload(false);
			} finally {
				setUploading(false);
			}
		},
		[loadDocuments],
	);

	const handleDownload = useCallback(async (doc: Document) => {
		const result = await getDownloadUrl(doc.pid);
		if (result.success) {
			window.open(result.data.downloadUrl, '_blank');
		}
	}, []);

	const handleDelete = useCallback(async (pid: string) => {
		const result = await deleteDocument(pid);
		if (result.success) {
			setDocuments((prev) => prev.filter((d) => d.pid !== pid));
		}
	}, []);

	return (
		<div id="files_page__container" className="mx-auto max-w-5xl p-6">
			<div id="files_page__header" className="mb-6 flex items-center justify-between">
				<div>
					<h1 className="text-2xl font-bold">Files</h1>
					<p className="text-sm text-base-content/60">Manage your uploaded documents</p>
				</div>
				<button type="button" className="btn btn-primary btn-sm gap-2" onClick={() => setShowUpload(!showUpload)}>
					<Upload className="size-4" />
					Upload
				</button>
			</div>

			{showUpload && (
				<div id="files_page__upload" className="mb-6 rounded-xl border border-base-300 bg-base-200 p-4">
					<FileUpload
						ref={fileUploadRef}
						onFilesSelected={handleUpload}
						maxFiles={MAX_FILES_PER_UPLOAD}
						maxSizeMB={MAX_FILE_SIZE_MB}
						acceptedTypes={ACCEPTED_MIME_TYPES}
						disabled={uploading}
					/>
					{uploading && (
						<div className="mt-3 flex items-center gap-2 text-sm text-base-content/60">
							<Loader2 className="size-4 animate-spin" />
							Uploading...
						</div>
					)}
				</div>
			)}

			{loading ? (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="size-6 animate-spin text-base-content/40" />
				</div>
			) : error ? (
				<div className="py-16 text-center text-sm text-error">{error}</div>
			) : documents.length === 0 ? (
				<div className="py-16 text-center">
					<FileIcon className="mx-auto mb-3 size-10 text-base-content/30" />
					<p className="text-sm text-base-content/60">No files uploaded yet</p>
					<p className="text-xs text-base-content/40">Upload files to use them across conversations</p>
				</div>
			) : (
				<div id="files_page__list" className="overflow-x-auto">
					<table className="table">
						<thead>
							<tr>
								<th>Name</th>
								<th>Size</th>
								<th>Status</th>
								<th>Uploaded</th>
								<th />
							</tr>
						</thead>
						<tbody>
							{documents.map((doc) => {
								const Icon = getFileIcon(doc.contentType);
								return (
									<tr key={doc.pid} className="hover">
										<td>
											<div className="flex items-center gap-3">
												<Icon className="size-5 shrink-0 text-base-content/60" />
												<span className="truncate font-medium">{doc.filename}</span>
											</div>
										</td>
										<td className="text-base-content/60">{formatFileSize(doc.sizeBytes)}</td>
										<td>
											<StatusBadge status={doc.status} />
										</td>
										<td className="text-base-content/60">{new Date(doc.createdAt).toLocaleDateString()}</td>
										<td>
											<div className="flex items-center gap-1">
												<button
													type="button"
													className="btn btn-ghost btn-xs"
													onClick={() => handleDownload(doc)}
													title="Download">
													<Download className="size-4" />
												</button>
												<button
													type="button"
													className="btn btn-ghost btn-xs text-error"
													onClick={() => handleDelete(doc.pid)}
													title="Delete">
													<Trash2 className="size-4" />
												</button>
											</div>
										</td>
									</tr>
								);
							})}
						</tbody>
					</table>
				</div>
			)}
		</div>
	);
}
