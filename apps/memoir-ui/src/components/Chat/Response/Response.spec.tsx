import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Response from './Response';

describe('Response', () => {
	it('renders text content correctly', () => {
		render(<Response data-testid="response">Hello world</Response>);

		const response = screen.getByTestId('response');
		expect(response).toBeInTheDocument();
		expect(screen.getByText('Hello world')).toBeInTheDocument();
	});

	it('applies custom className', () => {
		render(
			<Response className="custom-class" data-testid="response">
				Test content
			</Response>,
		);

		const response = screen.getByTestId('response');
		expect(response).toHaveClass('custom-class');
	});

	it('renders markdown-like content as children', () => {
		const markdownContent = `# Heading

This is **bold** and *italic* text.

- List item 1
- List item 2

\`code example\``;

		render(<Response data-testid="response">{markdownContent}</Response>);

		const response = screen.getByTestId('response');
		expect(response).toBeInTheDocument();

		// Check that markdown is properly rendered as HTML elements
		expect(screen.getByRole('heading', { level: 1 })).toHaveTextContent('Heading');

		// Check for bold text (rendered as span with font-semibold class)
		const boldElement = response.querySelector('span.font-semibold');
		expect(boldElement).toBeInTheDocument();
		expect(boldElement).toHaveTextContent('bold');

		// Check for italic text in <em> tag
		const italicElement = response.querySelector('em');
		expect(italicElement).toBeInTheDocument();
		expect(italicElement).toHaveTextContent('italic');

		// Check for list structure
		const list = response.querySelector('ul');
		expect(list).toBeInTheDocument();

		const listItems = response.querySelectorAll('li');
		expect(listItems).toHaveLength(2);
		expect(listItems[0]).toHaveTextContent('List item 1');
		expect(listItems[1]).toHaveTextContent('List item 2');

		// Check for code element
		const codeElement = response.querySelector('code');
		expect(codeElement).toBeInTheDocument();
		expect(codeElement).toHaveTextContent('code example');
	});

	it('handles empty content', () => {
		render(<Response data-testid="response" />);

		const response = screen.getByTestId('response');
		expect(response).toBeInTheDocument();
		expect(response).toBeEmptyDOMElement();
	});

	it('renders complex markdown content', () => {
		const complexMarkdown = `# Title

Paragraph text

- Item 1
- Item 2

## Subheading

More content with **bold** text.`;

		render(<Response data-testid="response">{complexMarkdown}</Response>);

		const response = screen.getByTestId('response');
		expect(response).toBeInTheDocument();
		expect(screen.getByRole('heading', { level: 1 })).toHaveTextContent('Title');
		expect(screen.getByRole('heading', { level: 2 })).toHaveTextContent('Subheading');
		expect(screen.getByText('Paragraph text')).toBeInTheDocument();
		expect(screen.getByText('Item 1')).toBeInTheDocument();
		expect(screen.getByText('Item 2')).toBeInTheDocument();
		expect(screen.getByText(/More content with.*text\./)).toBeInTheDocument();
		expect(response.querySelector('span.font-semibold')).toHaveTextContent('bold');
	});

	it('preserves component structure with default classes', () => {
		render(<Response data-testid="response">Test content</Response>);

		const response = screen.getByTestId('response');
		// The component should have the default classes from the cn() call
		expect(response).toHaveClass('size-full');
	});

	it('handles string content properly', () => {
		const textContent = 'This is a simple string response';

		render(<Response data-testid="response">{textContent}</Response>);

		const response = screen.getByTestId('response');
		expect(response).toBeInTheDocument();
		expect(response).toHaveTextContent(textContent);
	});

	it('handles multiline content', () => {
		const multilineContent = `Line 1
Line 2
Line 3`;

		render(<Response data-testid="response">{multilineContent}</Response>);

		const response = screen.getByTestId('response');
		expect(response).toBeInTheDocument();
		// Markdown processing normalizes whitespace, so newlines become spaces
		expect(response).toHaveTextContent('Line 1 Line 2 Line 3');
	});

	it('memoizes correctly with same content', () => {
		const content = 'Memoized content';
		const { rerender } = render(<Response>{content}</Response>);

		// Re-render with same content - should use memoized version
		rerender(<Response>{content}</Response>);

		expect(screen.getByText(content)).toBeInTheDocument();
	});
});
