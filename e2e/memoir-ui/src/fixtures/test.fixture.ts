import { test as base } from '@playwright/test';
import { AuthFixture } from './auth.fixture';

/**
 * Extended test fixture that provides auth capabilities.
 *
 * Usage:
 *   test('my test', async ({ page, auth }) => {
 *     await auth.ensureTestAccountExists();
 *     await auth.login();
 *     // ... test authenticated flows
 *   });
 */
export const test = base.extend<{ auth: AuthFixture }>({
	auth: async ({ page }, use) => {
		const auth = new AuthFixture(page);
		await use(auth);
	},
});

export { expect } from '@playwright/test';
