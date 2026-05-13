'use client';

import { LayoutProvider } from '@providers';
import { AuthContextProvider, type User } from '@providers/AuthContextProvider';
import { OrganizationContextProvider } from '@providers/OrganizationContextProvider';
import type { Organization, ResourcePermission } from '@/lib/proto-shims';

interface AppProvidersProps {
	children: React.ReactNode;
	user?: User | null;
	organizations?: Organization[];
	initialOrgPid?: string;
	initialPermissions?: Record<string, ResourcePermission>;
	cookieOrgPid?: string;
}

export function AppProviders({
	children,
	user,
	organizations = [],
	initialOrgPid,
	initialPermissions,
	cookieOrgPid,
}: AppProvidersProps) {
	return (
		<AuthContextProvider user={user}>
			<OrganizationContextProvider
				organizations={organizations}
				initialOrgPid={initialOrgPid}
				initialPermissions={initialPermissions}
				cookieOrgPid={cookieOrgPid}>
				<LayoutProvider>{children}</LayoutProvider>
			</OrganizationContextProvider>
		</AuthContextProvider>
	);
}
