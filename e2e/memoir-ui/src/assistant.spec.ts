import { expect, test } from './fixtures';
import { ROUTES, TIMEOUTS } from './utils';

/**
 * Assistant tests - chat with the built-in assistant.
 */
test.describe('Assistant', () => {
	test.beforeEach(async ({ auth }) => {
		await auth.ensureTestAccountExists();
		await auth.login();
	});

	test('should_chat_with_assistant_and_see_tool_call', async ({ page }) => {
		// LLM cold starts can be slow - extend timeout
		test.setTimeout(TIMEOUTS.LLM);
		await page.goto(ROUTES.ASSISTANT);
		await page.waitForLoadState('networkidle');

		// Wait for chat input to be ready and enabled
		const chatInput = page.locator('#assistant_chat__input');
		await chatInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });

		// Send a message asking for the time (triggers tool call)
		await chatInput.fill('What time is it right now?');
		await chatInput.press('Enter');

		// Wait for tool call to appear (don't wait for full response - it may be slow)
		// Tool call with "time" in the name should be visible once executed
		const toolCallName = page.locator('#tool_call__name').filter({ hasText: /time/i });
		await expect(toolCallName.first()).toBeVisible({ timeout: TIMEOUTS.LLM });
	});
});
