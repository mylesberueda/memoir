import { type MessagePart, MessagePartKind } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';

/**
 * Check if any parts are thinking parts
 */
export function hasThinkingParts(parts?: MessagePart[]): boolean {
	return parts?.some((part) => part.kind === MessagePartKind.THINKING) ?? false;
}

/**
 * Get the text content from message parts.
 * Only extracts content from TEXT kind parts.
 */
export function getTextContent(parts?: MessagePart[]): string {
	if (!parts || parts.length === 0) return '';

	const textParts = parts.filter((p) => p.kind === MessagePartKind.TEXT && p.content);
	return textParts.map((p) => p.content).join('');
}
