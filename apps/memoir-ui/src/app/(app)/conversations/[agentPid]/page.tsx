import { getAgent } from '@actions/agents';
import { fetchConversations } from '@actions/infer';
import { getModels } from '@actions/models';
import { getProviders } from '@actions/providers';
import { getTools } from '@actions/tools';
import type { ChatSession } from '@lib/chat-state';
import { getTextContent } from '@lib/message';
import type { Metadata } from 'next';
import { notFound } from 'next/navigation';
import AgentConversationClient from './client';

export const metadata: Metadata = {
	title: 'Conversation',
};

interface PageProps {
	params: Promise<{ agentPid: string }>;
	searchParams: Promise<{ session?: string }>;
}

export default async function AgentConversationPage({ params, searchParams }: PageProps) {
	const { agentPid } = await params;
	const { session: initialSessionId } = await searchParams;

	const [agentResult, sessionsResult, modelsResult, providersResult, toolsResult] = await Promise.allSettled([
		getAgent(agentPid),
		fetchConversations({ agentPid }),
		getModels(),
		getProviders(),
		getTools(),
	]);

	// Agent is required - 404 if not found
	if (agentResult.status === 'rejected') {
		console.error('[AGENT] Failed to fetch agent:', agentResult.reason);
		notFound();
	}

	if (!agentResult.value.success) {
		console.error('[AGENT] Failed to fetch agent:', agentResult.value.error);
		notFound();
	}

	const agent = agentResult.value.data;

	// Map conversations to ChatSession format
	let sessions: ChatSession[] = [];
	if (
		sessionsResult.status === 'fulfilled' &&
		sessionsResult.value.success &&
		sessionsResult.value.data.conversations
	) {
		sessions = sessionsResult.value.data.conversations.map((c) => {
			const lastMessage = c.messages.length > 0 ? getTextContent(c.messages.at(-1)?.parts) : '';
			return {
				id: c.pid,
				title: c.title || 'Untitled',
				last_message: lastMessage,
				timestamp: c.lastMessageAt ? new Date(c.lastMessageAt).toISOString() : new Date().toISOString(),
				agent_id: c.agentPid,
			};
		});
	}

	const models =
		modelsResult.status === 'fulfilled' && modelsResult.value.success ? modelsResult.value.data : undefined;
	const providers =
		providersResult.status === 'fulfilled' && providersResult.value.success ? providersResult.value.data : undefined;
	const tools = toolsResult.status === 'fulfilled' && toolsResult.value.success ? toolsResult.value.data : undefined;

	if (sessionsResult.status === 'rejected') {
		console.error('[SESSIONS] Failed to fetch sessions:', sessionsResult.reason);
	}

	if (modelsResult.status === 'rejected') {
		console.error('[MODELS] Failed to fetch models:', modelsResult.reason);
	}

	if (providersResult.status === 'rejected') {
		console.error('[PROVIDERS] Failed to fetch providers:', providersResult.reason);
	}

	if (toolsResult.status === 'rejected') {
		console.error('[TOOLS] Failed to fetch tools:', toolsResult.reason);
	}

	return (
		<AgentConversationClient
			agent={agent}
			initialSessions={sessions}
			initialSessionId={initialSessionId}
			initialModels={models}
			initialProviders={providers}
			initialTools={tools}
		/>
	);
}
