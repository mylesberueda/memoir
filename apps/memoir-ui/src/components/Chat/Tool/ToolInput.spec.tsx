import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import ToolInput from './ToolInput';

describe('ToolInput', () => {
	const defaultInput = { param1: 'value1', param2: 123 };

	it('renders with required props', () => {
		render(<ToolInput input={defaultInput} />);

		expect(screen.getByText('Parameters')).toBeInTheDocument();
	});

	it('displays "Parameters" header', () => {
		render(<ToolInput input={defaultInput} />);

		const header = screen.getByText('Parameters');
		expect(header).toBeInTheDocument();
		expect(header).toHaveClass('font-medium', 'text-muted-foreground', 'text-xs', 'uppercase', 'tracking-wide');
	});

	it('renders input as formatted JSON', () => {
		const input = { query: 'test', limit: 10 };
		const { container } = render(<ToolInput input={input} />);

		const _formattedJSON = JSON.stringify(input, null, 2);
		expect(container.textContent).toContain('"query"');
		expect(container.textContent).toContain('"test"');
		expect(container.textContent).toContain('"limit"');
		expect(container.textContent).toContain('10');
	});

	it('handles simple string input', () => {
		const input = { message: 'hello world' };
		render(<ToolInput input={input} />);

		expect(screen.getByText('Parameters')).toBeInTheDocument();
	});

	it('handles complex nested objects', () => {
		const input = {
			user: {
				name: 'John',
				settings: {
					theme: 'dark',
					notifications: true,
				},
			},
			count: 5,
		};
		const { container } = render(<ToolInput input={input} />);

		expect(container.textContent).toContain('"user"');
		expect(container.textContent).toContain('"name"');
		expect(container.textContent).toContain('"settings"');
		expect(container.textContent).toContain('"theme"');
	});

	it('handles arrays in input', () => {
		const input = { items: ['item1', 'item2', 'item3'] };
		const { container } = render(<ToolInput input={input} />);

		expect(container.textContent).toContain('"items"');
		expect(container.textContent).toContain('"item1"');
		expect(container.textContent).toContain('"item2"');
	});

	it('handles empty object input', () => {
		const { container } = render(<ToolInput input={{}} />);

		expect(screen.getByText('Parameters')).toBeInTheDocument();
		// Syntax highlighter may split up {} so check container text
		expect(container.textContent).toContain('{');
		expect(container.textContent).toContain('}');
	});

	it('handles null and undefined values in input', () => {
		const input = { value1: null, value2: undefined };
		const { container } = render(<ToolInput input={input} />);

		expect(container.textContent).toContain('"value1"');
		expect(container.textContent).toContain('null');
	});

	it('applies default className', () => {
		const { container } = render(<ToolInput input={defaultInput} />);

		const inputContainer = container.querySelector('.space-y-2');
		expect(inputContainer).toBeInTheDocument();
		expect(inputContainer).toHaveClass('overflow-hidden', 'p-4');
	});

	it('applies custom className', () => {
		const { container } = render(<ToolInput input={defaultInput} className="custom-input" />);

		const inputContainer = container.querySelector('.custom-input');
		expect(inputContainer).toBeInTheDocument();
		expect(inputContainer).toHaveClass('space-y-2', 'overflow-hidden', 'p-4');
	});

	it('passes through additional props', () => {
		render(<ToolInput input={defaultInput} data-testid="tool-input" role="region" />);

		const input = screen.getByTestId('tool-input');
		expect(input).toBeInTheDocument();
		expect(input).toHaveAttribute('role', 'region');
	});

	it('renders CodeBlock component', () => {
		const { container } = render(<ToolInput input={defaultInput} />);

		// CodeBlock renders with specific classes
		const codeBlock = container.querySelector('.relative.w-full.overflow-hidden.rounded-md.border');
		expect(codeBlock).toBeInTheDocument();
	});

	it('wraps CodeBlock in muted background', () => {
		const { container } = render(<ToolInput input={defaultInput} />);

		const wrapper = container.querySelector('.rounded-md.bg-muted\\/50');
		expect(wrapper).toBeInTheDocument();
	});

	it('formats JSON with 2-space indentation', () => {
		const input = {
			level1: {
				level2: 'value',
			},
		};
		const { container } = render(<ToolInput input={input} />);

		// Check if the formatted JSON contains proper indentation (2 spaces)
		const formattedJSON = JSON.stringify(input, null, 2);
		expect(container.textContent).toContain(formattedJSON);
	});

	it('handles boolean values in input', () => {
		const input = { enabled: true, disabled: false };
		const { container } = render(<ToolInput input={input} />);

		expect(container.textContent).toContain('true');
		expect(container.textContent).toContain('false');
	});

	it('handles numeric values in input', () => {
		const input = { integer: 42, float: 3.14, negative: -10 };
		const { container } = render(<ToolInput input={input} />);

		expect(container.textContent).toContain('42');
		expect(container.textContent).toContain('3.14');
		expect(container.textContent).toContain('-10');
	});

	it('matches snapshot with simple input', () => {
		const { container } = render(<ToolInput input={{ param: 'value' }} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with complex input', () => {
		const { container } = render(
			<ToolInput
				input={{
					user: { name: 'John', age: 30 },
					items: [1, 2, 3],
					active: true,
				}}
			/>,
		);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with empty input', () => {
		const { container } = render(<ToolInput input={{}} />);
		expect(container.firstChild).toMatchSnapshot();
	});

	it('matches snapshot with custom className', () => {
		const { container } = render(<ToolInput input={defaultInput} className="custom-class" />);
		expect(container.firstChild).toMatchSnapshot();
	});
});
