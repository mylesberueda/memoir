import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ToolUIPart } from 'ai';
import { describe, expect, it } from 'vitest';
import Tool from './Tool';

describe('Tool', () => {
	const defaultProps = {
		type: 'Test Tool',
		state: 'input-streaming' as ToolUIPart['state'],
	};

	it('renders with required props', () => {
		render(<Tool {...defaultProps} />);

		expect(screen.getByText('Test Tool')).toBeInTheDocument();
		expect(screen.getByRole('button')).toBeInTheDocument();
	});

	it('renders tool icon', () => {
		const { container } = render(<Tool {...defaultProps} />);

		const icon = container.querySelector('svg');
		expect(icon).toBeInTheDocument();
		expect(icon).toHaveClass('size-4', 'text-muted-foreground');
	});

	it('displays tool type correctly', () => {
		render(<Tool type="Custom Tool" state="input-available" />);

		expect(screen.getByText('Custom Tool')).toBeInTheDocument();
	});

	it('renders status badge for input-streaming state', () => {
		render(<Tool {...defaultProps} state="input-streaming" />);

		expect(screen.getByText('Pending')).toBeInTheDocument();
	});

	it('renders status badge for input-available state', () => {
		render(<Tool {...defaultProps} state="input-available" />);

		expect(screen.getByText('Running')).toBeInTheDocument();
	});

	it('renders status badge for output-available state', () => {
		render(<Tool {...defaultProps} state="output-available" />);

		expect(screen.getByText('Completed')).toBeInTheDocument();
	});

	it('renders status badge for output-error state', () => {
		render(<Tool {...defaultProps} state="output-error" />);

		expect(screen.getByText('Error')).toBeInTheDocument();
	});

	it('applies correct border color for input-streaming state', () => {
		const { container } = render(<Tool {...defaultProps} state="input-streaming" />);

		const toolContainer = container.querySelector('.border-info');
		expect(toolContainer).toBeInTheDocument();
	});

	it('applies correct border color for input-available state', () => {
		const { container } = render(<Tool {...defaultProps} state="input-available" />);

		const toolContainer = container.querySelector('.border-info');
		expect(toolContainer).toBeInTheDocument();
	});

	it('applies correct border color for output-available state', () => {
		const { container } = render(<Tool {...defaultProps} state="output-available" />);

		const toolContainer = container.querySelector('.border-success');
		expect(toolContainer).toBeInTheDocument();
	});

	it('applies correct border color for output-error state', () => {
		const { container } = render(<Tool {...defaultProps} state="output-error" />);

		const toolContainer = container.querySelector('.border-error');
		expect(toolContainer).toBeInTheDocument();
	});

	it('starts in closed state', () => {
		render(
			<Tool {...defaultProps}>
				<div data-testid="tool-content">Tool Content</div>
			</Tool>,
		);

		const button = screen.getByRole('button');
		expect(button).toHaveAttribute('aria-expanded', 'false');
		expect(screen.queryByTestId('tool-content')).not.toBeInTheDocument();
	});

	it('toggles open when button is clicked', async () => {
		const user = userEvent.setup();

		render(
			<Tool {...defaultProps}>
				<div data-testid="tool-content">Tool Content</div>
			</Tool>,
		);

		const button = screen.getByRole('button');

		// Initially closed
		expect(button).toHaveAttribute('aria-expanded', 'false');
		expect(screen.queryByTestId('tool-content')).not.toBeInTheDocument();

		// Click to open
		await user.click(button);

		expect(button).toHaveAttribute('aria-expanded', 'true');
		expect(screen.getByTestId('tool-content')).toBeInTheDocument();
	});

	it('toggles closed when button is clicked again', async () => {
		const user = userEvent.setup();

		render(
			<Tool {...defaultProps}>
				<div data-testid="tool-content">Tool Content</div>
			</Tool>,
		);

		const button = screen.getByRole('button');

		// Open
		await user.click(button);
		expect(screen.getByTestId('tool-content')).toBeInTheDocument();

		// Close
		await user.click(button);
		expect(screen.queryByTestId('tool-content')).not.toBeInTheDocument();
	});

	it('renders children when open', async () => {
		const user = userEvent.setup();

		render(
			<Tool {...defaultProps}>
				<div data-testid="tool-content">Custom Tool Content</div>
			</Tool>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(screen.getByText('Custom Tool Content')).toBeInTheDocument();
	});

	it('applies custom className', () => {
		const { container } = render(<Tool {...defaultProps} className="custom-tool" />);

		const toolContainer = container.querySelector('.custom-tool');
		expect(toolContainer).toBeInTheDocument();
		expect(toolContainer).toHaveClass('not-prose', 'w-full', 'rounded-md', 'border');
	});

	it('passes through additional props', () => {
		render(<Tool {...defaultProps} data-testid="tool-component" />);

		expect(screen.getByTestId('tool-component')).toBeInTheDocument();
	});

	it('has correct button styling', () => {
		render(<Tool {...defaultProps} />);

		const button = screen.getByRole('button');
		expect(button).toHaveClass('flex', 'w-full', 'items-center', 'justify-between', 'gap-4', 'p-3', 'cursor-pointer');
	});

	it('does not render children container when no children provided', () => {
		const { container } = render(<Tool {...defaultProps} />);

		const button = screen.getByRole('button');
		expect(button).toBeInTheDocument();

		// Should not have content div
		const contentDiv = container.querySelector('.text-popover-foreground');
		expect(contentDiv).not.toBeInTheDocument();
	});

	it('handles multiple children', async () => {
		const user = userEvent.setup();

		render(
			<Tool {...defaultProps}>
				<div data-testid="child-1">Child 1</div>
				<div data-testid="child-2">Child 2</div>
			</Tool>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(screen.getByTestId('child-1')).toBeInTheDocument();
		expect(screen.getByTestId('child-2')).toBeInTheDocument();
	});

	it('maintains state across re-renders', async () => {
		const user = userEvent.setup();
		const { rerender } = render(
			<Tool {...defaultProps}>
				<div data-testid="content">Content</div>
			</Tool>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(screen.getByTestId('content')).toBeInTheDocument();

		// Re-render with different state
		rerender(
			<Tool {...defaultProps} state="output-available">
				<div data-testid="content">Content</div>
			</Tool>,
		);

		// Should still be open
		expect(screen.getByTestId('content')).toBeInTheDocument();
		expect(button).toHaveAttribute('aria-expanded', 'true');
	});

	it('matches snapshot when closed', () => {
		const { container } = render(
			<Tool {...defaultProps}>
				<div>Content</div>
			</Tool>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot when open', async () => {
		const user = userEvent.setup();
		const { container } = render(
			<Tool {...defaultProps}>
				<div>Content</div>
			</Tool>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with different states', () => {
		const states: Array<ToolUIPart['state']> = [
			'input-streaming',
			'input-available',
			'output-available',
			'output-error',
		];

		for (const state of states) {
			const { container } = render(<Tool {...defaultProps} state={state} />);
			expect(container.firstChild).toMatchSnapshot();
		}
	});
});
