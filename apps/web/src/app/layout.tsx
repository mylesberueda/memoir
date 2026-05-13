import { ToastContainer } from '@components/Toast';
import type { Metadata } from 'next';
import './global.css';

export const metadata: Metadata = {
	title: {
		template: '%s | Startup AI',
		default: 'Startup AI',
	},
	description: 'Your next startup team',
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
