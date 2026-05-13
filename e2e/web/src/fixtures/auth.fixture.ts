import type { Page } from '@playwright/test';
import { generateTestAccount } from '../utils/constants';
import { MaildevClient } from '../utils/maildev';

/**
 * Authentication fixture for e2e tests.
 *
 * Strategy: Generate a unique test account per test.
 * Each test gets complete isolation - no shared accounts between tests.
 */
export class AuthFixture {
	private testAccount: { email: string; password: string; name: string } | null = null;

	constructor(private page: Page) {}

	/**
	 * Logs in with the test account.
	 * Account must exist (either via env vars or ensureTestAccountExists).
	 */
	async login() {
		const email = process.env.E2E_EMAIL || this.testAccount?.email;
		const password = process.env.E2E_PASSWORD || this.testAccount?.password;

		if (!email || !password) {
			throw new Error(
				`No test account available. testAccount=${JSON.stringify(this.testAccount)}, E2E_EMAIL=${process.env.E2E_EMAIL}`,
			);
		}

		// Navigate to login - server component handles OIDC redirect automatically
		// Page will redirect: /auth/login -> Zitadel -> /auth/login?authRequest=...
		await this.page.goto('/auth/login');
		await this.page.waitForURL('**/auth/login?authRequest=**', { timeout: 15000 });
		// Use domcontentloaded instead of networkidle - Firefox can hang on networkidle with Zitadel
		await this.page.waitForLoadState('domcontentloaded');

		// Fill form fields
		await this.page.fill('#email', email);
		await this.page.fill('#password', password);

		// Click sign in button
		await this.page.click('button[type="submit"]');

		// Wait for navigation to dashboard (successful login redirects there)
		try {
			await this.page.waitForURL('**/dashboard', { timeout: 15000 });
		} catch {
			const currentUrl = this.page.url();
			throw new Error(`Login failed: Did not redirect to dashboard. URL: ${currentUrl}`);
		}
	}

	/**
	 * Creates a fresh test account for this test.
	 * Each test gets its own isolated account - no sharing between tests.
	 * - If E2E_EMAIL/E2E_PASSWORD are set, uses those (assumes account exists)
	 * - Otherwise, generates and registers a new account via maildev
	 */
	async ensureTestAccountExists() {
		// If env vars are set, use those (for manual/CI override)
		if (process.env.E2E_EMAIL && process.env.E2E_PASSWORD) {
			this.testAccount = {
				email: process.env.E2E_EMAIL,
				password: process.env.E2E_PASSWORD,
				name: 'E2E Test User',
			};
			return;
		}

		// Always generate a fresh account for each test - complete isolation
		this.testAccount = generateTestAccount();
		await this.registerAccount(this.testAccount);
	}

	/**
	 * Registers a new account and verifies via maildev.
	 */
	private async registerAccount(account: { email: string; password: string; name: string }) {
		// Navigate to registration - redirects to /auth/login?mode=register
		await this.page.goto('/auth/registration');
		await this.page.waitForURL('**/auth/login?mode=register', { timeout: 10000 });
		// Use domcontentloaded instead of networkidle - Firefox can hang on networkidle
		await this.page.waitForLoadState('domcontentloaded');

		// Fill registration form
		await this.page.fill('#name', account.name);
		await this.page.fill('#email', account.email);
		await this.page.fill('#password', account.password);
		await this.page.fill('#confirmPassword', account.password);
		await this.page.click('button[type="submit"]');

		// After registration, shows success message
		await this.page.waitForSelector('text=Registration successful', { timeout: 15000 });

		// Verify email via maildev
		const maildev = new MaildevClient();
		const verificationEmail = await maildev.waitForEmail(account.email, 15000);
		const verificationUrl = maildev.extractVerificationUrl(verificationEmail.html);

		if (!verificationUrl) {
			throw new Error('Could not extract verification URL from email');
		}

		// Navigate to Zitadel verification URL
		await this.page.goto(verificationUrl);
		// Wait for Zitadel to confirm verification
		await this.page.waitForLoadState('networkidle');
	}

	async logout() {
		await this.page.context().clearCookies();
		await this.page.goto('/auth/login');
		await this.page.waitForURL('/auth/login', { timeout: 8000 });
	}

	/**
	 * Returns the current test account credentials.
	 */
	getTestAccount() {
		return this.testAccount;
	}
}
