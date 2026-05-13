'use client';

import { createContext, type ReactNode, useCallback, useContext, useState } from 'react';

interface SettingsPageHeaderConfig {
	title: string;
	description: string;
	actions?: ReactNode;
}

interface SettingsPageContextValue {
	headerConfig: SettingsPageHeaderConfig | null;
	setHeaderConfig: (config: SettingsPageHeaderConfig | null) => void;
}

const SettingsPageContext = createContext<SettingsPageContextValue | null>(null);

export function SettingsPageProvider({ children }: { children: ReactNode }) {
	const [headerConfig, setHeaderConfigState] = useState<SettingsPageHeaderConfig | null>(null);

	const setHeaderConfig = useCallback((config: SettingsPageHeaderConfig | null) => {
		setHeaderConfigState(config);
	}, []);

	return (
		<SettingsPageContext.Provider value={{ headerConfig, setHeaderConfig }}>{children}</SettingsPageContext.Provider>
	);
}

export function useSettingsPage() {
	const context = useContext(SettingsPageContext);
	if (!context) {
		throw new Error('useSettingsPage must be used within a SettingsPageProvider');
	}
	return context;
}

export function SettingsPageHeader() {
	const { headerConfig } = useSettingsPage();

	if (!headerConfig) {
		return null;
	}

	return (
		<div className="mb-8 flex items-center justify-between">
			<div>
				<h1 className="text-3xl font-bold text-base-content">{headerConfig.title}</h1>
				<p className="mt-2 text-base-content/70">{headerConfig.description}</p>
			</div>
			{headerConfig.actions && <div>{headerConfig.actions}</div>}
		</div>
	);
}
