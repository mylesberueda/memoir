import '@testing-library/jest-dom/vitest';

import type { Agent } from '@actions/agents';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import AgentCard from './AgentCard';

// Mock Next.js Link
vi.mock('next/link', () => ({
	default: ({ children, href, ...props }: { children: React.ReactNode; href: string; [key: string]: unknown }) => (
		<a href={href} data-testid="nav-link" {...props}>
			{children}
		</a>
	),
}));

const mockAgent: Agent = {
	identifier: { value: 'agent-1' },
	name: 'Test Agent',
	model: { modelId: 'gpt-4' },
	systemPrompt: 'You are a helpful assistant.',
	isActive: true,
	temperature: 70,
	createdAt: '2025-01-01T00:00:00Z',
	createdByUserId: 'user-1',
} as unknown as Agent;

describe('AgentCard', () => {
	describe('action button visibility', () => {
		it('should show edit button when onEdit is provided', () => {
			render(<AgentCard agent={mockAgent} onEdit={vi.fn()} />);

			expect(screen.getByTitle('Edit agent')).toBeInTheDocument();
		});

		it('should hide edit button when onEdit is not provided', () => {
			render(<AgentCard agent={mockAgent} />);

			expect(screen.queryByTitle('Edit agent')).not.toBeInTheDocument();
		});

		it('should show delete button when onDelete is provided', () => {
			render(<AgentCard agent={mockAgent} onDelete={vi.fn()} />);

			expect(screen.getByTitle('Delete agent')).toBeInTheDocument();
		});

		it('should hide delete button when onDelete is not provided', () => {
			render(<AgentCard agent={mockAgent} />);

			expect(screen.queryByTitle('Delete agent')).not.toBeInTheDocument();
		});

		it('should show share button when onShare is provided', () => {
			render(<AgentCard agent={mockAgent} onShare={vi.fn()} />);

			expect(screen.getByTitle('Share agent')).toBeInTheDocument();
		});

		it('should hide share button when onShare is not provided', () => {
			render(<AgentCard agent={mockAgent} />);

			expect(screen.queryByTitle('Share agent')).not.toBeInTheDocument();
		});

		it('should always show chat link regardless of permissions', () => {
			render(<AgentCard agent={mockAgent} />);

			expect(screen.getByTitle('Chat with agent')).toBeInTheDocument();
		});

		it('should show all action buttons when all handlers are provided', () => {
			render(<AgentCard agent={mockAgent} onEdit={vi.fn()} onDelete={vi.fn()} onShare={vi.fn()} />);

			expect(screen.getByTitle('Chat with agent')).toBeInTheDocument();
			expect(screen.getByTitle('Share agent')).toBeInTheDocument();
			expect(screen.getByTitle('Edit agent')).toBeInTheDocument();
			expect(screen.getByTitle('Delete agent')).toBeInTheDocument();
		});
	});

	describe('action button callbacks', () => {
		it('should call onEdit with agent when edit button is clicked', async () => {
			const onEdit = vi.fn();
			render(<AgentCard agent={mockAgent} onEdit={onEdit} />);

			const user = userEvent.setup();
			await user.click(screen.getByTitle('Edit agent'));

			expect(onEdit).toHaveBeenCalledWith(mockAgent);
		});

		it('should call onDelete with agent when delete button is clicked', async () => {
			const onDelete = vi.fn();
			render(<AgentCard agent={mockAgent} onDelete={onDelete} />);

			const user = userEvent.setup();
			await user.click(screen.getByTitle('Delete agent'));

			expect(onDelete).toHaveBeenCalledWith(mockAgent);
		});

		it('should call onShare with agent when share button is clicked', async () => {
			const onShare = vi.fn();
			render(<AgentCard agent={mockAgent} onShare={onShare} />);

			const user = userEvent.setup();
			await user.click(screen.getByTitle('Share agent'));

			expect(onShare).toHaveBeenCalledWith(mockAgent);
		});
	});

	describe('agent display', () => {
		it('should display agent name and model', () => {
			render(<AgentCard agent={mockAgent} />);

			expect(screen.getByText('Test Agent')).toBeInTheDocument();
			expect(screen.getByText('gpt-4')).toBeInTheDocument();
		});

		it('should show active badge when agent is active', () => {
			render(<AgentCard agent={mockAgent} />);

			expect(screen.getByText('Active')).toBeInTheDocument();
		});
	});
});
