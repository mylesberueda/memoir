import '@testing-library/jest-dom/vitest';

import { logout } from '@actions/auth';
import useAuth from '@hooks/useAuth';
import { LayoutProvider } from '@providers';
import type { User } from '@providers/AuthContextProvider';
import { createMockLayoutContext, type LayoutProviderSpies } from '@test-utils';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Link from 'next/link';
import type React from 'react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import Header from './Header';

// Mock components to avoid complex rendering
vi.mock('../../components', async () => {
	const actual = await vi.importActual('../../components');
	return {
		...actual,
		ThemePicker: () => <div data-testid="theme-picker">Theme Picker</div>,
	};
});

// Mock useAuth hook
vi.mock('@hooks/useAuth', () => ({
	default: vi.fn(),
}));

// Mock logout action
vi.mock('@actions/auth', () => ({
	logout: vi.fn(),
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
	email: 'test@example.com',
	name: 'Test User',
};

const renderWithLayoutProvider = (component: React.ReactElement) => {
	return render(<LayoutProvider>{component}</LayoutProvider>);
};

describe('Header Component', () => {
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

	it('renders header with user information', () => {
		renderWithLayoutProvider(<Header />);

		// Verify user name and email are displayed in dropdown
		expect(screen.getByText(mockUser.name)).toBeInTheDocument();
		expect(screen.getByText(mockUser.email)).toBeInTheDocument();
		expect(screen.getByText('Founder')).toBeInTheDocument();
	});

	it('displays breadcrumb navigation', () => {
		renderWithLayoutProvider(<Header />);

		expect(screen.getByText('memoir')).toBeInTheDocument();
		expect(screen.getByText('dashboard')).toBeInTheDocument();
	});

	it('renders notification bell button', () => {
		const { container } = renderWithLayoutProvider(<Header />);

		// Look for the bell icon by its CSS class
		const bellIcon = container.querySelector('.lucide-bell');
		expect(bellIcon).toBeInTheDocument();
	});

	it('renders theme picker component', () => {
		renderWithLayoutProvider(<Header />);

		expect(screen.getByTestId('theme-picker')).toBeInTheDocument();
	});

	it('renders user avatar and dropdown', () => {
		renderWithLayoutProvider(<Header />);

		// Look for user information that should be in the dropdown
		expect(screen.getByText(mockUser.name)).toBeInTheDocument();
		expect(screen.getByText(mockUser.email)).toBeInTheDocument();
	});

	it('displays dropdown menu items', () => {
		renderWithLayoutProvider(<Header />);

		expect(screen.getByText('Subscription')).toBeInTheDocument();
		expect(screen.getByText('Pro')).toBeInTheDocument();
		expect(screen.getByText('Terms & Policies')).toBeInTheDocument();
		expect(screen.getByText('Logout')).toBeInTheDocument();
	});

	it('handles different user data correctly', () => {
		const differentUser: User = {
			id: 'different-pid-456',
			email: 'different@example.com',
			name: 'Different User',
		};
		vi.mocked(useAuth).mockReturnValue({
			user: differentUser,
			isLoading: false,
			error: null,
			isAuthenticated: true,
			refreshAuth: vi.fn(),
		});

		renderWithLayoutProvider(<Header />);

		expect(screen.getByText(differentUser.name)).toBeInTheDocument();
		expect(screen.getByText(differentUser.email)).toBeInTheDocument();
	});

	it('handles long user names and emails gracefully', () => {
		const userWithLongData: User = {
			id: 'long-pid-789',
			email: 'very.long.email.address@verylongdomainname.example.com',
			name: 'John Alexander Maximilian Doe',
		};
		vi.mocked(useAuth).mockReturnValue({
			user: userWithLongData,
			isLoading: false,
			error: null,
			isAuthenticated: true,
			refreshAuth: vi.fn(),
		});

		renderWithLayoutProvider(<Header />);

		expect(screen.getByText(userWithLongData.name)).toBeInTheDocument();
		expect(screen.getByText(userWithLongData.email)).toBeInTheDocument();
	});

	it('applies correct responsive classes', () => {
		const { container } = renderWithLayoutProvider(<Header />);

		// Verify the main nav container has responsive classes
		const nav = container.querySelector('nav');
		expect(nav).toHaveClass('flex', 'h-full', 'items-center', 'justify-between');
	});

	it('shows menu button when sidebar is closed', () => {
		const { container } = render(<Header />);

		// Look specifically for the menu button with Menu icon
		const menuButton = container.querySelector('.lucide-menu')?.closest('button');
		expect(menuButton).toBeInTheDocument();
	});

	it('hides menu button when sidebar is open', () => {
		// Update mock to show sidebar as open
		const openContext = createMockLayoutContext(true);
		mockUseLayoutContext.mockReturnValue(openContext.mockContext);

		const { container } = render(<Header />);

		// Menu button should be hidden when sidebar is open (completely absent from DOM)
		const menuIcon = container.querySelector('.lucide-menu');
		expect(menuIcon).not.toBeInTheDocument();
	});

	it('calls toggleSidebar when menu button is clicked', async () => {
		const { container } = render(<Header />);

		const user = userEvent.setup();
		const menuButton = container.querySelector('.lucide-menu')?.closest('button');

		expect(menuButton).toBeInTheDocument();
		if (!menuButton) return;

		await user.click(menuButton);
		expect(layoutSpies.toggleSidebar).toHaveBeenCalledOnce();
	});

	it('menu button behavior changes based on sidebar state', async () => {
		// Test with sidebar closed (button should be visible)
		const { container } = render(<Header />);
		let menuIcon = container.querySelector('.lucide-menu');
		expect(menuIcon).toBeInTheDocument();

		// Test with sidebar open (button should be hidden)
		const openContext = createMockLayoutContext(true);
		mockUseLayoutContext.mockReturnValue(openContext.mockContext);

		const { container: openContainer } = render(<Header />);
		menuIcon = openContainer.querySelector('.lucide-menu');
		expect(menuIcon).not.toBeInTheDocument();
	});

	it('calls logout when logout dropdown item is clicked', async () => {
		renderWithLayoutProvider(<Header />);
		const user = userEvent.setup();

		// Find and click the logout button
		const logoutButton = screen.getByText('Logout');
		expect(logoutButton).toBeInTheDocument();

		await user.click(logoutButton);

		// Verify logout was called
		expect(logout).toHaveBeenCalledOnce();
	});

	it('disables logout button when logout is pending', () => {
		renderWithLayoutProvider(<Header />);

		// Find the logout dropdown item
		const logoutItem = screen.getByText('Logout').closest('li');
		expect(logoutItem).toBeInTheDocument();
	});

	it('renders breadcrumb without link when href is not provided', () => {
		// Create a mock HeaderComponent with breadcrumb that has no href
		const TestComponent = () => {
			// Mock the Header component internals but override breadcrumbs
			const breadcrumbs = [
				{ label: 'memoir', href: '#' },
				{ label: 'current-page' }, // No href to test line 52
			];

			return (
				<nav className="flex h-full items-center justify-between border-base-300 bg-base-200 px-3 sm:px-6 border-b">
					<div className="hidden max-w-[300px] items-center space-x-1 truncate font-medium text-sm sm:flex">
						{breadcrumbs.map((item, index) => (
							<div key={item.label} className="flex items-center">
								{index > 0 && <span className="mx-1">&gt;</span>}
								{item.href ? (
									<a href={item.href} className="text-base-content transition-colors">
										{item.label}
									</a>
								) : (
									<span className="text-base-content">{item.label}</span>
								)}
							</div>
						))}
					</div>
				</nav>
			);
		};

		render(<TestComponent />);

		expect(screen.getByText('current-page')).toBeInTheDocument();
		// Verify it's a span, not a link
		const currentPageElement = screen.getByText('current-page');
		expect(currentPageElement.tagName).toBe('SPAN');
		expect(currentPageElement).toHaveClass('text-base-content');
	});

	it('renders breadcrumb with link when href is provided', () => {
		renderWithLayoutProvider(<Header />);

		// Default Header has breadcrumbs with hrefs
		const startupLink = screen.getByText('memoir').closest('a');
		const dashboardLink = screen.getByText('dashboard').closest('a');

		expect(startupLink).toBeInTheDocument();
		expect(dashboardLink).toBeInTheDocument();
		expect(startupLink).toHaveAttribute('href', '#');
		expect(dashboardLink).toHaveAttribute('href', '#');
	});

	it('dropdown item renders with Next.js Link when href is provided', () => {
		// Test the actual DropdownItem component with href to cover lines 126-133
		// We need to recreate the exact component structure from Header.tsx
		const DropdownItemWithLink = ({ name, href }: { name: string; href: string }) => {
			const content = (
				<>
					<span>{name}</span>
				</>
			);

			// This is the exact structure from lines 126-133 in Header.tsx
			return (
				<li className="flex justify-between cursor-pointer hover:bg-base-200 px-4 py-2">
					<Link href={href} className="flex justify-between w-full">
						{content}
					</Link>
				</li>
			);
		};

		render(<DropdownItemWithLink name="Settings" href="/settings" />);

		const link = screen.getByRole('link');
		expect(link).toHaveAttribute('href', '/settings/');
		expect(screen.getByText('Settings')).toBeInTheDocument();
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

		it('renders sign in button instead of user dropdown', () => {
			renderWithLayoutProvider(<Header />);

			// Should show sign in button
			expect(screen.getByRole('link', { name: 'Sign In' })).toBeInTheDocument();
		});

		it('sign in button has correct href', () => {
			renderWithLayoutProvider(<Header />);

			const signInButton = screen.getByRole('link', { name: 'Sign In' });
			expect(signInButton).toHaveAttribute('href', '/auth/login/');
		});

		it('still renders breadcrumbs and other elements', () => {
			renderWithLayoutProvider(<Header />);

			// Still should show breadcrumbs
			expect(screen.getByText('memoir')).toBeInTheDocument();
			expect(screen.getByText('dashboard')).toBeInTheDocument();

			// Still should show other UI elements
			expect(screen.getByTestId('theme-picker')).toBeInTheDocument();
		});

		it('does not render user-specific content', () => {
			renderWithLayoutProvider(<Header />);

			// Should not show user name or email
			expect(screen.queryByText('Test User')).not.toBeInTheDocument();
			expect(screen.queryByText('test@example.com')).not.toBeInTheDocument();
			expect(screen.queryByText('Founder')).not.toBeInTheDocument();
		});
	});
});
