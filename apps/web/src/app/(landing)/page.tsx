'use client';
import { Button } from '@components';
import Link from 'next/link';

export default function LandingPage() {
	return (
		<div className="flex items-center justify-center min-h-full p-4">
			<div className="w-full max-w-md">
				<div className="text-center mb-8">
					<h1 className="text-3xl font-bold">Welcome to startup.ai</h1>
					<p className="text-base-content/70 mt-2">Your agents, cooperating</p>
				</div>

				<div className="space-y-4">
					<div className="card bg-base-100 shadow-xl">
						<div className="card-body text-center">
							<h2 className="card-title justify-center mb-4">Sign In</h2>
							<p className="text-base-content/70 mb-4">Already have an account? Sign in to continue.</p>
							<Link href="/auth/login">
								<Button color="primary" className="btn-block">
									Sign In
								</Button>
							</Link>
						</div>
					</div>

					<div className="card bg-base-100 shadow-xl">
						<div className="card-body text-center">
							<h2 className="card-title justify-center mb-4">Create Account</h2>
							<p className="text-base-content/70 mb-4">New to startup.ai? Create your account to get started.</p>
							<Link href="/auth/registration">
								<Button color="secondary" className="btn-block">
									Create Account
								</Button>
							</Link>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}
