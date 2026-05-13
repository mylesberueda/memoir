'use client';

import { useEffect } from 'react';

export default function ErrorBoundary({ error, reset }: { error: Error & { digest?: string }; reset: () => void }) {
	useEffect(() => {
		console.error(error);
	}, [error]);

	return (
		<div className="flex min-h-screen min-w-screen items-center justify-center">
			<div className="text-center">
				<h2 className="text-2xl font-bold mb-4">Something went wrong!</h2>
				<button type="button" className="btn btn-primary" onClick={() => reset()}>
					Try again
				</button>
			</div>
		</div>
	);
}
