'use client';

import type { ActionResult } from '@actions';
import {
	createProvider,
	deleteProvider,
	type ListProvidersResponse,
	type Provider,
	updateProvider,
} from '@actions/providers';
import { Button } from '@components';
import useToast from '@hooks/useToast';
import { Plus } from 'lucide-react';
import { useCallback, useEffect, useState, useTransition } from 'react';
import { useSettingsPage } from '../_components/SettingsPageContext';
import { type ProviderFormData, ProviderList, ProviderModal } from './_components';

interface ProvidersClientProps {
	providers?: ActionResult<ListProvidersResponse>;
}

export default function ProvidersClient({ providers }: ProvidersClientProps) {
	const { setHeaderConfig } = useSettingsPage();
	const { error: showError } = useToast();
	const [modalState, setModalState] = useState<{
		isOpen: boolean;
		mode: 'create' | 'edit';
		provider?: Provider;
	}>({ isOpen: false, mode: 'create' });
	const [isPending, startTransition] = useTransition();

	const handleCreateProvider = useCallback(() => {
		setModalState({ isOpen: true, mode: 'create' });
	}, []);

	useEffect(() => {
		setHeaderConfig({
			title: 'Providers',
			description: 'Manage your LLM providers and API keys.',
			actions: (
				<Button type="button" className="btn btn-primary" onClick={handleCreateProvider}>
					<Plus className="mr-2 h-4 w-4" />
					Add Provider
				</Button>
			),
		});

		return () => setHeaderConfig(null);
	}, [setHeaderConfig, handleCreateProvider]);

	const handleEditProvider = (provider: Provider) => {
		setModalState({ isOpen: true, mode: 'edit', provider });
	};

	const handleDeleteProvider = (provider: Provider) => {
		if (!confirm(`Are you sure you want to delete "${provider.name}"?`)) {
			return;
		}

		startTransition(async () => {
			const result = await deleteProvider(provider.identifier.value);

			if (!result.success) {
				showError(result.error || 'Failed to delete provider');
			}
		});
	};

	const handleProviderSubmit = async (data: ProviderFormData) => {
		startTransition(async () => {
			let result: ActionResult<Provider> | undefined;

			if (modalState.mode === 'create') {
				result = await createProvider({
					name: data.name,
					providerType: data.provider_type,
					credentials: data.credentials || undefined,
					endpointUrl: data.endpoint_url || undefined,
				});
			} else if (modalState.mode === 'edit' && modalState.provider) {
				result = await updateProvider({
					pid: modalState.provider.identifier.value,
					name: data.name,
					providerType: data.provider_type,
					credentials: data.credentials || undefined,
					endpointUrl: data.endpoint_url || undefined,
				});
			}

			if (result && !result.success) {
				showError(result.error || `Failed to ${modalState.mode} provider`);
				return;
			}

			setModalState({ isOpen: false, mode: 'create' });
		});
	};

	return (
		<div className="pb-20">
			<ProviderList
				providers={providers}
				isLoading={false}
				onRefresh={() => window.location.reload()}
				onEdit={handleEditProvider}
				onDelete={handleDeleteProvider}
			/>
			<ProviderModal
				isOpen={modalState.isOpen}
				onClose={() => setModalState({ isOpen: false, mode: 'create' })}
				mode={modalState.mode}
				provider={modalState.provider}
				onSubmit={handleProviderSubmit}
				isSubmitting={isPending}
			/>
		</div>
	);
}
