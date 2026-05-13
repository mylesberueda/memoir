import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import PromptInput, {
	PromptInputButton,
	PromptInputSubmit,
	PromptInputTextarea,
	PromptInputToolbar,
	PromptInputTools,
} from '.';

describe('PromptInput', () => {
	it('renders form element with correct styling', () => {
		const onSubmit = vi.fn();
		const { container } = render(<PromptInput onSubmit={onSubmit} />);

		const form = container.querySelector('form');
		expect(form).toBeInTheDocument();

		if (!form) {
			fail('no form was found');
		}

		expect(form.tagName).toBe('FORM');
		expect(form).toHaveClass('w-full', 'divide-y', 'overflow-hidden', 'rounded-xl', 'border');
	});

	it('handles form submission with message', async () => {
		const onSubmit = vi.fn();
		const user = userEvent.setup();

		render(<PromptInput onSubmit={onSubmit} />);

		const textarea = screen.getByTestId('chat-input');
		await user.type(textarea, 'Test message');

		// Submit using Enter key
		await user.keyboard('{Enter}');

		await waitFor(() => {
			expect(onSubmit).toHaveBeenCalledWith({
				prompt: 'Test message',
				files: [],
				enableDeepResearch: undefined,
				enableWebSearch: undefined,
				enableFileUpload: undefined,
			});
		});
	});

	it('does not submit when message is empty', async () => {
		const onSubmit = vi.fn();
		const user = userEvent.setup();

		render(<PromptInput onSubmit={onSubmit} />);

		const _textarea = screen.getByTestId('chat-input');

		// Try to submit with empty message
		await user.keyboard('{Enter}');

		expect(onSubmit).not.toHaveBeenCalled();
	});

	it('disables input when disabled prop is true', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} disabled />);

		const textarea = screen.getByTestId('chat-input');
		expect(textarea).toBeDisabled();
	});

	it('shows loading state', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} isLoading />);

		const textarea = screen.getByTestId('chat-input');
		expect(textarea).toBeDisabled();
	});

	it('should show cancel button during streaming that stops the stream', () => {
		const onSubmit = vi.fn();
		const onStopStreaming = vi.fn();

		render(<PromptInput onSubmit={onSubmit} isStreaming onStopStreaming={onStopStreaming} />);

		const cancelButton = document.getElementById('prompt_actions__cancel');
		expect(cancelButton).toBeInTheDocument();

		if (!cancelButton) {
			fail();
		}

		fireEvent.click(cancelButton);
		expect(onStopStreaming).toHaveBeenCalled();
	});

	it('shows file upload when enabled', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} enableFileUpload />);

		const fileButton = screen.getByTitle('Attach file');
		expect(fileButton).toBeInTheDocument();
	});

	it('shows web search when enabled', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} enableWebSearch />);

		const searchButton = screen.getByText('Search');
		expect(searchButton).toBeInTheDocument();
	});

	it('shows deep research when enabled', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} enableDeepResearch />);

		const researchButton = screen.getByText('Deep Research');
		expect(researchButton).toBeInTheDocument();
	});

	it('shows mic when enabled', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} enableMic />);

		const micButton = screen.getByTitle('Voice input');
		expect(micButton).toBeInTheDocument();
	});

	it('allows multiline input with Shift+Enter', async () => {
		const onSubmit = vi.fn();
		const user = userEvent.setup();

		render(<PromptInput onSubmit={onSubmit} />);

		const textarea = screen.getByTestId('chat-input');
		await user.type(textarea, 'Line 1');
		await user.keyboard('{Shift>}{Enter}{/Shift}');
		await user.type(textarea, 'Line 2');

		expect(textarea).toHaveValue('Line 1\nLine 2');
		expect(onSubmit).not.toHaveBeenCalled();
	});

	it('clears message after submission', async () => {
		const onSubmit = vi.fn();
		const user = userEvent.setup();

		render(<PromptInput onSubmit={onSubmit} />);

		const textarea = screen.getByTestId('chat-input');
		await user.type(textarea, 'Test message');
		await user.keyboard('{Enter}');

		await waitFor(() => {
			expect(textarea).toHaveValue('');
		});
	});

	it('supports custom placeholder', () => {
		const onSubmit = vi.fn();
		render(<PromptInput onSubmit={onSubmit} placeholder="Custom placeholder" />);

		const textarea = screen.getByTestId('chat-input');
		expect(textarea).toHaveAttribute('placeholder', 'Custom placeholder');
	});
});

describe('PromptInputTextarea', () => {
	it('renders textarea with correct styling', () => {
		render(<PromptInputTextarea data-testid="textarea" />);

		const textarea = screen.getByTestId('textarea');
		expect(textarea).toBeInTheDocument();
		expect(textarea).toHaveClass('w-full', 'resize-none', 'rounded-none');
	});

	it('handles onChange event', async () => {
		const onChange = vi.fn();
		const user = userEvent.setup();

		render(<PromptInputTextarea onChange={onChange} data-testid="textarea" />);

		const textarea = screen.getByTestId('textarea');
		await user.type(textarea, 'Hello');

		expect(onChange).toHaveBeenCalled();
	});

	it('submits form on Enter key', async () => {
		const onSubmit = vi.fn((e) => e.preventDefault());
		const user = userEvent.setup();

		render(
			<form onSubmit={onSubmit}>
				<PromptInputTextarea data-testid="textarea" />
			</form>,
		);

		const textarea = screen.getByTestId('textarea');
		await user.type(textarea, 'Test');
		await user.keyboard('{Enter}');

		expect(onSubmit).toHaveBeenCalled();
	});

	it('allows newline with Shift+Enter', async () => {
		const onSubmit = vi.fn((e) => e.preventDefault());
		const user = userEvent.setup();

		render(
			<form onSubmit={onSubmit}>
				<PromptInputTextarea data-testid="textarea" />
			</form>,
		);

		const textarea = screen.getByTestId('textarea');
		await user.type(textarea, 'Line 1');
		await user.keyboard('{Shift>}{Enter}{/Shift}');
		await user.type(textarea, 'Line 2');

		expect(textarea).toHaveValue('Line 1\nLine 2');
		expect(onSubmit).not.toHaveBeenCalled();
	});
});

describe('PromptInputToolbar', () => {
	it('renders with correct styling', () => {
		render(
			<PromptInputToolbar data-testid="toolbar">
				<div>Content</div>
			</PromptInputToolbar>,
		);

		const toolbar = screen.getByTestId('toolbar');
		expect(toolbar).toBeInTheDocument();
		expect(toolbar).toHaveClass('flex', 'items-center', 'justify-between', 'p-2');
	});
});

describe('PromptInputTools', () => {
	it('renders with correct styling', () => {
		render(
			<PromptInputTools data-testid="tools">
				<button type="button">Tool</button>
			</PromptInputTools>,
		);

		const tools = screen.getByTestId('tools');
		expect(tools).toBeInTheDocument();
		expect(tools).toHaveClass('flex', 'items-center', 'gap-1');
	});
});

describe('PromptInputButton', () => {
	it('renders button with default props', () => {
		render(<PromptInputButton>Click me</PromptInputButton>);

		const button = screen.getByText('Click me');
		expect(button).toBeInTheDocument();
		expect(button).toHaveAttribute('type', 'button');
		expect(button).toHaveClass('shrink-0', 'gap-1.5', 'rounded-lg');
	});

	it('applies ghost color by default', () => {
		render(<PromptInputButton>Ghost</PromptInputButton>);

		const button = screen.getByText('Ghost');
		expect(button).toHaveClass('btn-ghost');
	});

	it('handles click event', () => {
		const onClick = vi.fn();
		render(<PromptInputButton onClick={onClick}>Click</PromptInputButton>);

		const button = screen.getByText('Click');
		fireEvent.click(button);

		expect(onClick).toHaveBeenCalled();
	});
});

describe('PromptInputSubmit', () => {
	it('renders submit button with send icon by default', () => {
		render(<PromptInputSubmit data-testid="submit" />);

		const button = screen.getByTestId('submit');
		expect(button).toBeInTheDocument();
		expect(button).toHaveAttribute('type', 'submit');
	});

	it('shows loading icon when status is submitted', () => {
		render(<PromptInputSubmit status="submitted" data-testid="submit" />);

		const button = screen.getByTestId('submit');
		const spinner = button.querySelector('.animate-spin');
		expect(spinner).toBeInTheDocument();
	});

	it('shows stop icon when status is streaming', () => {
		render(<PromptInputSubmit status="streaming" data-testid="submit" />);

		const button = screen.getByTestId('submit');
		expect(button).toBeInTheDocument();
	});

	it('shows error icon when status is error', () => {
		render(<PromptInputSubmit status="error" data-testid="submit" />);

		const button = screen.getByTestId('submit');
		expect(button).toBeInTheDocument();
	});

	it('renders custom children when provided', () => {
		render(<PromptInputSubmit>Custom Submit</PromptInputSubmit>);

		const button = screen.getByText('Custom Submit');
		expect(button).toBeInTheDocument();
	});
});
