import 'server-only';

import type { Scope } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';
import type { ActionResult } from '@/actions';
import { getAccessToken } from '@/actions/auth';
import { getOrganizationContext } from '@/lib/grpc/transport';
import { getSession } from '@/lib/session';

export type Resolved = { ok: true; scope: Scope; accessToken: string } | { ok: false; failure: ActionResult<never> };

export async function resolveScopeAndToken(agentId: string): Promise<Resolved> {
	const session = await getSession();
	if (!session) {
		return { ok: false, failure: { success: false, error: 'Not authenticated' } };
	}

	const orgId = (await getOrganizationContext()) ?? session.userId;

	const trimmedAgentId = agentId.trim();
	if (!trimmedAgentId) {
		return { ok: false, failure: { success: false, error: 'Agent id is required' } };
	}

	const accessToken = await getAccessToken();
	if (!accessToken) {
		return { ok: false, failure: { success: false, error: 'Not authenticated' } };
	}

	return {
		ok: true,
		scope: { agentId: trimmedAgentId, orgId, userId: session.userId } as Scope,
		accessToken,
	};
}
