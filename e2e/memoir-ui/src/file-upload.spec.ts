import { expect, test } from './fixtures';
import { ROUTES, TIMEOUTS } from './utils';

/**
 * File upload + conversation-scoped document search tests.
 *
 * Verifies that files uploaded in a new conversation are linked to it and
 * searchable by the assistant's document_search tool. This is the E2E
 * counterpart to the backend defect fixes in task 04-conversation-scoped-rag.
 */
test.describe('File Upload', () => {
	test.beforeEach(async ({ auth }) => {
		await auth.ensureTestAccountExists();
		await auth.login();
	});

	test('should_create_conversation_before_file_upload_when_no_session', async ({ page }) => {
		// This test verifies the eager conversation creation fix:
		// when a user uploads a file in a brand-new chat (no session yet),
		// the conversation is created BEFORE the file upload so the document
		// is automatically attached via conversation_pid.
		test.setTimeout(TIMEOUTS.LLM);
		await page.goto(ROUTES.ASSISTANT);
		await page.waitForLoadState('networkidle');

		// Wait for chat to be ready
		const chatInput = page.locator('#assistant_chat__input');
		await chatInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });

		// Click the paperclip to open file upload
		const attachButton = page.locator('button[title="Attach file"]');
		await attachButton.click();

		// Upload a small text file
		const fileInput = page.locator('[data-testid="file-upload-input"]');
		await fileInput.setInputFiles({
			name: 'test-notes.txt',
			mimeType: 'text/plain',
			buffer: Buffer.from(
				'The capital of France is Paris. The Eiffel Tower was completed in 1889. ' +
					'The Seine river flows through the city center.',
			),
		});

		// Verify file appears in the upload list
		const uploadedFile = page.locator('[data-testid="uploaded-file"]');
		await expect(uploadedFile).toBeVisible({ timeout: TIMEOUTS.SHORT });
		await expect(uploadedFile).toContainText('test-notes.txt');

		// Type a message asking about the uploaded file content
		await chatInput.fill('What is the capital of France according to my document?');
		await chatInput.press('Enter');

		// The attachment should show processing/ready status indicators
		const attachmentContainer = page.locator('#message_attachments__container');
		await expect(attachmentContainer).toBeVisible({ timeout: TIMEOUTS.MEDIUM });

		// Wait for the document_search tool call — this proves the assistant knows
		// about the uploaded document. If the conversation wasn't eagerly created,
		// the document wouldn't be linked and we'd see "No documents are attached."
		const toolCallName = page.locator('#tool_call__name').filter({ hasText: /document.*search/i });
		await expect(toolCallName.first()).toBeVisible({ timeout: TIMEOUTS.LLM });
	});

	test('should_not_create_conversation_when_no_files_attached', async ({ page }) => {
		// Text-only messages should NOT trigger eager conversation creation.
		// The conversation is created lazily on first inference response.
		// We verify this indirectly: a text-only message should succeed and
		// produce a response without any file-related UI elements.
		test.setTimeout(TIMEOUTS.LLM);
		await page.goto(ROUTES.ASSISTANT);
		await page.waitForLoadState('networkidle');

		const chatInput = page.locator('#assistant_chat__input');
		await chatInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });

		// Send a text-only message (no files)
		await chatInput.fill('What time is it?');
		await chatInput.press('Enter');

		// Should get a response (tool call for current_time)
		const toolCallName = page.locator('#tool_call__name').filter({ hasText: /time/i });
		await expect(toolCallName.first()).toBeVisible({ timeout: TIMEOUTS.LLM });

		// No attachment container should exist on the user message
		const attachmentContainer = page.locator('#message_attachments__container');
		await expect(attachmentContainer).not.toBeVisible();
	});

	test('should_not_create_duplicate_conversation_when_session_exists', async ({ page }) => {
		// When a conversation already exists (sessionId is set), uploading files
		// should reuse the existing session — not create a duplicate.
		// We verify by: send a text message first (creates session), then send
		// a second message with a file in the same conversation.
		test.setTimeout(TIMEOUTS.LLM * 2); // Two LLM round-trips
		await page.goto(ROUTES.ASSISTANT);
		await page.waitForLoadState('networkidle');

		const chatInput = page.locator('#assistant_chat__input');
		await chatInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });

		// First message — creates the conversation via normal lazy flow
		await chatInput.fill('Hello, how are you?');
		await chatInput.press('Enter');

		// Wait for assistant to respond (conversation is now created)
		const firstResponse = page.locator('[class*="prose"]').first();
		await expect(firstResponse).toBeVisible({ timeout: TIMEOUTS.LLM });

		// Wait for input to be re-enabled after first response completes
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });

		// Now upload a file in the same conversation
		const attachButton = page.locator('button[title="Attach file"]');
		await attachButton.click();

		const fileInput = page.locator('[data-testid="file-upload-input"]');
		await fileInput.setInputFiles({
			name: 'follow-up.txt',
			mimeType: 'text/plain',
			buffer: Buffer.from('The answer to the ultimate question is 42.'),
		});

		const uploadedFile = page.locator('[data-testid="uploaded-file"]');
		await expect(uploadedFile).toBeVisible({ timeout: TIMEOUTS.SHORT });

		// Send second message with file
		await chatInput.fill('What does my document say about the ultimate question?');
		await chatInput.press('Enter');

		// The tool should find the document in the existing conversation
		const toolCallName = page.locator('#tool_call__name').filter({ hasText: /document.*search/i });
		await expect(toolCallName.first()).toBeVisible({ timeout: TIMEOUTS.LLM });
	});

	test('should_pass_conversation_pid_to_create_document', async ({ page }) => {
		// Verify that uploaded files get the conversation_pid by checking
		// the attachment transitions through PENDING → PROCESSING → READY states.
		// A file that is properly linked will be ingested; an unlinked file would
		// not transition to READY within the conversation context.
		test.setTimeout(TIMEOUTS.LLM);
		await page.goto(ROUTES.ASSISTANT);
		await page.waitForLoadState('networkidle');

		const chatInput = page.locator('#assistant_chat__input');
		await chatInput.waitFor({ state: 'visible', timeout: TIMEOUTS.MEDIUM });
		await expect(chatInput).toBeEnabled({ timeout: TIMEOUTS.MEDIUM });

		// Open file upload and attach file
		const attachButton = page.locator('button[title="Attach file"]');
		await attachButton.click();

		const fileInput = page.locator('[data-testid="file-upload-input"]');
		await fileInput.setInputFiles({
			name: 'linked-doc.txt',
			mimeType: 'text/plain',
			buffer: Buffer.from('This document should be linked to the conversation.'),
		});

		await chatInput.fill('Summarize my attached document.');
		await chatInput.press('Enter');

		// Wait for attachment to appear and reach READY state (green checkmark icon)
		// The success icon (CheckCircle2) indicates the file was properly uploaded,
		// confirmed, and ingested — which requires the conversation link to exist.
		const readyIcon = page.locator('#message_attachments__container .text-success');
		await expect(readyIcon.first()).toBeVisible({ timeout: TIMEOUTS.LONG });
	});
});
