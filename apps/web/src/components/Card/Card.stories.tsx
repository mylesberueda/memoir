import { Button } from '@components';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { FiHome, FiSettings, FiUser } from 'react-icons/fi';
import Card from './Card';

const meta: Meta<typeof Card> = {
	title: 'Components/Card',
	component: Card,
	parameters: {
		layout: 'centered',
	},
	tags: ['autodocs'],
	argTypes: {
		icon: {
			control: false,
			description: 'Icon component to display in the title',
		},
		title: {
			control: 'text',
			description: 'The title of the card',
		},
		description: {
			control: 'text',
			description: 'The description text below the title',
		},
		children: {
			control: false,
			description: 'The main content of the card',
		},
		action: {
			control: false,
			description: 'Action buttons or elements to display at the bottom',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic card with just title and description
export const Default: Story = {
	args: {
		title: 'Default Card',
		description: 'This is a basic card with title and description',
		children: (
			<div className="space-y-2">
				<p>This is the main content of the card.</p>
				<p>You can put any React components here.</p>
			</div>
		),
	},
};

// Card with icon
export const WithIcon: Story = {
	args: {
		icon: FiHome,
		title: 'Home Dashboard',
		description: 'Welcome to your personal dashboard',
		children: (
			<div className="space-y-3">
				<div className="stats shadow">
					<div className="stat">
						<div className="stat-title">Total Users</div>
						<div className="stat-value">1,234</div>
					</div>
				</div>
			</div>
		),
	},
};

// Card with action buttons
export const WithActions: Story = {
	args: {
		icon: FiUser,
		title: 'User Profile',
		description: 'Manage your account settings and preferences',
		children: (
			<div className="space-y-2">
				<div className="form-control">
					{/* biome-ignore lint/a11y/noLabelWithoutControl: This is a demo component for Storybook */}
					<label className="label">
						<span className="label-text">Username</span>
					</label>
					<input
						type="text"
						placeholder="Enter username"
						className="input input-bordered w-full"
						defaultValue="john.doe"
					/>
				</div>
				<div className="form-control">
					{/* biome-ignore lint/a11y/noLabelWithoutControl: This is a demo component for Storybook */}
					<label className="label">
						<span className="label-text">Email</span>
					</label>
					<input
						type="email"
						placeholder="Enter email"
						className="input input-bordered w-full"
						defaultValue="john.doe@example.com"
					/>
				</div>
			</div>
		),
		action: (
			<div className="space-x-2">
				<Button color="primary" size="sm">
					Save Changes
				</Button>
				<Button outline size="sm">
					Cancel
				</Button>
			</div>
		),
	},
};

// Card with settings content
export const Settings: Story = {
	args: {
		icon: FiSettings,
		title: 'Application Settings',
		description: 'Configure your application preferences',
		children: (
			<div className="space-y-4">
				<div className="form-control">
					<label className="cursor-pointer label">
						<span className="label-text">Enable notifications</span>
						<input type="checkbox" className="toggle toggle-primary" defaultChecked />
					</label>
				</div>
				<div className="form-control">
					<label className="cursor-pointer label">
						<span className="label-text">Dark mode</span>
						<input type="checkbox" className="toggle toggle-secondary" />
					</label>
				</div>
				<div className="form-control">
					{/* biome-ignore lint/a11y/noLabelWithoutControl: This is a demo component for Storybook */}
					<label className="label">
						<span className="label-text">Language</span>
					</label>
					<select className="select select-bordered w-full">
						<option>English</option>
						<option>Spanish</option>
						<option>French</option>
					</select>
				</div>
			</div>
		),
		action: (
			<Button color="accent" size="sm" className="w-full">
				Apply Settings
			</Button>
		),
	},
};

// Minimal card
export const Minimal: Story = {
	args: {
		title: 'Simple Card',
		description: 'A minimal card example',
		children: <p>Just some simple text content.</p>,
	},
};
