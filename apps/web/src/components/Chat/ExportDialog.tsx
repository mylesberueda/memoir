'use client';

import { downloadFile, exportToJSON, exportToMarkdown, generateFilename, importFromJSON } from '@lib/chat-export';
import type { ChatStateActions } from '@lib/chat-state';
import { AlertCircle, CheckCircle, Download, FileJson, FileText, Upload } from 'lucide-react';
import { useRef, useState } from 'react';

interface ExportDialogProps {
	isOpen: boolean;
	onClose: () => void;
	chat: ChatStateActions;
}

export function ExportDialog({ isOpen, onClose, chat }: ExportDialogProps) {
	const { state, setMessages } = chat;
	const { messages, sessionId } = state;
	const [importError, setImportError] = useState<string | null>(null);
	const [importSuccess, setImportSuccess] = useState<string | null>(null);
	const [isImporting, setIsImporting] = useState(false);
	const fileInputRef = useRef<HTMLInputElement>(null);

	if (!isOpen) return null;

	const handleExportJSON = () => {
		try {
			const json = exportToJSON(messages, sessionId ?? null);
			const filename = generateFilename('json');
			downloadFile(json, filename, 'application/json');
		} catch (error) {
			console.error('Export failed:', error);
		}
	};

	const handleExportMarkdown = () => {
		try {
			const markdown = exportToMarkdown(messages);
			const filename = generateFilename('md');
			downloadFile(markdown, filename, 'text/markdown');
		} catch (error) {
			console.error('Export failed:', error);
		}
	};

	const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
		const file = e.target.files?.[0];
		if (!file) return;

		setImportError(null);
		setImportSuccess(null);
		setIsImporting(true);

		try {
			const text = await file.text();
			const conversation = await importFromJSON(text);

			// Ask for confirmation if there are existing messages
			if (messages.length > 0) {
				const confirmReplace = confirm(
					`This will replace your current conversation with ${conversation.messages.length} imported messages. Continue?`,
				);
				if (!confirmReplace) {
					setIsImporting(false);
					return;
				}
			}

			setMessages(conversation.messages);
			setImportSuccess(
				`Successfully imported ${conversation.messages.length} messages from ${conversation.exportDate ? new Date(conversation.exportDate).toLocaleDateString() : 'unknown date'}`,
			);

			// Auto-close after successful import
			setTimeout(() => {
				onClose();
			}, 2000);
		} catch (error) {
			setImportError(error instanceof Error ? error.message : 'Import failed');
		} finally {
			setIsImporting(false);
			// Reset input
			if (fileInputRef.current) {
				fileInputRef.current.value = '';
			}
		}
	};

	const handleClose = () => {
		setImportError(null);
		setImportSuccess(null);
		onClose();
	};

	const canExport = messages.length > 0;

	if (!isOpen) return null;

	return (
		<>
			<input type="checkbox" className="modal-toggle" checked={isOpen} readOnly />
			<dialog className="modal backdrop-blur-sm" data-testid="export-dialog">
				<div className="modal-box max-w-lg">
					<h3 className="font-bold text-xl mb-4">Export/Import Conversation</h3>
					<div className="space-y-6">
						{/* Export Section */}
						<div>
							<h3 className="font-medium mb-3 flex items-center gap-2">
								<Download size={16} />
								Export Conversation
							</h3>

							{!canExport && (
								<div className="text-sm text-base-content/60 mb-3 p-3 bg-base-200 rounded-lg">
									No messages to export. Start a conversation first.
								</div>
							)}

							<div className="flex flex-col gap-2">
								<button
									type="button"
									onClick={handleExportJSON}
									disabled={!canExport}
									className="btn btn-primary flex items-center gap-2 justify-start"
									data-testid="export-json-button">
									<FileJson size={16} />
									Export as JSON
									<span className="text-xs opacity-75 ml-auto">(Preserves all data)</span>
								</button>

								<button
									type="button"
									onClick={handleExportMarkdown}
									disabled={!canExport}
									className="btn btn-secondary flex items-center gap-2 justify-start"
									data-testid="export-markdown-button">
									<FileText size={16} />
									Export as Markdown
									<span className="text-xs opacity-75 ml-auto">(Human readable)</span>
								</button>
							</div>

							{canExport && (
								<div className="text-xs text-base-content/60 mt-2">
									Exporting {messages.length} message
									{messages.length !== 1 ? 's' : ''}
								</div>
							)}
						</div>

						<div className="divider" />

						{/* Import Section */}
						<div>
							<h3 className="font-medium mb-3 flex items-center gap-2">
								<Upload size={16} />
								Import Conversation
							</h3>

							<label className="btn btn-accent flex items-center gap-2 cursor-pointer w-full">
								<Upload size={16} />
								{isImporting ? 'Importing...' : 'Import JSON File'}
								<input
									ref={fileInputRef}
									type="file"
									accept=".json"
									onChange={handleImport}
									className="hidden"
									data-testid="import-file-input"
									disabled={isImporting}
								/>
							</label>

							<div className="text-xs text-base-content/60 mt-2">
								Only JSON files exported from this app are supported
							</div>

							{/* Import feedback */}
							{importError && (
								<div className="alert alert-error mt-3" data-testid="import-error">
									<AlertCircle size={16} />
									<span>{importError}</span>
								</div>
							)}

							{importSuccess && (
								<div className="alert alert-success mt-3" data-testid="import-success">
									<CheckCircle size={16} />
									<span>{importSuccess}</span>
								</div>
							)}

							{messages.length > 0 && (
								<div className="alert alert-warning mt-3">
									<AlertCircle size={16} />
									<span>! Importing will replace your current conversation</span>
								</div>
							)}
						</div>
					</div>

					{/* Actions */}
					<div className="modal-action">
						<button type="button" className="btn" onClick={handleClose}>
							Close
						</button>
					</div>
				</div>

				{/* Backdrop button that closes modal when clicked */}
				<button
					type="button"
					className="modal-backdrop"
					onClick={handleClose}
					onKeyDown={(e) => {
						if (e.key === 'Escape') handleClose();
					}}
					aria-label="Close modal"
				/>
			</dialog>
		</>
	);
}

export default ExportDialog;
