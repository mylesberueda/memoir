import { vi } from 'vitest';

export interface LayoutProviderSpies {
	openSidebar: ReturnType<typeof vi.fn>;
	closeSidebar: ReturnType<typeof vi.fn>;
	toggleSidebar: ReturnType<typeof vi.fn>;
}

/**
 * Creates mock functions for testing LayoutProvider method calls.
 * This is used with vi.mock to replace the useLayoutContext hook.
 */
export function createMockLayoutContext(initialSidebarOpen = false) {
	const spies: LayoutProviderSpies = {
		openSidebar: vi.fn(),
		closeSidebar: vi.fn(),
		toggleSidebar: vi.fn(),
	};

	const mockContext = {
		isSidebarOpen: initialSidebarOpen,
		openSidebar: spies.openSidebar,
		closeSidebar: spies.closeSidebar,
		toggleSidebar: spies.toggleSidebar,
	};

	return { mockContext, spies };
}
