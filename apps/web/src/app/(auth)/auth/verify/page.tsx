'use client';

import { verifyEmail } from '@actions/auth';
import { AlertTriangle, CheckCircle } from 'lucide-react';
import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense, useEffect, useState } from 'react';
import { Card } from 'rsc-daisyui';

function VerifyEmailContent() {
	const router = useRouter();
	const searchParams = useSearchParams();
	const token = searchParams.get('token');

	const [verificationState, setVerificationState] = useState<'loading' | 'success' | 'error'>('loading');
	const [errorMessage, setErrorMessage] = useState<string>('');

	useEffect(() => {
		if (!token) {
			setVerificationState('error');
			setErrorMessage('No verification token provided. Please check your email for the verification link.');
			return;
		}

		const performVerification = async () => {
			try {
				const result = await verifyEmail(token);

				if (result.success) {
					setVerificationState('success');
					// Redirect to login after 3 seconds
					setTimeout(() => {
						router.push('/auth/login?verified=true');
					}, 3000);
				} else {
					setVerificationState('error');
					setErrorMessage(result.error || 'Email verification failed. Please try again.');
				}
			} catch (_) {
				setVerificationState('error');
				setErrorMessage('An unexpected error occurred during verification. Please try again later.');
			}
		};

		performVerification();
	}, [token, router]);

	return (
		<div className="flex items-center justify-center h-full">
			<Card className="w-full max-w-md p-8 text-base-content">
				<div className="mb-8 text-center">
					<h1 className="mt-6 text-2xl font-bold">Email Verification</h1>
				</div>

				{verificationState === 'loading' && (
					<div className="text-center">
						<div className="mb-4">
							<div className="inline-flex h-12 w-12 animate-spin rounded-full border-4 border-solid border-blue-600 border-r-transparent" />
						</div>
						<p className="">Verifying your email address...</p>
					</div>
				)}

				{verificationState === 'success' && (
					<div className="text-center">
						<div className="mb-4 flex justify-center">
							<div className="flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
								<CheckCircle className="h-6 w-6 text-green-600" />
							</div>
						</div>
						<h2 className="mb-2 text-lg font-semibold">Email Verified Successfully</h2>
						<p className="mb-6">
							Your email has been successfully verified. You will be redirected to the login page shortly.
						</p>
						<Link href="/auth/login?verified=true" className="btn btn-primary w-full">
							Continue to Login
						</Link>
					</div>
				)}

				{verificationState === 'error' && (
					<div className="text-center">
						<div className="mb-4 flex justify-center">
							<div className="flex h-12 w-12 items-center justify-center rounded-full bg-red-100">
								<AlertTriangle className="h-6 w-6 text-red-600" />
							</div>
						</div>
						<h2 className="mb-2 text-lg font-semibold">Verification Failed</h2>
						<p className="mb-6">{errorMessage}</p>
						<div className="space-y-3">
							<Link href="/auth/login" className="btn btn-primary w-full">
								Go to Login
							</Link>
							<Link href="/auth/registration" className="btn btn-outline w-full">
								Create New Account
							</Link>
						</div>
					</div>
				)}
			</Card>
		</div>
	);
}

export default function VerifyEmailPage() {
	return (
		<Suspense
			fallback={
				<div className="flex min-h-screen items-center justify-center bg-gradient-to-br from-gray-50 to-gray-100">
					<Card className="w-full max-w-md p-8">
						<div className="text-center">
							<div className="mb-4">
								<div className="inline-flex h-12 w-12 animate-spin rounded-full border-4 border-solid border-blue-600 border-r-transparent" />
							</div>
							<p className="">Loading...</p>
						</div>
					</Card>
				</div>
			}>
			<VerifyEmailContent />
		</Suspense>
	);
}
