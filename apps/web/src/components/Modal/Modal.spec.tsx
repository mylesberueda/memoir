import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Modal, { type ModalProps } from './Modal';

const defaultProps: ModalProps = {
	open: true,
	children: <div>Modal content</div>,
};

describe('Modal', () => {
	it('renders with required props', () => {
		render(<Modal {...defaultProps} />);

		expect(screen.getByRole('dialog', { hidden: true })).toBeInTheDocument();
		expect(screen.getByText('Modal content')).toBeInTheDocument();
	});

	it('renders when open is true', () => {
		render(<Modal {...defaultProps} open={true} />);

		const modal = screen.getByRole('dialog', { hidden: true });
		expect(modal).toBeInTheDocument();
		expect(modal).toHaveClass('modal-open');
	});

	it('does not render when open is false', () => {
		render(<Modal {...defaultProps} open={false} />);

		const modal = screen.getByRole('dialog', { hidden: true });
		expect(modal).toBeInTheDocument();
		expect(modal).not.toHaveClass('modal-open');
	});

	it('renders children content', () => {
		const customContent = <div data-testid="custom-content">Custom modal content</div>;

		render(<Modal {...defaultProps}>{customContent}</Modal>);

		expect(screen.getByTestId('custom-content')).toBeInTheDocument();
		expect(screen.getByText('Custom modal content')).toBeInTheDocument();
	});

	it('applies custom className when provided', () => {
		const customClass = 'custom-modal-class';

		render(<Modal {...defaultProps} className={customClass} />);

		const modal = screen.getByRole('dialog', { hidden: true });
		expect(modal).toHaveClass(customClass);
	});

	it('passes through additional props to underlying Modal component', () => {
		render(<Modal {...defaultProps} data-testid="modal-component" />);

		const modal = screen.getByTestId('modal-component');
		expect(modal).toBeInTheDocument();
	});

	it('forwards ref correctly', () => {
		const ref = { current: null };

		render(<Modal {...defaultProps} ref={ref} />);

		expect(ref.current).toBeInstanceOf(HTMLDialogElement);
	});

	it('renders with backdrop', () => {
		render(<Modal {...defaultProps} />);

		const modal = screen.getByRole('dialog', { hidden: true });
		expect(modal).toBeInTheDocument();
	});

	it('has correct display name', () => {
		expect(Modal.displayName).toBe('Modal');
	});

	it('matches snapshot when open', () => {
		const { container } = render(<Modal {...defaultProps} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot when closed', () => {
		const { container } = render(<Modal {...defaultProps} open={false} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with custom content', () => {
		const customContent = (
			<div className="modal-box">
				<h3 className="font-bold text-lg">Custom Title</h3>
				<p className="py-4">Custom modal content with multiple elements</p>
				<div className="modal-action">
					<button type="button" className="btn">
						Close
					</button>
				</div>
			</div>
		);

		const { container } = render(<Modal {...defaultProps}>{customContent}</Modal>);
		expect(container.firstChild).toMatchSnapshot();
	});
});
