import { create } from '@bufbuild/protobuf';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import AgentMessage from './AgentMessage';

const createPart = (
	kind: MessagePartKind,
	id: string,
	content: string,
	status: MessagePartStatus = MessagePartStatus.COMPLETE,
) => create(MessagePartSchema, { kind, id, content, status });

const meta: Meta<typeof AgentMessage> = {
	title: 'Components/Chat/AgentMessage',
	component: AgentMessage,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component:
					'Agent message component for displaying assistant responses with thinking content, tool calls, and actions.',
			},
		},
	},
	tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Basic: Story = {
	args: {
		id: 'assistant-1',
		variant: 'tx',
		timestamp: new Date(),
		parts: [
			createPart(
				MessagePartKind.TEXT,
				'text-1',
				'This is an assistant message. It appears on the left side of the conversation.',
			),
		],
	},
};

export const WithThinking: Story = {
	args: {
		id: 'assistant-thinking-1',
		variant: 'tx',
		timestamp: new Date(),
		parts: [
			createPart(
				MessagePartKind.THINKING,
				'thinking-1',
				"Let me think about this problem step by step...\nFirst, I need to analyze the question.\nThen, I'll calculate the result.",
			),
			createPart(MessagePartKind.TEXT, 'text-1', "I'll help you solve this problem. The answer is 42."),
		],
	},
};

export const StreamingThinking: Story = {
	args: {
		id: 'assistant-streaming-1',
		variant: 'tx',
		timestamp: new Date(),
		parts: [
			createPart(
				MessagePartKind.THINKING,
				'thinking-1',
				'Currently analyzing the problem...',
				MessagePartStatus.STREAMING,
			),
			createPart(MessagePartKind.TEXT, 'text-1', 'Processing your request...', MessagePartStatus.STREAMING),
		],
	},
};

export const WithActions: Story = {
	args: {
		id: 'assistant-actions-1',
		variant: 'tx',
		timestamp: new Date(),
		parts: [createPart(MessagePartKind.TEXT, 'text-1', "Here's a helpful response that you can interact with.")],
		onCopy: () => console.log('Message copied!'),
		onRetry: () => console.log('Retry requested!'),
		onFeedback: (type) => console.log(`Feedback: ${type}`),
	},
};

export const ComplexMessage: Story = {
	args: {
		id: 'assistant-complex-1',
		variant: 'tx',
		timestamp: new Date(),
		parts: [
			createPart(
				MessagePartKind.THINKING,
				'thinking-1',
				'The user is asking about a code optimization problem.\nI should analyze the current implementation and suggest improvements.\nLet me check the performance implications...',
			),
			createPart(MessagePartKind.TEXT, 'text-1', "I've analyzed your code and found a solution."),
		],
		onCopy: () => console.log('Message copied!'),
		onRetry: () => console.log('Retry requested!'),
		onFeedback: (type) => console.log(`Feedback: ${type}`),
	},
};
