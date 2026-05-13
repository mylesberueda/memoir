import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import FormInput, { type FormInputProps } from './FormInput';

const mockRegister = {
	name: 'test-field',
	onChange: vi.fn(),
	onBlur: vi.fn(),
	ref: vi.fn(),
};

const defaultProps: FormInputProps = {
	label: 'Test Label',
	type: 'text',
	name: 'test-field',
	placeholder: 'Test placeholder',
	register: mockRegister,
};

describe('FormInput', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('renders with required props', () => {
		render(<FormInput {...defaultProps} />);

		expect(screen.getByLabelText('Test Label')).toBeInTheDocument();
		expect(screen.getByPlaceholderText('Test placeholder')).toBeInTheDocument();

		const input = screen.getByRole('textbox');
		expect(input).toHaveAttribute('type', 'text');
		expect(input).toHaveAttribute('id', 'test-field');
		expect(input).toHaveClass('input-bordered', 'w-full');
	});

	it('renders password input type correctly', () => {
		render(<FormInput {...defaultProps} type="password" label="Password" />);

		const input = screen.getByLabelText('Password');
		expect(input).toHaveAttribute('type', 'password');
	});

	it('renders email input type correctly', () => {
		render(<FormInput {...defaultProps} type="email" label="Email" />);

		const input = screen.getByLabelText('Email');
		expect(input).toHaveAttribute('type', 'email');
	});

	it('displays error message when error is provided', () => {
		const error = {
			type: 'required',
			message: 'This field is required',
		};

		render(<FormInput {...defaultProps} error={error} />);

		expect(screen.getByText('This field is required')).toBeInTheDocument();

		const errorMessage = screen.getByText('This field is required');
		expect(errorMessage).toHaveClass('label', 'text-error');
	});

	it('applies error styling when error is present', () => {
		const error = {
			type: 'required',
			message: 'This field is required',
		};

		render(<FormInput {...defaultProps} error={error} />);

		const input = screen.getByRole('textbox');
		expect(input).toHaveClass('input-error');
	});

	it('does not show error message when no error is provided', () => {
		render(<FormInput {...defaultProps} />);

		expect(screen.queryByText(/required/i)).not.toBeInTheDocument();

		const input = screen.getByRole('textbox');
		expect(input).not.toHaveClass('input-error');
	});

	it('passes register props to input', () => {
		render(<FormInput {...defaultProps} />);

		const input = screen.getByRole('textbox');

		// Verify the register object is spread onto the input
		// We can't directly test the spread, but we can verify the name attribute
		expect(input).toHaveAttribute('name', mockRegister.name);
	});

	it('renders custom placeholder text', () => {
		render(<FormInput {...defaultProps} placeholder="Enter your custom text here" />);

		expect(screen.getByPlaceholderText('Enter your custom text here')).toBeInTheDocument();
	});

	it('renders custom label text', () => {
		render(<FormInput {...defaultProps} label="Custom Label Text" />);

		expect(screen.getByText('Custom Label Text')).toBeInTheDocument();
		const label = screen.getByText('Custom Label Text');
		expect(label.tagName).toBe('LABEL');
		expect(label).toHaveClass('fieldset-legend');
	});

	it('has correct display name', () => {
		expect(FormInput.displayName).toBe('FormInput');
	});

	it('renders with validation error message', () => {
		const validationError = {
			type: 'pattern',
			message: 'Invalid email format',
		};

		render(<FormInput {...defaultProps} error={validationError} />);

		expect(screen.getByText('Invalid email format')).toBeInTheDocument();
	});

	it('renders with minLength validation error', () => {
		const minLengthError = {
			type: 'minLength',
			message: 'Password must be at least 8 characters',
		};

		render(<FormInput {...defaultProps} error={minLengthError} />);

		expect(screen.getByText('Password must be at least 8 characters')).toBeInTheDocument();
	});

	it('matches snapshot', () => {
		const { container } = render(<FormInput {...defaultProps} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with error', () => {
		const error = {
			type: 'required',
			message: 'This field is required',
		};

		const { container } = render(<FormInput {...defaultProps} error={error} />);
		expect(container.firstChild).toMatchSnapshot();
	});
});
