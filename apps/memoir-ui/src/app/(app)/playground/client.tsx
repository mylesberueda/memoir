'use client';

import { AgentIdInput } from '@components';
import useAgentIds from '@hooks/useAgentIds';
import { Send } from 'lucide-react';
import { useRef, useState } from 'react';

interface ChatTurn {
	role: 'user' | 'assistant';
	content: string;
}

export default function PlaygroundClient() {
	const [agentId, setAgentId] = useState('playground');
	const agents = useAgentIds();
	const [input, setInput] = useState('');
	const [history, setHistory] = useState<ChatTurn[]>([]);
	const [streaming, setStreaming] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const scrollRef = useRef<HTMLDivElement>(null);

	async function send() {
		const message = input.trim();
		if (!message || streaming) return;
		setInput('');
		setError(null);

		const priorHistory = history;
		setHistory([...priorHistory, { role: 'user', content: message }, { role: 'assistant', content: '' }]);
		setStreaming(true);

		try {
			const response = await fetch('/api/playground/chat', {
				method: 'POST',
				headers: { 'content-type': 'application/json' },
				body: JSON.stringify({ message, history: priorHistory, agentId }),
			});
			if (!response.ok || !response.body) {
				const text = await response.text().catch(() => 'request failed');
				throw new Error(text || `HTTP ${response.status}`);
			}

			const reader = response.body.getReader();
			const decoder = new TextDecoder();
			let buffer = '';

			while (true) {
				const { done, value } = await reader.read();
				if (done) break;
				buffer += decoder.decode(value, { stream: true });
				let nl: number;
				// biome-ignore lint/suspicious/noAssignInExpressions: SSE frame loop
				while ((nl = buffer.indexOf('\n\n')) !== -1) {
					const frame = buffer.slice(0, nl);
					buffer = buffer.slice(nl + 2);
					const dataLine = frame.split('\n').find((line) => line.startsWith('data:'));
					if (!dataLine) continue;
					const token = dataLine.slice(5).replace(/^ /, '');
					setHistory((h) => {
						const next = h.slice();
						const last = next[next.length - 1];
						if (last && last.role === 'assistant') {
							next[next.length - 1] = { role: 'assistant', content: last.content + token };
						}
						return next;
					});
					scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight });
				}
			}
		} catch (err) {
			setError(err instanceof Error ? err.message : 'Chat failed');
		} finally {
			setStreaming(false);
		}
	}

	return (
		<div className="mx-auto flex h-full max-w-4xl flex-col px-4 py-8 sm:px-6">
			<div className="mb-5 flex flex-wrap items-start justify-between gap-4">
				<div className="min-w-0">
					<p className="mb-2 font-medium text-[0.6875rem] text-primary uppercase tracking-[0.16em]">Playground</p>
					<h1 className="font-display text-3xl text-base-content">Chat</h1>
					<p className="mt-2 max-w-xl text-base-content/60 text-sm leading-relaxed">
						Chat with an agent that uses memoir as memory. Each turn pulls context via Query and writes the user message
						back via Remember — watch the timeline / query / audit views populate as you talk.
					</p>
				</div>
				<label htmlFor="playground-agent-id" className="flex shrink-0 items-center gap-2 text-sm">
					<span className="text-base-content/60">Agent</span>
					<div className="w-40">
						<AgentIdInput
							id="playground-agent-id"
							value={agentId}
							onChange={setAgentId}
							agents={agents}
							disabled={streaming}
							className="input-sm font-mono"
						/>
					</div>
				</label>
			</div>

			<div
				id="playground-messages"
				ref={scrollRef}
				className="min-h-0 flex-1 space-y-3 overflow-y-auto rounded-box border border-base-300 bg-base-100 p-4">
				{history.length === 0 && (
					<p className="text-center text-base-content/40 text-sm">No messages yet. Say something.</p>
				)}
				{history.map((turn, idx) => (
					<div
						// biome-ignore lint/suspicious/noArrayIndexKey: chat turn order is stable for the session
						key={idx}
						className={`flex ${turn.role === 'user' ? 'justify-end' : 'justify-start'}`}>
						<div
							className={`max-w-[80%] whitespace-pre-wrap rounded-box px-4 py-2 text-sm ${
								turn.role === 'user' ? 'bg-primary text-primary-content' : 'bg-base-200 text-base-content'
							}`}>
							{turn.content || (streaming && turn.role === 'assistant' ? '...' : '')}
						</div>
					</div>
				))}
			</div>

			{error && (
				<div className="alert alert-error mt-4">
					<span>{error}</span>
				</div>
			)}

			<form
				id="playground-input"
				className="mt-4 flex gap-2"
				onSubmit={(e) => {
					e.preventDefault();
					send();
				}}>
				<input
					type="text"
					className="input input-bordered flex-1"
					placeholder="ask a question..."
					value={input}
					disabled={streaming}
					onChange={(e) => setInput(e.target.value)}
				/>
				<button type="submit" className="btn btn-primary" disabled={streaming || !input.trim()}>
					{streaming ? <span className="loading loading-spinner loading-sm" /> : <Send className="h-4 w-4" />}
				</button>
			</form>
		</div>
	);
}
