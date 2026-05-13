import '@testing-library/jest-dom/vitest';

// Import for mock
import useAuth from '@hooks/useAuth';
import { LayoutProvider } from '@providers';
import type { User } from '@providers/AuthContextProvider';
import { createMockLayoutContext, type LayoutProviderSpies } from '@test-utils';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import Sidebar from './Sidebar';

// Mock useAuth hook
vi.mock('@hooks/useAuth', () => ({
	default: vi.fn(),
}));

// Mock useOrganizationsOptional
const mockCan = vi.fn().mockReturnValue(true);
vi.mock('@providers/OrganizationContextProvider', () => ({
	useOrganizationsOptional: () => ({
		currentOrg: { pid: 'org-1', name: 'Test Org', slug: 'test-org' },
		currentOrgPid: 'org-1',
		organizations: [{ pid: 'org-1', name: 'Test Org', slug: 'test-org' }],
		can: (...args: unknown[]) => mockCan(...args),
	}),
}));

// Mock Next.js navigation hooks
vi.mock('next/navigation', () => ({
	usePathname: () => '/dashboard',
	useRouter: () => ({
		push: vi.fn(),
		replace: vi.fn(),
		refresh: vi.fn(),
		back: vi.fn(),
		forward: vi.fn(),
		prefetch: vi.fn(),
	}),
}));

// Mock Next.js Link
vi.mock('next/link', () => ({
	default: ({
		children,
		href,
		onClick,
		...props
	}: {
		children: React.ReactNode;
		href: string;
		onClick?: () => void;
		[key: string]: unknown;
	}) => (
		<a
			href={href}
			onClick={(e) => {
				e.preventDefault(); // Prevent actual navigation in tests
				onClick?.();
			}}
			data-testid="nav-link"
			{...props}>
			{children}
		</a>
	),
}));

// Create mock layout context for method call testing
let mockLayoutContext: ReturnType<typeof createMockLayoutContext>;
let layoutSpies: LayoutProviderSpies;

// Mock the providers module when we need to test method calls
const mockUseLayoutContext = vi.fn();
vi.mock('@providers', async () => {
	const actual = await vi.importActual('@providers');
	return {
		...actual,
		useLayoutContext: () => mockUseLayoutContext(),
	};
});

const mockUser: User = {
	id: 'test-pid-123',
	email: 'test@startup.ai',
	name: 'Test User',
};

const renderWithLayoutProvider = (component: React.ReactElement) => {
	return render(<LayoutProvider>{component}</LayoutProvider>);
};

describe('Sidebar Component', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		// Reset mock layout context for each test
		mockLayoutContext = createMockLayoutContext(false);
		layoutSpies = mockLayoutContext.spies;
		mockUseLayoutContext.mockReturnValue(mockLayoutContext.mockContext);
		// Mock useAuth to return the default user
		vi.mocked(useAuth).mockReturnValue({
			user: mockUser,
			isLoading: false,
			error: null,
			isAuthenticated: true,
			refreshAuth: vi.fn(),
		});
	});

	it('renders sidebar with branding', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify branding is displayed
		expect(screen.getByText('STARTUP')).toBeInTheDocument();
		expect(screen.getByText('.ai')).toBeInTheDocument();
	});

	it('renders all navigation sections', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify main sections
		expect(screen.getByText('Overview')).toBeInTheDocument();
		expect(screen.getAllByText('Test Org').length).toBeGreaterThanOrEqual(1); // org section header
	});

	it('renders overview section navigation items', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify Overview section items
		expect(screen.getByText('Assistant')).toBeInTheDocument();
		expect(screen.getByText('Dashboard')).toBeInTheDocument();
		expect(screen.getByText('Agents')).toBeInTheDocument();
		expect(screen.getByText('Conversations')).toBeInTheDocument();
		expect(screen.getByText('Projects')).toBeInTheDocument();
	});

	it('renders org section navigation items', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify org section items
		expect(screen.getByText('Details')).toBeInTheDocument();
		expect(screen.getByText('Members')).toBeInTheDocument();
		expect(screen.getByText('Billing')).toBeInTheDocument();
	});

	it('renders footer navigation items', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify footer items
		expect(screen.getByText('Settings')).toBeInTheDocument();
		expect(screen.getByText('Help')).toBeInTheDocument();
	});

	it('handles navigation link clicks', async () => {
		renderWithLayoutProvider(<Sidebar />);

		const assistantLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Assistant'));

		expect(assistantLink).toBeInTheDocument();
		expect(assistantLink).toHaveAttribute('href', '/assistant');
	});

	it('calls closeSidebar when navigation link is clicked', async () => {
		renderWithLayoutProvider(<Sidebar />);

		const user = userEvent.setup();
		const assistantLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Assistant'));

		expect(assistantLink).toBeInTheDocument();
		if (!assistantLink) return;

		await user.click(assistantLink);
		expect(layoutSpies.closeSidebar).toHaveBeenCalledOnce();
	});

	it('calls closeSidebar for all navigation links', async () => {
		renderWithLayoutProvider(<Sidebar />);

		const user = userEvent.setup();
		const navigationLinks = [
			{ text: 'Assistant', expectedCalls: 1 },
			{ text: 'Dashboard', expectedCalls: 2 },
			{ text: 'Details', expectedCalls: 3 },
		];

		for (const { text, expectedCalls } of navigationLinks) {
			const link = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes(text));

			expect(link).toBeInTheDocument();
			if (!link) return;

			await user.click(link);
			expect(layoutSpies.closeSidebar).toHaveBeenCalledTimes(expectedCalls);
		}
	});

	it('renders with correct sidebar state based on context', async () => {
		// Test with sidebar closed
		const { container } = render(<Sidebar />);
		expect(container.querySelector('nav')).toHaveClass('-translate-x-full');

		// Test with sidebar open by updating the mock
		const openContext = createMockLayoutContext(true);
		mockUseLayoutContext.mockReturnValue(openContext.mockContext);

		const { container: openContainer } = render(<Sidebar />);
		expect(openContainer.querySelector('nav')).toHaveClass('translate-x-0');
	});

	it('applies correct href attributes to navigation links', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify key navigation links have correct hrefs
		const assistantLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Assistant'));
		const dashboardLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Dashboard'));
		const agentsLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Agents'));
		const detailsLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Details'));
		const membersLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Members'));
		const billingLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('Billing'));

		expect(assistantLink).toHaveAttribute('href', '/assistant');
		expect(dashboardLink).toHaveAttribute('href', '/dashboard');
		expect(agentsLink).toHaveAttribute('href', '/agents');
		expect(detailsLink).toHaveAttribute('href', '/org/details');
		expect(membersLink).toHaveAttribute('href', '/org/members');
		expect(billingLink).toHaveAttribute('href', '/org/billing');
	});

	it('handles different user types', () => {
		const adminUser: User = {
			id: 'admin-456',
			email: 'admin@startup.ai',
			name: 'Admin User',
		};

		vi.mocked(useAuth).mockReturnValue({
			user: adminUser,
			isLoading: false,
			error: null,
			isAuthenticated: true,
			refreshAuth: vi.fn(),
		});
		renderWithLayoutProvider(<Sidebar />);

		// Sidebar should render the same regardless of user type (for now)
		expect(screen.getByText('STARTUP')).toBeInTheDocument();
		expect(screen.getByText('Assistant')).toBeInTheDocument();
		expect(screen.getByText('Dashboard')).toBeInTheDocument();
	});

	it('applies responsive classes correctly', () => {
		const { container } = renderWithLayoutProvider(<Sidebar />);

		// Verify main navigation container has responsive classes
		const nav = container.querySelector('nav');
		expect(nav).toHaveClass('fixed', 'inset-y-0', 'left-0', 'z-[70]', 'w-64');
	});

	it('renders with proper accessibility attributes', () => {
		renderWithLayoutProvider(<Sidebar />);

		// Verify navigation structure
		const nav = screen.getByRole('navigation');
		expect(nav).toBeInTheDocument();

		// Verify brand link
		const brandLink = screen.getAllByTestId('nav-link').find((link) => link.textContent?.includes('STARTUP'));
		expect(brandLink).toHaveAttribute('href', '/');
		expect(brandLink).toHaveAttribute('target', '_blank');
		expect(brandLink).toHaveAttribute('rel', 'noopener noreferrer');
	});

	describe('when user is not provided', () => {
		beforeEach(() => {
			// Mock useAuth to return null user
			vi.mocked(useAuth).mockReturnValue({
				user: null,
				isLoading: false,
				error: null,
				isAuthenticated: false,
				refreshAuth: vi.fn(),
			});
		});

		it('renders sign in and sign up buttons instead of settings', () => {
			renderWithLayoutProvider(<Sidebar />);

			// Should not show Settings and Help when no user
			expect(screen.queryByText('Settings')).not.toBeInTheDocument();
			expect(screen.queryByText('Help')).not.toBeInTheDocument();

			// Should show sign in and sign up buttons
			expect(screen.getByRole('link', { name: 'Sign In' })).toBeInTheDocument();
			expect(screen.getByRole('link', { name: 'Sign Up' })).toBeInTheDocument();
		});

		it('sign in and sign up buttons have correct hrefs', () => {
			renderWithLayoutProvider(<Sidebar />);

			const signInButton = screen.getByRole('link', { name: 'Sign In' });
			const signUpButton = screen.getByRole('link', { name: 'Sign Up' });

			expect(signInButton).toHaveAttribute('href', '/auth/login');
			expect(signUpButton).toHaveAttribute('href', '/auth/register');
		});

		it('sign in and sign up buttons close sidebar when clicked', async () => {
			render(<Sidebar />);

			const user = userEvent.setup();
			const signInButton = screen.getByRole('link', { name: 'Sign In' });
			const signUpButton = screen.getByRole('link', { name: 'Sign Up' });

			// Test sign in button closes sidebar
			await user.click(signInButton);
			expect(layoutSpies.closeSidebar).toHaveBeenCalledOnce();

			// Test sign up button closes sidebar
			await user.click(signUpButton);
			expect(layoutSpies.closeSidebar).toHaveBeenCalledTimes(2);
		});

		it('still renders all navigation sections and branding', () => {
			renderWithLayoutProvider(<Sidebar />);

			// Should still show branding
			expect(screen.getByText('STARTUP')).toBeInTheDocument();
			expect(screen.getByText('.ai')).toBeInTheDocument();

			// Should still show navigation sections
			expect(screen.getByText('Overview')).toBeInTheDocument();
			expect(screen.getAllByText('Test Org').length).toBeGreaterThanOrEqual(1);

			// Should still show navigation items
			expect(screen.getByText('Assistant')).toBeInTheDocument();
			expect(screen.getByText('Dashboard')).toBeInTheDocument();
		});
	});

	describe('permission-based gating', () => {
		it('should hide billing nav when billing.read is denied', () => {
			mockCan.mockImplementation((resource: string) => resource !== 'billing');
			renderWithLayoutProvider(<Sidebar />);

			expect(screen.queryByText('Billing')).not.toBeInTheDocument();
			// Other org items should still be visible
			expect(screen.getByText('Details')).toBeInTheDocument();
			expect(screen.getByText('Members')).toBeInTheDocument();
		});

		it('should show billing nav when billing.read is allowed', () => {
			mockCan.mockReturnValue(true);
			renderWithLayoutProvider(<Sidebar />);

			expect(screen.getByText('Billing')).toBeInTheDocument();
		});

		it('should call can with correct arguments for billing', () => {
			mockCan.mockReturnValue(true);
			renderWithLayoutProvider(<Sidebar />);

			expect(mockCan).toHaveBeenCalledWith('billing', 'read');
		});
	});
});
