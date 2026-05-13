import { LayoutProvider } from '@providers';
import { Header } from '../components';

interface AuthLayoutProps {
	children: React.ReactNode;
}

export default function AuthLayout({ children }: AuthLayoutProps) {
	return (
		<LayoutProvider>
			<div className="flex h-screen flex-col w-screen">
				<header className="h-16 min-h-16 border-base-200 border-b">
					<Header />
				</header>
				<main className="flex-1 bg-base-200">{children}</main>
			</div>
		</LayoutProvider>
	);
}
