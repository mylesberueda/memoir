import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import type { ToolUIPart } from 'ai';
import { describe, expect, it } from 'vitest';
import ToolHeader from './ToolHeader';

describe('ToolHeader', () => {
	const defaultProps = {
		type: 'Test Tool',
		state: 'input-streaming' as ToolUIPart['state'],
	};

	it('renders with required props', () => {
		render(<ToolHeader {...defaultProps} />);

		expect(screen.getByText('Test Tool')).toBeInTheDocument();
	});

	it('displays tool type correctly', () => {
		render(<ToolHeader type="Custom Tool" state="input-available" />);

		expect(screen.getByText('Custom Tool')).toBeInTheDocument();
	});

	it('renders wrench icon', () => {
		const { container } = render(<ToolHeader {...defaultProps} />);

		const icon = container.querySelector('svg');
		expect(icon).toBeInTheDocument();
		expect(icon).toHaveClass('size-4', 'text-muted-foreground');
	});

	it('renders status badge for input-streaming state', () => {
		render(<ToolHeader {...defaultProps} state="input-streaming" />);

		expect(screen.getByText('Pending')).toBeInTheDocument();
	});

	it('renders status badge for input-available state', () => {
		render(<ToolHeader {...defaultProps} state="input-available" />);

		expect(screen.getByText('Running')).toBeInTheDocument();
	});

	it('renders status badge for output-available state', () => {
		render(<ToolHeader {...defaultProps} state="output-available" />);

		expect(screen.getByText('Completed')).toBeInTheDocument();
	});

	it('renders status badge for output-error state', () => {
		render(<ToolHeader {...defaultProps} state="output-error" />);

		expect(screen.getByText('Error')).toBeInTheDocument();
	});

	it('applies default className', () => {
		const { container } = render(<ToolHeader {...defaultProps} />);

		const header = container.querySelector('.flex');
		expect(header).toBeInTheDocument();
		expect(header).toHaveClass('w-full', 'items-center', 'justify-between', 'gap-4', 'p-3', 'cursor-pointer');
	});

	it('applies custom className', () => {
		const { container } = render(<ToolHeader {...defaultProps} className="custom-header" />);

		const header = container.querySelector('.custom-header');
		expect(header).toBeInTheDocument();
		expect(header).toHaveClass('flex', 'w-full', 'items-center', 'justify-between');
	});

	it('passes through additional props', () => {
		render(<ToolHeader {...defaultProps} data-testid="tool-header" role="heading" />);

		const header = screen.getByTestId('tool-header');
		expect(header).toBeInTheDocument();
		expect(header).toHaveAttribute('role', 'heading');
	});

	it('renders icon and type in correct container', () => {
		const { container } = render(<ToolHeader {...defaultProps} />);

		const iconTypeContainer = container.querySelector('.flex.items-center.gap-2');
		expect(iconTypeContainer).toBeInTheDocument();

		const icon = iconTypeContainer?.querySelector('svg');
		expect(icon).toBeInTheDocument();

		const typeText = iconTypeContainer?.querySelector('span');
		expect(typeText).toHaveTextContent('Test Tool');
	});

	it('applies correct styling to type text', () => {
		const { container } = render(<ToolHeader {...defaultProps} />);

		const typeSpan = container.querySelector('span.font-medium.text-sm');
		expect(typeSpan).toBeInTheDocument();
		expect(typeSpan).toHaveTextContent('Test Tool');
	});

	it('renders all required elements', () => {
		const { container } = render(<ToolHeader {...defaultProps} />);

		// Main container
		expect(container.querySelector('.flex.w-full')).toBeInTheDocument();

		// Icon container
		expect(container.querySelector('.flex.items-center.gap-2')).toBeInTheDocument();

		// Icon
		expect(container.querySelector('svg')).toBeInTheDocument();

		// Type text
		expect(screen.getByText('Test Tool')).toBeInTheDocument();

		// Status badge
		expect(screen.getByText('Pending')).toBeInTheDocument();
	});

	it('matches snapshot with input-streaming state', () => {
		const { container } = render(<ToolHeader {...defaultProps} state="input-streaming" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with input-available state', () => {
		const { container } = render(<ToolHeader {...defaultProps} state="input-available" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with output-available state', () => {
		const { container } = render(<ToolHeader {...defaultProps} state="output-available" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with output-error state', () => {
		const { container } = render(<ToolHeader {...defaultProps} state="output-error" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with custom className', () => {
		const { container } = render(<ToolHeader {...defaultProps} className="custom-class" />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('handles different tool display names', () => {
		const toolTypes = ['Web Search', 'Database Query', 'Create Agent', 'Current Time'];

		for (const type of toolTypes) {
			render(<ToolHeader type={type} state="input-available" />);
			expect(screen.getByText(type)).toBeInTheDocument();
		}
	});
});
