import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import ErrorBoundary from './error';

// Mock console.error to avoid noise in test output
const mockConsoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

describe('ErrorBoundary', () => {
	const mockError = new Error('Test error message');
	const mockReset = vi.fn();

	const defaultProps = {
		error: mockError,
		reset: mockReset,
	};

	afterEach(() => {
		vi.clearAllMocks();
	});

	it('renders error boundary with correct elements', () => {
		render(<ErrorBoundary {...defaultProps} />);

		expect(screen.getByText('Something went wrong!')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Try again' })).toBeInTheDocument();
	});

	it('logs error to console on mount', () => {
		render(<ErrorBoundary {...defaultProps} />);

		expect(mockConsoleError).toHaveBeenCalledWith(mockError);
		expect(mockConsoleError).toHaveBeenCalledTimes(1);
	});

	it('calls reset function when Try again button is clicked', async () => {
		const user = userEvent.setup();
		render(<ErrorBoundary {...defaultProps} />);

		const tryAgainButton = screen.getByRole('button', { name: 'Try again' });
		await user.click(tryAgainButton);

		expect(mockReset).toHaveBeenCalledTimes(1);
	});

	it('handles error with digest property', () => {
		const errorWithDigest = Object.assign(new Error('Error with digest'), { digest: 'abc123' });
		render(<ErrorBoundary error={errorWithDigest} reset={mockReset} />);

		expect(mockConsoleError).toHaveBeenCalledWith(errorWithDigest);
		expect(screen.getByText('Something went wrong!')).toBeInTheDocument();
	});

	it('re-logs error when error prop changes', () => {
		const { rerender } = render(<ErrorBoundary {...defaultProps} />);

		expect(mockConsoleError).toHaveBeenCalledTimes(1);

		const newError = new Error('New error message');
		rerender(<ErrorBoundary error={newError} reset={mockReset} />);

		expect(mockConsoleError).toHaveBeenCalledTimes(2);
		expect(mockConsoleError).toHaveBeenLastCalledWith(newError);
	});

	it('has correct button type attribute', () => {
		render(<ErrorBoundary {...defaultProps} />);

		const button = screen.getByRole('button', { name: 'Try again' });
		expect(button).toHaveAttribute('type', 'button');
	});

	it('applies correct CSS classes', () => {
		render(<ErrorBoundary {...defaultProps} />);

		const container = screen.getByText('Something went wrong!').closest('div');
		expect(container).toHaveClass('text-center');

		const outerContainer = screen.getByText('Something went wrong!').closest('div')?.parentElement;
		expect(outerContainer).toHaveClass('flex', 'min-h-screen', 'items-center', 'justify-center');
	});
});
