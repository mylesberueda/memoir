import { create } from '@bufbuild/protobuf';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { fn } from 'storybook/test';

import type { AgentMessageProps, ChatMessageProps } from '../Message';

import Conversation from './Conversation';

// Mock messages for testing
let messageIdCounter = 0;
const createMockMessage = (variant: 'rx' | 'tx', timestamp: Date, content: string): ChatMessageProps =>
	({
		id: `mock-msg-${++messageIdCounter}`,
		variant,
		timestamp,
		parts: [
			create(MessagePartSchema, {
				kind: MessagePartKind.TEXT,
				id: `text-${messageIdCounter}`,
				content,
				status: MessagePartStatus.COMPLETE,
			}),
		],
	}) as ChatMessageProps;

const mockMessages: ChatMessageProps[] = [
	createMockMessage('rx', new Date('2024-01-01T10:00:00Z'), 'Hello, can you help me with React?'),
	createMockMessage(
		'tx',
		new Date('2024-01-01T10:00:01Z'),
		"Of course! I'd be happy to help you with React. What specific aspect would you like to learn about?",
	),
	createMockMessage('rx', new Date('2024-01-01T10:00:02Z'), 'How do I create a custom hook?'),
	createMockMessage(
		'tx',
		new Date('2024-01-01T10:00:03Z'),
		'A custom hook is a JavaScript function whose name starts with "use" and that may call other hooks. Here\'s a simple example:\n\n```javascript\nfunction useCounter(initialValue = 0) {\n  const [count, setCount] = useState(initialValue);\n  \n  const increment = () => setCount(count + 1);\n  const decrement = () => setCount(count - 1);\n  \n  return { count, increment, decrement };\n}\n```',
	),
	createMockMessage('rx', new Date('2024-01-01T10:00:04Z'), "That's helpful! Can you show me how to use it?"),
	createMockMessage(
		'tx',
		new Date('2024-01-01T10:00:05Z'),
		"Sure! Here's how you would use the custom hook in a component:\n\n```javascript\nfunction Counter() {\n  const { count, increment, decrement } = useCounter(0);\n  \n  return (\n    <div>\n      <p>Count: {count}</p>\n      <button onClick={increment}>+</button>\n      <button onClick={decrement}>-</button>\n    </div>\n  );\n}\n```",
	),
];

// Create many messages for virtual scrolling test
const manyMessages: ChatMessageProps[] = Array.from({ length: 100 }, (_, i) =>
	createMockMessage(
		i % 2 === 0 ? 'rx' : 'tx',
		new Date(`2024-01-01T${String(Math.floor(i / 60) + 10).padStart(2, '0')}:${String(i % 60).padStart(2, '0')}:00Z`),
		`This is message number ${i + 1}. Lorem ipsum dolor sit amet, consectetur adipiscing elit. ${i % 5 === 0 ? 'This is a longer message to test variable height handling in the virtual scroll component.' : ''}`,
	),
);

const meta = {
	title: 'Chat/Conversation',
	component: Conversation,
	parameters: {
		layout: 'fullscreen',
		docs: {
			description: {
				component:
					'Conversation component that automatically switches between regular rendering for few messages and virtual scrolling for large conversation histories.',
			},
		},
	},
	args: {
		onRetry: fn(),
		onCopy: fn(),
		onFeedback: fn(),
	},
	argTypes: {
		messages: {
			description: 'Array of messages to display',
			control: { type: 'object' },
		},
		isLoading: {
			description: 'Whether the chat is currently loading',
			control: { type: 'boolean' },
		},
		isStreaming: {
			description: 'Whether messages are being streamed',
			control: { type: 'boolean' },
		},
		className: {
			description: 'Additional CSS classes',
			control: { type: 'text' },
		},
	},
} satisfies Meta<typeof Conversation>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		messages: mockMessages,
	},
	parameters: {
		docs: {
			description: {
				story: 'Basic conversation with a few messages (uses regular scrolling).',
			},
		},
	},
};

export const Empty: Story = {
	args: {
		messages: [],
	},
	parameters: {
		docs: {
			description: {
				story: 'Empty state when no messages are present.',
			},
		},
	},
};

export const Loading: Story = {
	args: {
		messages: mockMessages,
		isLoading: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Loading state with spinner indicator.',
			},
		},
	},
};

export const Streaming: Story = {
	args: {
		messages: mockMessages,
		isStreaming: true,
	},
	parameters: {
		docs: {
			description: {
				story: 'Streaming state while receiving new messages.',
			},
		},
	},
};

export const ManyMessages: Story = {
	args: {
		messages: manyMessages,
	},
	parameters: {
		docs: {
			description: {
				story:
					'Virtual scrolling with many messages to test performance (automatically uses virtual scrolling for 10+ messages).',
			},
		},
	},
};

export const WithToolCalls: Story = {
	args: {
		messages: [
			createMockMessage('rx', new Date('2024-01-01T10:00:00Z'), 'Can you search for information about React hooks?'),
			{
				...createMockMessage(
					'tx',
					new Date('2024-01-01T10:00:01Z'),
					"I'll search for information about React hooks for you.",
				),
			},
		],
	},
	parameters: {
		docs: {
			description: {
				story: 'Messages with tool calls to test tool execution display.',
			},
		},
	},
};

export const WithThinking: Story = {
	args: {
		messages: [
			createMockMessage('rx', new Date('2024-01-01T10:00:00Z'), 'Explain quantum computing'),
			{
				...createMockMessage(
					'tx',
					new Date('2024-01-01T10:00:01Z'),
					'Quantum computing is a revolutionary computing paradigm...',
				),
				parts: [
					create(MessagePartSchema, {
						kind: MessagePartKind.THINKING,
						id: 'thinking-1',
						content:
							"This is a complex topic. I should start with the basics and explain quantum bits, superposition, and entanglement in a way that's accessible...",
						status: MessagePartStatus.COMPLETE,
					}),
					create(MessagePartSchema, {
						kind: MessagePartKind.TEXT,
						id: 'text-1',
						content:
							'Quantum computing is a revolutionary computing paradigm that leverages quantum mechanical phenomena to process information in fundamentally different ways than classical computers.',
						status: MessagePartStatus.COMPLETE,
					}),
				],
				thinkingDuration: 5,
			} as AgentMessageProps,
		],
	},
	parameters: {
		docs: {
			description: {
				story: 'Messages with thinking content to test reasoning display.',
			},
		},
	},
};

export const VariableHeights: Story = {
	args: {
		messages: [
			createMockMessage('rx', new Date('2024-01-01T10:00:00Z'), 'Short'),
			createMockMessage(
				'tx',
				new Date('2024-01-01T10:00:01Z'),
				'This is a medium length response that spans multiple lines and contains more detailed information about the topic at hand.',
			),
			createMockMessage(
				'rx',
				new Date('2024-01-01T10:00:02Z'),
				'Very long message that contains a lot of text and goes on for quite a while, testing how the component handles messages of varying heights. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.',
			),
			createMockMessage('tx', new Date('2024-01-01T10:00:03Z'), 'Short reply.'),
			createMockMessage(
				'rx',
				new Date('2024-01-01T10:00:04Z'),
				'Another message with different length to test the dynamic height calculation and ensure proper scrolling behavior.',
			),
		],
	},
	parameters: {
		docs: {
			description: {
				story: 'Messages with varying heights to test dynamic height calculation.',
			},
		},
	},
};

export const ForcedVirtualScrolling: Story = {
	args: {
		messages: Array.from({ length: 15 }, (_, i) =>
			createMockMessage(
				i % 2 === 0 ? 'rx' : 'tx',
				new Date(`2024-01-01T10:${String(i).padStart(2, '0')}:00Z`),
				`Message ${i + 1} - This message tests the virtual scrolling behavior when there are more than 10 messages.`,
			),
		),
	},
	parameters: {
		docs: {
			description: {
				story: 'Forces virtual scrolling mode with exactly 15 messages to test the 10+ threshold.',
			},
		},
	},
};
