import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, within } from 'storybook/test';

import Tool from './Tool';
import ToolInput from './ToolInput';
import ToolOutput from './ToolOutput';

const meta: Meta<typeof Tool> = {
	title: 'Components/Chat/Tool/Tool',
	component: Tool,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component:
					'Main container component for tool UI. Provides consistent border and spacing for tool interactions with accordion functionality.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		type: {
			control: 'text',
			description: 'The tool type for display',
		},
		state: {
			control: 'select',
			options: ['input-streaming', 'input-available', 'output-available', 'output-error'],
			description: 'The current state of the tool execution',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic empty tool container
export const Default: Story = {
	args: {
		type: 'tool-example',
		state: 'output-available',
		children: 'Basic tool container',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify tool container is rendered with correct styling
		const toolContainer = canvasElement.querySelector('[class*="border-success"]');
		await expect(toolContainer).toBeInTheDocument();
		await expect(canvas.getByText('tool-example')).toBeInTheDocument();
	},
};

// Tool with custom styling
export const CustomStyling: Story = {
	args: {
		type: 'tool-custom',
		state: 'output-error',
		className: 'bg-primary-content border-primary',
		children: 'Tool container with custom styling',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify custom styling is applied
		const toolContainer = canvasElement.querySelector('.bg-primary-content');
		await expect(toolContainer).toHaveClass('bg-primary-content', 'border-primary');
		await expect(canvas.getByText('tool-custom')).toBeInTheDocument();
	},
};

// Complete tool structure with header and content
export const CompleteToolStructure: Story = {
	args: {
		type: 'tool-search',
		state: 'output-available',
		children: (
			<>
				<ToolInput
					input={{
						query: 'JavaScript best practices',
						maxResults: 10,
					}}
				/>
				<ToolOutput output="Found 10 results about JavaScript best practices" errorText={undefined} />
			</>
		),
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify tool header is present and clickable
		const toolHeader = canvas.getByText('tool-search').closest('button');
		await expect(toolHeader).toHaveClass('cursor-pointer');
		await expect(canvas.getByText('tool-search')).toBeInTheDocument();
		await expect(canvas.getByText('Completed')).toBeInTheDocument();

		// Content should be hidden by default (closed state)
		// Tool starts in closed state, so content won't be visible initially
	},
};

// Tool with running state
export const RunningTool: Story = {
	args: {
		type: 'tool-code_execution',
		state: 'input-available',
		children: (
			<ToolInput
				input={{
					language: 'python',
					code: "print('Hello, World!')",
				}}
			/>
		),
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify running state and info border
		const toolContainer = canvasElement.querySelector('[class*="border-info"]');
		await expect(toolContainer).toBeInTheDocument();
		await expect(canvas.getByText('tool-code_execution')).toBeInTheDocument();
		await expect(canvas.getByText('Running')).toBeInTheDocument();
	},
};

// Tool with error state
export const ErrorTool: Story = {
	args: {
		type: 'tool-file_read',
		state: 'output-error',
		children: (
			<>
				<ToolInput
					input={{
						path: '/nonexistent/file.txt',
					}}
				/>
				<ToolOutput output={undefined} errorText="File not found: /nonexistent/file.txt" />
			</>
		),
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify error state and error border
		const toolContainer = canvasElement.querySelector('[class*="border-error"]');
		await expect(toolContainer).toBeInTheDocument();
		await expect(canvas.getByText('tool-file_read')).toBeInTheDocument();
		await expect(canvas.getAllByText('Error').length).toBeGreaterThan(0);
	},
};

// Tool with pending state
export const PendingTool: Story = {
	args: {
		type: 'tool-web_search',
		state: 'input-streaming',
		children: <div className="p-4 text-muted-foreground">Preparing to search the web...</div>,
	},
};

// Multiple tools stacked
export const MultipleTools: Story = {
	render: () => (
		<div className="space-y-4">
			<Tool type="tool-search" state="output-available">
				<ToolInput input={{ query: 'React hooks' }} />
				<ToolOutput output="Found comprehensive guide on React hooks" errorText={undefined} />
			</Tool>
			<Tool type="tool-code_generation" state="input-available">
				<ToolInput input={{ prompt: 'Create a useCounter hook' }} />
			</Tool>
			<Tool type="tool-file_write" state="output-error">
				<ToolInput input={{ filename: 'counter.ts', content: '...' }} />
				<ToolOutput output={undefined} errorText="Permission denied" />
			</Tool>
		</div>
	),
};

// Minimal tool
export const MinimalTool: Story = {
	args: {
		type: 'tool-minimal',
		state: 'input-streaming',
		children: <div className="p-4">Minimal tool content</div>,
	},
};

// Tool with complex content
export const ComplexContent: Story = {
	args: {
		type: 'tool-data_analysis',
		state: 'output-available',
		children: (
			<>
				<ToolInput
					input={{
						dataset: 'sales_data.csv',
						analysis_type: 'correlation',
						columns: ['revenue', 'marketing_spend', 'customer_count'],
						filters: {
							date_range: '2023-01-01 to 2023-12-31',
							region: 'North America',
						},
					}}
				/>
				<ToolOutput
					output={
						<div className="space-y-2">
							<div className="font-medium">Analysis Results:</div>
							<ul className="list-disc list-inside text-sm space-y-1">
								<li>Strong positive correlation (0.82) between marketing spend and revenue</li>
								<li>Moderate correlation (0.65) between customer count and revenue</li>
								<li>Weak correlation (0.23) between marketing spend and customer count</li>
							</ul>
							<div className="text-xs text-muted-foreground mt-2">Analysis completed in 2.3 seconds</div>
						</div>
					}
					errorText={undefined}
				/>
			</>
		),
	},
};
