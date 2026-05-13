import { create } from '@bufbuild/protobuf';
import { type Message, MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { MessagePartSchema } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, fn, userEvent, within } from 'storybook/test';

import { MessageActions } from './MessageActions';

// Helper to create mock messages
const createMockMessage = (overrides: Partial<Message> = {}): Message => ({
	id: 'msg-123',
	role: 'user',
	timestamp: new Date(),
	status: 'complete',
	parts: [
		create(MessagePartSchema, {
			id: 'part-1',
			kind: MessagePartKind.TEXT,
			status: MessagePartStatus.COMPLETE,
			content: 'This is a sample message',
		}),
	],
	...overrides,
});

const meta = {
	title: 'Chat/MessageActions',
	component: MessageActions,
	parameters: {
		layout: 'centered',
		docs: {
			description: {
				component: 'Dropdown menu with message actions like retry, edit, and delete.',
			},
		},
	},
	args: {
		message: createMockMessage(),
		onRetry: fn(),
		onEdit: fn(),
		onDelete: fn(),
	},
	argTypes: {
		message: {
			description: 'The message object',
			control: { type: 'object' },
		},
		canRetry: {
			description: 'Whether the retry action is available',
			control: { type: 'boolean' },
		},
		canEdit: {
			description: 'Whether the edit action is available',
			control: { type: 'boolean' },
		},
		canDelete: {
			description: 'Whether the delete action is available',
			control: { type: 'boolean' },
		},
	},
} satisfies Meta<typeof MessageActions>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		message: createMockMessage(),
		canRetry: true,
		canEdit: true,
		canDelete: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Default message actions with all options available.',
			},
		},
	},
};

export const NoActions: Story = {
	args: {
		message: createMockMessage(),
		canRetry: false,
		canEdit: false,
		canDelete: false,
	},
	parameters: {
		docs: {
			description: {
				story: 'Message with no available actions (component will render nothing).',
			},
		},
	},
};

export const UserMessage: Story = {
	args: {
		message: createMockMessage({ role: 'user', id: 'msg-user-123' }),
		canRetry: false,
		canEdit: true,
		canDelete: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Actions for user messages (edit and delete available).',
			},
		},
	},
};

export const AssistantMessage: Story = {
	args: {
		message: createMockMessage({ role: 'assistant', id: 'msg-assistant-123' }),
		canRetry: true,
		canEdit: false,
		canDelete: false,
	},
	parameters: {
		docs: {
			description: {
				story: 'Actions for assistant messages (only retry available).',
			},
		},
	},
};

export const FailedMessage: Story = {
	args: {
		message: createMockMessage({
			status: 'failed',
			id: 'msg-failed-123',
			role: 'user',
		}),
		canRetry: true,
		canEdit: true,
		canDelete: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Actions for failed messages (retry, edit, delete available).',
			},
		},
	},
};

export const ReadOnlyMode: Story = {
	args: {
		message: createMockMessage({ id: 'msg-readonly-123' }),
		canRetry: false,
		canEdit: false,
		canDelete: false,
	},
	parameters: {
		docs: {
			description: {
				story: 'Message actions in read-only mode (no actions available).',
			},
		},
	},
};

export const Interactive: Story = {
	args: {
		message: createMockMessage({
			id: 'msg-interactive-123',
			role: 'user',
		}),
		canRetry: true,
		canEdit: true,
		canDelete: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Interactive story for testing action menu behavior.',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Find and click the actions trigger button
		const trigger = await canvas.findByTestId('message-actions-trigger');
		await expect(trigger).toBeInTheDocument();

		await userEvent.click(trigger);

		// Test edit action
		const editButton = canvas.getByTestId('edit-message-button');
		await expect(editButton).toBeInTheDocument();
		await userEvent.click(editButton);
	},
};

export const DeleteInteraction: Story = {
	args: {
		message: createMockMessage({
			id: 'msg-delete-123',
			role: 'user',
		}),
		canRetry: false,
		canEdit: false,
		canDelete: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Interactive story for testing delete functionality.',
			},
		},
	},
	play: async ({ canvasElement, args }) => {
		const canvas = within(canvasElement);

		// Open the actions menu
		const trigger = await canvas.findByTestId('message-actions-trigger');
		await userEvent.click(trigger);

		// Test delete button
		const deleteButton = canvas.getByTestId('delete-message-button');
		await expect(deleteButton).toBeInTheDocument();
		await userEvent.click(deleteButton);

		// Confirm delete
		const confirmButton = await canvas.findByTestId('confirm-delete');
		await expect(confirmButton).toBeInTheDocument();
		await userEvent.click(confirmButton);
		await expect(args.onDelete).toHaveBeenCalled();
	},
};

export const KeyboardNavigation: Story = {
	args: {
		message: createMockMessage({
			id: 'msg-keyboard-123',
			role: 'user',
		}),
		canRetry: true,
		canEdit: true,
		canDelete: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Test keyboard navigation through the actions menu.',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Focus the trigger button
		const trigger = await canvas.findByTestId('message-actions-trigger');
		trigger.focus();

		// Open with Enter key
		await userEvent.keyboard('{Enter}');

		// Close with Escape
		await userEvent.keyboard('{Escape}');
	},
};

export const MobileView: Story = {
	args: {
		message: createMockMessage({
			id: 'msg-mobile-123',
			role: 'user',
		}),
		canRetry: true,
		canEdit: true,
		canDelete: true,
	},
	parameters: {
		viewport: {
			defaultViewport: 'mobile1',
		},
		docs: {
			description: {
				story: 'Message actions optimized for mobile viewport.',
			},
		},
	},
};
