import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { useEffect, useState } from 'react';
import { expect, userEvent, waitFor, within } from 'storybook/test';
import Reasoning from './Reasoning';

const meta: Meta<typeof Reasoning> = {
	title: 'Components/Chat/Reasoning',
	component: Reasoning,
	parameters: {
		layout: 'centered',
		docs: {
			description: {
				component:
					'A collapsible reasoning component that displays AI thinking process with a live timer during streaming.',
			},
		},
	},
	decorators: [
		(Story) => (
			<div style={{ width: '600px', padding: '20px' }}>
				<Story />
			</div>
		),
	],
	tags: ['autodocs'],
	argTypes: {
		content: {
			control: 'text',
			description: 'The reasoning content to display',
		},
		title: {
			control: 'text',
			description: 'Custom title for the reasoning section',
		},
		defaultOpen: {
			control: 'boolean',
			description: 'Whether the reasoning section should be open by default',
		},
		thinkingDuration: {
			control: 'number',
			description: 'Duration of thinking in seconds (for completed reasoning)',
		},
	},
} satisfies Meta<typeof Reasoning>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		content:
			"Let me analyze this step by step:\n\n1. First, I need to understand the context\n2. Then, I'll evaluate the options\n3. Finally, I'll provide a solution\n\nThis approach ensures accuracy and completeness.",
		title: 'Reasoning',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Check that the component renders with collapsed state
		const button = canvas.getByRole('button');
		await expect(button).toBeInTheDocument();
		await expect(button).toHaveAttribute('aria-expanded', 'false');

		// Click to expand
		await userEvent.click(button);
		await expect(button).toHaveAttribute('aria-expanded', 'true');

		// Content should be visible
		const content = canvas.getByText(/Let me analyze this step by step/);
		await expect(content).toBeInTheDocument();

		// Click to collapse
		await userEvent.click(button);
		await expect(button).toHaveAttribute('aria-expanded', 'false');
	},
};

export const CustomTitle: Story = {
	args: {
		content: 'Processing the request with advanced algorithms...',
		title: 'AI Analysis',
	},
};

export const DefaultOpen: Story = {
	args: {
		content: 'This reasoning section is open by default, showing the thinking process immediately.',
		defaultOpen: true,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Should be open by default
		const button = canvas.getByRole('button');
		await expect(button).toHaveAttribute('aria-expanded', 'true');

		// Content should be visible
		const content = canvas.getByText(/This reasoning section is open by default/);
		await expect(content).toBeInTheDocument();
	},
};

export const WithThinkingDuration: Story = {
	args: {
		content: 'This was a complex problem that required careful analysis.',
		thinkingDuration: 8,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Should display "Thought for Xs"
		await expect(canvas.getByText('Thought for 8s')).toBeInTheDocument();

		// Should be collapsed by default
		const button = canvas.getByRole('button');
		await expect(button).toHaveAttribute('aria-expanded', 'false');

		// Click to expand and verify content
		await userEvent.click(button);
		await expect(button).toHaveAttribute('aria-expanded', 'true');
		const content = canvas.getByText(/complex problem that required careful analysis/);
		await expect(content).toBeInTheDocument();
	},
};

export const LongContent: Story = {
	args: {
		content: `Let me break down this complex problem:

1. **Understanding the Requirements**
   - We need to implement a feature that handles real-time updates
   - The system must be scalable and performant
   - Security is a top priority

2. **Analyzing Current Architecture**
   - The existing system uses WebSockets for real-time communication
   - We have a microservices architecture with service mesh
   - Authentication is handled via JWT tokens

3. **Proposed Solution**
   - Implement a pub/sub pattern using Redis
   - Add caching layer to reduce database load
   - Use connection pooling for better resource management

4. **Implementation Steps**
   - Set up Redis cluster
   - Implement message broker service
   - Add monitoring and alerting
   - Write comprehensive tests

5. **Potential Challenges**
   - Network latency in distributed system
   - Handling connection drops gracefully
   - Ensuring message delivery guarantees

This approach balances performance, reliability, and maintainability.`,
		defaultOpen: true,
	},
};

export const ReactNodeContent: Story = {
	args: {
		content: (
			<div>
				<p>
					<strong>Analysis Phase:</strong>
				</p>
				<ul>
					<li>Evaluated 3 different approaches</li>
					<li>Considered performance implications</li>
					<li>Reviewed security requirements</li>
				</ul>
				<p>
					<strong>Decision:</strong>
				</p>
				<p>Based on the analysis, Option B provides the best balance of features and performance.</p>
			</div>
		),
		defaultOpen: true,
	},
};

// Interactive streaming story
export const StreamingSimulation: Story = {
	render: function StreamingComponent() {
		const [isStreaming, setIsStreaming] = useState(false);
		const [content, setContent] = useState('');
		const [thinkingDuration, setThinkingDuration] = useState<number | undefined>();

		const startStreaming = () => {
			setIsStreaming(true);
			setThinkingDuration(undefined);
			setContent('');

			// Simulate streaming content
			const messages = [
				'Let me think about this problem...\n',
				'I need to consider several factors:\n',
				'1. Performance requirements\n',
				'2. Scalability needs\n',
				'3. Security implications\n',
				'\nAnalyzing the data...\n',
				'Processing information...\n',
				'\nFormulating response...',
			];

			let index = 0;
			const interval = setInterval(() => {
				if (index < messages.length) {
					setContent((prev) => prev + messages[index]);
					index++;
				} else {
					clearInterval(interval);
					setIsStreaming(false);
				}
			}, 1500);

			return () => clearInterval(interval);
		};

		const _handleThinkingComplete = (duration: number) => {
			setThinkingDuration(duration);
			console.log(`Thinking completed in ${duration} seconds`);
		};

		return (
			<div>
				<div style={{ marginBottom: '20px' }}>
					<button
						type="button"
						onClick={startStreaming}
						disabled={isStreaming}
						style={{
							padding: '10px 20px',
							backgroundColor: isStreaming ? '#ccc' : '#007bff',
							color: 'white',
							border: 'none',
							borderRadius: '4px',
							cursor: isStreaming ? 'not-allowed' : 'pointer',
						}}>
						{isStreaming ? 'Streaming...' : 'Start Streaming'}
					</button>
				</div>
				<Reasoning
					content={content || 'Click "Start Streaming" to see the timer in action'}
					thinkingDuration={thinkingDuration}
				/>
			</div>
		);
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Find and click the start streaming button
		const button = canvas.getByText('Start Streaming');
		await expect(button).toBeInTheDocument();

		// Click to start streaming
		await userEvent.click(button);

		// Verify button changes to "Streaming..."
		await waitFor(() => {
			expect(canvas.getByText('Streaming...')).toBeInTheDocument();
		});

		// Verify the reasoning component shows streaming state
		const reasoningButton = canvas.getAllByRole('button')[1]; // Second button is the reasoning toggle
		await expect(reasoningButton).toHaveAttribute('aria-expanded', 'true');

		// Should show "Thinking... 0s" initially
		await expect(canvas.getByText('Thinking... 0s')).toBeInTheDocument();
	},
};

// Live timer demonstration
export const LiveTimer: Story = {
	render: function LiveTimerComponent() {
		const [isStreaming, setIsStreaming] = useState(true);
		const [duration, setDuration] = useState<number | undefined>();

		useEffect(() => {
			// Auto-stop after 10 seconds
			const timeout = setTimeout(() => {
				setIsStreaming(false);
			}, 10000);

			return () => clearTimeout(timeout);
		}, []);

		const _handleComplete = (d: number) => {
			setDuration(d);
		};

		const resetTimer = () => {
			setIsStreaming(true);
			setDuration(undefined);

			// Auto-stop after 10 seconds
			setTimeout(() => {
				setIsStreaming(false);
			}, 10000);
		};

		return (
			<div>
				<div style={{ marginBottom: '20px', textAlign: 'center' }}>
					<p style={{ color: '#666' }}>
						{isStreaming
							? '⏱ Timer is running... (will stop after 10 seconds)'
							: `✅ Timer stopped at ${duration} seconds`}
					</p>
					{!isStreaming && (
						<button
							type="button"
							onClick={resetTimer}
							style={{
								marginTop: '10px',
								padding: '8px 16px',
								backgroundColor: '#28a745',
								color: 'white',
								border: 'none',
								borderRadius: '4px',
								cursor: 'pointer',
							}}>
							Reset Timer
						</button>
					)}
				</div>
				<Reasoning
					content={`This demonstration shows the live timer functionality.

The timer:
• Starts at 0 seconds when streaming begins
• Updates every second while streaming
• Stops and displays final duration when streaming ends
• Auto-opens the reasoning section during streaming
• Auto-closes after streaming completes (if not manually toggled)

Watch the timer count up in real-time above!`}
					thinkingDuration={duration}
					defaultOpen={true}
				/>
			</div>
		);
	},
};

// Error state / Empty content
export const EmptyContent: Story = {
	args: {
		content: '',
	},
};

// Code block content
export const WithCodeContent: Story = {
	args: {
		content: `Analyzing the code structure:

\`\`\`typescript
function calculateTimer(startTime: number): number {
  const elapsed = Date.now() - startTime;
  return Math.floor(elapsed / 1000);
}
\`\`\`

This function calculates elapsed time in seconds.`,
		defaultOpen: true,
	},
};
