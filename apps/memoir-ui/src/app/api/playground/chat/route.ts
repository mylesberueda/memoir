import { getAccessToken } from '@/actions/auth';
import { getOrganizationContext } from '@/lib/grpc/transport';
import { getSession } from '@/lib/session';

export const dynamic = 'force-dynamic';

interface ProxyBody {
	message: string;
	history?: Array<{ role: 'user' | 'assistant'; content: string }>;
	agentId?: string;
}

export async function POST(request: Request) {
	const httpUrl = process.env.HTTP_URL;
	if (!httpUrl) {
		return new Response('HTTP_URL is not set', { status: 503 });
	}

	const session = await getSession();
	if (!session) return new Response('Not authenticated', { status: 401 });

	const orgId = (await getOrganizationContext()) ?? session.userId;

	const accessToken = await getAccessToken();
	if (!accessToken) return new Response('Not authenticated', { status: 401 });

	const body = (await request.json()) as ProxyBody;
	if (!body.message?.trim()) return new Response('message: required', { status: 400 });

	const agentId = (body.agentId ?? 'playground').trim() || 'playground';

	const upstream = await fetch(`${httpUrl}/playground/chat`, {
		method: 'POST',
		headers: {
			'content-type': 'application/json',
			authorization: `Bearer ${accessToken}`,
		},
		body: JSON.stringify({
			scope: { agent_id: agentId, org_id: orgId, user_id: session.userId },
			message: body.message,
			history: body.history ?? [],
		}),
	});

	if (!upstream.ok || !upstream.body) {
		const text = await upstream.text().catch(() => 'upstream failed');
		return new Response(text, { status: upstream.status });
	}

	return new Response(upstream.body, {
		status: 200,
		headers: {
			'content-type': 'text/event-stream',
			'cache-control': 'no-cache',
			connection: 'keep-alive',
		},
	});
}
