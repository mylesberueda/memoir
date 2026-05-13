'use client';

import type { Agent } from '@actions/agents';
import { Bot, MessageSquare, Sparkles } from 'lucide-react';
import Link from 'next/link';

interface ConversationsClientProps {
	agents: Agent[];
}

function AgentChatCard({ agent }: { agent: Agent }) {
	return (
		<Link
			id="agent_chat_card__link"
			href={`/conversations/${agent.identifier.value}`}
			className="card bg-base-100 shadow transition-all duration-200 hover:shadow-lg hover:border-primary border border-transparent">
			<div className="card-body">
				<div id="agent_chat_card__content" className="flex items-start gap-4">
					<div
						id="agent_chat_card__avatar"
						className="w-12 h-12 rounded-full bg-gradient-to-br from-primary to-accent flex items-center justify-center flex-shrink-0">
						<Bot className="h-6 w-6 text-white" />
					</div>
					<div id="agent_chat_card__info" className="flex-1 min-w-0">
						<h3 className="card-title text-base-content text-lg">{agent.name}</h3>
						<p className="text-sm text-base-content/70 mt-1">{agent.model?.modelId}</p>
						{agent.systemPrompt && (
							<p className="text-sm text-base-content/60 mt-2 line-clamp-2">{agent.systemPrompt}</p>
						)}
					</div>
					<MessageSquare className="h-5 w-5 text-base-content/40 flex-shrink-0" />
				</div>
			</div>
		</Link>
	);
}

export default function ConversationsClient({ agents }: ConversationsClientProps) {
	if (agents.length === 0) {
		return (
			<div id="conversations_page__container" className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
				<div id="conversations_page__header" className="mb-8">
					<h1 className="text-3xl font-bold text-base-content">Conversations</h1>
					<p className="mt-2 text-base-content/70">Chat with your AI agents.</p>
				</div>

				<div id="conversations_page__empty" className="text-center py-16">
					<Bot className="mx-auto h-16 w-16 text-base-content/20 mb-6" />
					<h3 className="text-xl font-medium text-base-content mb-2">No agents yet</h3>
					<p className="text-base-content/70 mb-6 max-w-md mx-auto">
						Create an agent to start having conversations. Or use the Assistant for quick chats.
					</p>
					<div id="conversations_page__empty_actions" className="flex gap-4 justify-center">
						<Link href="/agents" className="btn btn-primary">
							Create Agent
						</Link>
						<Link href="/assistant" className="btn btn-outline">
							<Sparkles className="h-4 w-4 mr-2" />
							Use Assistant
						</Link>
					</div>
				</div>
			</div>
		);
	}

	return (
		<div id="conversations_page__container" className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div id="conversations_page__header" className="mb-8 flex items-center justify-between">
				<div>
					<h1 className="text-3xl font-bold text-base-content">Conversations</h1>
					<p className="mt-2 text-base-content/70">Select an agent to start or continue a conversation.</p>
				</div>
				<Link href="/assistant" className="btn btn-primary">
					<Sparkles className="mr-2 h-4 w-4" />
					Assistant
				</Link>
			</div>

			<div id="conversations_page__grid" className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
				{agents.map((agent) => (
					<AgentChatCard key={agent.identifier.value} agent={agent} />
				))}
			</div>
		</div>
	);
}
