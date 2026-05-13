import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { fn, userEvent, within } from 'storybook/test';

import FileUpload from './FileUpload';

const meta: Meta = {
	title: 'Chat/FileUpload',
	component: FileUpload,
	parameters: {
		layout: 'centered',
		docs: {
			description: {
				component: 'File upload component with drag-and-drop support, file validation, and progress indicators.',
			},
		},
	},
	args: {
		onFilesSelected: fn(),
	},
	argTypes: {
		maxFiles: {
			description: 'Maximum number of files allowed',
			control: { type: 'number', min: 1, max: 20 },
		},
		maxSizeMB: {
			description: 'Maximum file size in megabytes',
			control: { type: 'number', min: 1, max: 100 },
		},
		acceptedTypes: {
			description: 'Accepted file types',
			control: { type: 'object' },
		},
		disabled: {
			description: 'Whether the component is disabled',
			control: { type: 'boolean' },
		},
	},
} satisfies Meta<typeof FileUpload>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		maxFiles: 5,
		maxSizeMB: 10,
	},
	parameters: {
		docs: {
			description: {
				story: 'Default file upload component with standard settings.',
			},
		},
	},
};

export const SingleFile: Story = {
	args: {
		maxFiles: 1,
		maxSizeMB: 5,
	},
	parameters: {
		docs: {
			description: {
				story: 'File upload limited to a single file.',
			},
		},
	},
};

export const ImageOnly: Story = {
	args: {
		maxFiles: 3,
		maxSizeMB: 10,
		acceptedTypes: ['.jpg', '.jpeg', '.png', '.gif', '.webp'],
	},
	parameters: {
		docs: {
			description: {
				story: 'File upload restricted to image files only.',
			},
		},
	},
};

export const DocumentsOnly: Story = {
	args: {
		maxFiles: 5,
		maxSizeMB: 50,
		acceptedTypes: ['.pdf', '.doc', '.docx', '.txt', '.md'],
	},
	parameters: {
		docs: {
			description: {
				story: 'File upload restricted to document files.',
			},
		},
	},
};

export const LargeFiles: Story = {
	args: {
		maxFiles: 2,
		maxSizeMB: 100,
	},
	parameters: {
		docs: {
			description: {
				story: 'File upload configured for larger files (up to 100MB).',
			},
		},
	},
};

export const ManyFiles: Story = {
	args: {
		maxFiles: 20,
		maxSizeMB: 5,
	},
	parameters: {
		docs: {
			description: {
				story: 'File upload allowing many small files.',
			},
		},
	},
};

export const Disabled: Story = {
	args: {
		maxFiles: 5,
		maxSizeMB: 10,
		disabled: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Disabled file upload component.',
			},
		},
	},
};

export const WithFiles: Story = {
	args: {
		maxFiles: 5,
		maxSizeMB: 10,
	},
	parameters: {
		docs: {
			description: {
				story: 'File upload component ready for file selection.',
			},
		},
	},
};

export const Interactive: Story = {
	args: {
		maxFiles: 3,
		maxSizeMB: 10,
	},
	parameters: {
		docs: {
			description: {
				story: 'Interactive story for testing file selection behavior.',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);
		const dropZone = canvas.getByTestId('file-upload-dropzone');

		// Test hover state
		await userEvent.hover(dropZone);

		// Note: File upload testing in Storybook is limited
		// Real file interactions would need to be tested in a proper test environment
	},
};

export const ErrorStates: Story = {
	args: {
		maxFiles: 2,
		maxSizeMB: 1, // Very small to trigger size errors
	},
	parameters: {
		docs: {
			description: {
				story: 'Component configured to easily trigger validation errors for testing.',
			},
		},
	},
};
