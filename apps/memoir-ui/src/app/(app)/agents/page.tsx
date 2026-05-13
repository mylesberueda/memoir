import { getAgents } from '@actions/agents';
import { getModels } from '@actions/models';
import { getProviders } from '@actions/providers';
import { getTools } from '@actions/tools';
import type { Metadata } from 'next';
import AgentsClient from './client';

export const metadata: Metadata = {
	title: 'Agents',
};

export default async function AgentsPage() {
	const [agents, models, providers, tools] = await Promise.all([getAgents(), getModels(), getProviders(), getTools()]);

	return <AgentsClient agents={agents} models={models} providers={providers} tools={tools} />;
}
