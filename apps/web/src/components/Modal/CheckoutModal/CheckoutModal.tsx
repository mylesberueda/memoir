'use client';

import { Modal } from '@components';
import { CheckCircle, X, XCircle } from 'lucide-react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useCallback, useEffect, useState } from 'react';

type CheckoutStatus = 'success' | 'cancelled' | null;

interface CheckoutModalProps {
	onClose?: () => void;
}

export default function CheckoutModal({ onClose }: CheckoutModalProps) {
	const router = useRouter();
	const searchParams = useSearchParams();
	const [status, setStatus] = useState<CheckoutStatus>(null);

	useEffect(() => {
		const checkoutParam = searchParams.get('checkout');
		if (checkoutParam === 'success' || checkoutParam === 'cancelled') {
			setStatus(checkoutParam);
		}
	}, [searchParams]);

	const handleClose = useCallback(() => {
		setStatus(null);
		// Remove query params from URL without navigation
		const url = new URL(window.location.href);
		url.searchParams.delete('checkout');
		url.searchParams.delete('session_id');
		router.replace(url.pathname, { scroll: false });
		onClose?.();
	}, [router, onClose]);

	if (!status) return null;

	const isSuccess = status === 'success';

	return (
		<Modal open={true}>
			<div id="checkout_modal__container" className="modal-box max-w-md text-center">
				<button
					type="button"
					className="btn btn-circle btn-ghost btn-sm absolute right-4 top-4"
					onClick={handleClose}
					aria-label="Close">
					<X className="h-4 w-4" />
				</button>

				<div id="checkout_modal__icon" className="mb-4 flex justify-center pt-2">
					{isSuccess ? (
						<div data-testid="checkout-modal-icon-container" className="rounded-full bg-success/10 p-4">
							<CheckCircle className="h-12 w-12 text-success" />
						</div>
					) : (
						<div data-testid="checkout-modal-icon-container" className="rounded-full bg-warning/10 p-4">
							<XCircle className="h-12 w-12 text-warning" />
						</div>
					)}
				</div>

				<h3 id="checkout_modal__title" className="mb-2 text-xl font-bold">
					{isSuccess ? 'Welcome to your new plan!' : 'Checkout cancelled'}
				</h3>

				<p id="checkout_modal__description" className="mb-6 text-base-content/70">
					{isSuccess
						? 'Your subscription is now active. Enjoy your upgraded features!'
						: 'No worries! You can upgrade anytime from the settings page.'}
				</p>

				<div id="checkout_modal__actions" className="modal-action justify-center">
					<button type="button" className="btn btn-primary" onClick={handleClose}>
						{isSuccess ? 'Get started' : 'Close'}
					</button>
				</div>
			</div>
		</Modal>
	);
}
