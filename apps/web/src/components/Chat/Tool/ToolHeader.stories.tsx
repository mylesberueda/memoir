import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, within } from 'storybook/test';
import ToolHeader from './ToolHeader';

const meta: Meta<typeof ToolHeader> = {
	title: 'Components/Chat/Tool/ToolHeader',
	component: ToolHeader,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component:
					'Header component for tools showing the tool type, execution state, and status badge. Includes a wrench icon and chevron for expandable UI.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		type: {
			control: 'text',
			description: 'The type/name of the tool being displayed',
		},
		state: {
			control: 'select',
			options: ['input-streaming', 'input-available', 'output-available', 'output-error'],
			description: 'Current execution state of the tool',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Default state - input streaming (pending)
export const InputStreaming: Story = {
	args: {
		type: 'tool-search',
		state: 'input-streaming',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify tool type and status
		await expect(canvas.getByText('tool-search')).toBeInTheDocument();
		await expect(canvas.getByText('Pending')).toBeInTheDocument();

		// Verify wrench icon and chevron are present
		const wrenches = canvasElement.querySelectorAll('svg');
		await expect(wrenches.length).toBeGreaterThan(0);
	},
};

// Input available state (running)
export const InputAvailable: Story = {
	args: {
		type: 'tool-code_execution',
		state: 'input-available',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify running state with animation
		await expect(canvas.getByText('tool-code_execution')).toBeInTheDocument();
		await expect(canvas.getByText('Running')).toBeInTheDocument();

		// Verify animated clock icon (should have animate-pulse class)
		const runningBadge = canvas.getByText('Running').closest('.badge');
		const clockIcon = runningBadge?.querySelector('svg.animate-pulse');
		await expect(clockIcon).toBeInTheDocument();
	},
};

// Output available state (completed)
export const OutputAvailable: Story = {
	args: {
		type: 'tool-file_read',
		state: 'output-available',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify completed state
		await expect(canvas.getByText('tool-file_read')).toBeInTheDocument();
		await expect(canvas.getByText('Completed')).toBeInTheDocument();

		// Verify green check icon
		const completedBadge = canvas.getByText('Completed').closest('.badge');
		const checkIcon = completedBadge?.querySelector('svg.text-green-600');
		await expect(checkIcon).toBeInTheDocument();
	},
};

// Error state
export const OutputError: Story = {
	args: {
		type: 'tool-web_search',
		state: 'output-error',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify error state
		await expect(canvas.getByText('tool-web_search')).toBeInTheDocument();
		await expect(canvas.getByText('Error')).toBeInTheDocument();

		// Verify red error icon
		const errorBadge = canvas.getByText('Error').closest('.badge');
		const errorIcon = errorBadge?.querySelector('svg.text-red-600');
		await expect(errorIcon).toBeInTheDocument();
	},
};

// Different tool types
export const SearchTool: Story = {
	args: {
		type: 'tool-search',
		state: 'output-available',
	},
};

export const CodeExecutionTool: Story = {
	args: {
		type: 'tool-code_execution',
		state: 'input-available',
	},
};

export const FileReadTool: Story = {
	args: {
		type: 'tool-file_read',
		state: 'output-available',
	},
};

export const FileWriteTool: Story = {
	args: {
		type: 'tool-file_write',
		state: 'output-error',
	},
};

export const WebSearchTool: Story = {
	args: {
		type: 'tool-web_search',
		state: 'input-streaming',
	},
};

export const DataAnalysisTool: Story = {
	args: {
		type: 'tool-data_analysis',
		state: 'output-available',
	},
};

export const APICallTool: Story = {
	args: {
		type: 'tool-api_call',
		state: 'input-available',
	},
};

// Long tool name
export const LongToolName: Story = {
	args: {
		type: 'tool-advanced_statistical_analysis_with_machine_learning',
		state: 'output-available',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify long tool name is displayed
		await expect(canvas.getByText('tool-advanced_statistical_analysis_with_machine_learning')).toBeInTheDocument();
		await expect(canvas.getByText('Completed')).toBeInTheDocument();

		// Verify layout handles long names properly
		const headerContainer = canvasElement.querySelector('.flex.w-full.items-center.justify-between');
		await expect(headerContainer).toBeInTheDocument();
	},
};

// Custom styling
export const CustomStyling: Story = {
	args: {
		type: 'tool-custom_tool',
		state: 'output-available',
		className: 'bg-primary-content border-l-4 border-l-primary',
	},
};

// Interactive example (simulate click)
export const Interactive: Story = {
	args: {
		type: 'tool-interactive_tool',
		state: 'input-available',
		onClick: () => alert('Tool header clicked!'),
	},
};

// All states showcase
export const AllStates: Story = {
	render: () => (
		<div className="space-y-2">
			<ToolHeader type="tool-tool_1" state="input-streaming" />
			<ToolHeader type="tool-tool_2" state="input-available" />
			<ToolHeader type="tool-tool_3" state="output-available" />
			<ToolHeader type="tool-tool_4" state="output-error" />
		</div>
	),
};

// Different tool types comparison
export const ToolTypesComparison: Story = {
	render: () => (
		<div className="space-y-2">
			<ToolHeader type="tool-search" state="output-available" />
			<ToolHeader type="tool-code_execution" state="output-available" />
			<ToolHeader type="tool-file_read" state="output-available" />
			<ToolHeader type="tool-web_search" state="output-available" />
			<ToolHeader type="tool-data_analysis" state="output-available" />
		</div>
	),
};

// Error states showcase
export const ErrorStates: Story = {
	render: () => (
		<div className="space-y-2">
			<ToolHeader type="tool-file_not_found" state="output-error" />
			<ToolHeader type="tool-permission_denied" state="output-error" />
			<ToolHeader type="tool-network_timeout" state="output-error" />
			<ToolHeader type="tool-syntax_error" state="output-error" />
		</div>
	),
};

// Progress states showcase
export const ProgressStates: Story = {
	render: () => (
		<div className="space-y-2">
			<div className="text-sm font-medium mb-2">Tool Execution Flow:</div>
			<ToolHeader type="tool-data_processing" state="input-streaming" />
			<div className="text-xs text-muted-foreground ml-8">↓ Receiving parameters...</div>
			<ToolHeader type="tool-data_processing" state="input-available" />
			<div className="text-xs text-muted-foreground ml-8">↓ Processing data...</div>
			<ToolHeader type="tool-data_processing" state="output-available" />
		</div>
	),
};
