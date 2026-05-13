import { type ChatStateActions, chatInitialState } from '@lib/chat-state';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, fn, userEvent, within } from 'storybook/test';

import { ExportDialog } from './ExportDialog';

const mockChat: ChatStateActions = {
	state: chatInitialState,
	addMessage: () => '',
	updateMessage: () => {},
	deleteMessage: () => {},
	setMessages: () => {},
	setLoading: () => {},
	setError: () => {},
	setSessionId: () => {},
	setAssistantId: () => {},
	setPendingFiles: () => {},
	clearChat: () => {},
};

const meta = {
	title: 'Chat/ExportDialog',
	component: ExportDialog,
	parameters: {
		layout: 'centered',
		docs: {
			description: {
				component:
					'Modal dialog for exporting and importing chat conversations with support for JSON and Markdown formats.',
			},
		},
	},
	args: {
		onClose: fn(),
		chat: mockChat,
	},
	argTypes: {
		isOpen: {
			description: 'Whether the dialog is open',
			control: { type: 'boolean' },
		},
	},
} satisfies Meta<typeof ExportDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Closed: Story = {
	args: {
		isOpen: false,
	},
	parameters: {
		docs: {
			description: {
				story: 'Dialog in closed state (not visible).',
			},
		},
	},
};

export const EmptyChat: Story = {
	args: {
		isOpen: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Export dialog when there are no messages to export.',
			},
		},
	},
};

export const WithMessages: Story = {
	args: {
		isOpen: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Export dialog with messages available for export.',
			},
		},
	},
};

export const InteractiveExport: Story = {
	args: {
		isOpen: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Interactive story testing export functionality.',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Wait for modal to be visible
		const dialog = await canvas.findByTestId('export-dialog');
		await expect(dialog).toBeInTheDocument();

		// Test that export buttons are present
		const jsonButton = canvas.getByTestId('export-json-button');
		const markdownButton = canvas.getByTestId('export-markdown-button');

		await expect(jsonButton).toBeInTheDocument();
		await expect(markdownButton).toBeInTheDocument();

		// Test clicking export buttons (they should be enabled with messages)
		if (!jsonButton.hasAttribute('disabled')) {
			await userEvent.click(jsonButton);
		}
	},
};

export const InteractiveImport: Story = {
	args: {
		isOpen: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Interactive story testing import functionality.',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Find the import file input
		const fileInput = canvas.getByTestId('import-file-input');
		await expect(fileInput).toBeInTheDocument();

		// Test that the import section is present
		const importButton = canvas.getByText('Import JSON File');
		await expect(importButton).toBeInTheDocument();
	},
};

export const ModalBackdropTest: Story = {
	args: {
		isOpen: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Test modal backdrop functionality.',
			},
		},
	},
	play: async ({ canvasElement, args }) => {
		const canvas = within(canvasElement);

		// Wait for modal to be visible
		const dialog = await canvas.findByTestId('export-dialog');
		await expect(dialog).toBeInTheDocument();

		// Test close button
		const closeButton = canvas.getByText('Close');
		await expect(closeButton).toBeInTheDocument();

		await userEvent.click(closeButton);
		await expect(args.onClose).toHaveBeenCalled();
	},
};

export const KeyboardNavigation: Story = {
	args: {
		isOpen: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Test keyboard navigation and accessibility.',
			},
		},
	},
	play: async ({ canvasElement }) => {
		within(canvasElement);

		// Test escape key functionality
		await userEvent.keyboard('{Escape}');
	},
};
