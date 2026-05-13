'use client';

import { AssistantChatProvider } from '@lib/chat-state';
import { AssistantChatProvider as AssistantChatUIProvider, LayoutProvider } from '@providers';
import { AuthContextProvider, type User } from '@providers/AuthContextProvider';
import { OrganizationContextProvider } from '@providers/OrganizationContextProvider';
import type { Organization, ResourcePermission } from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { AssistantChat } from '../components';

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
				<LayoutProvider>
					<AssistantChatProvider>
						<AssistantChatUIProvider>
							{children}
							<AssistantChat />
						</AssistantChatUIProvider>
					</AssistantChatProvider>
				</LayoutProvider>
			</OrganizationContextProvider>
		</AuthContextProvider>
	);
}
