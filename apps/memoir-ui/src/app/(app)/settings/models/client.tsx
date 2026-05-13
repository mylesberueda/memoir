'use client';

import type { ActionResult } from '@actions';
import type { ListModelsResponse } from '@actions/models';
import type { ListProvidersResponse } from '@actions/providers';
import { useEffect } from 'react';
import { useSettingsPage } from '../_components/SettingsPageContext';
import { ModelFilters, ModelList } from './_components';

interface ModelsClientProps {
	models?: ActionResult<ListModelsResponse>;
	providers?: ActionResult<ListProvidersResponse>;
}

export default function ModelsClient({ models, providers }: ModelsClientProps) {
	const { setHeaderConfig } = useSettingsPage();

	useEffect(() => {
		setHeaderConfig({
			title: 'Models',
			description: 'View available LLM models across your providers.',
		});

		return () => setHeaderConfig(null);
	}, [setHeaderConfig]);

	return (
		<div className="space-y-4 pb-20">
			<ModelFilters providers={providers} />
			<ModelList models={models} providers={providers} />
		</div>
	);
}
