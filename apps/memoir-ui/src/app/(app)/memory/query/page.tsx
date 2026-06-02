import type { Metadata } from 'next';

import QueryClient from './client';

export const metadata: Metadata = {
	title: 'Query | Memory',
};

export default function QueryPage() {
	return <QueryClient />;
}
