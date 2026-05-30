import { SettingsPageHeader, SettingsPageProvider } from './_components/SettingsPageContext';
import SettingsSidebar from './_components/SettingsSidebar';

interface SettingsLayoutProps {
	children: React.ReactNode;
}

export default function SettingsLayout({ children }: SettingsLayoutProps) {
	return (
		<SettingsPageProvider>
			<SettingsSidebar>
				<div className="mx-auto h-full max-w-3xl px-4 py-8 sm:px-6 lg:px-8">
					<SettingsPageHeader />
					{children}
				</div>
			</SettingsSidebar>
		</SettingsPageProvider>
	);
}
