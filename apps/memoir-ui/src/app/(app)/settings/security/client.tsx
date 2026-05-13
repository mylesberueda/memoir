'use client';

import { useEffect } from 'react';
import PasswordChangeForm from '../_components/PasswordChangeForm';
import { useSettingsPage } from '../_components/SettingsPageContext';

export default function SecurityClient() {
	const { setHeaderConfig } = useSettingsPage();

	useEffect(() => {
		setHeaderConfig({
			title: 'Security',
			description: 'Manage your account security settings.',
		});

		return () => setHeaderConfig(null);
	}, [setHeaderConfig]);

	return <PasswordChangeForm />;
}
