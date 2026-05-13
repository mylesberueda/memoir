import type { Metadata } from 'next';
import SecurityClient from './client';

export const metadata: Metadata = {
	title: 'Security | Settings',
};

export default function SecuritySettingsPage() {
	return <SecurityClient />;
}
