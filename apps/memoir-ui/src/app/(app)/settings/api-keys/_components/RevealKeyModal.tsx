'use client';

import { Modal } from '@components';
import { AlertTriangle, Check, Copy } from 'lucide-react';
import { useEffect, useState } from 'react';

interface RevealKeyModalProps {
	plaintext: string | null;
	keyName: string;
	rotated: boolean;
	onClose: () => void;
}

export default function RevealKeyModal({ plaintext, keyName, rotated, onClose }: RevealKeyModalProps) {
	const [acknowledged, setAcknowledged] = useState(false);
	const [copied, setCopied] = useState(false);
	const [copyFailed, setCopyFailed] = useState(false);

	useEffect(() => {
		if (plaintext) {
			setAcknowledged(false);
			setCopied(false);
			setCopyFailed(false);
		}
	}, [plaintext]);

	async function copy() {
		if (!plaintext) return;
		try {
			await navigator.clipboard.writeText(plaintext);
			setCopied(true);
			setCopyFailed(false);
		} catch {
			setCopyFailed(true);
		}
	}

	return (
		<Modal open={plaintext !== null}>
			<div className="modal-box max-w-2xl">
				<h3 className="font-bold text-lg">
					{rotated ? 'New key for' : 'API key created:'} {keyName}
				</h3>

				<div role="alert" className="alert alert-warning my-4 text-sm">
					<AlertTriangle className="h-5 w-5 shrink-0" />
					<span>
						This is the only time this key will be shown. Memoir stores a hash, not the key itself, so it cannot be
						retrieved later. If you lose it, rotate the key to issue a new one.
						{rotated ? ' The previous key has been invalidated.' : ''}
					</span>
				</div>

				<div id="reveal_key__secret">
					<span className="label-text">Key</span>
					<code className="mt-1 block w-full select-all break-all rounded-lg bg-base-200 p-3 font-mono text-sm">
						{plaintext}
					</code>
				</div>

				{copyFailed && (
					<p className="mt-2 text-error text-sm">
						Could not access the clipboard. Select the key above and copy it manually.
					</p>
				)}

				<label htmlFor="reveal-key-acknowledge" className="label mt-4 cursor-pointer justify-start gap-2">
					<input
						id="reveal-key-acknowledge"
						type="checkbox"
						className="checkbox checkbox-sm checkbox-warning"
						checked={acknowledged}
						onChange={(e) => setAcknowledged(e.target.checked)}
					/>
					<span className="label-text">I have saved this key somewhere safe.</span>
				</label>

				<div id="reveal_key__actions" className="modal-action">
					<button type="button" className="btn btn-ghost gap-2" onClick={copy}>
						{copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
						{copied ? 'Copied' : 'Copy key'}
					</button>
					<button type="button" className="btn btn-primary" disabled={!acknowledged} onClick={onClose}>
						Done
					</button>
				</div>
			</div>
		</Modal>
	);
}
