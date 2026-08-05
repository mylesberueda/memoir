'use client';

import { Modal } from '@components';
import type { ApiKey } from '@polypixel/memoir-sdk/memoir/v1/auth_pb';
import { AlertTriangle } from 'lucide-react';
import { useEffect, useState } from 'react';

export type KeyAction = 'rotate' | 'revoke';

interface ConfirmKeyActionModalProps {
	action: KeyAction;
	apiKey: ApiKey | null;
	isPending: boolean;
	onConfirm: () => void;
	onClose: () => void;
}

const COPY: Record<KeyAction, { title: string; warning: string; confirm: string; button: string; style: string }> = {
	rotate: {
		title: 'Regenerate API key',
		warning:
			'The current key stops working immediately. Any service still using it will start failing authentication until you deploy the new key.',
		confirm: 'I understand the current key will stop working.',
		button: 'Regenerate key',
		style: 'btn-warning',
	},
	revoke: {
		title: 'Revoke API key',
		warning:
			'The key stops working immediately and cannot be reactivated. The record is retained for audit. Create a new key if you need to restore access.',
		confirm: 'I understand this cannot be undone.',
		button: 'Revoke key',
		style: 'btn-error',
	},
};

export default function ConfirmKeyActionModal({
	action,
	apiKey,
	isPending,
	onConfirm,
	onClose,
}: ConfirmKeyActionModalProps) {
	const [confirmed, setConfirmed] = useState(false);
	const copy = COPY[action];

	useEffect(() => {
		if (apiKey) setConfirmed(false);
	}, [apiKey]);

	return (
		<Modal open={apiKey !== null}>
			<div className="modal-box max-w-lg">
				<h3 className="font-bold text-lg">{copy.title}</h3>
				<p className="mt-2 text-base-content/70 text-sm">
					<span className="font-medium">{apiKey?.name}</span>{' '}
					<code className="font-mono text-xs">mk.{apiKey?.keyId}</code>
				</p>

				<div role="alert" className="alert alert-warning my-4 text-sm">
					<AlertTriangle className="h-5 w-5 shrink-0" />
					<span>{copy.warning}</span>
				</div>

				<label htmlFor="confirm-key-action" className="label cursor-pointer justify-start gap-2">
					<input
						id="confirm-key-action"
						type="checkbox"
						className="checkbox checkbox-sm checkbox-warning"
						checked={confirmed}
						disabled={isPending}
						onChange={(e) => setConfirmed(e.target.checked)}
					/>
					<span className="label-text">{copy.confirm}</span>
				</label>

				<div className="modal-action">
					<button type="button" className="btn btn-ghost" disabled={isPending} onClick={onClose}>
						Cancel
					</button>
					<button type="button" className={`btn ${copy.style}`} disabled={!confirmed || isPending} onClick={onConfirm}>
						{isPending ? (
							<>
								<span className="loading loading-spinner loading-sm" />
								Working...
							</>
						) : (
							copy.button
						)}
					</button>
				</div>
			</div>
		</Modal>
	);
}
