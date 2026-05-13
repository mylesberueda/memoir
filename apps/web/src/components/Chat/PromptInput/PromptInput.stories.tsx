import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { useState } from 'react';
import { fn } from 'storybook/test';
import type { PromptBoxForm } from './PromptInput';
import PromptInput from './PromptInput';

const meta: Meta<typeof PromptInput> = {
	title: 'Components/Chat/PromptInput',
	component: PromptInput,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component: 'Complete chat input component with message input, file upload, and various feature toggles.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		onSubmit: {
			description: 'Callback when form is submitted',
			action: 'submitted',
		},
		disabled: {
			control: 'boolean',
			description: 'Disable all input controls',
		},
		isLoading: {
			control: 'boolean',
			description: 'Show loading state',
		},
		isStreaming: {
			control: 'boolean',
			description: 'Show streaming state',
		},
		onStopStreaming: {
			description: 'Callback to stop streaming',
			action: 'stop-streaming',
		},
		enableFileUpload: {
			control: 'boolean',
			description: 'Enable file upload feature',
		},
		enableWebSearch: {
			control: 'boolean',
			description: 'Enable web search feature',
		},
		enableDeepResearch: {
			control: 'boolean',
			description: 'Enable deep research feature',
		},
		enableMic: {
			control: 'boolean',
			description: 'Enable voice input feature',
		},
		placeholder: {
			control: 'text',
			description: 'Placeholder text for input',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic prompt input
export const Basic: Story = {
	args: {
		onSubmit: fn(),
		placeholder: 'What would you like to know?',
	},
};

// With all features enabled
export const AllFeatures: Story = {
	args: {
		onSubmit: fn(),
		enableFileUpload: true,
		enableWebSearch: true,
		enableDeepResearch: true,
		enableMic: true,
		placeholder: 'Ask me anything...',
	},
};

// Loading state
export const Loading: Story = {
	args: {
		onSubmit: fn(),
		isLoading: true,
		placeholder: 'Processing your request...',
	},
};

// Streaming state with stop button
export const Streaming: Story = {
	args: {
		onSubmit: fn(),
		isStreaming: true,
		onStopStreaming: fn(),
		placeholder: 'Generating response...',
	},
};

// Disabled state
export const Disabled: Story = {
	args: {
		onSubmit: fn(),
		disabled: true,
		placeholder: 'Input disabled',
	},
};

// File upload only
export const FileUploadOnly: Story = {
	args: {
		onSubmit: fn(),
		enableFileUpload: true,
		placeholder: 'Upload files and ask questions...',
	},
};

// Web search and deep research
export const ResearchMode: Story = {
	args: {
		onSubmit: fn(),
		enableWebSearch: true,
		enableDeepResearch: true,
		placeholder: 'Research your question...',
	},
};

// Interactive example with state management
export const Interactive: Story = {
	render: () => {
		const [isLoading, setIsLoading] = useState(false);
		const [isStreaming, setIsStreaming] = useState(false);
		const [messages, setMessages] = useState<string[]>([]);

		const handleSubmit = async (form: PromptBoxForm) => {
			console.log('Form submitted:', form);
			setMessages((prev) => [...prev, `User: ${form.prompt}`]);

			// Simulate loading
			setIsLoading(true);
			await new Promise((resolve) => setTimeout(resolve, 1000));
			setIsLoading(false);

			// Simulate streaming
			setIsStreaming(true);
			await new Promise((resolve) => setTimeout(resolve, 2000));
			setIsStreaming(false);

			setMessages((prev) => [...prev, `Assistant: Response to "${form.prompt}"`]);
		};

		const handleStopStreaming = () => {
			setIsStreaming(false);
			setMessages((prev) => [...prev, 'Assistant: [Response stopped]']);
		};

		return (
			<div className="space-y-4">
				<div className="bg-gray-100 p-4 rounded min-h-[200px]">
					<h3 className="font-semibold mb-2">Messages:</h3>
					{messages.length === 0 ? (
						<p className="text-gray-500">No messages yet. Type something below!</p>
					) : (
						<div className="space-y-2">
							{messages.map((msg, _idx) => (
								<div key={msg} className="text-sm">
									{msg}
								</div>
							))}
						</div>
					)}
				</div>

				<PromptInput
					onSubmit={handleSubmit}
					isLoading={isLoading}
					isStreaming={isStreaming}
					onStopStreaming={handleStopStreaming}
					enableFileUpload
					enableWebSearch
					enableDeepResearch
					enableMic
					placeholder="Try typing a message..."
				/>
			</div>
		);
	},
};

// Custom placeholder
export const CustomPlaceholder: Story = {
	args: {
		onSubmit: fn(),
		placeholder: 'Tell me what you need help with today...',
	},
};

// Minimal configuration
export const Minimal: Story = {
	args: {
		onSubmit: fn(),
		placeholder: 'Simple input...',
	},
};
