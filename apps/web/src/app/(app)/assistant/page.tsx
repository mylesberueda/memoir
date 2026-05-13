import { getUserAssistant } from '@actions/agents';
import { fetchConversations } from '@actions/infer';
import { getModels } from '@actions/models';
import type { ChatSession } from '@lib/chat-state';
import { getTextContent } from '@lib/message';
import type { Metadata } from 'next';
import AssistantClient from './client';

export const metadata: Metadata = {
	title: 'Assistant',
};

export default async function AssistantIndex() {
	// Fetch initial data on the server
	const [assistantResult, modelsResult] = await Promise.allSettled([getUserAssistant(), getModels()]);

	const sessionsResult = await fetchConversations({
		agentPid:
			(assistantResult.status === 'fulfilled' &&
				assistantResult.value.success &&
				assistantResult.value.data.agent.identifier.value) ||
			undefined,
	});

	// Extract the data or use fallbacks - map Thread to ChatSession
	let sessions: ChatSession[] = [];
	if (sessionsResult.success && sessionsResult.data.conversations) {
		sessions = sessionsResult.data.conversations.map((c) => {
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

	const assistant =
		assistantResult.status === 'fulfilled' && assistantResult.value.success ? assistantResult.value.data : undefined;

	const models =
		modelsResult.status === 'fulfilled' && modelsResult.value.success ? modelsResult.value.data : undefined;

	if (assistantResult.status === 'rejected') {
		console.error('[ASSISTANT] Failed to fetch assistant:', assistantResult.reason);
	}

	if (modelsResult.status === 'rejected') {
		console.error('[MODELS] Failed to fetch models:', modelsResult.reason);
	}

	return <AssistantClient initialSessions={sessions} initialAssistant={assistant} initialModels={models} />;
}
