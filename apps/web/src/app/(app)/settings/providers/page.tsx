import { getProviders } from '@actions/providers';
import type { Metadata } from 'next';
import ProvidersClient from './client';

export const metadata: Metadata = {
	title: 'Providers | Settings',
};

export default async function ProvidersSettingsPage() {
	const providers = await getProviders();

	return <ProvidersClient providers={providers} />;
}
