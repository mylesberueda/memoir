import type { Metadata } from 'next';

import AuditClient from './client';

export const metadata: Metadata = {
	title: 'Supersession audit | Memory',
};

export default function AuditPage() {
	return <AuditClient />;
}
