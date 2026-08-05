'use client';

import { createApiKey } from '@actions/api-keys';
import { Modal } from '@components';
import useToast from '@hooks/useToast';
import { type ApiKey, ApiKeyRole } from '@polypixel/memoir-sdk/memoir/v1/auth_pb';
import { useEffect, useState, useTransition } from 'react';

interface CreateKeyModalProps {
	open: boolean;
	onClose: () => void;
	onCreated: (key: ApiKey, plaintext: string) => void;
}

export default function CreateKeyModal({ open, onClose, onCreated }: CreateKeyModalProps) {
	const [name, setName] = useState('');
	const [role, setRole] = useState<ApiKeyRole>(ApiKeyRole.INTEGRATION);
	const [orgId, setOrgId] = useState('');
	const [isPending, startTransition] = useTransition();
	const { error: showError } = useToast();

	useEffect(() => {
		if (open) {
			setName('');
			setRole(ApiKeyRole.INTEGRATION);
			setOrgId('');
		}
	}, [open]);

	function submit() {
		const trimmedName = name.trim();
		if (!trimmedName) {
			showError('Name is required');
			return;
		}
		startTransition(async () => {
			const result = await createApiKey(trimmedName, role, orgId);
			if (!result.success) {
				showError(result.error);
				return;
			}
			onCreated(result.data.key, result.data.plaintext);
			onClose();
		});
	}

	return (
		<Modal open={open}>
			<div className="modal-box max-w-2xl">
				<h3 className="font-bold text-lg">Create API key</h3>

				<form
					id="create-key-form"
					className="mt-4 space-y-4"
					onSubmit={(e) => {
						e.preventDefault();
						submit();
					}}>
					<div>
						<label htmlFor="create-key-name" className="label">
							<span className="label-text">Name</span>
						</label>
						<input
							id="create-key-name"
							type="text"
							className="input input-bordered w-full"
							placeholder="ci-runner"
							value={name}
							disabled={isPending}
							onChange={(e) => setName(e.target.value)}
						/>
					</div>

					<div>
						<label htmlFor="create-key-role" className="label">
							<span className="label-text">Role</span>
						</label>
						<select
							id="create-key-role"
							className="select select-bordered w-full"
							value={role}
							disabled={isPending}
							onChange={(e) => setRole(Number(e.target.value) as ApiKeyRole)}>
							<option value={ApiKeyRole.INTEGRATION}>Integration — memory RPCs only</option>
							<option value={ApiKeyRole.ADMIN}>Admin — full access, including key management</option>
						</select>
					</div>

					<div>
						<label htmlFor="create-key-org" className="label">
							<span className="label-text">Organization id (optional)</span>
						</label>
						<input
							id="create-key-org"
							type="text"
							className="input input-bordered w-full"
							placeholder="Leave blank for an unscoped key"
							value={orgId}
							disabled={isPending}
							onChange={(e) => setOrgId(e.target.value)}
						/>
					</div>

					<div className="modal-action">
						<button type="button" className="btn btn-ghost" disabled={isPending} onClick={onClose}>
							Cancel
						</button>
						<button type="submit" className="btn btn-primary" disabled={isPending}>
							{isPending ? (
								<>
									<span className="loading loading-spinner loading-sm" />
									Creating...
								</>
							) : (
								'Create key'
							)}
						</button>
					</div>
				</form>
			</div>
		</Modal>
	);
}
