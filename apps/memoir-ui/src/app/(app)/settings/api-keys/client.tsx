'use client';

import { listApiKeys, revokeApiKey, rotateApiKey } from '@actions/api-keys';
import { Button } from '@components';
import useToast from '@hooks/useToast';
import type { ApiKey } from '@polypixel/memoir-sdk/memoir/v1/auth_pb';
import { Plus } from 'lucide-react';
import { useCallback, useEffect, useState, useTransition } from 'react';

import { useSettingsPage } from '../_components/SettingsPageContext';
import ApiKeyList from './_components/ApiKeyList';
import ConfirmKeyActionModal, { type KeyAction } from './_components/ConfirmKeyActionModal';
import CreateKeyModal from './_components/CreateKeyModal';
import RevealKeyModal from './_components/RevealKeyModal';

interface RevealState {
	plaintext: string;
	keyName: string;
	rotated: boolean;
}

export default function ApiKeysClient() {
	const { setHeaderConfig } = useSettingsPage();
	const { success, error: showError } = useToast();

	const [keys, setKeys] = useState<ApiKey[]>([]);
	const [loading, setLoading] = useState(true);
	const [createOpen, setCreateOpen] = useState(false);
	const [reveal, setReveal] = useState<RevealState | null>(null);
	const [pendingAction, setPendingAction] = useState<{ action: KeyAction; key: ApiKey } | null>(null);
	const [isPending, startTransition] = useTransition();

	useEffect(() => {
		setHeaderConfig({
			title: 'API Keys',
			description: 'Issue and manage the keys downstream services use to authenticate against Memoir.',
		});

		return () => setHeaderConfig(null);
	}, [setHeaderConfig]);

	const refresh = useCallback(async () => {
		const result = await listApiKeys();
		if (!result.success) {
			showError(result.error);
			setLoading(false);
			return;
		}
		setKeys(result.data.keys);
		setLoading(false);
	}, [showError]);

	useEffect(() => {
		void refresh();
	}, [refresh]);

	function confirmAction() {
		if (!pendingAction) return;
		const { action, key } = pendingAction;

		startTransition(async () => {
			if (action === 'rotate') {
				const result = await rotateApiKey(key.pid);
				if (!result.success) {
					showError(result.error);
					return;
				}
				setPendingAction(null);
				setReveal({ plaintext: result.data.plaintext, keyName: result.data.key.name, rotated: true });
			} else {
				const result = await revokeApiKey(key.pid);
				if (!result.success) {
					showError(result.error);
					return;
				}
				setPendingAction(null);
				success(`Revoked ${key.name}`);
			}
			await refresh();
		});
	}

	return (
		<div id="api_keys__container">
			<div id="api_keys__header" className="mb-4 flex items-center justify-between gap-4">
				<p className="text-base-content/70 text-sm">
					Keys are shown once at creation. Memoir stores only a hash, so a lost key must be regenerated rather than
					recovered.
				</p>
				<Button color="primary" onClick={() => setCreateOpen(true)}>
					<Plus className="h-4 w-4" />
					New key
				</Button>
			</div>

			{loading ? (
				<div className="flex justify-center p-10">
					<span className="loading loading-spinner" />
				</div>
			) : (
				<ApiKeyList
					keys={keys}
					busyPid={isPending ? (pendingAction?.key.pid ?? null) : null}
					onRotate={(key) => setPendingAction({ action: 'rotate', key })}
					onRevoke={(key) => setPendingAction({ action: 'revoke', key })}
				/>
			)}

			<CreateKeyModal
				open={createOpen}
				onClose={() => setCreateOpen(false)}
				onCreated={(key, plaintext) => {
					setReveal({ plaintext, keyName: key.name, rotated: false });
					void refresh();
				}}
			/>

			<ConfirmKeyActionModal
				action={pendingAction?.action ?? 'rotate'}
				apiKey={pendingAction?.key ?? null}
				isPending={isPending}
				onConfirm={confirmAction}
				onClose={() => setPendingAction(null)}
			/>

			<RevealKeyModal
				plaintext={reveal?.plaintext ?? null}
				keyName={reveal?.keyName ?? ''}
				rotated={reveal?.rotated ?? false}
				onClose={() => setReveal(null)}
			/>
		</div>
	);
}
