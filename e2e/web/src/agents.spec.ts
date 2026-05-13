import { expect, test } from './fixtures';
import { ROUTES, TIMEOUTS } from './utils';

/**
 * Agent tests - create agent and chat with it.
 *
 * Creates an agent with the time tool, then chats with it to verify
 * the tool call works. Reuses the same agent (no extra accounts needed).
 */
test.describe('Agents', () => {
	test.beforeEach(async ({ auth }) => {
		await auth.ensureTestAccountExists();
		await auth.login();
	});

	test('should_create_agent_and_chat_with_tool_call', async ({ page }) => {
		// LLM responses can be slow - extend test timeout to accommodate TIMEOUTS.LLM
		test.setTimeout(150000);

		// Navigate to agents page
		await page.goto(ROUTES.AGENTS);
		await page.waitForLoadState('networkidle');

		// Click create agent button
		const createButton = page.locator(
			'button:has-text("Create Agent"), button:has-text("New Agent"), [id*="agents_page"] button:has([class*="Plus"])',
		);
		await createButton.first().click();

		// Wait for the agent creation modal
		const modal = page.locator('.modal-box.max-w-2xl');
		await expect(modal).toBeVisible({ timeout: TIMEOUTS.SHORT });

		// Fill in agent details
		const agentName = `E2E Time Agent ${Date.now()}`;
		await page.fill('#agent-name', agentName);

		// Get provider and model from env vars
		const testProvider = process.env.TEST_PROVIDER?.toLowerCase();
		const testModel = process.env.TEST_MODEL?.toLowerCase();
		if (!testProvider || !testModel) {
			throw new Error('TEST_PROVIDER and TEST_MODEL env vars are required');
		}

		// Select provider
		const providerSelect = page.locator('#agent-provider');
		await providerSelect.waitFor({ state: 'visible' });
		const providerOptions = await providerSelect.locator('option').allTextContents();
		const matchingProvider = providerOptions.find((opt) => opt.toLowerCase().includes(testProvider));
		if (!matchingProvider) {
			throw new Error(`No provider matching "${testProvider}". Available: ${providerOptions.join(', ')}`);
		}
		await providerSelect.selectOption({ label: matchingProvider });

		// Wait for models to load
		await page.waitForTimeout(500);

		// Select model
		const modelSelect = page.locator('#agent-model');
		const modelOptions = await modelSelect.locator('option').allTextContents();
		const matchingModel = modelOptions.find((opt) => opt.toLowerCase().includes(testModel));
		if (!matchingModel) {
			throw new Error(`No model matching "${testModel}". Available: ${modelOptions.join(', ')}`);
		}
		await modelSelect.selectOption({ label: matchingModel });

		// Add system prompt
		await page.fill(
			'#agent-system-prompt',
			'You are a time assistant. When asked for the time, use the current_time tool.',
		);

		// Select the Current Time tool
		const timeToolLabel = modal.locator('#tools_field__container label:has-text("Current Time")');
		if (await timeToolLabel.isVisible()) {
			await timeToolLabel.click();
		}

		// Submit the form
		await page.locator('button[type="submit"]:has-text("Create Agent")').click();

		// Wait for modal to close
		await expect(modal).not.toBeVisible({ timeout: TIMEOUTS.MEDIUM });

		// Find the agent card and click chat button
		const agentCard = page
			.locator(`[id*="agent_card"]:has-text("${agentName}"), .card:has-text("${agentName}")`)
			.first();
		await expect(agentCard).toBeVisible({ timeout: TIMEOUTS.MEDIUM });

		const chatButton = agentCard.locator('a:has([class*="MessageSquare"]), a[href*="/conversations/"]');
		await chatButton.click();

		// Wait for conversation page
		await page.waitForURL('**/conversations/**', { timeout: TIMEOUTS.MEDIUM });
		await page.waitForLoadState('networkidle');

		// Send a message asking for the time
		const chatInput = page.locator('#agent_chat__input');
		await chatInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		// Wait for input to be enabled (not disabled while agent loads)
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });
		await chatInput.fill('What is the current time?');
		await chatInput.press('Enter');

		// Verify user message appears
		await expect(page.locator('text=What is the current time?')).toBeVisible({ timeout: TIMEOUTS.SHORT });

		// Wait for tool call with "time" in the name
		const toolCallName = page.locator('#tool_call__name').filter({ hasText: /time/i });
		await expect(toolCallName.first()).toBeVisible({ timeout: TIMEOUTS.LLM });
	});
});
