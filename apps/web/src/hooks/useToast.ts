import { useCallback, useEffect, useState } from 'react';

export interface Toast {
	id: string;
	message: string;
	type: 'success' | 'error' | 'warning' | 'info';
	duration?: number;
}

export interface UseToastReturn {
	toasts: Toast[];
	addToast: (message: string, type?: Toast['type'], duration?: number) => void;
	removeToast: (id: string) => void;
	success: (message: string, duration?: number) => void;
	error: (message: string, duration?: number) => void;
	warning: (message: string, duration?: number) => void;
	info: (message: string, duration?: number) => void;
	clearAll: () => void;
}

// Global toast state (simple singleton pattern for cross-component usage)
let globalToasts: Toast[] = [];
const listeners = new Set<(toasts: Toast[]) => void>();

function notifyListeners() {
	for (const listener of listeners) {
		listener([...globalToasts]);
	}
}

function addGlobalToast(toast: Toast) {
	globalToasts = [...globalToasts, toast];
	notifyListeners();

	// Auto-remove toast after duration
	if (toast.duration && toast.duration > 0) {
		setTimeout(() => {
			removeGlobalToast(toast.id);
		}, toast.duration);
	}
}

function removeGlobalToast(id: string) {
	globalToasts = globalToasts.filter((toast) => toast.id !== id);
	notifyListeners();
}

function clearAllGlobalToasts() {
	globalToasts = [];
	notifyListeners();
}

export default function useToast(): UseToastReturn {
	const [toasts, setToasts] = useState<Toast[]>(globalToasts);

	// Subscribe to global toast changes
	const updateToasts = useCallback((newToasts: Toast[]) => {
		setToasts(newToasts);
	}, []);

	// Subscribe on mount, unsubscribe on unmount
	useEffect(() => {
		listeners.add(updateToasts);
		return () => {
			listeners.delete(updateToasts);
		};
	}, [updateToasts]);

	const addToast = useCallback((message: string, type: Toast['type'] = 'info', duration = 4000) => {
		const id = `toast_${Date.now()}_${Math.random().toString(36).slice(2)}`;
		const toast: Toast = { id, message, type, duration };
		addGlobalToast(toast);
	}, []);

	const removeToast = useCallback((id: string) => {
		removeGlobalToast(id);
	}, []);

	const success = useCallback(
		(message: string, duration?: number) => {
			addToast(message, 'success', duration);
		},
		[addToast],
	);

	const error = useCallback(
		(message: string, duration?: number) => {
			addToast(message, 'error', duration);
		},
		[addToast],
	);

	const warning = useCallback(
		(message: string, duration?: number) => {
			addToast(message, 'warning', duration);
		},
		[addToast],
	);

	const info = useCallback(
		(message: string, duration?: number) => {
			addToast(message, 'info', duration);
		},
		[addToast],
	);

	const clearAll = useCallback(() => {
		clearAllGlobalToasts();
	}, []);

	return {
		toasts,
		addToast,
		removeToast,
		success,
		error,
		warning,
		info,
		clearAll,
	};
}
