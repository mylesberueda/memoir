'use client';

import { getOrganizationByPid } from '@actions/organizations';
import { setOrganizationContext } from '@lib/grpc/transport';
import type { Organization, ResourcePermission } from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { useRouter } from 'next/navigation';
import {
	createContext,
	type ReactNode,
	useCallback,
	useContext,
	useEffect,
	useRef,
	useState,
	useTransition,
} from 'react';

type PermissionsMap = { [key: string]: ResourcePermission };

interface OrganizationContextType {
	organizations: Organization[];
	currentOrg?: Organization;
	currentOrgPid?: string;
	setCurrentOrg: (orgPid: string) => void;
	resetToFirstOrg: () => void;
	can: (resource: string, action: 'read' | 'write' | 'execute') => boolean;
}

const OrganizationContext = createContext<OrganizationContextType | undefined>(undefined);

export { OrganizationContext };

interface OrganizationContextProviderProps {
	children: ReactNode;
	organizations: Organization[];
	initialOrgPid?: string;
	initialPermissions?: PermissionsMap;
	cookieOrgPid?: string;
}

export function OrganizationContextProvider({
	children,
	organizations,
	initialOrgPid,
	initialPermissions,
	cookieOrgPid,
}: OrganizationContextProviderProps) {
	const router = useRouter();
	const [, startTransition] = useTransition();
	const hasReconciledCookie = useRef(false);

	const [currentOrgPid, setCurrentOrgPid] = useState<string | undefined>(initialOrgPid);
	const [permissions, setPermissions] = useState<PermissionsMap | undefined>(initialPermissions);

	const currentOrg = currentOrgPid ? organizations.find((o) => o.pid === currentOrgPid) : undefined;

	useEffect(() => {
		if (hasReconciledCookie.current) return;
		hasReconciledCookie.current = true;
		if (currentOrgPid && currentOrgPid !== cookieOrgPid) {
			setOrganizationContext(currentOrgPid);
		}
	}, [currentOrgPid, cookieOrgPid]);

	const setCurrentOrg = useCallback(
		(orgPid: string) => {
			if (orgPid === currentOrgPid) return;

			startTransition(async () => {
				const res = await getOrganizationByPid(orgPid);
				if (!res.success) return;

				await setOrganizationContext(orgPid);
				setPermissions(res.data.permissions);
				setCurrentOrgPid(orgPid);
				router.push('/dashboard');
			});
		},
		[currentOrgPid, router],
	);

	const resetToFirstOrg = useCallback(() => {
		const firstOrgPid = organizations.at(0)?.pid;
		if (firstOrgPid) {
			setCurrentOrg(firstOrgPid);
		}
	}, [organizations, setCurrentOrg]);

	const can = useCallback(
		(resource: string, action: 'read' | 'write' | 'execute'): boolean => {
			if (!permissions) return false;
			const perm = permissions[resource];
			if (!perm) return false;
			return perm[action] ?? false;
		},
		[permissions],
	);

	return (
		<OrganizationContext.Provider
			value={{
				organizations,
				currentOrg,
				currentOrgPid,
				setCurrentOrg,
				resetToFirstOrg,
				can,
			}}>
			{children}
		</OrganizationContext.Provider>
	);
}

export function useOrganizations() {
	const context = useContext(OrganizationContext);
	if (context === undefined) {
		throw new Error('useOrganizations must be used within an OrganizationContextProvider');
	}
	return context;
}

export function useOrganizationsOptional() {
	return useContext(OrganizationContext);
}
