import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, within } from 'storybook/test';
import { CodeBlock } from './CodeBlock';
import ToolOutput from './ToolOutput';

const meta: Meta<typeof ToolOutput> = {
	title: 'Components/Chat/Tool/ToolOutput',
	component: ToolOutput,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component:
					'Displays tool output or error messages. Shows "Result" for successful outputs and "Error" for error states with appropriate styling.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		output: {
			control: 'text',
			description: 'The output content from the tool execution (can be ReactNode)',
		},
		errorText: {
			control: 'text',
			description: 'Error text if the tool execution failed',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Successful text output
export const SuccessfulTextOutput: Story = {
	args: {
		output: 'Operation completed successfully. Found 15 matching results.',
		errorText: undefined,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify result header
		await expect(canvas.getByText('Result')).toBeInTheDocument();

		// Verify output content
		await expect(canvas.getByText('Operation completed successfully. Found 15 matching results.')).toBeInTheDocument();

		// Verify success styling (not error styling)
		// Find the container with the appropriate success styling
		const containers = canvasElement.querySelectorAll('div');
		const successContainer = Array.from(containers).find(
			(div) => div.classList.contains('bg-muted/50') && div.classList.contains('text-foreground'),
		);
		await expect(successContainer).toBeDefined();

		// Verify it does not have error styling
		const errorContainer = Array.from(containers).find((div) => div.classList.contains('bg-destructive/10'));
		await expect(errorContainer).toBeUndefined();
	},
};

// Error output
export const ErrorOutput: Story = {
	args: {
		output: undefined,
		errorText: 'File not found: /path/to/nonexistent/file.txt',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify error header
		await expect(canvas.getByText('Error')).toBeInTheDocument();

		// Verify error message
		await expect(canvas.getByText('File not found: /path/to/nonexistent/file.txt')).toBeInTheDocument();

		// Verify error styling
		const containers = canvasElement.querySelectorAll('div');
		const errorContainer = Array.from(containers).find(
			(div) => div.classList.contains('bg-destructive/10') && div.classList.contains('text-destructive'),
		);
		await expect(errorContainer).toBeDefined();
	},
};

// Both output and error (error takes precedence)
export const ErrorWithOutput: Story = {
	args: {
		output: 'Some partial results were generated',
		errorText: 'Process failed with exit code 1',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify error header (error takes precedence)
		await expect(canvas.getByText('Error')).toBeInTheDocument();

		// Verify error message is shown
		await expect(canvas.getByText('Process failed with exit code 1')).toBeInTheDocument();

		// Verify output is also shown
		await expect(canvas.getByText('Some partial results were generated')).toBeInTheDocument();

		// Verify error styling is applied (use class list check for Tailwind classes with /)
		const containers = canvasElement.querySelectorAll('div');
		const errorContainer = Array.from(containers).find((div) => div.classList.contains('bg-destructive/10'));
		await expect(errorContainer).toBeDefined();
	},
};

// Empty/null output (renders nothing)
export const EmptyOutput: Story = {
	args: {
		output: undefined,
		errorText: undefined,
	},
	play: async ({ canvasElement }) => {
		// Verify component renders nothing when no output or error
		// The component returns null, so Storybook wrapper should be empty
		const textContent = canvasElement.textContent || '';
		await expect(textContent.trim()).toBe('');
	},
};

// Multiline text output
export const MultilineOutput: Story = {
	args: {
		output: `Search Results:
1. React Hooks Documentation - https://react.dev/hooks
2. useState Hook Guide - Comprehensive tutorial
3. useEffect Hook Patterns - Best practices
4. Custom Hooks Examples - Real-world implementations
5. Hook Testing Strategies - Unit testing approaches`,
		errorText: undefined,
	},
};

// JSON output
export const JSONOutput: Story = {
	args: {
		output: JSON.stringify(
			{
				status: 'success',
				results: [
					{ id: 1, title: 'React Hooks', score: 0.95 },
					{ id: 2, title: 'Vue Composition API', score: 0.87 },
					{ id: 3, title: 'Angular Signals', score: 0.82 },
				],
				metadata: {
					totalResults: 3,
					processingTime: '124ms',
					timestamp: '2024-01-15T10:30:00Z',
				},
			},
			null,
			2,
		),
		errorText: undefined,
	},
};

// HTML/JSX content output
export const HTMLOutput: Story = {
	args: {
		output: (
			<div>
				<h4 className="font-bold mb-2">Analysis Results</h4>
				<ul className="list-disc list-inside space-y-1">
					<li>
						Performance: <span className="text-green-600 font-medium">Excellent</span>
					</li>
					<li>
						Accessibility: <span className="text-yellow-600 font-medium">Good</span>
					</li>
					<li>
						SEO: <span className="text-red-600 font-medium">Needs Improvement</span>
					</li>
				</ul>
				<div className="mt-3 p-2 bg-blue-50 rounded text-sm">
					<strong>Recommendation:</strong> Focus on meta tags and structured data
				</div>
			</div>
		),
		errorText: undefined,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify JSX content renders correctly
		await expect(canvas.getByText('Analysis Results')).toBeInTheDocument();
		await expect(canvas.getByText('Excellent')).toBeInTheDocument();
		await expect(canvas.getByText('Good')).toBeInTheDocument();
		await expect(canvas.getByText('Needs Improvement')).toBeInTheDocument();
		await expect(canvas.getByText('Focus on meta tags and structured data')).toBeInTheDocument();

		// Verify styling is preserved
		const excellentSpan = canvas.getByText('Excellent');
		await expect(excellentSpan).toHaveClass('text-green-600', 'font-medium');
	},
};

// Code output with syntax highlighting
export const CodeOutput: Story = {
	args: {
		output: (
			<CodeBlock
				code={`function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

// Test the function
console.log('First 10 Fibonacci numbers:');
for (let i = 0; i < 10; i++) {
  console.log(\`F(\${i}) = \${fibonacci(i)}\`);
}`}
				language="javascript"
				showLineNumbers={true}
			/>
		),
		errorText: undefined,
	},
};

// Table output
export const TableOutput: Story = {
	args: {
		output: (
			<div className="overflow-x-auto">
				<table className="w-full border-collapse border border-gray-200 text-sm">
					<thead>
						<tr className="bg-gray-50">
							<th className="border border-gray-200 px-3 py-2 text-left">Framework</th>
							<th className="border border-gray-200 px-3 py-2 text-left">Stars</th>
							<th className="border border-gray-200 px-3 py-2 text-left">Size</th>
							<th className="border border-gray-200 px-3 py-2 text-left">Learning Curve</th>
						</tr>
					</thead>
					<tbody>
						<tr>
							<td className="border border-gray-200 px-3 py-2">React</td>
							<td className="border border-gray-200 px-3 py-2">200k+</td>
							<td className="border border-gray-200 px-3 py-2">42kb</td>
							<td className="border border-gray-200 px-3 py-2">Moderate</td>
						</tr>
						<tr>
							<td className="border border-gray-200 px-3 py-2">Vue</td>
							<td className="border border-gray-200 px-3 py-2">180k+</td>
							<td className="border border-gray-200 px-3 py-2">34kb</td>
							<td className="border border-gray-200 px-3 py-2">Easy</td>
						</tr>
						<tr>
							<td className="border border-gray-200 px-3 py-2">Svelte</td>
							<td className="border border-gray-200 px-3 py-2">65k+</td>
							<td className="border border-gray-200 px-3 py-2">10kb</td>
							<td className="border border-gray-200 px-3 py-2">Easy</td>
						</tr>
					</tbody>
				</table>
			</div>
		),
		errorText: undefined,
	},
};

// File system error
export const FileSystemError: Story = {
	args: {
		output: undefined,
		errorText: 'Permission denied: Cannot write to /system/protected/file.txt',
	},
};

// Network error
export const NetworkError: Story = {
	args: {
		output: undefined,
		errorText: 'Network timeout: Unable to reach https://api.example.com after 30 seconds',
	},
};

// Syntax error
export const SyntaxErrorExample: Story = {
	args: {
		output: undefined,
		errorText: "SyntaxError: Unexpected token '}' at line 23, column 5",
	},
};

// Long error message
export const LongErrorMessage: Story = {
	args: {
		output: undefined,
		errorText:
			'Database connection failed: Connection to database server at localhost:5432 was refused. Check that the hostname and port are correct and that the postmaster is accepting TCP/IP connections. This error often occurs when the database service is not running or when firewall rules are blocking the connection.',
	},
};

// Success with metadata
export const SuccessWithMetadata: Story = {
	args: {
		output: (
			<div className="space-y-2">
				<div className="font-medium">File processed successfully</div>
				<div className="text-sm text-muted-foreground grid grid-cols-2 gap-2">
					<div>Size: 2.4 MB</div>
					<div>Format: PDF</div>
					<div>Pages: 127</div>
					<div>Processing time: 3.2s</div>
				</div>
				<div className="text-xs text-muted-foreground">Extracted 15,847 words and 234 images</div>
			</div>
		),
		errorText: undefined,
	},
};

// Progress or partial results
export const PartialResults: Story = {
	args: {
		output: (
			<div className="space-y-2">
				<div>Processing... 75% complete</div>
				<div className="text-sm text-muted-foreground">
					<div>✓ Analyzed 1,250 files</div>
					<div>✓ Generated 43 reports</div>
					<div>⏳ Processing large datasets...</div>
				</div>
			</div>
		),
		errorText: undefined,
	},
};

// Custom styling
export const CustomStyling: Story = {
	args: {
		output: 'Operation completed with custom styling',
		errorText: undefined,
		className: 'border-2 border-green-200 bg-green-50',
	},
};

// Multiple outputs comparison
export const MultipleOutputsComparison: Story = {
	render: () => (
		<div className="space-y-4">
			<ToolOutput output="Successful operation" errorText={undefined} />
			<ToolOutput output={undefined} errorText="Failed operation" />
			<ToolOutput output="Warning: Some issues detected" errorText={undefined} />
		</div>
	),
};

// Empty states showcase
export const EmptyStates: Story = {
	render: () => (
		<div className="space-y-4">
			<div className="text-sm font-medium">These should not render anything:</div>
			<ToolOutput output={undefined} errorText={undefined} />
			<ToolOutput output="" errorText="" />
			<ToolOutput output={null} errorText={undefined} />
			<div className="text-sm text-muted-foreground">(Empty outputs are not displayed)</div>
		</div>
	),
};

// Rich content with images (simulated)
export const RichContent: Story = {
	args: {
		output: (
			<div className="space-y-3">
				<div className="font-medium">Image Analysis Results</div>
				<div className="grid grid-cols-2 gap-2 text-sm">
					<div>Objects detected: 5</div>
					<div>Confidence: 94.2%</div>
					<div>Resolution: 1920x1080</div>
					<div>File size: 856 KB</div>
				</div>
				<div className="space-y-1">
					<div className="text-sm font-medium">Detected objects:</div>
					<ul className="text-sm space-y-1">
						<li>🐶 Dog (97.3% confidence)</li>
						<li>🌳 Tree (89.1% confidence)</li>
						<li>🏠 House (85.4% confidence)</li>
						<li>☁ Sky (92.7% confidence)</li>
						<li>🌿 Grass (78.9% confidence)</li>
					</ul>
				</div>
			</div>
		),
		errorText: undefined,
	},
};
