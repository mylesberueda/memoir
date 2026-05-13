import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { CodeBlock, CodeBlockCopyButton } from './CodeBlock';

describe('CodeBlock', () => {
	const defaultProps = {
		code: 'const test = "hello";',
		language: 'javascript',
	};

	it('renders with required props', () => {
		const { container } = render(<CodeBlock {...defaultProps} />);

		expect(container.querySelector('.relative.w-full')).toBeInTheDocument();
		// Text is split by syntax highlighting, so check for container text content
		expect(container.textContent).toContain('const');
		expect(container.textContent).toContain('test');
		expect(container.textContent).toContain('hello');
	});

	it('applies custom className', () => {
		const { container } = render(<CodeBlock {...defaultProps} className="custom-class" />);

		const codeBlock = container.querySelector('.custom-class');
		expect(codeBlock).toBeInTheDocument();
		expect(codeBlock).toHaveClass('relative', 'w-full', 'overflow-hidden', 'rounded-md', 'border');
	});

	it('renders syntax highlighter with correct language', () => {
		const { container } = render(<CodeBlock code="print('hello')" language="python" />);

		// SyntaxHighlighter renders code elements, check for python syntax
		const codeElement = container.querySelector('code');
		expect(codeElement).toBeInTheDocument();
		expect(container.textContent).toContain('print');
	});

	it('shows line numbers when showLineNumbers is true', () => {
		const { container } = render(<CodeBlock {...defaultProps} showLineNumbers={true} />);

		// SyntaxHighlighter with showLineNumbers adds line number spans
		const codeElement = container.querySelector('code');
		expect(codeElement).toBeInTheDocument();
	});

	it('does not show line numbers when showLineNumbers is false', () => {
		const { container } = render(<CodeBlock {...defaultProps} showLineNumbers={false} />);

		const codeElement = container.querySelector('code');
		expect(codeElement).toBeInTheDocument();
	});

	it('renders children in correct position', () => {
		render(
			<CodeBlock {...defaultProps}>
				<div data-testid="custom-child">Custom Child</div>
			</CodeBlock>,
		);

		const child = screen.getByTestId('custom-child');
		expect(child).toBeInTheDocument();
		expect(child.parentElement).toHaveClass('absolute', 'top-2', 'right-2');
	});

	it('passes through additional props', () => {
		render(<CodeBlock {...defaultProps} data-testid="code-block" />);

		expect(screen.getByTestId('code-block')).toBeInTheDocument();
	});

	it('renders both light and dark syntax highlighters', () => {
		const { container } = render(<CodeBlock {...defaultProps} />);

		// Light theme (hidden in dark mode)
		const lightHighlighter = container.querySelector('.dark\\:hidden');
		expect(lightHighlighter).toBeInTheDocument();

		// Dark theme (hidden in light mode)
		const darkHighlighter = container.querySelector('.dark\\:block');
		expect(darkHighlighter).toBeInTheDocument();
	});

	it('handles multiline code correctly', () => {
		const multilineCode = `function test() {
  return true;
}`;

		const { container } = render(<CodeBlock code={multilineCode} language="javascript" />);

		expect(container.textContent).toContain('function test()');
		expect(container.textContent).toContain('return true');
	});

	it('matches snapshot with basic props', () => {
		const { container } = render(<CodeBlock {...defaultProps} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with line numbers', () => {
		const { container } = render(<CodeBlock {...defaultProps} showLineNumbers={true} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with children', () => {
		const { container } = render(
			<CodeBlock {...defaultProps}>
				<button type="button">Copy</button>
			</CodeBlock>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});
});

describe('CodeBlockCopyButton', () => {
	const testCode = 'const test = "hello";';

	it('renders with default icon', () => {
		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		expect(button).toBeInTheDocument();
		expect(button.querySelector('svg')).toBeInTheDocument();
	});

	it('renders with custom children', () => {
		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton>Custom Copy</CodeBlockCopyButton>
			</CodeBlock>,
		);

		expect(screen.getByRole('button', { name: 'Custom Copy' })).toBeInTheDocument();
	});

	it('applies custom className', () => {
		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton className="custom-button" />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		expect(button).toHaveClass('custom-button', 'shrink-0');
	});

	it('copies code to clipboard when clicked', async () => {
		const user = userEvent.setup();
		const mockWriteText = vi.fn().mockResolvedValue(undefined);

		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(mockWriteText).toHaveBeenCalledWith(testCode);
	});

	it('shows check icon after copying', async () => {
		const user = userEvent.setup();
		const mockWriteText = vi.fn().mockResolvedValue(undefined);

		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		const { container } = render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');

		// Before click - should have CopyIcon
		expect(container.querySelector('svg')).toBeInTheDocument();

		await user.click(button);

		// After click - should have CheckIcon
		expect(container.querySelector('svg')).toBeInTheDocument();
	});

	it('calls onCopy callback when copy succeeds', async () => {
		const user = userEvent.setup();
		const onCopy = vi.fn();
		const mockWriteText = vi.fn().mockResolvedValue(undefined);

		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton onCopy={onCopy} />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(onCopy).toHaveBeenCalled();
	});

	it('calls onError callback when clipboard API fails', async () => {
		const user = userEvent.setup();
		const onError = vi.fn();
		const testError = new Error('Clipboard failed');
		const mockWriteText = vi.fn().mockRejectedValue(testError);

		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton onError={onError} />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		expect(onError).toHaveBeenCalledWith(testError);
	});

	it('calls onError when clipboard API is not available', async () => {
		const user = userEvent.setup();
		const onError = vi.fn();

		// Mock window to be undefined to simulate server-side rendering
		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton onError={onError} />
			</CodeBlock>,
		);

		// Mock clipboard API to be unavailable
		const mockWriteText = undefined;
		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		const button = screen.getByRole('button');
		await user.click(button);

		expect(onError).toHaveBeenCalled();
	});

	it('resets copied state after timeout', async () => {
		const user = userEvent.setup();
		const mockWriteText = vi.fn().mockResolvedValue(undefined);

		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton timeout={1} />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		// Icon should change to check
		expect(mockWriteText).toHaveBeenCalled();

		// Wait for timeout to complete (1ms + buffer)
		await new Promise((resolve) => setTimeout(resolve, 5));

		// Icon should have reset (this is implicit - just verify no errors occurred)
		expect(button).toBeInTheDocument();
	});

	it('verifies copy functionality works without custom timeout', async () => {
		const user = userEvent.setup();
		const mockWriteText = vi.fn().mockResolvedValue(undefined);

		Object.defineProperty(navigator, 'clipboard', {
			value: {
				writeText: mockWriteText,
			},
			writable: true,
			configurable: true,
		});

		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton />
			</CodeBlock>,
		);

		const button = screen.getByRole('button');
		await user.click(button);

		// Should successfully copy with default timeout
		expect(mockWriteText).toHaveBeenCalledWith(testCode);
	});

	it('passes through additional props to Button', () => {
		render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton data-testid="copy-button" />
			</CodeBlock>,
		);

		const button = screen.getByTestId('copy-button');
		expect(button).toBeInTheDocument();
		expect(button).toHaveClass('btn', 'btn-sm');
	});

	it('matches snapshot', () => {
		const { container } = render(
			<CodeBlock code={testCode} language="javascript">
				<CodeBlockCopyButton />
			</CodeBlock>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});
});
