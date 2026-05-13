import '@testing-library/jest-dom/vitest';
import React from 'react';
import { vi } from 'vitest';

// Mock Popover API - jsdom doesn't implement it
HTMLElement.prototype.showPopover = vi.fn();
HTMLElement.prototype.hidePopover = vi.fn();
HTMLElement.prototype.togglePopover = vi.fn();

// Mock Next.js Image component to avoid hostname validation in tests
vi.mock('next/image', () => ({
	default: (props: Record<string, unknown>) => {
		return React.createElement('img', props);
	},
}));

// Mock console methods to reduce noise in test output
global.console = {
	...console,
	log: vi.fn(),
	warn: vi.fn(),
	error: vi.fn(),
	info: vi.fn(),
	debug: vi.fn(),
};
