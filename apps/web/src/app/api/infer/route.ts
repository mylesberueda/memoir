import { create, toJson } from '@bufbuild/protobuf';
import { inferenceServiceClient } from '@lib/grpc/clients';
import { createChildLogger } from '@lib/logger';
import { InferRequestSchema, InferResponseSchema } from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import { type NextRequest, NextResponse } from 'next/server';

const log = createChildLogger({ route: 'api/infer' });

/**
 * Streaming inference API route.
 * Uses NDJSON to stream events in real-time.
 *
 * This bypasses the buffering issue with async generators in Next.js server actions.
 */
export async function POST(request: NextRequest) {
	const client = await inferenceServiceClient();
	if (!client) {
		return NextResponse.json({ error: 'Authentication required' }, { status: 401 });
	}

	const body = await request.json();
	const { agentPid, conversationPid, message, requestId, documentPids } = body;

	if (!agentPid || !message) {
		return NextResponse.json({ error: 'agentPid and message are required' }, { status: 400 });
	}

	const inferRequest = create(InferRequestSchema, {
		agentPid,
		conversationPid,
		message,
		requestId,
		documentPids,
	});

	// Create a ReadableStream that emits NDJSON
	const encoder = new TextEncoder();
	const stream = new ReadableStream({
		async start(controller) {
			try {
				for await (const res of client.infer(inferRequest)) {
					const json = JSON.stringify(toJson(InferResponseSchema, res));
					controller.enqueue(encoder.encode(`${json}\n`));
				}
				controller.close();
			} catch (error) {
				log.error('Streaming error', { error: error instanceof Error ? error.message : error });
				const errorJson = JSON.stringify({ error: error instanceof Error ? error.message : 'Unknown error' });
				controller.enqueue(encoder.encode(`${errorJson}\n`));
				controller.close();
			}
		},
	});

	return new Response(stream, {
		headers: {
			'Content-Type': 'application/x-ndjson',
			'Cache-Control': 'no-cache',
			Connection: 'keep-alive',
		},
	});
}
