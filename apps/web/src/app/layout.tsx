import { ToastContainer } from '@components/Toast';
import type { Metadata } from 'next';
import './global.css';

export const metadata: Metadata = {
	title: {
		template: '%s | Memoir',
		default: 'Memoir',
	},
	description: 'A self-hosted memory service for AI agents',
};

interface RootLayoutProps {
	children: React.ReactNode;
}

export default function RootLayout({ children }: RootLayoutProps) {
	return (
		<html lang="en" data-theme="dark">
			<body className="flex h-screen">
				{children}
				<ToastContainer />
			</body>
		</html>
	);
}
