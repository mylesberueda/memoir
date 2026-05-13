import { getTextContent, type Message } from '@lib/chat-state';
import { MessagePartKind } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';

export interface ExportedConversation {
	version: '1.0';
	exportDate: string;
	sessionId: string | null;
	messages: Message[];
	metadata?: {
		assistantId?: string;
		userId?: string;
		title?: string;
	};
}

// biome-ignore lint/suspicious/noExplicitAny: metadata can contain arbitrary key-value pairs from different sources
export function exportToJSON(messages: Message[], sessionId: string | null, metadata?: any): string {
	const exportData: ExportedConversation = {
		version: '1.0',
		exportDate: new Date().toISOString(),
		sessionId,
		messages: messages.map((msg) => ({
			...msg,
			timestamp: new Date(msg.timestamp), // Ensure it's a Date object
		})),
		metadata,
	};

	return JSON.stringify(exportData, null, 2);
}

export function exportToMarkdown(messages: Message[]): string {
	let markdown = '# Chat Conversation\n\n';
	// Use toISOString for better performance in CI environments
	const exportDate = new Date();
	const dateStr = exportDate.toISOString().split('T')[0];
	const timeStr = exportDate.toISOString().split('T')[1].split('.')[0];
	markdown += `*Exported on ${dateStr} ${timeStr}*\n\n---\n\n`;

	for (const message of messages) {
		const role = message.role === 'user' ? '**You**' : '**Assistant**';
		// Use custom formatting for better performance
		const msgDate = new Date(message.timestamp);
		const hours = msgDate.getHours().toString().padStart(2, '0');
		const minutes = msgDate.getMinutes().toString().padStart(2, '0');
		const seconds = msgDate.getSeconds().toString().padStart(2, '0');
		const time = `${hours}:${minutes}:${seconds}`;

		markdown += `### ${role} - ${time}\n\n`;
		markdown += `${getTextContent(message.parts)}\n\n`;

		// Add attachments if present
		if (message.attachments && message.attachments.length > 0) {
			markdown += '**Attachments:**\n';
			for (const attachment of message.attachments) {
				markdown += `- ${attachment.name} (${attachment.type}, ${formatFileSize(attachment.size)})\n`;
			}
			markdown += '\n';
		}

		// Add tool calls if present (extract from parts)
		const toolCalls = message.parts.filter((part) => part.kind === MessagePartKind.TOOL_CALL);
		if (toolCalls && toolCalls.length > 0) {
			markdown += '**Tools Used:**\n';
			for (const tool of toolCalls) {
				markdown += `- ${tool.toolCall?.name || 'unknown'}: ${tool.status}\n`;
			}
			markdown += '\n';
		}

		markdown += '---\n\n';
	}

	return markdown;
}

export async function importFromJSON(jsonString: string): Promise<ExportedConversation> {
	try {
		const data = JSON.parse(jsonString);

		// Validate structure
		if (!data.version || !data.messages || !Array.isArray(data.messages)) {
			throw new Error('Invalid conversation format');
		}

		// Validate version compatibility
		if (data.version !== '1.0') {
			throw new Error(`Unsupported conversation version: ${data.version}`);
		}

		// Convert date strings back to Date objects
		// biome-ignore lint/suspicious/noExplicitAny: JSON parsing results in any type, but we validate the structure above
		data.messages = data.messages.map((msg: any) => ({
			...msg,
			timestamp: new Date(msg.timestamp),
		}));

		return data as ExportedConversation;
	} catch (error) {
		if (error instanceof SyntaxError) {
			throw new Error('Invalid JSON format');
		}
		throw new Error(`Failed to parse conversation: ${error instanceof Error ? error.message : 'Unknown error'}`);
	}
}

export function downloadFile(content: string, filename: string, mimeType: string) {
	const blob = new Blob([content], { type: mimeType });
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);
}

export function generateFilename(format: 'json' | 'md'): string {
	const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
	return `chat-conversation-${timestamp}.${format}`;
}

function formatFileSize(bytes: number): string {
	if (bytes === 0) return '0 Bytes';
	const k = 1024;
	const sizes = ['Bytes', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
}
