import Link from 'next/link';

export default function NotFound() {
	return (
		<div className="flex min-h-screen items-center justify-center min-w-screen">
			<div className="text-center">
				<h2 className="text-4xl font-bold mb-4">404</h2>
				<p className="text-xl mb-4">Page not found</p>
				<Link href="/" className="btn btn-primary">
					Go back home
				</Link>
			</div>
		</div>
	);
}
