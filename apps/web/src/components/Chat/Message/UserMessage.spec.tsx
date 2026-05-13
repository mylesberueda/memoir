import { create } from '@bufbuild/protobuf';
import { type MessagePart, MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { MessagePartSchema } from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import UserMessage from './UserMessage';

function createTextPart(id: string, content: string, status = MessagePartStatus.COMPLETE): MessagePart {
	return create(MessagePartSchema, { id, kind: MessagePartKind.TEXT, content, status });
}

describe('UserMessage', () => {
	it('renders user message with correct styling', () => {
		render(
			<UserMessage
				id="msg-1"
				variant="rx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test user message')]}
			/>,
		);

		const messageContainer = screen.getByText('Test user message').closest('.group');
		expect(messageContainer).toHaveClass('is-user', 'grid-cols-[1fr_auto]', 'justify-items-end');
		expect(screen.getByText('Test user message')).toBeInTheDocument();
	});

	it('renders content correctly', () => {
		render(
			<UserMessage
				id="msg-1"
				variant="rx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Custom content')]}
			/>,
		);

		expect(screen.getByText('Custom content')).toBeInTheDocument();
	});

	it('preserves whitespace in content', () => {
		const multiLineText = `Line 1
Line 2
Line 3`;

		render(
			<UserMessage id="msg-1" variant="rx" timestamp={new Date()} parts={[createTextPart('text-1', multiLineText)]} />,
		);

		const textElement = screen.getByText(/Line 1.*Line 2.*Line 3/);
		expect(textElement).toBeInTheDocument();
		expect(textElement).toHaveClass('whitespace-pre-wrap');
	});

	it('renders multiple text parts', () => {
		render(
			<UserMessage
				id="msg-1"
				variant="rx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Paragraph text'), createTextPart('text-2', 'Span text')]}
			/>,
		);

		expect(screen.getByText('Paragraph text')).toBeInTheDocument();
		expect(screen.getByText('Span text')).toBeInTheDocument();
	});

	it('renders user avatar with custom avatar prop', () => {
		render(
			<UserMessage
				id="msg-1"
				variant="rx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test')]}
				avatar="https://example.com/avatar.jpg"
			/>,
		);

		const avatar = document.querySelector('img[src="https://example.com/avatar.jpg"]');
		expect(avatar).toBeInTheDocument();
	});

	it('renders footer content when provided', () => {
		render(
			<UserMessage
				id="msg-1"
				variant="rx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', 'Test')]}
				footer={<div>Custom footer</div>}
			/>,
		);

		expect(screen.getByText('Custom footer')).toBeInTheDocument();
	});

	it('skips rendering text parts with empty content', () => {
		render(
			<UserMessage
				id="msg-1"
				variant="rx"
				timestamp={new Date()}
				parts={[createTextPart('text-1', ''), createTextPart('text-2', '   ')]}
			/>,
		);

		const textMessageBubbles = document.querySelectorAll('.rounded-lg.px-4.py-3');
		expect(textMessageBubbles).toHaveLength(0);
	});
});
