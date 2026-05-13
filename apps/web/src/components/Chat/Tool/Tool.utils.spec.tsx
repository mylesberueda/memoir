import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import React from 'react';
import { describe, expect, it } from 'vitest';
import { getStatusBadge } from './Tool.utils';

describe('Tool.utils', () => {
	describe('getStatusBadge', () => {
		it('renders correct badge for input-streaming status', () => {
			render(<div>{getStatusBadge('input-streaming')}</div>);

			const badge = screen.getByText('Pending').closest('.badge');
			expect(badge).toBeInTheDocument();
			expect(badge).toHaveClass('rounded-full', 'text-xs', 'gap-1');
			expect(screen.getByText('Pending')).toBeInTheDocument();

			// Check for CircleIcon presence by looking for lucide icon
			const circleIcon = badge?.querySelector('svg');
			expect(circleIcon).toBeInTheDocument();
			expect(circleIcon).toHaveClass('size-4');
		});

		it('renders correct badge for input-available status', () => {
			render(<div>{getStatusBadge('input-available')}</div>);

			const badge = screen.getByText('Running').closest('.badge');
			expect(badge).toBeInTheDocument();
			expect(badge).toHaveClass('rounded-full', 'text-xs', 'gap-1');
			expect(screen.getByText('Running')).toBeInTheDocument();

			// Check for ClockIcon with animation
			const clockIcon = badge?.querySelector('svg');
			expect(clockIcon).toBeInTheDocument();
			expect(clockIcon).toHaveClass('size-4', 'animate-pulse');
		});

		it('renders correct badge for output-available status', () => {
			render(<div>{getStatusBadge('output-available')}</div>);

			const badge = screen.getByText('Completed').closest('.badge');
			expect(badge).toBeInTheDocument();
			expect(badge).toHaveClass('rounded-full', 'text-xs', 'gap-1');
			expect(screen.getByText('Completed')).toBeInTheDocument();

			// Check for CheckCircleIcon with green color
			const checkIcon = badge?.querySelector('svg');
			expect(checkIcon).toBeInTheDocument();
			expect(checkIcon).toHaveClass('size-4', 'text-green-600');
		});

		it('renders correct badge for output-error status', () => {
			render(<div>{getStatusBadge('output-error')}</div>);

			const badge = screen.getByText('Error').closest('.badge');
			expect(badge).toBeInTheDocument();
			expect(badge).toHaveClass('rounded-full', 'text-xs', 'gap-1');
			expect(screen.getByText('Error')).toBeInTheDocument();

			// Check for XCircleIcon with red color
			const errorIcon = badge?.querySelector('svg');
			expect(errorIcon).toBeInTheDocument();
			expect(errorIcon).toHaveClass('size-4', 'text-red-600');
		});

		it('returns JSX element with correct structure for all statuses', () => {
			const statuses: Array<'input-streaming' | 'input-available' | 'output-available' | 'output-error'> = [
				'input-streaming',
				'input-available',
				'output-available',
				'output-error',
			];

			for (const status of statuses) {
				const element = getStatusBadge(status);
				expect(React.isValidElement(element)).toBe(true);
				expect(element.type).toBeDefined();
			}
		});

		it('maintains correct label mappings', () => {
			const expectedLabels = {
				'input-streaming': 'Pending',
				'input-available': 'Running',
				'output-available': 'Completed',
				'output-error': 'Error',
			};

			for (const [status, expectedLabel] of Object.entries(expectedLabels)) {
				render(
					<div>
						{getStatusBadge(status as 'input-streaming' | 'input-available' | 'output-available' | 'output-error')}
					</div>,
				);
				expect(screen.getByText(expectedLabel)).toBeInTheDocument();
			}
		});

		it('applies consistent badge styling across all statuses', () => {
			const statuses: Array<'input-streaming' | 'input-available' | 'output-available' | 'output-error'> = [
				'input-streaming',
				'input-available',
				'output-available',
				'output-error',
			];

			for (const status of statuses) {
				const { container } = render(<div>{getStatusBadge(status)}</div>);
				const badge = container.querySelector('.badge');

				expect(badge).toHaveClass('rounded-full', 'text-xs');
			}
		});

		it('includes icons for all status types', () => {
			const statuses: Array<'input-streaming' | 'input-available' | 'output-available' | 'output-error'> = [
				'input-streaming',
				'input-available',
				'output-available',
				'output-error',
			];

			for (const status of statuses) {
				const { container } = render(<div>{getStatusBadge(status)}</div>);
				const icon = container.querySelector('svg');

				expect(icon).toBeInTheDocument();
				expect(icon).toHaveClass('size-4');
			}
		});

		it('matches snapshot for input-streaming', () => {
			const { container } = render(<div>{getStatusBadge('input-streaming')}</div>);
			expect(container.firstChild).toMatchSnapshot();
		});

		it('matches snapshot for input-available', () => {
			const { container } = render(<div>{getStatusBadge('input-available')}</div>);
			expect(container.firstChild).toMatchSnapshot();
		});

		it('matches snapshot for output-available', () => {
			const { container } = render(<div>{getStatusBadge('output-available')}</div>);
			expect(container.firstChild).toMatchSnapshot();
		});

		it('matches snapshot for output-error', () => {
			const { container } = render(<div>{getStatusBadge('output-error')}</div>);
			expect(container.firstChild).toMatchSnapshot();
		});
	});
});
