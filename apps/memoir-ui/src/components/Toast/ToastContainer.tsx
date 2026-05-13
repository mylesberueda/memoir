'use client';

import useToast from '@hooks/useToast';
import { cn } from '@lib/utils';
import { useEffect } from 'react';

interface ToastItemProps {
	id: string;
	message: string;
	type: 'success' | 'error' | 'warning' | 'info';
	onRemove: (id: string) => void;
}

function ToastItem({ id, message, type, onRemove }: ToastItemProps) {
	const alertTypeClasses = {
		success: 'alert-success',
		error: 'alert-error',
		warning: 'alert-warning',
		info: 'alert-info',
	};

	return (
		<div className={cn('alert shadow-lg', alertTypeClasses[type])}>
			<span>{message}</span>
			<button
				type="button"
				className="btn btn-sm btn-circle btn-ghost"
				onClick={() => onRemove(id)}
				aria-label="Close toast">
				✕
			</button>
		</div>
	);
}

export default function ToastContainer() {
	const { toasts, removeToast } = useToast();

	// Cleanup on unmount
	useEffect(() => {
		return () => {
			// Optional: Clear all toasts when component unmounts
			// clearAll();
		};
	}, []);

	if (toasts.length === 0) {
		return null;
	}

	return (
		<div className="toast toast-top toast-center z-50">
			{toasts.map((toast) => (
				<ToastItem key={toast.id} id={toast.id} message={toast.message} type={toast.type} onRemove={removeToast} />
			))}
		</div>
	);
}
