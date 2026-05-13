'use client';

import { ACCEPTED_MIME_TYPES, MAX_FILE_SIZE_MB, MAX_FILES_PER_UPLOAD } from '@lib/documents';
import { AlertCircle, File, Image, X } from 'lucide-react';
import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';

interface FileUploadProps {
	onFilesSelected: (files: File[]) => void;
	maxFiles?: number;
	maxSizeMB?: number;
	acceptedTypes?: string[];
	disabled?: boolean;
}

export interface FileUploadRef {
	triggerFileInput: () => void;
}

const FileUpload = forwardRef<FileUploadRef, FileUploadProps>(function FileUpload(
	{
		onFilesSelected,
		maxFiles = MAX_FILES_PER_UPLOAD,
		maxSizeMB = MAX_FILE_SIZE_MB,
		acceptedTypes = ACCEPTED_MIME_TYPES,
		disabled = false,
	},
	ref,
) {
	const [selectedFiles, setSelectedFiles] = useState<File[]>([]);
	const [error, setError] = useState<string | null>(null);
	const [uploadProgress, setUploadProgress] = useState<Map<string, number>>(new Map());
	const [isDragging, setIsDragging] = useState(false);
	const [showDropzone, setShowDropzone] = useState(false);
	const inputRef = useRef<HTMLInputElement>(null);
	const dragCounterRef = useRef(0);

	useImperativeHandle(ref, () => ({
		triggerFileInput: () => {
			inputRef.current?.click();
		},
	}));

	// Document-level drag detection
	useEffect(() => {
		if (disabled) return;

		const handleDocumentDragEnter = (e: DragEvent) => {
			e.preventDefault();
			dragCounterRef.current++;

			// Check if we're dragging files (not just any draggable element)
			const hasFiles =
				e.dataTransfer?.types?.includes('Files') ||
				e.dataTransfer?.types?.some(
					(type) => type.startsWith('application/') || type.startsWith('text/') || type.startsWith('image/'),
				);

			if (hasFiles) {
				setShowDropzone(true);
			}
		};

		const handleDocumentDragLeave = (e: DragEvent) => {
			e.preventDefault();
			dragCounterRef.current--;

			// Only hide if we've left all elements (counter is 0)
			if (dragCounterRef.current <= 0) {
				dragCounterRef.current = 0; // Reset to prevent negative values
				setShowDropzone(false);
				setIsDragging(false);
			}
		};

		const handleDocumentDragOver = (e: DragEvent) => {
			e.preventDefault();

			// Check if we're dragging files
			const hasFiles = e.dataTransfer?.types?.includes('Files');
			if (hasFiles && !showDropzone) {
				setShowDropzone(true);
			}
		};

		const handleDocumentDrop = (e: DragEvent) => {
			e.preventDefault();
			dragCounterRef.current = 0;
			setShowDropzone(false);
			setIsDragging(false);
		};

		document.addEventListener('dragenter', handleDocumentDragEnter);
		document.addEventListener('dragleave', handleDocumentDragLeave);
		document.addEventListener('dragover', handleDocumentDragOver);
		document.addEventListener('drop', handleDocumentDrop);

		return () => {
			document.removeEventListener('dragenter', handleDocumentDragEnter);
			document.removeEventListener('dragleave', handleDocumentDragLeave);
			document.removeEventListener('dragover', handleDocumentDragOver);
			document.removeEventListener('drop', handleDocumentDrop);
		};
	}, [disabled, showDropzone]);

	const validateAndProcessFiles = (files: File[]) => {
		setError(null);

		// Validate file count
		if (files.length > maxFiles) {
			setError(`Maximum ${maxFiles} files allowed`);
			return;
		}

		// Validate file sizes
		const maxSizeBytes = maxSizeMB * 1024 * 1024;
		const oversizedFiles = files.filter((f) => f.size > maxSizeBytes);
		if (oversizedFiles.length > 0) {
			setError(`Files must be under ${maxSizeMB}MB`);
			return;
		}

		// Validate file types
		const invalidFiles = files.filter((file) => {
			return !acceptedTypes.some((type) => {
				if (type.endsWith('/*')) {
					return file.type.startsWith(type.slice(0, -1));
				}
				return file.type === type;
			});
		});

		if (invalidFiles.length > 0) {
			setError(`File type not supported. Accepted types: ${acceptedTypes.join(', ')}`);
			return;
		}

		setSelectedFiles(files);
		onFilesSelected(files);

		// Reset input value to allow selecting the same files again
		if (inputRef.current) {
			inputRef.current.value = '';
		}
	};

	const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
		const files = Array.from(e.target.files || []);
		validateAndProcessFiles(files);
	};

	const handleDrop = (e: React.DragEvent) => {
		e.preventDefault();
		e.stopPropagation();

		if (disabled) return;

		const files = Array.from(e.dataTransfer.files);
		validateAndProcessFiles(files);

		// Reset window drag state
		dragCounterRef.current = 0;
		setShowDropzone(false);
		setIsDragging(false);
	};

	const handleDragOver = (e: React.DragEvent) => {
		e.preventDefault();
		e.stopPropagation();
	};

	const handleDragEnter = (e: React.DragEvent) => {
		e.preventDefault();
		e.stopPropagation();
		if (!disabled) {
			setIsDragging(true);
		}
	};

	const handleDragLeave = (e: React.DragEvent) => {
		e.preventDefault();
		e.stopPropagation();
		// Only set dragging to false if we're leaving the dropzone itself, not a child
		if (!e.currentTarget.contains(e.relatedTarget as Node)) {
			setIsDragging(false);
		}
	};

	const removeFile = (index: number) => {
		const newFiles = selectedFiles.filter((_, i) => i !== index);
		setSelectedFiles(newFiles);
		onFilesSelected(newFiles);

		// Remove upload progress for removed file
		const removedFile = selectedFiles[index];
		if (removedFile) {
			setUploadProgress((prev) => {
				const next = new Map(prev);
				next.delete(removedFile.name);
				return next;
			});
		}
	};

	const clearAllFiles = () => {
		setSelectedFiles([]);
		onFilesSelected([]);
		setUploadProgress(new Map());
		setError(null);
	};

	const getFileIcon = (file: File) => {
		if (file.type.startsWith('image/')) return <Image size={16} className="text-blue-600" />;
		if (file.type === 'application/pdf') return <File size={16} className="text-red-600" />;
		return <File size={16} className="text-gray-600" />;
	};

	const formatFileSize = (bytes: number) => {
		if (bytes === 0) return '0 Bytes';
		const k = 1024;
		const sizes = ['Bytes', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
	};

	return (
		<div className="space-y-2">
			<input
				ref={inputRef}
				type="file"
				multiple
				accept={acceptedTypes.join(',')}
				onChange={handleFileSelect}
				className="hidden"
				data-testid="file-upload-input"
				disabled={disabled}
			/>

			{/* Drag and drop zone - always show when file upload section is visible */}
			<button
				type="button"
				className={`
					border-2 border-dashed rounded-lg p-6 text-center cursor-pointer transition-colors w-full
					${
						isDragging
							? 'border-blue-500 bg-blue-50 text-blue-600'
							: 'border-gray-300 hover:border-gray-400 text-gray-600'
					}
					${disabled ? 'opacity-50 cursor-not-allowed' : ''}
				`}
				data-testid="file-upload-dropzone"
				onClick={() => !disabled && inputRef.current?.click()}
				onDrop={handleDrop}
				onDragOver={handleDragOver}
				onDragEnter={handleDragEnter}
				onDragLeave={handleDragLeave}
				disabled={disabled}
				aria-label="Click to upload files or drag and drop files here">
				<div className="space-y-2">
					<File size={32} className="mx-auto opacity-50" />
					<div>
						<p className="text-sm font-medium">{isDragging ? 'Drop files here' : 'Click to upload or drag and drop'}</p>
						<p className="text-xs text-muted-foreground">
							Max {maxFiles} files, {maxSizeMB}MB each
						</p>
						<p className="text-xs text-muted-foreground">{acceptedTypes.join(', ')}</p>
					</div>
				</div>
			</button>

			{selectedFiles.length > 0 && (
				<div className="flex items-center justify-between">
					<span className="text-sm text-muted-foreground">
						{selectedFiles.length} file{selectedFiles.length !== 1 ? 's' : ''} selected
					</span>
					<button
						type="button"
						onClick={clearAllFiles}
						className="text-xs text-muted-foreground hover:text-foreground underline"
						data-testid="clear-all-files">
						Clear all
					</button>
				</div>
			)}

			{/* File list */}
			{selectedFiles.length > 0 && (
				<div className="space-y-2 max-h-32 overflow-y-auto">
					{selectedFiles.map((file, index) => {
						const progress = uploadProgress.get(file.name);
						return (
							<div
								key={`${file.name}-${index}`}
								className="flex items-center gap-2 p-2 bg-base-100 rounded-lg text-sm"
								data-testid="uploaded-file">
								{getFileIcon(file)}
								<div className="flex-1 min-w-0">
									<div className="flex items-center justify-between">
										<span className="truncate font-medium">{file.name}</span>
										<button
											type="button"
											onClick={() => removeFile(index)}
											className="text-gray-500 hover:text-gray-700 ml-2 cursor-pointer"
											data-testid="remove-file-button"
											aria-label={`Remove ${file.name}`}>
											<X size={14} />
										</button>
									</div>
									<div className="flex items-center gap-2 mt-1">
										<span className="text-xs text-muted-foreground">{formatFileSize(file.size)}</span>
										{progress !== undefined && (
											<>
												<div className="flex-1 bg-gray-200 rounded-full h-1">
													<div
														className="bg-blue-600 h-1 rounded-full transition-all duration-300"
														style={{ width: `${progress}%` }}
														data-testid="file-upload-progress"
													/>
												</div>
												<span className="text-xs text-muted-foreground">{progress}%</span>
											</>
										)}
									</div>
								</div>
							</div>
						);
					})}
				</div>
			)}

			{/* Error message */}
			{error && (
				<div className="flex items-center gap-2 text-red-600 text-sm" data-testid="file-upload-error">
					<AlertCircle size={16} />
					<span>{error}</span>
				</div>
			)}
		</div>
	);
});

export default FileUpload;
