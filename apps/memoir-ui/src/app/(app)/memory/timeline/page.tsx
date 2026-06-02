import type { Metadata } from 'next';

import TimelineClient from './client';

export const metadata: Metadata = {
	title: 'Timeline | Memory',
};

export default function TimelinePage() {
	return <TimelineClient />;
}
