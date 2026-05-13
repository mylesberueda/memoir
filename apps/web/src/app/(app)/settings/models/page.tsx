import { getModels } from '@actions/models';
import { getProviders } from '@actions/providers';
import type { Metadata } from 'next';
import ModelsClient from './client';

export const metadata: Metadata = {
	title: 'Models | Settings',
};

export default async function ModelsSettingsPage() {
	const [models, providers] = await Promise.all([getModels(), getProviders()]);

	return <ModelsClient models={models} providers={providers} />;
}
