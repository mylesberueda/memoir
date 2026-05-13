'use client';

import { getTextContent, type Message } from '@lib/chat-state';
import { Check, Edit2, MoreHorizontal, RotateCw, Trash2, X } from 'lucide-react';
import { useState } from 'react';

interface MessageActionsProps {
	message: Message;
	onRetry?: () => void;
	onEdit?: (newContent: string) => void;
	onDelete?: () => void;
	canEdit?: boolean;
	canRetry?: boolean;
	canDelete?: boolean;
	className?: string;
}

export function MessageActions({
	message,
	onRetry,
	onEdit,
	onDelete,
	canEdit = false,
	canRetry = false,
	canDelete = true,
	className = '',
}: MessageActionsProps) {
	const messageContent = getTextContent(message.parts);
	const [isEditing, setIsEditing] = useState(false);
	const [editedContent, setEditedContent] = useState(messageContent);
	const [showActions, setShowActions] = useState(false);
	const [isDeleting, setIsDeleting] = useState(false);

	const handleSaveEdit = () => {
		const trimmedContent = editedContent.trim();
		if (trimmedContent && onEdit && trimmedContent !== messageContent) {
			onEdit(trimmedContent);
		}
		setIsEditing(false);
	};

	const handleCancelEdit = () => {
		setEditedContent(messageContent);
		setIsEditing(false);
	};

	const handleDeleteClick = () => {
		if (onDelete) {
			setIsDeleting(true);
			setShowActions(false);
		}
	};

	const confirmDelete = () => {
		if (onDelete) {
			onDelete();
		}
		setIsDeleting(false);
	};

	const cancelDelete = () => {
		setIsDeleting(false);
	};

	const handleRetry = () => {
		if (onRetry) {
			onRetry();
		}
		setShowActions(false);
	};

	const handleEditClick = () => {
		setIsEditing(true);
		setShowActions(false);
	};

	// Show edit mode if editing
	if (isEditing) {
		return (
			<div className={`flex flex-col gap-2 ${className}`}>
				<textarea
					value={editedContent}
					onChange={(e) => setEditedContent(e.target.value)}
					className="w-full p-2 border border-base-300 rounded-lg resize-none text-sm bg-base-100 focus:outline-none focus:ring-2 focus:ring-primary"
					rows={Math.max(2, editedContent.split('\n').length)}
					data-testid="edit-message-input"
					placeholder="Edit your message..."
				/>
				<div className="flex items-center gap-2 justify-end">
					<button
						type="button"
						onClick={handleCancelEdit}
						className="flex items-center gap-1 px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-base-200 rounded transition-colors"
						data-testid="cancel-edit-button">
						<X size={14} />
						Cancel
					</button>
					<button
						type="button"
						onClick={handleSaveEdit}
						disabled={!editedContent.trim() || editedContent.trim() === messageContent}
						className="flex items-center gap-1 px-2 py-1 text-xs bg-primary text-primary-content hover:bg-primary-focus rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
						data-testid="save-edit-button">
						<Check size={14} />
						Save
					</button>
				</div>
			</div>
		);
	}

	// Show delete confirmation if deleting
	if (isDeleting) {
		return (
			<div className={`flex items-center gap-2 p-2 bg-red-50 border border-red-200 rounded-lg ${className}`}>
				<span className="text-sm text-red-700">Delete this message?</span>
				<div className="flex gap-1 ml-auto">
					<button
						type="button"
						onClick={cancelDelete}
						className="px-2 py-1 text-xs text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded transition-colors">
						Cancel
					</button>
					<button
						type="button"
						onClick={confirmDelete}
						className="px-2 py-1 text-xs bg-red-600 text-white hover:bg-red-700 rounded transition-colors"
						data-testid="confirm-delete">
						Delete
					</button>
				</div>
			</div>
		);
	}

	// Determine which actions to show
	const showRetryAction = canRetry && (message.status === 'failed' || message.status === 'sending');
	const showEditAction = canEdit && message.role === 'user' && message.status !== 'sending';
	const showDeleteAction = canDelete;

	const hasActions = showRetryAction || showEditAction || showDeleteAction;

	if (!hasActions) {
		return null;
	}

	return (
		<div className={`relative ${className}`}>
			{/* Action trigger button */}
			<button
				type="button"
				onClick={() => setShowActions(!showActions)}
				className="p-1 opacity-0 group-hover:opacity-100 text-muted-foreground hover:text-foreground hover:bg-base-200 rounded transition-all"
				aria-label="Message actions"
				data-testid="message-actions-trigger">
				<MoreHorizontal size={14} />
			</button>

			{/* Actions dropdown */}
			{showActions && (
				<>
					{/* Backdrop to close dropdown */}
					<button
						className="fixed inset-0 z-10"
						onClick={() => setShowActions(false)}
						onKeyDown={(e) => {
							if (e.key === 'Escape') {
								setShowActions(false);
							}
						}}
						type="button"
						tabIndex={-1}
					/>

					{/* Actions menu */}
					<div className="absolute right-0 top-full mt-1 bg-base-100 border border-base-300 rounded-lg shadow-lg z-20 py-1 min-w-[120px]">
						{showRetryAction && (
							<button
								type="button"
								onClick={handleRetry}
								className="flex items-center gap-2 w-full px-3 py-2 text-sm text-left hover:bg-base-200 transition-colors"
								data-testid="retry-message-button">
								<RotateCw size={14} />
								Retry
							</button>
						)}

						{showEditAction && (
							<button
								type="button"
								onClick={handleEditClick}
								className="flex items-center gap-2 w-full px-3 py-2 text-sm text-left hover:bg-base-200 transition-colors"
								data-testid="edit-message-button">
								<Edit2 size={14} />
								Edit
							</button>
						)}

						{showDeleteAction && (
							<button
								type="button"
								onClick={handleDeleteClick}
								className="flex items-center gap-2 w-full px-3 py-2 text-sm text-left text-red-600 hover:bg-red-50 transition-colors"
								data-testid="delete-message-button">
								<Trash2 size={14} />
								Delete
							</button>
						)}
					</div>
				</>
			)}
		</div>
	);
}

export default MessageActions;
