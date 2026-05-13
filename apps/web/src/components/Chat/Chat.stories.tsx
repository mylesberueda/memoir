import { create } from '@bufbuild/protobuf';
import { AssistantChatProvider, MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { MessagePartSchema } from '@polypixel/proto-ts/rig-service/rig/v1/inference_pb';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { sub } from 'date-fns';
import { useCallback, useState } from 'react';
import { expect, fn, userEvent, within } from 'storybook/test';
import Chat from './Chat';
import type { ChatMessageProps } from './Message';
import type { PromptBoxForm } from './PromptInput';

const meta: Meta<typeof Chat> = {
	title: 'Components/Chat',
	component: Chat,
	parameters: {
		layout: 'fullscreen',
		docs: {
			description: {
				component: 'A complete chat interface combining message history and input prompt box.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		messages: {
			control: false,
			description: 'Array of chat messages to display',
		},
		placeholder: {
			control: 'text',
			description: 'Placeholder text for the input field',
		},
		onSubmit: {
			control: false,
			description: 'Callback function when a message is submitted',
		},
		enableDeepResearch: {
			control: 'boolean',
			description: 'Enable deep research functionality',
		},
		enableWebSearch: {
			control: 'boolean',
			description: 'Enable web search functionality',
		},
		enableFileUpload: {
			control: 'boolean',
			description: 'Enable file upload functionality',
		},
	},
	decorators: [
		(Story) => (
			<AssistantChatProvider>
				<div className="h-screen max-h-[600px] w-full max-w-4xl mx-auto p-4">
					<Story />
				</div>
			</AssistantChatProvider>
		),
	],
};

export default meta;
type Story = StoryObj<typeof meta>;

// Mock data for different message types
const MOCK_USER_AVATAR = 'https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp';

let mockMessageCounter = 0;
const createMockMessage = (
	variant: 'tx' | 'rx',
	content: string,
	_author?: string,
	minutesAgo = 0,
	avatar?: string,
): ChatMessageProps =>
	({
		id: `mock-${++mockMessageCounter}`,
		variant,
		timestamp: sub(new Date(), { minutes: minutesAgo }),
		avatar,
		parts: [
			create(MessagePartSchema, {
				id: `part-${mockMessageCounter}`,
				kind: MessagePartKind.TEXT,
				status: MessagePartStatus.COMPLETE,
				content,
			}),
		],
	}) as ChatMessageProps;

// Basic empty chat
export const Default: Story = {
	args: {
		messages: [],
		placeholder: 'Type your message...',
		onSubmit: fn(),
	},
};

// Chat with a few messages
export const WithMessages: Story = {
	args: {
		messages: [
			createMockMessage('tx', 'Hello! How can I help you today?', 'Assistant', 10),
			createMockMessage('rx', 'Hi there! I need help with my project.', 'You', 9, MOCK_USER_AVATAR),
			createMockMessage('tx', "I'd be happy to help! What kind of project are you working on?", 'Assistant', 8),
			createMockMessage(
				'rx',
				"I'm building a React application and need advice on state management.",
				'You',
				7,
				MOCK_USER_AVATAR,
			),
		],
		placeholder: 'Continue the conversation...',
		onSubmit: fn(),
	},
};

// Chat with loading/typing indicator
export const LoadingState: Story = {
	args: {
		messages: [
			createMockMessage('rx', "What's the weather like today?", 'You', 2, MOCK_USER_AVATAR),
			createMockMessage('tx', 'Let me check that for you...', 'Assistant', 1),
		],
		placeholder: 'Please wait...',
		onSubmit: fn(),
		isLoading: true,
	},
};

// Chat with error state
export const ErrorState: Story = {
	args: {
		messages: [
			createMockMessage('rx', 'Can you help me with this complex calculation?', 'You', 3, MOCK_USER_AVATAR),
			{
				id: 'error-msg',
				variant: 'tx' as const,
				timestamp: sub(new Date(), { minutes: 1 }),
				footer: (
					<button type="button" className="btn btn-sm btn-outline btn-error mt-2">
						Retry
					</button>
				),
				parts: [
					create(MessagePartSchema, {
						id: 'part-error',
						kind: MessagePartKind.TEXT,
						status: MessagePartStatus.FAILED,
						content: 'Sorry, I encountered an error processing your request. Please try again.',
					}),
				],
			},
		],
		placeholder: 'Try asking something else...',
		onSubmit: fn(),
	},
};

// Chat with different user types
export const DifferentUserTypes: Story = {
	args: {
		messages: [
			createMockMessage('tx', 'Welcome to the support chat!', 'System', 15),
			createMockMessage('rx', 'Hello, I need help with my account.', 'Customer', 14, MOCK_USER_AVATAR),
			createMockMessage(
				'tx',
				"I'll be happy to assist you with your account. Can you provide your email?",
				'Support Agent',
				13,
			),
			createMockMessage('rx', "Sure, it's john.doe@example.com", 'Customer', 12, MOCK_USER_AVATAR),
			createMockMessage(
				'tx',
				'Thank you! I can see your account. What specific issue are you experiencing?',
				'Support Agent',
				11,
			),
			createMockMessage(
				'rx',
				"I can't access my dashboard. It keeps showing an error.",
				'Customer',
				10,
				MOCK_USER_AVATAR,
			),
			createMockMessage(
				'tx',
				'I see the issue. There was a temporary service disruption. It should be resolved now. Please try logging in again.',
				'Support Agent',
				9,
			),
			createMockMessage('tx', 'This conversation has been logged for quality assurance.', 'System', 8),
		],
		placeholder: 'Type your response...',
		onSubmit: fn(),
	},
};

// Chat with all features enabled
export const WithAllFeatures: Story = {
	args: {
		messages: [
			createMockMessage(
				'tx',
				'I have access to web search, deep research, and file upload capabilities. How can I help you today?',
				'AI Assistant',
				5,
			),
			createMockMessage('rx', 'I need to research the latest trends in AI development.', 'You', 4, MOCK_USER_AVATAR),
		],
		placeholder: 'Ask me anything - I have enhanced capabilities!',
		enableDeepResearch: true,
		enableWebSearch: true,
		enableFileUpload: true,
		onSubmit: fn(),
	},
};

// Long conversation to test scrolling
export const LongConversation: Story = {
	args: {
		messages: [
			createMockMessage('tx', "Hello! I'm here to help you with any questions you might have.", 'Assistant', 30),
			createMockMessage('rx', "Hi! I'm working on a new project and need some guidance.", 'You', 29, MOCK_USER_AVATAR),
			createMockMessage('tx', 'That sounds exciting! What kind of project are you working on?', 'Assistant', 28),
			createMockMessage('rx', "It's a web application for managing tasks and projects.", 'You', 27, MOCK_USER_AVATAR),
			createMockMessage(
				'tx',
				'Great choice! Task management apps are very useful. What technologies are you planning to use?',
				'Assistant',
				26,
			),
			createMockMessage(
				'rx',
				"I'm thinking of using React for the frontend and Node.js for the backend.",
				'You',
				25,
				MOCK_USER_AVATAR,
			),
			createMockMessage(
				'tx',
				"Excellent stack! React and Node.js work very well together. Have you considered what database you'll use?",
				'Assistant',
				24,
			),
			createMockMessage('rx', 'I was thinking PostgreSQL. Is that a good choice?', 'You', 23, MOCK_USER_AVATAR),
			createMockMessage(
				'tx',
				"PostgreSQL is an excellent choice! It's robust, reliable, and has great support for complex queries.",
				'Assistant',
				22,
			),
			createMockMessage(
				'rx',
				'What about authentication? Should I build my own or use a service?',
				'You',
				21,
				MOCK_USER_AVATAR,
			),
			createMockMessage(
				'tx',
				"For authentication, I'd recommend using a service like Auth0, Firebase Auth, or AWS Cognito for production apps.",
				'Assistant',
				20,
			),
			createMockMessage('rx', 'That makes sense. What about state management in React?', 'You', 19, MOCK_USER_AVATAR),
			createMockMessage(
				'tx',
				"For state management, you have several options: React Context, Redux Toolkit, or Zustand. What's your experience level?",
				'Assistant',
				18,
			),
			createMockMessage(
				'rx',
				"I'm intermediate with React but new to complex state management.",
				'You',
				17,
				MOCK_USER_AVATAR,
			),
			createMockMessage(
				'tx',
				"I'd recommend starting with React Context for simple state and Zustand for more complex scenarios. They're easier to learn than Redux.",
				'Assistant',
				16,
			),
			createMockMessage('rx', 'Thanks! What about testing? What should I focus on?', 'You', 15, MOCK_USER_AVATAR),
			createMockMessage(
				'tx',
				'For testing, start with Jest and React Testing Library for unit tests, and consider Cypress or Playwright for end-to-end testing.',
				'Assistant',
				14,
			),
			createMockMessage(
				'rx',
				'This is really helpful! Do you have any recommendations for deployment?',
				'You',
				13,
				MOCK_USER_AVATAR,
			),
			createMockMessage(
				'tx',
				"For deployment, Vercel is great for React apps, and Railway or Render work well for Node.js backends. They're beginner-friendly!",
				'Assistant',
				12,
			),
			createMockMessage(
				'rx',
				'Perfect! One last question - any tips for project structure?',
				'You',
				11,
				MOCK_USER_AVATAR,
			),
			createMockMessage(
				'tx',
				'Keep it simple at first: separate components, pages, hooks, and utils into different folders. You can always refactor as the project grows!',
				'Assistant',
				10,
			),
		],
		placeholder: 'Continue the conversation...',
		onSubmit: fn(),
	},
};

// Minimal chat for testing basic functionality
export const Minimal: Story = {
	args: {
		messages: [createMockMessage('tx', 'Hi there!', 'Bot', 1)],
		placeholder: 'Say hello...',
		onSubmit: fn(),
	},
};

// Chat with rich content (code, links, etc.) - uses markdown content that renders as code
export const WithRichContent: Story = {
	args: {
		messages: [
			createMockMessage('rx', 'Can you show me how to create a React component?', 'You', 5, MOCK_USER_AVATAR),
			{
				id: 'rich-content-msg',
				variant: 'tx' as const,
				timestamp: sub(new Date(), { minutes: 3 }),
				parts: [
					create(MessagePartSchema, {
						id: 'part-rich',
						kind: MessagePartKind.TEXT,
						status: MessagePartStatus.COMPLETE,
						content: `Here's a simple React component example:

\`\`\`jsx
import React from 'react';

function MyComponent() {
  return (
    <div>
      <h1>Hello World!</h1>
    </div>
  );
}

export default MyComponent;
\`\`\`

You can also check out the [React documentation](https://react.dev/learn) for more examples.`,
					}),
				],
			},
			createMockMessage('rx', "Thanks! That's very helpful.", 'You', 2, MOCK_USER_AVATAR),
		],
		placeholder: 'Ask about code examples...',
		onSubmit: fn(),
	},
};

// Interactive chat with testing - demonstrates form submission
export const InteractiveSubmission: Story = {
	args: {
		messages: [
			createMockMessage(
				'tx',
				'Hello! Type a message and press Enter or click Send to test the submission.',
				'Assistant',
				2,
			),
		],
		placeholder: 'Type your message here...',
		onSubmit: fn(),
	},
	play: async ({ args, canvasElement }) => {
		const canvas = within(canvasElement);

		const MESSAGE = 'Hello, this is a test message!';

		// Find the textarea input
		const textarea = canvas.getByPlaceholderText('Type your message here...');

		// Type a test message
		await userEvent.type(textarea, MESSAGE);

		// Wait a moment for the button state to update
		await new Promise((resolve) => setTimeout(resolve, 100));

		// Find the submit button by querying DOM directly
		const form = canvasElement.querySelector('form');
		expect(form).toBeInTheDocument();
		const sendButton = form?.querySelector('button[type="submit"]');
		expect(sendButton).toBeInTheDocument();
		if (sendButton) {
			await userEvent.click(sendButton);
		}

		// Verify that onSubmit was called with the correct data
		await expect(args.onSubmit).toHaveBeenCalledWith({
			prompt: MESSAGE,
			files: [],
			enableDeepResearch: undefined,
			enableWebSearch: undefined,
			enableFileUpload: undefined,
		});
	},
};

// Interactive chat with Enter key submission
export const InteractiveEnterKey: Story = {
	args: {
		messages: [createMockMessage('tx', 'Try pressing Enter to submit your message!', 'Assistant', 1)],
		placeholder: 'Press Enter to send...',
		onSubmit: fn(),
	},
	play: async ({ args, canvasElement }) => {
		const canvas = within(canvasElement);

		// Find the textarea input
		const textarea = canvas.getByPlaceholderText('Press Enter to send...');

		// Type a test message and press Enter
		await userEvent.type(textarea, 'Testing Enter key submission{Enter}');

		// Verify that onSubmit was called
		await expect(args.onSubmit).toHaveBeenCalledWith({
			prompt: 'Testing Enter key submission',
			files: [],
			enableDeepResearch: undefined,
			enableWebSearch: undefined,
			enableFileUpload: undefined,
		});
	},
};

// Interactive chat that actually adds messages (nice-to-have feature)
export const LiveInteractiveChat: Story = {
	render: () => {
		const LiveChatWrapper = () => {
			const [messageId, setMessageId] = useState(1);
			const [messages, setMessages] = useState<ChatMessageProps[]>([
				{
					id: 'live-welcome',
					variant: 'tx' as const,
					timestamp: sub(new Date(), { minutes: 2 }),
					parts: [
						create(MessagePartSchema, {
							id: 'part-live-welcome',
							kind: MessagePartKind.TEXT,
							status: MessagePartStatus.COMPLETE,
							content: "Hello! I'm a live chat demo. Type a message and watch it appear in the conversation!",
						}),
					],
				},
			]);

			const handleSubmit = useCallback(
				async (formData: PromptBoxForm) => {
					const userMessage: ChatMessageProps = {
						id: `live-user-${messageId}`,
						variant: 'rx' as const,
						timestamp: new Date(),
						avatar: MOCK_USER_AVATAR,
						parts: [
							create(MessagePartSchema, {
								id: `part-live-user-${messageId}`,
								kind: MessagePartKind.TEXT,
								status: MessagePartStatus.COMPLETE,
								content: formData.prompt,
							}),
						],
					};

					setMessages((prev) => [...prev, userMessage]);
					const currentId = messageId;
					setMessageId((prev) => prev + 1);

					setTimeout(() => {
						const assistantMessage: ChatMessageProps = {
							id: `live-assistant-${currentId}`,
							variant: 'tx' as const,
							timestamp: new Date(),
							parts: [
								create(MessagePartSchema, {
									id: `part-live-assistant-${currentId}`,
									kind: MessagePartKind.TEXT,
									status: MessagePartStatus.COMPLETE,
									content: `I received your message: "${formData.prompt}". This is a simulated response to demonstrate the live chat functionality!`,
								}),
							],
						};
						setMessages((prev) => [...prev, assistantMessage]);
					}, 1000);
				},
				[messageId],
			);

			return (
				<Chat
					messages={messages}
					placeholder="Type a message to see it appear in the chat..."
					onSubmit={handleSubmit}
					enableDeepResearch={true}
					enableWebSearch={true}
					enableFileUpload={true}
				/>
			);
		};

		return <LiveChatWrapper />;
	},
	parameters: {
		docs: {
			description: {
				story:
					'A fully interactive chat that adds your messages to the conversation and simulates responses. Perfect for testing the complete user experience.',
			},
		},
	},
};
