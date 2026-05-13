import type { Page } from '@playwright/test';

export async function waitForToast(page: Page, message: string) {
	await page.waitForSelector(`text="${message}"`, { timeout: 5000 });
}

export async function dismissToast(page: Page) {
	const toast = page.locator('[data-testid="toast-close"]');
	if (await toast.isVisible()) {
		await toast.click();
	}
}
