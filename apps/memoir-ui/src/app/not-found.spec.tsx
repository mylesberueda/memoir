import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import NotFound from './not-found';

// Mock Next.js Link component
vi.mock('next/link', () => ({
	default: ({ children, href, ...props }: { children: React.ReactNode; href: string }) => (
		<a href={href} {...props}>
			{children}
		</a>
	),
}));

describe('NotFound', () => {
	it('renders 404 page with correct elements', () => {
		render(<NotFound />);

		expect(screen.getByText('404')).toBeInTheDocument();
		expect(screen.getByText('Page not found')).toBeInTheDocument();
		expect(screen.getByText('Go back home')).toBeInTheDocument();
	});

	it('renders 404 heading with correct styling', () => {
		render(<NotFound />);

		const heading = screen.getByText('404');
		expect(heading).toBeInTheDocument();
		expect(heading.tagName).toBe('H2');
		expect(heading).toHaveClass('text-4xl', 'font-bold', 'mb-4');
	});

	it('renders page not found text with correct styling', () => {
		render(<NotFound />);

		const description = screen.getByText('Page not found');
		expect(description).toBeInTheDocument();
		expect(description.tagName).toBe('P');
		expect(description).toHaveClass('text-xl', 'mb-4');
	});

	it('renders home link with correct attributes', () => {
		render(<NotFound />);

		const homeLink = screen.getByText('Go back home');
		expect(homeLink).toBeInTheDocument();
		expect(homeLink.tagName).toBe('A');
		expect(homeLink).toHaveAttribute('href', '/');
		expect(homeLink).toHaveClass('btn', 'btn-primary');
	});

	it('applies correct container CSS classes', () => {
		render(<NotFound />);

		const container = screen.getByText('404').closest('div');
		expect(container).toHaveClass('text-center');

		const outerContainer = screen.getByText('404').closest('div')?.parentElement;
		expect(outerContainer).toHaveClass('flex', 'min-h-screen', 'items-center', 'justify-center');
	});

	it('has correct DOM structure', () => {
		const { container } = render(<NotFound />);

		const outerDiv = container.querySelector('div.flex.min-h-screen.items-center.justify-center');
		const innerDiv = outerDiv?.querySelector('div.text-center');
		const h2 = innerDiv?.querySelector('h2');
		const p = innerDiv?.querySelector('p');
		const a = innerDiv?.querySelector('a');

		expect(outerDiv).toBeInTheDocument();
		expect(innerDiv).toBeInTheDocument();
		expect(h2).toBeInTheDocument();
		expect(p).toBeInTheDocument();
		expect(a).toBeInTheDocument();
	});

	it('matches snapshot', () => {
		const { container } = render(<NotFound />);
		expect(container.firstChild).toMatchSnapshot();
	});
});
