import { create } from '@bufbuild/protobuf';
import { type MessagePart, MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { MessagePartSchema, ToolCallSchema, ToolResultSchema } from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import AgentMessage from './AgentMessage';

function createTextPart(id: string, content: string, status = MessagePartStatus.COMPLETE): MessagePart {
	return create(MessagePartSchema, { id, kind: MessagePartKind.TEXT, content, status });
}

function createThinkingPart(id: string, content: string, status = MessagePartStatus.COMPLETE): MessagePart {
	return create(MessagePartSchema, { id, kind: MessagePartKind.THINKING, content, status });
}

function createToolCallPart(
	id: string,
	name: string,
	args: Record<string, string | number | boolean>,
	status = MessagePartStatus.COMPLETE,
): MessagePart {
	return create(MessagePartSchema, {
		id,
		kind: MessagePartKind.TOOL_CALL,
		status,
		toolCall: create(ToolCallSchema, { id, name, arguments: args }),
	});
}

function createToolResultPart(
	id: string,
	toolCallId: string,
	result: string,
	status = MessagePartStatus.COMPLETE,
): MessagePart {
	return create(MessagePartSchema, {
		id,
		kind: MessagePartKind.TOOL_RESULT,
		status,
		toolResult: create(ToolResultSchema, { toolCallId, result }),
	});
}

describe('AgentMessage', () => {
	it('renders assistant message with correct styling', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test assistant message')]}
			/>,
		);

		const messageContainer = screen.getByText('Test assistant message').closest('.group');
		expect(messageContainer).toHaveClass('is-assistant');
		expect(screen.getByText('Test assistant message')).toBeInTheDocument();
	});

	it('does not render message bubble for empty parts', () => {
		render(<AgentMessage id="msg-1" variant="tx" timestamp={new Date()} parts={[]} />);

		const outerContainer = document.querySelector('.group');
		expect(outerContainer).toBeInTheDocument();
		expect(outerContainer).toHaveClass('is-assistant');

		const textMessageBubbles = document.querySelectorAll('.rounded-lg.px-4.py-3');
		expect(textMessageBubbles).toHaveLength(0);
	});

	it('renders thinking section but not message bubble for messages with only thinking content', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createThinkingPart('thinking-1', 'thinking content')]}
			/>,
		);

		const reasoningButton = screen.getByLabelText('Show reasoning section');
		expect(reasoningButton).toBeInTheDocument();

		const textMessageBubbles = document.querySelectorAll('.rounded-lg.px-4.py-3');
		expect(textMessageBubbles).toHaveLength(0);
	});

	it('renders tool calls section but not message bubble for messages with only tool calls', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[
					createToolCallPart('tool-1', 'test-tool', { param: 'value' }),
					createToolResultPart('result-1', 'tool-1', 'Tool result'),
				]}
			/>,
		);

		const toolHeader = screen.getByText('test-tool');
		expect(toolHeader).toBeInTheDocument();

		const textMessageBubbles = document.querySelectorAll('.rounded-lg.px-4.py-3');
		expect(textMessageBubbles).toHaveLength(0);
	});

	it('renders message bubble for messages with text content', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'This is actual text content')]}
			/>,
		);

		const textContent = screen.getByText('This is actual text content');
		expect(textContent).toBeInTheDocument();
	});
});

describe('AgentMessage content rendering', () => {
	it('renders text content', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test content')]}
			/>,
		);

		expect(screen.getByText('Test content')).toBeInTheDocument();
	});
});

describe('AgentMessage action buttons', () => {
	it('calls custom onCopy callback when copy button is clicked', async () => {
		const user = userEvent.setup();
		const onCopy = vi.fn();

		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test message')]}
				onCopy={onCopy}
			/>,
		);

		const copyButton = screen.getByRole('button', { name: 'Copy' });
		await user.click(copyButton);

		expect(onCopy).toHaveBeenCalled();
	});

	it('uses clipboard fallback when no custom onCopy is provided', async () => {
		const user = userEvent.setup();
		const mockWriteText = vi.fn().mockResolvedValue(undefined);

		const originalClipboard = navigator.clipboard;

		Object.defineProperty(navigator, 'clipboard', {
			value: { writeText: mockWriteText },
			writable: true,
			configurable: true,
		});

		try {
			render(
				<AgentMessage
					id="msg-1"
					variant="tx"
					timestamp={new Date()}
					parts={[createTextPart('text-1', 'Test message content')]}
				/>,
			);

			const copyButton = screen.getByRole('button', { name: 'Copy' });
			await user.click(copyButton);

			expect(mockWriteText).toHaveBeenCalledWith('Test message content');
		} finally {
			Object.defineProperty(navigator, 'clipboard', {
				value: originalClipboard,
				writable: true,
				configurable: true,
			});
		}
	});

	it('calls onRetry when retry button is clicked', async () => {
		const user = userEvent.setup();
		const onRetry = vi.fn();

		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test message')]}
				onRetry={onRetry}
			/>,
		);

		const retryButton = screen.getByRole('button', { name: 'Retry' });
		await user.click(retryButton);

		expect(onRetry).toHaveBeenCalled();
	});

	it('calls onFeedback with "like" when like button is clicked', async () => {
		const user = userEvent.setup();
		const onFeedback = vi.fn();

		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test message')]}
				onFeedback={onFeedback}
			/>,
		);

		const likeButton = screen.getByRole('button', { name: 'Like' });
		await user.click(likeButton);

		expect(onFeedback).toHaveBeenCalledWith('like');
	});

	it('calls onFeedback with "dislike" when dislike button is clicked', async () => {
		const user = userEvent.setup();
		const onFeedback = vi.fn();

		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test message')]}
				onFeedback={onFeedback}
			/>,
		);

		const dislikeButton = screen.getByRole('button', { name: 'Dislike' });
		await user.click(dislikeButton);

		expect(onFeedback).toHaveBeenCalledWith('dislike');
	});
});

describe('AgentMessage tool status', () => {
	it('renders tool with calling status', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createToolCallPart('tool-1', 'search', { query: 'test' }, MessagePartStatus.STREAMING)]}
			/>,
		);

		expect(screen.getByText('search')).toBeInTheDocument();
	});

	it('renders tool with failed status showing error state', () => {
		const { container } = render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[
					createToolCallPart('tool-1', 'search', { query: 'test' }),
					createToolResultPart('result-1', 'tool-1', 'Error occurred', MessagePartStatus.FAILED),
				]}
			/>,
		);

		expect(screen.getByText('search')).toBeInTheDocument();
		expect(container.querySelector('.border-error')).toBeInTheDocument();
	});
});

describe('AgentMessage footer', () => {
	it('renders footer content when provided', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test')]}
				footer={<div>Custom footer</div>}
			/>,
		);

		expect(screen.getByText('Custom footer')).toBeInTheDocument();
	});
});

describe('AgentMessage edge cases', () => {
	it('handles unknown part types gracefully', () => {
		const validPart = createTextPart('text-1', 'Valid text');
		const invalidPart = { ...validPart, kind: 999 as MessagePartKind, id: 'unknown-1', content: 'Unknown type' };

		render(<AgentMessage id="msg-1" variant="tx" timestamp={new Date()} parts={[validPart, invalidPart]} />);

		expect(screen.getByText('Valid text')).toBeInTheDocument();
		expect(screen.queryByText('Unknown type')).not.toBeInTheDocument();
	});

	it('skips rendering text parts with empty content', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', ''), createTextPart('text-2', '   ')]}
			/>,
		);

		const textMessageBubbles = document.querySelectorAll('.rounded-lg.px-4.py-3');
		expect(textMessageBubbles).toHaveLength(0);
	});

	it('renders thinking section with thinkingDuration', () => {
		render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[createThinkingPart('thinking-1', 'Deep thought')]}
				thinkingDuration={5}
			/>,
		);

		expect(screen.getByLabelText('Show reasoning section')).toBeInTheDocument();
	});

	it('renders tool with JSON result showing success state', () => {
		const { container } = render(
			<AgentMessage
				id="msg-1"
				variant="tx"
				timestamp={new Date()}
				parts={[
					createToolCallPart('tool-1', 'fetch', { url: 'https://api.example.com' }),
					createToolResultPart('result-1', 'tool-1', JSON.stringify({ data: 'response' })),
				]}
			/>,
		);

		expect(screen.getByText('fetch')).toBeInTheDocument();
		expect(container.querySelector('.border-success')).toBeInTheDocument();
	});
});
