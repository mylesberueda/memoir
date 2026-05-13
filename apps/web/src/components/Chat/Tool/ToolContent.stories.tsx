import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, within } from 'storybook/test';

import ToolContent from './ToolContent';
import ToolInput from './ToolInput';
import ToolOutput from './ToolOutput';

const meta: Meta<typeof ToolContent> = {
	title: 'Components/Chat/Tool/ToolContent',
	component: ToolContent,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component:
					'Content container for tool UI components. Provides consistent styling and layout for tool input and output components.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		children: {
			control: 'text',
			description: 'Content to display inside the tool content container',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic text content
export const Default: Story = {
	args: {
		children: 'Basic tool content container with simple text.',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify content is rendered
		await expect(canvas.getByText('Basic tool content container with simple text.')).toBeInTheDocument();

		// Verify container styling
		const container = canvas.getByText('Basic tool content container with simple text.').closest('div');
		await expect(container).toHaveClass('text-popover-foreground', 'outline-none');
	},
};

// With ToolInput component
export const WithToolInput: Story = {
	args: {
		children: (
			<ToolInput
				input={{
					query: 'React hooks tutorial',
					maxResults: 10,
					language: 'en',
				}}
			/>
		),
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify ToolInput is rendered within the content
		await expect(canvas.getByText('Parameters')).toBeInTheDocument();
		await expect(canvasElement.textContent).toContain('React hooks tutorial');
		await expect(canvasElement.textContent).toContain('10');
	},
};

// With ToolOutput component
export const WithToolOutput: Story = {
	args: {
		children: <ToolOutput output="Successfully found 10 results about React hooks tutorial." errorText={undefined} />,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify ToolOutput is rendered within the content
		await expect(canvas.getByText('Result')).toBeInTheDocument();
		await expect(canvas.getByText('Successfully found 10 results about React hooks tutorial.')).toBeInTheDocument();
	},
};

// With ToolOutput error
export const WithToolError: Story = {
	args: {
		children: (
			<ToolOutput output={undefined} errorText="Search service is currently unavailable. Please try again later." />
		),
	},
};

// With both input and output
export const WithInputAndOutput: Story = {
	args: {
		children: (
			<>
				<ToolInput
					input={{
						operation: 'file_read',
						path: '/home/user/document.pdf',
						format: 'text',
					}}
				/>
				<ToolOutput output="File read successfully. Extracted 1,247 words from 5 pages." errorText={undefined} />
			</>
		),
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify both input and output are present
		await expect(canvas.getByText('Parameters')).toBeInTheDocument();
		// Use textContent for JSON values that may be syntax-highlighted
		expect(canvasElement.textContent).toContain('file_read');
		expect(canvasElement.textContent).toContain('/home/user/document.pdf');

		await expect(canvas.getByText('Result')).toBeInTheDocument();
		await expect(canvas.getByText('File read successfully. Extracted 1,247 words from 5 pages.')).toBeInTheDocument();
	},
};

// With input and error
export const WithInputAndError: Story = {
	args: {
		children: (
			<>
				<ToolInput
					input={{
						operation: 'file_write',
						path: '/system/protected/config.txt',
						content: 'new configuration data',
					}}
				/>
				<ToolOutput output={undefined} errorText="Permission denied: Cannot write to protected system directory." />
			</>
		),
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify input parameters
		await expect(canvas.getByText('Parameters')).toBeInTheDocument();
		// Use textContent for JSON values that may be syntax-highlighted
		expect(canvasElement.textContent).toContain('file_write');

		// Verify error output
		await expect(canvas.getByText('Error')).toBeInTheDocument();
		await expect(
			canvas.getByText('Permission denied: Cannot write to protected system directory.'),
		).toBeInTheDocument();

		// Verify error styling (use class list check for Tailwind classes with /)
		const containers = canvasElement.querySelectorAll('div');
		const errorContainer = Array.from(containers).find((div) => div.classList.contains('bg-destructive/10'));
		await expect(errorContainer).toBeDefined();
	},
};

// Complex nested content
export const ComplexContent: Story = {
	args: {
		children: (
			<div className="space-y-4">
				<ToolInput
					input={{
						analysis_type: 'sentiment_analysis',
						text: 'I love this new feature! It makes everything so much easier to use.',
						options: {
							include_confidence: true,
							detailed_emotions: true,
							language: 'auto-detect',
						},
					}}
				/>
				<ToolOutput
					output={
						<div className="space-y-2">
							<div className="font-medium">Sentiment Analysis Results</div>
							<div className="grid grid-cols-2 gap-2 text-sm">
								<div>
									Overall Sentiment: <span className="text-green-600 font-medium">Positive</span>
								</div>
								<div>Confidence: 94.7%</div>
							</div>
							<div className="space-y-1">
								<div className="text-sm font-medium">Detailed Emotions:</div>
								<ul className="text-sm space-y-1">
									<li>Joy: 89.2%</li>
									<li>Excitement: 76.4%</li>
									<li>Satisfaction: 81.7%</li>
								</ul>
							</div>
						</div>
					}
					errorText={undefined}
				/>
			</div>
		),
	},
};

// Multiple operations
export const MultipleOperations: Story = {
	args: {
		children: (
			<div className="space-y-6">
				<div>
					<ToolInput input={{ query: 'search step 1', term: 'React' }} />
					<ToolOutput output="Step 1: Found 150 React-related results" errorText={undefined} />
				</div>
				<div>
					<ToolInput input={{ query: 'filter step 2', minScore: 0.8 }} />
					<ToolOutput output="Step 2: Filtered to 45 high-quality results" errorText={undefined} />
				</div>
				<div>
					<ToolInput input={{ query: 'summarize step 3', maxLength: 500 }} />
					<ToolOutput output="Step 3: Generated comprehensive summary of findings" errorText={undefined} />
				</div>
			</div>
		),
	},
};

// Processing states
export const ProcessingStates: Story = {
	args: {
		children: (
			<div className="space-y-4">
				<ToolInput
					input={{
						model: 'gpt-4',
						prompt: 'Analyze this code for potential improvements',
						temperature: 0.3,
					}}
				/>
				<div className="p-4 text-center text-muted-foreground">
					<div className="animate-pulse">Processing request...</div>
					<div className="text-xs mt-1">This may take a few moments</div>
				</div>
			</div>
		),
	},
};

// Empty content
export const EmptyContent: Story = {
	args: {
		children: undefined,
	},
	play: async ({ canvasElement }) => {
		// Verify container is present but empty (excluding Storybook wrapper)
		const toolContentDiv = canvasElement.querySelector('.text-popover-foreground');
		await expect(toolContentDiv).toBeInTheDocument();

		// Container should have the expected styling classes
		await expect(toolContentDiv).toHaveClass('text-popover-foreground', 'outline-none');

		// Verify it's empty (no text content in the actual component)
		await expect(toolContentDiv?.textContent?.trim() || '').toBe('');
	},
};

// Custom HTML content
export const CustomHTMLContent: Story = {
	args: {
		children: (
			<div className="prose prose-sm max-w-none">
				<h4>Tool Execution Summary</h4>
				<p>The following operations were performed:</p>
				<ol>
					<li>Data validation and preprocessing</li>
					<li>Model inference and prediction</li>
					<li>Result formatting and optimization</li>
				</ol>
				<blockquote>
					<p>All operations completed successfully with high confidence scores.</p>
				</blockquote>
			</div>
		),
	},
};

// Custom styling
export const CustomStyling: Story = {
	args: {
		children: 'Tool content with custom styling applied',
		className: 'bg-blue-50 border-l-4 border-l-blue-400 p-4',
	},
};

// Interactive content
export const InteractiveContent: Story = {
	args: {
		children: (
			<div className="space-y-3">
				<div className="text-sm font-medium">Interactive Results</div>
				<div className="grid grid-cols-3 gap-2">
					<button type="button" className="p-2 text-xs bg-gray-100 hover:bg-gray-200 rounded">
						View Details
					</button>
					<button type="button" className="p-2 text-xs bg-gray-100 hover:bg-gray-200 rounded">
						Export Data
					</button>
					<button type="button" className="p-2 text-xs bg-gray-100 hover:bg-gray-200 rounded">
						Run Again
					</button>
				</div>
			</div>
		),
	},
};

// Long scrollable content
export const LongScrollableContent: Story = {
	args: {
		children: (
			<div className="space-y-4">
				<ToolInput
					input={{
						operation: 'log_analysis',
						file: 'server.log',
						date_range: '2024-01-01 to 2024-01-31',
					}}
				/>
				<ToolOutput
					output={
						<div className="max-h-64 overflow-y-auto">
							<div className="font-medium mb-2">Log Analysis Results (500 entries)</div>
							{Array.from({ length: 50 }, (_, i) => {
								const date = `2024-01-${String(i + 1).padStart(2, '0')}`;
								const time = `12:${String(i).padStart(2, '0')}:00`;
								const duration = Math.floor(Math.random() * 1000);
								const logId = `${date}-${time}-${duration}`;
								return (
									<div key={logId} className="text-xs py-1 border-b border-gray-100">
										{`${date} ${time} - Info: Request processed successfully (${duration}ms)`}
									</div>
								);
							})}
						</div>
					}
					errorText={undefined}
				/>
			</div>
		),
	},
};

// Content with tables
export const ContentWithTables: Story = {
	args: {
		children: (
			<div className="space-y-4">
				<ToolInput
					input={{
						query: 'performance_metrics',
						period: 'last_7_days',
					}}
				/>
				<ToolOutput
					output={
						<div className="overflow-x-auto">
							<table className="w-full text-xs">
								<thead>
									<tr className="border-b">
										<th className="text-left p-2">Metric</th>
										<th className="text-left p-2">Value</th>
										<th className="text-left p-2">Change</th>
									</tr>
								</thead>
								<tbody>
									<tr className="border-b">
										<td className="p-2">Response Time</td>
										<td className="p-2">245ms</td>
										<td className="p-2 text-green-600">↓ 12%</td>
									</tr>
									<tr className="border-b">
										<td className="p-2">Error Rate</td>
										<td className="p-2">0.03%</td>
										<td className="p-2 text-green-600">↓ 45%</td>
									</tr>
									<tr>
										<td className="p-2">Throughput</td>
										<td className="p-2">1.2k req/s</td>
										<td className="p-2 text-green-600">↑ 8%</td>
									</tr>
								</tbody>
							</table>
						</div>
					}
					errorText={undefined}
				/>
			</div>
		),
	},
};

// Mixed content showcase
export const MixedContentShowcase: Story = {
	render: () => (
		<div className="grid gap-4 md:grid-cols-2">
			<ToolContent>
				<ToolInput input={{ type: 'simple', value: 'test' }} />
				<ToolOutput output="Simple success" errorText={undefined} />
			</ToolContent>
			<ToolContent>
				<ToolInput input={{ type: 'error', value: 'fail' }} />
				<ToolOutput output={undefined} errorText="Something went wrong" />
			</ToolContent>
		</div>
	),
};
