import { getCurrentUser } from '@actions/auth';
import { loadLayoutContext } from '@lib/layout-context';
import { redirect } from 'next/navigation';
import { Footer, Header, Sidebar } from '../components';
import { AppProviders } from './providers';

export const dynamic = 'force-dynamic';

const SHOW_FOOTER = false;

function requireEnv(name: string): string {
	const value = process.env[name];
	if (!value) {
		throw new Error(`Missing required environment variable: ${name}`);
	}
	return value;
}

interface AppLayoutProps {
	children: React.ReactNode;
}

export default async function AppLayout({ children }: AppLayoutProps) {
	const [rawUser, ctx] = await Promise.all([
		getCurrentUser().catch(() => null),
		loadLayoutContext(requireEnv('API_SERVICE_URL')),
	]);

	if (!rawUser) {
		redirect('/auth/login?logout=1');
	}

	const user = { id: rawUser.id, email: rawUser.email, name: rawUser.name };

	return (
		<AppProviders
			user={user}
			organizations={ctx.orgs}
			initialOrgPid={ctx.resolvedPid}
			initialPermissions={ctx.permissions}
			cookieOrgPid={ctx.cookieOrgPid}>
			<Sidebar />
			<div className="flex w-full flex-1 flex-col relative">
				<header className="h-16 min-h-16 border-base-200 border-b">
					<Header />
				</header>
				<main className="flex-1 bg-base-200 h-full overflow-auto">{children}</main>
				{SHOW_FOOTER && (
					<footer>
						<Footer />
					</footer>
				)}
			</div>
		</AppProviders>
	);
}
