import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, userEvent, waitFor, within } from 'storybook/test';
import LoginPage from './page';

const meta: Meta<typeof LoginPage> = {
	title: 'Pages/Auth/Login',
	component: LoginPage,
	parameters: {
		layout: 'fullscreen',
		docs: {
			description: {
				component: 'Login page with form validation and API integration',
			},
		},
	},
};

export default meta;
type Story = StoryObj<typeof LoginPage>;

export const Default: Story = {
	name: 'Default State',
	parameters: {
		docs: {
			description: {
				story: 'The default login form with email and password fields',
			},
		},
	},
};

export const WithValidationErrors: Story = {
	name: 'Validation Errors',
	parameters: {
		docs: {
			description: {
				story: 'Login form showing validation errors for empty required fields',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Focus and blur to trigger validation
		const emailInput = canvas.getByLabelText(/email/i);
		const passwordInput = canvas.getByLabelText(/password/i);

		await userEvent.click(emailInput);
		await userEvent.tab();
		await userEvent.click(passwordInput);
		await userEvent.tab();

		// Try to submit to trigger validation
		const submitButton = canvas.getByRole('button', { name: /sign in/i });
		await userEvent.click(submitButton);
	},
};

export const WithInvalidEmail: Story = {
	name: 'Invalid Email Validation',
	parameters: {
		docs: {
			description: {
				story: 'Login form showing email format validation error',
			},
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		const emailInput = canvas.getByLabelText(/email/i);
		await userEvent.type(emailInput, 'invalid-email');
		await userEvent.tab(); // Trigger blur validation

		await waitFor(() => {
			expect(canvas.getByText(/invalid email address/i)).toBeInTheDocument();
		});
	},
};
