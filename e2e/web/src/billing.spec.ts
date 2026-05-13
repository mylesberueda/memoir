import { expect, test } from './fixtures';
import { ROUTES, TIMEOUTS } from './utils';

/**
 * Billing tests - upgrade to Plus tier via Stripe.
 */
test.describe('Billing', () => {
	test.beforeEach(async ({ auth }) => {
		await auth.ensureTestAccountExists();
		await auth.login();
	});

	test('should_upgrade_to_plus_tier_via_stripe', async ({ page }) => {
		// Stripe sandbox can be slow, especially in Firefox - extend timeout
		test.setTimeout(90000);
		// Navigate to billing settings
		await page.goto(ROUTES.BILLING);
		await page.waitForLoadState('networkidle');

		// Verify we're on the billing page
		await expect(page.locator('#billing_page__container')).toBeVisible({ timeout: TIMEOUTS.SHORT });

		// Click the Plus upgrade button
		const plusUpgradeButton = page.locator('#upgrade_button__plus');
		await expect(plusUpgradeButton).toBeVisible();
		await plusUpgradeButton.click();

		// Wait for redirect to Stripe Checkout
		await page.waitForURL('**/checkout.stripe.com/**', { timeout: TIMEOUTS.LONG });
		await page.waitForLoadState('domcontentloaded');

		// Wait for payment methods to load - Stripe uses iframes and dynamic loading
		await page.waitForTimeout(2000);

		// Select Card payment method - use radio button role for reliability
		const cardRadio = page.getByRole('radio', { name: 'Card' });
		await cardRadio.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await cardRadio.click({ force: true });

		// Wait for card number input to be visible (confirms card form expanded)
		const cardNumberInput = page.getByLabel('Card number');
		await cardNumberInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });

		// Fill card details (Stripe test card)
		await cardNumberInput.fill('4242424242424242');
		await page.getByLabel('Expiration').fill('1230');
		await page.getByRole('textbox', { name: 'CVC' }).fill('123');

		// Fill billing details
		await page.getByRole('textbox', { name: /cardholder name/i }).fill('E2E Test User');
		await page.getByRole('textbox', { name: 'ZIP' }).fill('12345');

		// Uncheck "Save my information" to avoid phone number requirement
		const saveInfoCheckbox = page.getByRole('checkbox', { name: /Save my information/i });
		if (await saveInfoCheckbox.isChecked()) {
			await saveInfoCheckbox.click();
		}

		// Submit payment - Stripe uses "Subscribe" button text
		const subscribeButton = page.getByRole('button', { name: 'Subscribe' });
		await subscribeButton.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await subscribeButton.click();

		// Wait for payment to start processing (button changes to "Processing")
		// This confirms form submission was successful - redirect timing varies in sandbox
		const processingButton = page.getByRole('button', { name: 'Processing' });
		await expect(processingButton).toBeVisible({ timeout: TIMEOUTS.MEDIUM });

		// Wait for redirect back to our app after payment processing
		// Use longer timeout since Stripe sandbox can be slow
		await page.waitForURL('**/localhost:3000/**', { timeout: TIMEOUTS.PAYMENT });

		// Verify we're back on our site (success page or dashboard)
		await expect(page).toHaveURL(/localhost:3000/);

		// === POST-UPGRADE VERIFICATION ===
		// Poll billing page until tier reflects the upgrade.
		// Webhook processing may take a few seconds after redirect.
		await expect(async () => {
			// Navigate to billing page (RSC fetches fresh tier data on each load)
			await page.goto(ROUTES.BILLING);
			await page.waitForLoadState('networkidle');

			// Verify current plan text shows "Plus"
			const currentPlanText = page.locator('#billing_page__current_plan');
			await expect(currentPlanText).toContainText('Plus');

			// Verify Plus card button is disabled (shows "Manage" for current plan)
			const plusButton = page.locator('#upgrade_button__plus');
			await expect(plusButton).toBeDisabled();
		}).toPass({
			timeout: TIMEOUTS.LONG,
			intervals: [1000, 2000, 3000, 5000], // Retry at 1s, 2s, 3s, 5s intervals
		});
	});
});
