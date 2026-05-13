import { createChildLogger } from '@lib/logger';
import { type NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

const log = createChildLogger({ route: 'api/webhooks/stripe' });

const UPSTREAM_TIMEOUT_MS = 8000;

export async function POST(request: NextRequest) {
	const signature = request.headers.get('stripe-signature');
	if (!signature) {
		return NextResponse.json({ error: 'Missing Stripe-Signature header' }, { status: 400 });
	}

	const apiServiceUrl = process.env.API_SERVICE_URL;
	if (!apiServiceUrl) {
		throw new Error('API_SERVICE_URL is not set');
	}

	const rawBody = await request.arrayBuffer();
	const contentType = request.headers.get('content-type') ?? 'application/json';

	const upstream = `${apiServiceUrl}/v1/webhooks/stripe`;

	const controller = new AbortController();
	const timeout = setTimeout(() => controller.abort(), UPSTREAM_TIMEOUT_MS);

	try {
		const upstreamResponse = await fetch(upstream, {
			method: 'POST',
			headers: {
				'Stripe-Signature': signature,
				'Content-Type': contentType,
			},
			body: rawBody,
			signal: controller.signal,
		});

		const responseBody = await upstreamResponse.arrayBuffer();
		return new NextResponse(responseBody, {
			status: upstreamResponse.status,
			headers: {
				'Content-Type': upstreamResponse.headers.get('content-type') ?? 'text/plain',
			},
		});
	} catch (error) {
		log.error('Failed to forward Stripe webhook to api-service', {
			error: error instanceof Error ? error.message : error,
		});
		return NextResponse.json({ error: 'Upstream unavailable' }, { status: 502 });
	} finally {
		clearTimeout(timeout);
	}
}
