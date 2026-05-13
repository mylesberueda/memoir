import { fromJson } from '@bufbuild/protobuf';
import { type InferResponse, InferResponseSchema } from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';

export interface StreamInferenceInput {
	agentPid: string;
	conversationPid?: string;
	message: string;
	// A request id, so the api knows which infer to cancel if the user cancels the request
	requestId: string;
	// PIDs of documents attached to this specific message
	documentPids?: string[];
}

export interface StreamError {
	error: string;
}

/**
 * Streams inference events from the API route using NDJSON.
 * Each yielded value is a properly typed InferResponse.
 */
export async function* streamInferenceEvents(
	input: StreamInferenceInput,
	signal?: AbortSignal,
): AsyncGenerator<InferResponse, void, unknown> {
	const response = await fetch('/api/infer', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(input),
		signal,
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.error || `HTTP ${response.status}`);
	}

	if (!response.body) {
		throw new Error('No response body');
	}

	const reader = response.body.getReader();
	const decoder = new TextDecoder();
	let buffer = '';

	try {
		while (true) {
			const { done, value } = await reader.read();

			if (done) {
				// Process any remaining data in buffer
				if (buffer.trim()) {
					const event = parseNdjsonLine(buffer.trim());
					if (event) {
						if ('error' in event) {
							throw new Error(event.error);
						}
						yield event;
					}
				}
				break;
			}

			buffer += decoder.decode(value, { stream: true });

			// Process complete lines
			const lines = buffer.split('\n');
			buffer = lines.pop() || ''; // Keep incomplete line in buffer

			for (const line of lines) {
				if (!line.trim()) continue;

				const event = parseNdjsonLine(line);
				if (event) {
					if ('error' in event) {
						throw new Error(event.error);
					}
					yield event;
				}
			}
		}
	} finally {
		reader.releaseLock();
	}
}

/**
 * Parse a single NDJSON line into an InferResponse or error object.
 */
function parseNdjsonLine(line: string): InferResponse | StreamError | null {
	try {
		const json = JSON.parse(line);

		// Check if it's an error response
		if (json.error) {
			return { error: json.error };
		}

		// Convert JSON back to InferResponse using protobuf fromJson
		return fromJson(InferResponseSchema, json);
	} catch (e) {
		console.error('Failed to parse NDJSON line:', line, e);
		return null;
	}
}
