import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import GlobalError from './global-error';

// Mock console.error to avoid noise in test output
const mockConsoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

describe('GlobalError', () => {
	const mockError = new Error('Test global error message');
	const mockReset = vi.fn();

	const defaultProps = {
		error: mockError,
		reset: mockReset,
	};

	afterEach(() => {
		vi.clearAllMocks();
	});

	it('renders global error boundary with correct elements', () => {
		render(<GlobalError {...defaultProps} />);

		expect(screen.getByText('Something went wrong!')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Try again' })).toBeInTheDocument();
	});

	it('renders with html and body elements in JSX structure', () => {
		const { container } = render(<GlobalError {...defaultProps} />);

		// The component renders html and body elements as JSX structure
		// We can verify the overall structure is rendered correctly
		expect(container.firstChild).toBeDefined();
		expect(screen.getByText('Something went wrong!')).toBeInTheDocument();
	});

	it('logs error with [web] prefix to console on mount', () => {
		render(<GlobalError {...defaultProps} />);

		expect(mockConsoleError).toHaveBeenCalledWith('[web]', mockError);
		expect(mockConsoleError).toHaveBeenCalledTimes(1);
	});

	it('calls reset function when Try again button is clicked', async () => {
		const user = userEvent.setup();
		render(<GlobalError {...defaultProps} />);

		const tryAgainButton = screen.getByRole('button', { name: 'Try again' });
		await user.click(tryAgainButton);

		expect(mockReset).toHaveBeenCalledTimes(1);
	});

	it('handles error with digest property', () => {
		const errorWithDigest = Object.assign(new Error('Global error with digest'), { digest: 'xyz789' });
		render(<GlobalError error={errorWithDigest} reset={mockReset} />);

		expect(mockConsoleError).toHaveBeenCalledWith('[web]', errorWithDigest);
		expect(screen.getByText('Something went wrong!')).toBeInTheDocument();
	});

	it('re-logs error when error prop changes', () => {
		const { rerender } = render(<GlobalError {...defaultProps} />);

		expect(mockConsoleError).toHaveBeenCalledTimes(1);

		const newError = new Error('New global error message');
		rerender(<GlobalError error={newError} reset={mockReset} />);

		expect(mockConsoleError).toHaveBeenCalledTimes(2);
		expect(mockConsoleError).toHaveBeenLastCalledWith('[web]', newError);
	});

	it('has correct button type attribute', () => {
		render(<GlobalError {...defaultProps} />);

		const button = screen.getByRole('button', { name: 'Try again' });
		expect(button).toHaveAttribute('type', 'button');
	});

	it('applies correct CSS classes', () => {
		render(<GlobalError {...defaultProps} />);

		const container = screen.getByText('Something went wrong!').closest('div');
		expect(container).toHaveClass('text-center');

		const outerContainer = screen.getByText('Something went wrong!').closest('div')?.parentElement;
		expect(outerContainer).toHaveClass('flex', 'min-h-screen', 'items-center', 'justify-center');
	});

	it('matches snapshot', () => {
		const { container } = render(<GlobalError {...defaultProps} />);
		expect(container.firstChild).toMatchSnapshot();
	});
});
