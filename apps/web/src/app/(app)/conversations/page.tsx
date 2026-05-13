import { getAgents } from '@actions/agents';
import type { Metadata } from 'next';
import ConversationsClient from './client';

export const metadata: Metadata = {
	title: 'Conversations',
};

export default async function ConversationsPage() {
	const agentsResult = await getAgents();

	const agents = agentsResult.success ? agentsResult.data.agents : [];

	if (!agentsResult.success) {
		console.error('[CONVERSATIONS] Failed to fetch agents:', agentsResult.error);
	}

	return <ConversationsClient agents={agents} />;
}
