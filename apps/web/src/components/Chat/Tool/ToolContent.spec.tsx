import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import ToolContent from './ToolContent';

describe('ToolContent', () => {
	it('renders with default isOpen (true)', () => {
		render(
			<ToolContent>
				<div data-testid="content">Tool Content</div>
			</ToolContent>,
		);

		expect(screen.getByTestId('content')).toBeInTheDocument();
	});

	it('renders when isOpen is explicitly true', () => {
		render(
			<ToolContent isOpen={true}>
				<div data-testid="content">Tool Content</div>
			</ToolContent>,
		);

		expect(screen.getByTestId('content')).toBeInTheDocument();
	});

	it('does not render when isOpen is false', () => {
		render(
			<ToolContent isOpen={false}>
				<div data-testid="content">Tool Content</div>
			</ToolContent>,
		);

		expect(screen.queryByTestId('content')).not.toBeInTheDocument();
	});

	it('renders children correctly', () => {
		render(
			<ToolContent>
				<div data-testid="child-1">Child 1</div>
				<div data-testid="child-2">Child 2</div>
			</ToolContent>,
		);

		expect(screen.getByTestId('child-1')).toBeInTheDocument();
		expect(screen.getByTestId('child-2')).toBeInTheDocument();
	});

	it('applies default className', () => {
		const { container } = render(
			<ToolContent>
				<div>Content</div>
			</ToolContent>,
		);

		const contentDiv = container.querySelector('.text-popover-foreground');
		expect(contentDiv).toBeInTheDocument();
		expect(contentDiv).toHaveClass('outline-none');
	});

	it('applies custom className', () => {
		const { container } = render(
			<ToolContent className="custom-content">
				<div>Content</div>
			</ToolContent>,
		);

		const contentDiv = container.querySelector('.custom-content');
		expect(contentDiv).toBeInTheDocument();
		expect(contentDiv).toHaveClass('text-popover-foreground', 'outline-none');
	});

	it('passes through additional props', () => {
		render(
			<ToolContent data-testid="tool-content" role="region">
				<div>Content</div>
			</ToolContent>,
		);

		const content = screen.getByTestId('tool-content');
		expect(content).toBeInTheDocument();
		expect(content).toHaveAttribute('role', 'region');
	});

	it('returns null when isOpen is false', () => {
		const { container } = render(
			<ToolContent isOpen={false}>
				<div>Content</div>
			</ToolContent>,
		);

		expect(container.firstChild).toBeNull();
	});

	it('handles complex children', () => {
		render(
			<ToolContent>
				<div className="header">Header</div>
				<div className="body">
					<p>Paragraph 1</p>
					<p>Paragraph 2</p>
				</div>
			</ToolContent>,
		);

		expect(screen.getByText('Header')).toBeInTheDocument();
		expect(screen.getByText('Paragraph 1')).toBeInTheDocument();
		expect(screen.getByText('Paragraph 2')).toBeInTheDocument();
	});

	it('matches snapshot when open', () => {
		const { container } = render(
			<ToolContent>
				<div>Content</div>
			</ToolContent>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot when closed', () => {
		const { container } = render(
			<ToolContent isOpen={false}>
				<div>Content</div>
			</ToolContent>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with custom className', () => {
		const { container } = render(
			<ToolContent className="custom-class">
				<div>Content</div>
			</ToolContent>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});
});
