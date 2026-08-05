import type { Metadata } from 'next';
import ApiKeysClient from './client';

export const metadata: Metadata = {
	title: 'API Keys | Settings',
};

export default function ApiKeysSettingsPage() {
	return <ApiKeysClient />;
}
