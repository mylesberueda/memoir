'use client';

import { Button, Card } from '@components';
import { LogIn, UserPlus } from 'lucide-react';
import Link from 'next/link';

export default function AuthPage() {
	return (
		<div className="flex items-center justify-center min-h-full p-4">
			<div className="w-full max-w-md">
				<div className="text-center mb-8">
					<h1 className="text-3xl font-bold">Welcome to Memoir</h1>
					<p className="text-base-content/70 mt-2">Your agents, cooperating</p>
				</div>

				<div className="space-y-4">
					<Card
						icon={LogIn}
						title="Sign In"
						description="Already have an account? Sign in to continue."
						action={
							<Link href="/auth/login" className="w-full">
								<Button color="primary" className="btn-block">
									Sign In
								</Button>
							</Link>
						}>
						<div />
					</Card>

					<Card
						icon={UserPlus}
						title="Create Account"
						description="New to Memoir? Create your account to get started."
						action={
							<Link href="/auth/registration" className="w-full">
								<Button color="secondary" className="btn-block">
									Create Account
								</Button>
							</Link>
						}>
						<div />
					</Card>
				</div>
			</div>
		</div>
	);
}
