import LoginClient from './client';

interface LoginPageProps {
	searchParams: Promise<{ [key: string]: string | string[] | undefined }>;
}

export default async function LoginPage({ searchParams }: LoginPageProps) {
	const params = await searchParams;
	const mode = params.mode as string | undefined;
	const verified = params.verified === 'true';
	const error = params.error as string | undefined;

	return (
		<LoginClient
			initialMode={mode === 'register' ? 'register' : 'login'}
			verified={verified}
			error={error}
		/>
	);
}
