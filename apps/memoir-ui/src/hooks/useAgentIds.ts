'use client';

import { listAgents } from '@actions/agents';
import { useEffect, useState } from 'react';

/**
 * Fetches the caller's distinct agent ids once on mount. Returns an empty
 * list (never throws) on failure — the agent picker degrades to free-text.
 */
export default function useAgentIds(): string[] {
	const [agents, setAgents] = useState<string[]>([]);

	useEffect(() => {
		let cancelled = false;
		listAgents().then((res) => {
			if (!cancelled && res.success) setAgents(res.data.agentIds);
		});
		return () => {
			cancelled = true;
		};
	}, []);

	return agents;
}
