import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import CheckoutModal from './CheckoutModal';

// Mock Next.js navigation hooks
const mockReplace = vi.fn();
const mockGet = vi.fn();

vi.mock('next/navigation', () => ({
	useRouter: () => ({
		replace: mockReplace,
		push: vi.fn(),
		refresh: vi.fn(),
		back: vi.fn(),
		forward: vi.fn(),
		prefetch: vi.fn(),
	}),
	useSearchParams: () => ({
		get: mockGet,
	}),
}));

// Mock window.location for URL manipulation tests
const mockLocation = {
	href: 'http://localhost:3000/dashboard',
	pathname: '/dashboard',
};

Object.defineProperty(window, 'location', {
	value: mockLocation,
	writable: true,
});

describe('CheckoutModal', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mockLocation.href = 'http://localhost:3000/dashboard';
		mockLocation.pathname = '/dashboard';
	});

	describe('when no checkout query param is present', () => {
		beforeEach(() => {
			mockGet.mockReturnValue(null);
		});

		it('should not render the modal', () => {
			render(<CheckoutModal />);

			expect(screen.queryByRole('dialog', { hidden: true })).not.toBeInTheDocument();
		});

		it('should not render any content', () => {
			render(<CheckoutModal />);

			expect(screen.queryByText('Welcome to your new plan!')).not.toBeInTheDocument();
			expect(screen.queryByText('Checkout cancelled')).not.toBeInTheDocument();
		});
	});

	describe('when checkout=success query param is present', () => {
		beforeEach(() => {
			mockGet.mockImplementation((key: string) => (key === 'checkout' ? 'success' : null));
			mockLocation.href = 'http://localhost:3000/dashboard?checkout=success&session_id=cs_test_123';
		});

		it('should render the success modal', () => {
			render(<CheckoutModal />);

			expect(screen.getByRole('dialog', { hidden: true })).toBeInTheDocument();
		});

		it('should display success title', () => {
			render(<CheckoutModal />);

			expect(screen.getByText('Welcome to your new plan!')).toBeInTheDocument();
		});

		it('should display success description', () => {
			render(<CheckoutModal />);

			expect(screen.getByText('Your subscription is now active. Enjoy your upgraded features!')).toBeInTheDocument();
		});

		it('should display success icon with correct styling', () => {
			render(<CheckoutModal />);

			const iconContainer = screen.getByTestId('checkout-modal-icon-container');
			expect(iconContainer).toHaveClass('bg-success/10');
		});

		it('should display "Get started" button', () => {
			render(<CheckoutModal />);

			expect(screen.getByRole('button', { name: 'Get started', hidden: true })).toBeInTheDocument();
		});

		it('should clear query params when close button is clicked', async () => {
			const user = userEvent.setup();
			render(<CheckoutModal />);

			const closeButton = screen.getByRole('button', { name: 'Close', hidden: true });
			await user.click(closeButton);

			expect(mockReplace).toHaveBeenCalledWith('/dashboard', { scroll: false });
		});

		it('should clear query params when "Get started" button is clicked', async () => {
			const user = userEvent.setup();
			render(<CheckoutModal />);

			const getStartedButton = screen.getByRole('button', { name: 'Get started', hidden: true });
			await user.click(getStartedButton);

			expect(mockReplace).toHaveBeenCalledWith('/dashboard', { scroll: false });
		});

		it('should call onClose callback when modal is closed', async () => {
			const onClose = vi.fn();
			const user = userEvent.setup();
			render(<CheckoutModal onClose={onClose} />);

			const closeButton = screen.getByRole('button', { name: 'Close', hidden: true });
			await user.click(closeButton);

			expect(onClose).toHaveBeenCalledOnce();
		});
	});

	describe('when checkout=cancelled query param is present', () => {
		beforeEach(() => {
			mockGet.mockImplementation((key: string) => (key === 'checkout' ? 'cancelled' : null));
			mockLocation.href = 'http://localhost:3000/dashboard?checkout=cancelled';
		});

		it('should render the cancelled modal', () => {
			render(<CheckoutModal />);

			expect(screen.getByRole('dialog', { hidden: true })).toBeInTheDocument();
		});

		it('should display cancelled title', () => {
			render(<CheckoutModal />);

			expect(screen.getByText('Checkout cancelled')).toBeInTheDocument();
		});

		it('should display cancelled description', () => {
			render(<CheckoutModal />);

			expect(screen.getByText('No worries! You can upgrade anytime from the settings page.')).toBeInTheDocument();
		});

		it('should display warning icon with correct styling', () => {
			render(<CheckoutModal />);

			const iconContainer = screen.getByTestId('checkout-modal-icon-container');
			expect(iconContainer).toHaveClass('bg-warning/10');
		});

		it('should display "Close" button', () => {
			render(<CheckoutModal />);

			// Both the X button and the action button should be present
			const buttons = screen.getAllByRole('button', { name: 'Close', hidden: true });
			expect(buttons.length).toBeGreaterThanOrEqual(1);
		});

		it('should clear query params when closed', async () => {
			const user = userEvent.setup();
			render(<CheckoutModal />);

			// Click the main action button (not the X) - select the primary button
			const buttons = screen.getAllByRole('button', { name: 'Close', hidden: true });
			const actionButton = buttons.find((btn) => btn.classList.contains('btn-primary'));
			expect(actionButton).toBeDefined();
			await user.click(actionButton as HTMLElement);

			expect(mockReplace).toHaveBeenCalledWith('/dashboard', { scroll: false });
		});
	});

	describe('when an invalid checkout query param is present', () => {
		beforeEach(() => {
			mockGet.mockImplementation((key: string) => (key === 'checkout' ? 'invalid' : null));
		});

		it('should not render the modal', () => {
			render(<CheckoutModal />);

			expect(screen.queryByRole('dialog', { hidden: true })).not.toBeInTheDocument();
		});
	});

	describe('URL handling', () => {
		beforeEach(() => {
			mockGet.mockImplementation((key: string) => (key === 'checkout' ? 'success' : null));
		});

		it('should remove both checkout and session_id params from URL', async () => {
			mockLocation.href = 'http://localhost:3000/dashboard?checkout=success&session_id=cs_test_123';
			const user = userEvent.setup();
			render(<CheckoutModal />);

			const closeButton = screen.getByRole('button', { name: 'Close', hidden: true });
			await user.click(closeButton);

			// Should navigate to clean pathname
			expect(mockReplace).toHaveBeenCalledWith('/dashboard', { scroll: false });
		});

		it('should preserve pathname when clearing params', async () => {
			mockLocation.href = 'http://localhost:3000/dashboard?checkout=success';
			mockLocation.pathname = '/dashboard';
			const user = userEvent.setup();
			render(<CheckoutModal />);

			const closeButton = screen.getByRole('button', { name: 'Close', hidden: true });
			await user.click(closeButton);

			expect(mockReplace).toHaveBeenCalledWith('/dashboard', { scroll: false });
		});
	});

	describe('accessibility', () => {
		beforeEach(() => {
			mockGet.mockImplementation((key: string) => (key === 'checkout' ? 'success' : null));
		});

		it('should have accessible close button with aria-label', () => {
			render(<CheckoutModal />);

			const closeButton = screen.getByRole('button', { name: 'Close', hidden: true });
			expect(closeButton).toHaveAttribute('aria-label', 'Close');
		});

		it('should render as a dialog', () => {
			render(<CheckoutModal />);

			expect(screen.getByRole('dialog', { hidden: true })).toBeInTheDocument();
		});
	});

	describe('component structure', () => {
		beforeEach(() => {
			mockGet.mockImplementation((key: string) => (key === 'checkout' ? 'success' : null));
		});

		it('should have semantic id attributes for sections', () => {
			const { container } = render(<CheckoutModal />);

			expect(container.querySelector('#checkout_modal__container')).toBeInTheDocument();
			expect(container.querySelector('#checkout_modal__icon')).toBeInTheDocument();
			expect(container.querySelector('#checkout_modal__title')).toBeInTheDocument();
			expect(container.querySelector('#checkout_modal__description')).toBeInTheDocument();
			expect(container.querySelector('#checkout_modal__actions')).toBeInTheDocument();
		});
	});
});
