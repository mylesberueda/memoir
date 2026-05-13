import { create } from '@bufbuild/protobuf';
import { MessagePartSchema } from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { downloadFile, exportToJSON, exportToMarkdown, generateFilename, importFromJSON } from './chat-export';
import { type Message, MessagePartKind, MessagePartStatus } from './chat-state';

// Mock DOM APIs
const mockCreateElement = vi.fn();
const mockAppendChild = vi.fn();
const mockRemoveChild = vi.fn();
const mockClick = vi.fn();
const mockRevokeObjectURL = vi.fn();
const mockCreateObjectURL = vi.fn();

const mockLink = {
	href: '',
	download: '',
	click: mockClick,
};

beforeEach(() => {
	// Mock DOM methods
	mockCreateElement.mockReturnValue(mockLink);
	Object.defineProperty(document, 'createElement', {
		value: mockCreateElement,
		writable: true,
	});
	Object.defineProperty(document.body, 'appendChild', {
		value: mockAppendChild,
		writable: true,
	});
	Object.defineProperty(document.body, 'removeChild', {
		value: mockRemoveChild,
		writable: true,
	});

	// Mock URL API
	Object.defineProperty(URL, 'createObjectURL', {
		value: mockCreateObjectURL.mockReturnValue('blob:mock-url'),
		writable: true,
	});
	Object.defineProperty(URL, 'revokeObjectURL', {
		value: mockRevokeObjectURL,
		writable: true,
	});

	// Mock Blob
	global.Blob = vi.fn().mockImplementation((content, options) => ({
		content,
		type: options?.type || 'text/plain',
	})) as unknown as {
		new (blobParts?: BlobPart[], options?: BlobPropertyBag): Blob;
		prototype: Blob;
	};
});

afterEach(() => {
	vi.resetAllMocks();
});

const mockMessages: Message[] = [
	{
		id: 'msg-1',
		role: 'user',
		timestamp: new Date('2024-01-15T10:30:00Z'),
		status: 'complete',
		parts: [
			create(MessagePartSchema, {
				id: 'part-1',
				kind: MessagePartKind.TEXT,
				content: 'Hello, world!',
				status: MessagePartStatus.COMPLETE,
			}),
		],
	},
	{
		id: 'msg-2',
		role: 'assistant',
		timestamp: new Date('2024-01-15T10:30:15Z'),
		status: 'complete',
		parts: [
			create(MessagePartSchema, {
				id: 'part-2',
				kind: MessagePartKind.TEXT,
				content: 'Hello! How can I help you today?',
				status: MessagePartStatus.COMPLETE,
			}),
			create(MessagePartSchema, {
				id: 'tool-1',
				kind: MessagePartKind.TOOL_CALL,
				status: MessagePartStatus.COMPLETE,
				toolCall: { id: 'tool-1', name: 'greeting', arguments: {} },
			}),
		],
	},
	{
		id: 'msg-3',
		role: 'user',
		timestamp: new Date('2024-01-15T10:31:00Z'),
		status: 'complete',
		parts: [
			create(MessagePartSchema, {
				id: 'part-3',
				kind: MessagePartKind.TEXT,
				content: 'I need help with React testing.',
				status: MessagePartStatus.COMPLETE,
			}),
		],
		attachments: [
			{
				id: 'att-1',
				name: 'test-file.tsx',
				type: 'text/typescript',
				size: 1024,
				status: 'uploaded',
			},
		],
	},
];

describe('chat-export', () => {
	describe('exportToJSON', () => {
		it('should export messages to JSON format', () => {
			const result = exportToJSON(mockMessages, 'session-123', { userId: 'user-456' });
			const parsed = JSON.parse(result);

			expect(parsed.version).toBe('1.0');
			expect(parsed.sessionId).toBe('session-123');
			expect(parsed.metadata.userId).toBe('user-456');
			expect(parsed.messages).toHaveLength(3);
			expect(parsed.exportDate).toBeDefined();
		});

		it('should handle empty messages array', () => {
			const result = exportToJSON([], null, {});
			const parsed = JSON.parse(result);

			expect(parsed.messages).toHaveLength(0);
			expect(parsed.sessionId).toBeNull();
		});

		it('should preserve message structure', () => {
			const result = exportToJSON(mockMessages, 'session-123');
			const parsed = JSON.parse(result);

			const firstMessage = parsed.messages[0];
			expect(firstMessage.id).toBe('msg-1');
			expect(firstMessage.role).toBe('user');
			expect(firstMessage.status).toBe('complete');
			expect(firstMessage.parts).toBeDefined();
		});

		it('should include tool calls when present', () => {
			const result = exportToJSON(mockMessages, 'session-123');
			const parsed = JSON.parse(result);

			const messageWithTools = parsed.messages[1];
			expect(messageWithTools.parts.length).toBeGreaterThan(1);
			// Find the tool call part
			const toolCallPart = messageWithTools.parts.find(
				(p: { kind: MessagePartKind }) => p.kind === MessagePartKind.TOOL_CALL,
			);
			expect(toolCallPart).toBeDefined();
			expect(toolCallPart.toolCall.name).toBe('greeting');
		});

		it('should include attachments when present', () => {
			const result = exportToJSON(mockMessages, 'session-123');
			const parsed = JSON.parse(result);

			const messageWithAttachments = parsed.messages[2];
			expect(messageWithAttachments.attachments).toHaveLength(1);
			expect(messageWithAttachments.attachments[0].name).toBe('test-file.tsx');
		});
	});

	describe('exportToMarkdown', () => {
		it('should export messages to markdown format', () => {
			const result = exportToMarkdown(mockMessages);

			expect(result).toContain('# Chat Conversation');
			expect(result).toContain('**You**');
			expect(result).toContain('**Assistant**');
			expect(result).toContain('Hello, world!');
			expect(result).toContain('Hello! How can I help you today?');
		}, 10000); // Add explicit timeout for CI environments

		it('should include timestamps', () => {
			const result = exportToMarkdown(mockMessages);

			// Should contain time format HH:MM:SS
			expect(result).toMatch(/\d{2}:\d{2}:\d{2}/);
		});

		it('should include attachments section', () => {
			const result = exportToMarkdown(mockMessages);

			expect(result).toContain('**Attachments:**');
			expect(result).toContain('test-file.tsx');
			expect(result).toContain('(text/typescript, 1 KB)');
		});

		it('should include tool calls section', () => {
			const result = exportToMarkdown(mockMessages);

			expect(result).toContain('**Tools Used:**');
			expect(result).toContain('greeting');
		});

		it('should handle empty messages array', () => {
			const result = exportToMarkdown([]);

			expect(result).toContain('# Chat Conversation');
			expect(result).toContain('*Exported on');
		});

		it('should format file sizes correctly', () => {
			const messagesWithLargeFile: Message[] = [
				{
					...mockMessages[0],
					attachments: [
						{
							id: 'att-large',
							name: 'large-file.pdf',
							type: 'application/pdf',
							size: 1048576, // 1MB
							status: 'uploaded',
						},
					],
				},
			];

			const result = exportToMarkdown(messagesWithLargeFile);
			expect(result).toContain('1 MB');
		});
	});

	describe('importFromJSON', () => {
		const validJSON = {
			version: '1.0',
			exportDate: '2024-01-15T10:30:00Z',
			sessionId: 'session-123',
			messages: [
				{
					id: 'msg-1',
					role: 'user',
					timestamp: '2024-01-15T10:30:00Z',
					status: 'complete',
					parts: [
						{
							id: 'part-1',
							kind: MessagePartKind.TEXT,
							content: 'Test message',
							status: MessagePartStatus.COMPLETE,
						},
					],
				},
			],
			metadata: { userId: 'user-456' },
		};

		it('should import valid JSON', async () => {
			const result = await importFromJSON(JSON.stringify(validJSON));

			expect(result.version).toBe('1.0');
			expect(result.sessionId).toBe('session-123');
			expect(result.messages).toHaveLength(1);
			expect(result.messages[0].timestamp).toBeInstanceOf(Date);
		});

		it('should convert timestamp strings to Date objects', async () => {
			const result = await importFromJSON(JSON.stringify(validJSON));

			expect(result.messages[0].timestamp).toBeInstanceOf(Date);
			expect(result.messages[0].timestamp.getTime()).toBe(new Date('2024-01-15T10:30:00Z').getTime());
		});

		it('should throw error for invalid JSON', async () => {
			await expect(importFromJSON('invalid json')).rejects.toThrow('Invalid JSON format');
		});

		it('should throw error for missing version', async () => {
			const invalidData = { ...validJSON } as Record<string, unknown>;
			invalidData.version = undefined;

			await expect(importFromJSON(JSON.stringify(invalidData))).rejects.toThrow('Invalid conversation format');
		});

		it('should throw error for missing messages', async () => {
			const invalidData = { ...validJSON } as Record<string, unknown>;
			invalidData.messages = undefined;

			await expect(importFromJSON(JSON.stringify(invalidData))).rejects.toThrow('Invalid conversation format');
		});

		it('should throw error for non-array messages', async () => {
			const invalidData = { ...validJSON, messages: 'not an array' };

			await expect(importFromJSON(JSON.stringify(invalidData))).rejects.toThrow('Invalid conversation format');
		});

		it('should throw error for unsupported version', async () => {
			const invalidData = { ...validJSON, version: '2.0' };

			await expect(importFromJSON(JSON.stringify(invalidData))).rejects.toThrow(
				'Unsupported conversation version: 2.0',
			);
		});
	});

	describe('downloadFile', () => {
		it('should create download link and trigger download', () => {
			downloadFile('test content', 'test.txt', 'text/plain');

			expect(global.Blob).toHaveBeenCalledWith(['test content'], { type: 'text/plain' });
			expect(mockCreateObjectURL).toHaveBeenCalled();
			expect(mockCreateElement).toHaveBeenCalledWith('a');
			expect(mockLink.href).toBe('blob:mock-url');
			expect(mockLink.download).toBe('test.txt');
			expect(mockAppendChild).toHaveBeenCalledWith(mockLink);
			expect(mockClick).toHaveBeenCalled();
			expect(mockRemoveChild).toHaveBeenCalledWith(mockLink);
			expect(mockRevokeObjectURL).toHaveBeenCalledWith('blob:mock-url');
		});
	});

	describe('generateFilename', () => {
		beforeEach(() => {
			// Mock Date to return a fixed timestamp
			vi.useFakeTimers();
			vi.setSystemTime(new Date('2024-01-15T10:30:45Z'));
		});

		afterEach(() => {
			vi.useRealTimers();
		});

		it('should generate JSON filename', () => {
			const filename = generateFilename('json');
			expect(filename).toBe('chat-conversation-2024-01-15T10-30-45.json');
		});

		it('should generate Markdown filename', () => {
			const filename = generateFilename('md');
			expect(filename).toBe('chat-conversation-2024-01-15T10-30-45.md');
		});

		it('should replace special characters in timestamp', () => {
			const filename = generateFilename('json');
			// Should not contain : characters in timestamp part (but will have . in extension)
			expect(filename).not.toMatch(/\d+:\d+/);
		});
	});

	describe('file size formatting (internal function)', () => {
		it('should format bytes correctly', () => {
			// Test through markdown export which uses the formatFileSize function
			const messageWithFiles: Message[] = [
				{
					...mockMessages[0],
					attachments: [
						{
							id: 'small',
							name: 'small.txt',
							type: 'text/plain',
							size: 0,
							status: 'uploaded',
						},
						{
							id: 'bytes',
							name: 'bytes.txt',
							type: 'text/plain',
							size: 500,
							status: 'uploaded',
						},
						{
							id: 'kb',
							name: 'kb.txt',
							type: 'text/plain',
							size: 1536, // 1.5 KB
							status: 'uploaded',
						},
						{
							id: 'mb',
							name: 'mb.txt',
							type: 'text/plain',
							size: 2097152, // 2 MB
							status: 'uploaded',
						},
					],
				},
			];

			const result = exportToMarkdown(messageWithFiles);

			expect(result).toContain('(text/plain, 0 Bytes)');
			expect(result).toContain('(text/plain, 500 Bytes)');
			expect(result).toContain('(text/plain, 1.5 KB)');
			expect(result).toContain('(text/plain, 2 MB)');
		});
	});
});
