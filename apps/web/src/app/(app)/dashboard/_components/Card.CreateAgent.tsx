'use client';

import { Button, Card } from '@components';
import { Bot, Plus } from 'lucide-react';

export default function CreateAgentCard() {
	return (
		<Card
			icon={Bot}
			title="Create a new AI agent"
			description="Build custom AI agents to automate tasks, answer questions, and process data"
			action={
				<Button className="w-full" color="primary">
					<Plus className="mr-2 h-4 w-4" />
					<span>Create New Agent</span>
				</Button>
			}>
			<div className="flex flex-col items-center justify-center space-y-4 py-6">
				<div className="rounded-full bg-base-300 p-4">
					<Plus className="h-8 w-8 bg-base-300 text-base-content" />
				</div>
				<p className="max-w-md text-center text-base-content text-sm">
					Create a new AI agent with custom capabilities, knowledge base, and integration options
				</p>
			</div>
		</Card>
	);
}
