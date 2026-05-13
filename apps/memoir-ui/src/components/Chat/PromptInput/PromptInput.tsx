'use client';

import type { Model } from '@actions/models';
import { cn } from '@lib/utils';
import { BrainCircuitIcon, GlobeIcon, MicIcon, PaperclipIcon } from 'lucide-react';
import { forwardRef, useCallback, useEffect, useImperativeHandle, useRef, useState } from 'react';
import FileUpload, { type FileUploadRef } from '../FileUpload';
import PromptInputButton from './PromptInputButton';
import PromptInputModelSelect from './PromptInputModelSelect';
import PromptInputSubmit from './PromptInputSubmit';
import PromptInputTextarea from './PromptInputTextarea';
import PromptInputToolbar from './PromptInputToolbar';
import PromptInputTools from './PromptInputTools';

export type PromptBoxForm = {
	prompt: string;
	files: File[];
	modelId?: string;
	enableDeepResearch?: boolean;
	enableWebSearch?: boolean;
	enableFileUpload?: boolean;
};

export interface PromptInputProps {
	onSubmit: (form: PromptBoxForm) => void | Promise<void>;
	disabled?: boolean;
	isLoading?: boolean;
	isStreaming?: boolean;
	onStopStreaming?: () => void;
	enableFileUpload?: boolean;
	enableWebSearch?: boolean;
	enableDeepResearch?: boolean;
	enableMic?: boolean;
	enableModelSelect?: boolean;
	models?: Model[];
	modelsLoading?: boolean;
	modelsError?: Error | null;
	placeholder?: string;
	currentAssistantModel?: string | null;
	onModelChange?: (modelId: string) => void;
	id?: string;
}

export interface PromptInputRef {
	focus: () => void;
}

export default forwardRef<PromptInputRef, PromptInputProps>(function PromptInput(
	{
		onSubmit,
		disabled,
		isLoading,
		isStreaming,
		onStopStreaming,
		enableFileUpload,
		enableWebSearch,
		enableDeepResearch,
		enableMic,
		enableModelSelect = true,
		models,
		modelsLoading,
		modelsError,
		placeholder = 'What would you like to know?',
		currentAssistantModel,
		onModelChange,
		id,
	},
	ref,
) {
	const [message, setMessage] = useState('');
	const [selectedFiles, setSelectedFiles] = useState<File[]>([]);
	const [showFileUpload, setShowFileUpload] = useState(false);
	const [selectedModelId, setSelectedModelId] = useState<string>('');
	const fileUploadRef = useRef<FileUploadRef>(null);
	const textareaRef = useRef<HTMLTextAreaElement>(null);

	// Automatically select the assistant's current model when models are loaded
	useEffect(() => {
		if (currentAssistantModel && models && models.length > 0 && !selectedModelId) {
			// Find the model that matches the assistant's current model
			const matchingModel = models.find((model) => model.identifier.value === currentAssistantModel);
			if (matchingModel) {
				setSelectedModelId(currentAssistantModel);
			}
		}
	}, [currentAssistantModel, models, selectedModelId]);

	useImperativeHandle(
		ref,
		() => ({
			focus: () => {
				textareaRef.current?.focus();
			},
		}),
		[],
	);

	const handleSubmit = useCallback(
		(e: React.FormEvent) => {
			e.preventDefault();
			if (message.trim() && !disabled && !isLoading && !isStreaming) {
				onSubmit({
					prompt: message,
					files: selectedFiles,
					modelId: selectedModelId || undefined,
					enableDeepResearch,
					enableWebSearch,
					enableFileUpload,
				});
				setMessage('');
				setSelectedFiles([]);
				setShowFileUpload(false);
			}
		},
		[
			message,
			selectedFiles,
			selectedModelId,
			disabled,
			isLoading,
			isStreaming,
			onSubmit,
			enableDeepResearch,
			enableWebSearch,
			enableFileUpload,
		],
	);

	const handleFileUploadClick = useCallback(() => {
		setShowFileUpload(!showFileUpload);
	}, [showFileUpload]);

	const handleFilesSelected = useCallback((files: File[]) => {
		setSelectedFiles(files);
	}, []);

	const handleModelChange = useCallback(
		(modelId: string) => {
			setSelectedModelId(modelId);
			if (onModelChange) {
				onModelChange(modelId);
			}
		},
		[onModelChange],
	);

	const status = isStreaming ? 'streaming' : isLoading ? 'submitted' : undefined;

	return (
		<form
			onSubmit={handleSubmit}
			className={cn(
				'w-full divide-y overflow-hidden rounded-xl border bg-background shadow-sm',
				'mx-auto max-w-4xl bg-base-200 border-base-300',
			)}>
			{/* File upload section */}
			{enableFileUpload && showFileUpload && (
				<div className="p-3 border-b border-base-300">
					<FileUpload
						ref={fileUploadRef}
						onFilesSelected={handleFilesSelected}
						maxFiles={5}
						maxSizeMB={10}
						disabled={disabled || isLoading || isStreaming}
					/>
				</div>
			)}

			<PromptInputTextarea
				ref={textareaRef}
				id={id ? `${id}__input` : 'prompt_input__input'}
				value={message}
				onChange={(e) => setMessage(e.target.value)}
				placeholder={placeholder}
				disabled={disabled || isLoading || isStreaming}
			/>
			<PromptInputToolbar>
				<PromptInputTools className="gap-2">
					{enableFileUpload && (
						<PromptInputButton
							onClick={handleFileUploadClick}
							disabled={disabled || isLoading || isStreaming}
							title="Attach file"
							className={`hover:bg-base-300 ${showFileUpload || selectedFiles.length > 0 ? 'bg-primary text-primary-content' : ''}`}>
							<PaperclipIcon className="size-4" />
						</PromptInputButton>
					)}
					{enableMic && (
						<PromptInputButton
							disabled={disabled || isLoading || isStreaming}
							title="Voice input"
							className="hover:bg-base-300">
							<MicIcon className="size-4" />
						</PromptInputButton>
					)}
					{enableWebSearch && (
						<PromptInputButton
							disabled={disabled || isLoading || isStreaming}
							title="Enable web search"
							className="hover:bg-base-300">
							<GlobeIcon className="size-4" />
							<span className="text-xs">Search</span>
						</PromptInputButton>
					)}
					{enableDeepResearch && (
						<PromptInputButton
							disabled={disabled || isLoading || isStreaming}
							title="Enable deep research"
							className="hover:bg-base-300">
							<BrainCircuitIcon className="size-4" />
							<span className="text-xs">Deep Research</span>
						</PromptInputButton>
					)}
					{enableModelSelect && (
						<PromptInputModelSelect
							value={selectedModelId}
							onChange={handleModelChange}
							models={models}
							isLoading={modelsLoading}
							error={modelsError}
							disabled={disabled || isLoading || isStreaming}
						/>
					)}
				</PromptInputTools>
				<div className="flex items-center gap-2">
					{isStreaming && onStopStreaming ? (
						<PromptInputSubmit
							id="prompt_actions__cancel"
							status="streaming"
							type="button"
							onClick={onStopStreaming}
							color="error"
						/>
					) : (
						<PromptInputSubmit
							id="prompt_actions__submit"
							status={status}
							disabled={!message.trim() || disabled || isLoading}
							className="primary"
						/>
					)}
				</div>
			</PromptInputToolbar>
		</form>
	);
});
