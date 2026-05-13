import { getModels } from '@actions/models';
import type { Metadata } from 'next';

import ChatClient from './client';

export const metadata: Metadata = {
	title: 'Chat',
};

export default async function ChatIndex() {
	// Fetch models data on server side
	const modelsResult = await getModels();
	const result = modelsResult.success
		? { models: modelsResult.data, error: undefined }
		: { models: undefined, error: new Error(modelsResult.error) };

	return <ChatClient models={result.models} modelsError={result.error} />;
}
