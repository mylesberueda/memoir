import { nxE2EPreset } from '@nx/playwright/preset';
import { defineConfig, devices } from '@playwright/test';

// Apply test setup to suppress console noise
import './playwright.setup';

// Use WEB_URL from environment, fallback to BASE_URL, then localhost
const baseURL = process.env.WEB_URL ?? process.env.BASE_URL ?? 'http://localhost:3000';

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
	...nxE2EPreset(__filename, { testDir: './src' }),
	/* Configure workers and retries for better webkit stability */
	workers: process.env.CI ? 1 : 4, // Use 4 workers locally for faster execution
	retries: process.env.CI ? 2 : 1, // Enable retries for webkit timing issues

	/* Reduce noise in test output */
	quiet: true, // Suppress stdio
	reporter: process.env.CI ? 'github' : [['list', { printSteps: false }]], // Minimal reporter output

	/* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
	use: {
		baseURL,
		/* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
		trace: 'on-first-retry',
		/* Disable service workers to prevent webkit API response issues */
		serviceWorkers: 'block',
		/* Reduce video/screenshot noise unless debugging */
		video: 'retain-on-failure',
		screenshot: 'only-on-failure',
	},
	/* Note: Web server should be running independently before running E2E tests */
	projects: [
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] },
		},

		{
			name: 'firefox',
			use: { ...devices['Desktop Firefox'] },
		},

		// Webkit disabled - requires system libraries not available in WSL
		// To re-enable, run: pnpm exec playwright install --with-deps webkit
		// {
		// 	name: 'webkit',
		// 	use: {
		// 		...devices['Desktop Safari'],
		// 		actionTimeout: 8000,
		// 		navigationTimeout: 10000,
		// 	},
		// 	fullyParallel: false,
		// },

		// Uncomment for mobile browsers support
		/* {
      name: 'Mobile Chrome',
      use: { ...devices['Pixel 5'] },
    },
    {
      name: 'Mobile Safari',
      use: { ...devices['iPhone 12'] },
    }, */

		// Uncomment for branded browsers
		/* {
      name: 'Microsoft Edge',
      use: { ...devices['Desktop Edge'], channel: 'msedge' },
    },
    {
      name: 'Google Chrome',
      use: { ...devices['Desktop Chrome'], channel: 'chrome' },
    } */
	],
});
