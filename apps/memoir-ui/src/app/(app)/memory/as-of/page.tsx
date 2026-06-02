import type { Metadata } from 'next';

import AsOfClient from './client';

export const metadata: Metadata = {
	title: 'Point-in-time | Memory',
};

export default function AsOfPage() {
	return <AsOfClient />;
}
