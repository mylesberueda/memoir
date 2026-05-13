import { LayoutProvider } from '@providers';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, userEvent, within } from 'storybook/test';
import Header from './Header';

const meta: Meta<typeof Header> = {
	title: 'App/Header',
	component: Header,
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
				<div className="min-h-16">
					<Story />
				</div>
			</LayoutProvider>
		),
	],
	argTypes: {
		user: {
			control: 'object',
			description: 'User data to display in the header',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

const mockUser = {
	email: 'john.doe@memoir.local',
	name: 'John Doe',
	pid: 'user-123',
};

const mockAdmin = {
	email: 'admin@memoir.local',
	name: 'Admin User',
	pid: 'admin-456',
};

// Default header with standard user
export const Default: Story = {
	args: {
		user: mockUser,
	},
};

// Header with admin user
export const AdminUser: Story = {
	args: {
		user: mockAdmin,
	},
};

// Header with long user name
export const LongUserName: Story = {
	args: {
		user: {
			...mockUser,
			name: 'John Alexander Maximilian Doe',
			email: 'john.alexander.maximilian.doe@verylongdomainname.memoir.local',
		},
	},
};

// Interactive test - dropdown functionality
export const InteractiveDropdown: Story = {
	args: {
		user: mockUser,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Find and click the avatar to open dropdown
		const avatar = canvas.getByRole('button');
		await userEvent.click(avatar);

		// Verify dropdown content is visible
		await expect(canvas.getByText(mockUser.name)).toBeInTheDocument();
		await expect(canvas.getByText(mockUser.email)).toBeInTheDocument();
		await expect(canvas.getByText('Founder')).toBeInTheDocument();

		// Verify dropdown menu items
		await expect(canvas.getByText('Subscription')).toBeInTheDocument();
		await expect(canvas.getByText('Settings')).toBeInTheDocument();
		await expect(canvas.getByText('Terms & Policies')).toBeInTheDocument();
		await expect(canvas.getByText('Logout')).toBeInTheDocument();
	},
};

// Test breadcrumb display
export const BreadcrumbDisplay: Story = {
	args: {
		user: mockUser,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify breadcrumb elements are present
		await expect(canvas.getByText('memoir')).toBeInTheDocument();
		await expect(canvas.getByText('dashboard')).toBeInTheDocument();
	},
};

// Test notification bell
export const NotificationBell: Story = {
	args: {
		user: mockUser,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Find and click the notification bell
		const bellButton = canvas.getByRole('button', { name: /notification/i });
		await userEvent.click(bellButton);

		// Note: Currently no notification dropdown implemented,
		// but the button should be clickable
		await expect(bellButton).toBeInTheDocument();
	},
};
