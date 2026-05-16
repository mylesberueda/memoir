import { expect, test } from './fixtures';
import { ROUTES } from './utils';

/**
 * Smoke tests - minimal happy path verification.
 *
 * These tests verify core flows work end-to-end.
 * Keep this suite small and fast.
 */
test.describe('Smoke Tests', () => {
	test('should_register_login_and_reach_dashboard', async ({ page, auth }) => {
		// Ensure we have a test account (registers if needed)
		await auth.ensureTestAccountExists();

		// Login with the test account
		await auth.login();

		// Verify we reach the dashboard
		await expect(page).toHaveURL(ROUTES.DASHBOARD);
	});
});
