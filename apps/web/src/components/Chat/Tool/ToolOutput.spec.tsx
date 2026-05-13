import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import ToolOutput from './ToolOutput';

describe('ToolOutput', () => {
	it('renders with output content', () => {
		render(<ToolOutput output={<div>Test output</div>} errorText={undefined} />);

		expect(screen.getByText('Test output')).toBeInTheDocument();
		expect(screen.getByText('Result')).toBeInTheDocument();
	});

	it('renders with error text', () => {
		render(<ToolOutput output={null} errorText="Error occurred" />);

		expect(screen.getByText('Error occurred')).toBeInTheDocument();
		expect(screen.getByText('Error')).toBeInTheDocument();
	});

	it('returns null when no output or errorText', () => {
		const { container } = render(<ToolOutput output={null} errorText={undefined} />);

		expect(container.firstChild).toBeNull();
	});

	it('returns null when output is empty string', () => {
		const { container } = render(<ToolOutput output="" errorText={undefined} />);

		expect(container.firstChild).toBeNull();
	});

	it('displays "Result" header for successful output', () => {
		render(<ToolOutput output={<div>Success data</div>} errorText={undefined} />);

		const header = screen.getByText('Result');
		expect(header).toBeInTheDocument();
		expect(header).toHaveClass('font-medium', 'text-muted-foreground', 'text-xs', 'uppercase', 'tracking-wide');
	});

	it('displays "Error" header for error output', () => {
		render(<ToolOutput output={null} errorText="Something went wrong" />);

		const header = screen.getByText('Error');
		expect(header).toBeInTheDocument();
		expect(header).toHaveClass('font-medium', 'text-muted-foreground', 'text-xs', 'uppercase', 'tracking-wide');
	});

	it('applies error styling when errorText is present', () => {
		const { container } = render(<ToolOutput output={null} errorText="Error message" />);

		const outputContent = container.querySelector('.bg-destructive\\/10.text-destructive');
		expect(outputContent).toBeInTheDocument();
	});

	it('applies success styling when output is present', () => {
		const { container } = render(<ToolOutput output={<div>Output</div>} errorText={undefined} />);

		const outputContent = container.querySelector('.bg-muted\\/50.text-foreground');
		expect(outputContent).toBeInTheDocument();
	});

	it('prioritizes errorText over output when both are present', () => {
		render(<ToolOutput output={<div>Output</div>} errorText="Error message" />);

		expect(screen.getByText('Error')).toBeInTheDocument();
		expect(screen.getByText('Error message')).toBeInTheDocument();
		// Output should still be rendered
		expect(screen.getByText('Output')).toBeInTheDocument();
	});

	it('renders both errorText and output when both provided', () => {
		render(<ToolOutput output={<div>Output data</div>} errorText="Error occurred" />);

		expect(screen.getByText('Error occurred')).toBeInTheDocument();
		expect(screen.getByText('Output data')).toBeInTheDocument();
	});

	it('applies default className', () => {
		const { container } = render(<ToolOutput output={<div>Output</div>} errorText={undefined} />);

		const outputContainer = container.querySelector('.space-y-2');
		expect(outputContainer).toBeInTheDocument();
		expect(outputContainer).toHaveClass('p-4');
	});

	it('applies custom className', () => {
		const { container } = render(
			<ToolOutput output={<div>Output</div>} errorText={undefined} className="custom-output" />,
		);

		const outputContainer = container.querySelector('.custom-output');
		expect(outputContainer).toBeInTheDocument();
		expect(outputContainer).toHaveClass('space-y-2', 'p-4');
	});

	it('passes through additional props', () => {
		render(<ToolOutput output={<div>Output</div>} errorText={undefined} data-testid="tool-output" role="status" />);

		const output = screen.getByTestId('tool-output');
		expect(output).toBeInTheDocument();
		expect(output).toHaveAttribute('role', 'status');
	});

	it('handles string output', () => {
		render(<ToolOutput output="Plain text output" errorText={undefined} />);

		expect(screen.getByText('Plain text output')).toBeInTheDocument();
	});

	it('handles complex ReactNode output', () => {
		const complexOutput = (
			<div>
				<h3>Title</h3>
				<p>Paragraph 1</p>
				<p>Paragraph 2</p>
			</div>
		);

		render(<ToolOutput output={complexOutput} errorText={undefined} />);

		expect(screen.getByText('Title')).toBeInTheDocument();
		expect(screen.getByText('Paragraph 1')).toBeInTheDocument();
		expect(screen.getByText('Paragraph 2')).toBeInTheDocument();
	});

	it('handles table in output', () => {
		const tableOutput = (
			<table>
				<tbody>
					<tr>
						<td>Cell 1</td>
						<td>Cell 2</td>
					</tr>
				</tbody>
			</table>
		);

		render(<ToolOutput output={tableOutput} errorText={undefined} />);

		expect(screen.getByText('Cell 1')).toBeInTheDocument();
		expect(screen.getByText('Cell 2')).toBeInTheDocument();
	});

	it('applies correct content styling classes', () => {
		const { container } = render(<ToolOutput output={<div>Output</div>} errorText={undefined} />);

		const contentDiv = container.querySelector('.overflow-x-auto.rounded-md.text-xs');
		expect(contentDiv).toBeInTheDocument();
		expect(contentDiv).toHaveClass('[&_table]:w-full');
	});

	it('renders error text as plain text', () => {
		const { container } = render(<ToolOutput output={null} errorText="Error: File not found" />);

		const errorDiv = container.querySelector('.bg-destructive\\/10.text-destructive div');
		expect(errorDiv).toBeInTheDocument();
		expect(errorDiv).toHaveTextContent('Error: File not found');
	});

	it('handles numeric output', () => {
		render(<ToolOutput output={42} errorText={undefined} />);

		expect(screen.getByText('42')).toBeInTheDocument();
	});

	it('handles boolean output', () => {
		render(<ToolOutput output={<div>{String(true)}</div>} errorText={undefined} />);

		expect(screen.getByText('true')).toBeInTheDocument();
	});

	it('matches snapshot with output only', () => {
		const { container } = render(<ToolOutput output={<div>Test output</div>} errorText={undefined} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with error only', () => {
		const { container } = render(<ToolOutput output={null} errorText="Error message" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with both output and error', () => {
		const { container } = render(<ToolOutput output={<div>Output</div>} errorText="Error occurred" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot when null (no output or error)', () => {
		const { container } = render(<ToolOutput output={null} errorText={undefined} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with custom className', () => {
		const { container } = render(
			<ToolOutput output={<div>Output</div>} errorText={undefined} className="custom-class" />,
		);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with complex output', () => {
		const { container } = render(
			<ToolOutput
				output={
					<div>
						<table>
							<tbody>
								<tr>
									<td>Data</td>
								</tr>
							</tbody>
						</table>
					</div>
				}
				errorText={undefined}
			/>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});
});
