import { SettingsPageHeader, SettingsPageProvider } from './_components/SettingsPageContext';
import SettingsSidebar from './_components/SettingsSidebar';

interface SettingsLayoutProps {
	children: React.ReactNode;
}

export default function SettingsLayout({ children }: SettingsLayoutProps) {
	return (
		<SettingsPageProvider>
			<SettingsSidebar>
				<div className="h-full px-4 py-6 sm:px-6 lg:px-8">
					<SettingsPageHeader />
					{children}
				</div>
			</SettingsSidebar>
		</SettingsPageProvider>
	);
}
