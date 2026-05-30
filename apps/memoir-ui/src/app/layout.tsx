import { ToastContainer } from '@components/Toast';
import type { Metadata } from 'next';
import { Fraunces, Hanken_Grotesk, JetBrains_Mono } from 'next/font/google';
import './global.css';

const displaySerif = Fraunces({
	subsets: ['latin'],
	weight: ['400', '500', '600', '700', '900'],
	style: ['normal', 'italic'],
	variable: '--font-display',
	display: 'swap',
});

const bodyGrotesque = Hanken_Grotesk({
	subsets: ['latin'],
	weight: ['400', '500', '600', '700'],
	variable: '--font-body',
	display: 'swap',
});

const mono = JetBrains_Mono({
	subsets: ['latin'],
	weight: ['400', '500', '600'],
	variable: '--font-mono',
	display: 'swap',
});

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
		<html
			lang="en"
			data-theme="memoir-dark"
			className={`${displaySerif.variable} ${bodyGrotesque.variable} ${mono.variable}`}>
			<body className="flex h-screen">
				{children}
				<ToastContainer />
			</body>
		</html>
	);
}
