import { redirect } from 'next/navigation';
import { describe, expect, it, vi } from 'vitest';
import RegistrationPage from './page';

// Mock next/navigation with a function that throws
vi.mock('next/navigation', () => ({
	redirect: vi.fn(() => {
		throw new Error('NEXT_REDIRECT');
	}),
}));

describe('RegistrationPage', () => {
	it('redirects to login page with register mode', () => {
		// The component should throw (because redirect throws)
		expect(() => RegistrationPage()).toThrow('NEXT_REDIRECT');

		// Verify redirect was called with correct URL
		expect(redirect).toHaveBeenCalledWith('/auth/login?mode=register');
	});
});
