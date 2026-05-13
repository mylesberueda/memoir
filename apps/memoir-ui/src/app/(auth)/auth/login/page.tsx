import { redirect } from 'next/navigation';
import LoginClient from './client';

interface LoginPageProps {
	searchParams: Promise<{ [key: string]: string | string[] | undefined }>;
}

export default async function LoginPage({ searchParams }: LoginPageProps) {
	const params = await searchParams;
	const authRequest = params.authRequest as string | undefined;
	const mode = params.mode as string | undefined;
	const verified = params.verified === 'true';
	const error = params.error as string | undefined;

	// If no authRequest and not in registration mode, redirect to route handler
	// that starts OIDC flow (route handlers can set cookies, server components cannot)
	if (!authRequest && mode !== 'register') {
		redirect('/api/auth/start');
	}

	return (
		<LoginClient
			authRequest={authRequest}
			initialMode={mode === 'register' ? 'register' : 'login'}
			verified={verified}
			error={error}
		/>
	);
}
