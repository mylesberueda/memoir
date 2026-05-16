'use client';

import ResponsiveDock, { type DockLinkItem } from '@components/ResponsiveDock';
import { Cable, Cpu, Shield } from 'lucide-react';
import { usePathname } from 'next/navigation';
import type { ReactNode } from 'react';

const NAV_ITEMS = [
	{ id: 'security', href: '/settings/security', icon: Shield, label: 'Security' },
	{ id: 'providers', href: '/settings/providers', icon: Cable, label: 'Providers' },
	{ id: 'models', href: '/settings/models', icon: Cpu, label: 'Models' },
] as const;

interface SettingsSidebarProps {
	children: ReactNode;
}

export default function SettingsSidebar({ children }: SettingsSidebarProps) {
	const pathname = usePathname();

	const links: DockLinkItem[] = NAV_ITEMS.map((item) => ({
		id: item.id,
		icon: item.icon,
		label: item.label,
		href: item.href,
		isActive: pathname === item.href,
	}));

	return (
		<ResponsiveDock title="Settings" links={links}>
			{children}
		</ResponsiveDock>
	);
}
