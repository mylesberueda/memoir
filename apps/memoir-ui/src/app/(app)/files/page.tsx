import type { Metadata } from 'next';

import FilesClient from './client';

export const metadata: Metadata = {
	title: 'Files',
};

export default function FilesPage() {
	return <FilesClient />;
}
