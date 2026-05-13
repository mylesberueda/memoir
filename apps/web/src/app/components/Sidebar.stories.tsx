import { LayoutProvider } from '@providers';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, userEvent, within } from 'storybook/test';
import Sidebar from './Sidebar';

const meta: Meta<typeof Sidebar> = {
	title: 'App/Sidebar',
	component: Sidebar,
	parameters: {
		layout: 'fullscreen',
		nextjs: {
			appDirectory: true,
		},
	},
	tags: ['autodocs'],
	decorators: [
		(Story) => (
			<LayoutProvider>
				<div className="h-screen">
					<Story />
				</div>
			</LayoutProvider>
		),
	],
	argTypes: {
		user: {
			control: 'object',
			description: 'User data for potential future sidebar customization',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

const mockUser = {
	email: 'john.doe@startup.ai',
	name: 'John Doe',
	pid: 'user-123',
};

const mockAdmin = {
	email: 'admin@startup.ai',
	name: 'Admin User',
	pid: 'admin-456',
};

// Default sidebar
export const Default: Story = {
	args: {
		user: mockUser,
	},
};

// Sidebar with admin user (for future role-based features)
export const AdminUser: Story = {
	args: {
		user: mockAdmin,
	},
};

// Interactive test - navigation
export const NavigationInteraction: Story = {
	args: {
		user: mockUser,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify main navigation sections
		await expect(canvas.getByText('STARTUP')).toBeInTheDocument();
		await expect(canvas.getByText('.ai')).toBeInTheDocument();
		await expect(canvas.getByText('Assistant')).toBeInTheDocument();
		await expect(canvas.getByText('Dashboard')).toBeInTheDocument();

		// Verify navigation categories
		await expect(canvas.getByText('Overview')).toBeInTheDocument();
		await expect(canvas.getByText('Finance')).toBeInTheDocument();
		await expect(canvas.getByText('Team')).toBeInTheDocument();

		// Test navigation link interaction
		const assistantLink = canvas.getByRole('link', { name: /assistant/i });
		await userEvent.click(assistantLink);
	},
};

// Test with long user names (future-proofing)
export const LongUserName: Story = {
	args: {
		user: {
			...mockUser,
			name: 'John Alexander Maximilian Doe',
			email: 'john.alexander.maximilian.doe@verylongdomainname.startup.ai',
		},
	},
};

// Test sidebar sections
export const SidebarSections: Story = {
	args: {
		user: mockUser,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify all main sections exist
		await expect(canvas.getByText('Overview')).toBeInTheDocument();
		await expect(canvas.getByText('Finance')).toBeInTheDocument();
		await expect(canvas.getByText('Team')).toBeInTheDocument();

		// Verify Overview section links
		await expect(canvas.getByText('Dashboard')).toBeInTheDocument();
		await expect(canvas.getByText('Analytics')).toBeInTheDocument();
		await expect(canvas.getByText('Organization')).toBeInTheDocument();
		await expect(canvas.getByText('Projects')).toBeInTheDocument();

		// Verify Finance section links
		await expect(canvas.getByText('Transactions')).toBeInTheDocument();
		await expect(canvas.getByText('Invoices')).toBeInTheDocument();
		await expect(canvas.getByText('Payments')).toBeInTheDocument();

		// Verify Team section links
		await expect(canvas.getByText('Members')).toBeInTheDocument();
		await expect(canvas.getByText('Permissions')).toBeInTheDocument();
		await expect(canvas.getByText('Chat')).toBeInTheDocument();
		await expect(canvas.getByText('Meetings')).toBeInTheDocument();

		// Verify footer section
		await expect(canvas.getByText('Settings')).toBeInTheDocument();
		await expect(canvas.getByText('Help')).toBeInTheDocument();
	},
};
