'use client';

import { getOrganizationByPid } from '@actions/organizations';
import { useOrganizations } from '@providers/OrganizationContextProvider';
import { Building2 } from 'lucide-react';
import { useEffect, useState } from 'react';
import type { Organization } from '@/lib/proto-shims';
import OrganizationDetails from '../../settings/organization/_components/OrganizationDetails';

export default function OrgDetailsClient() {
	const { currentOrgPid } = useOrganizations();
	const [organization, setOrganization] = useState<Organization | null>(null);
	const [userRole, setUserRole] = useState<string>('');
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		if (!currentOrgPid) {
			setOrganization(null);
			setUserRole('');
			setError(null);
			return;
		}

		let cancelled = false;

		async function fetchOrg(pid: string) {
			setIsLoading(true);
			setError(null);

			try {
				const res = await getOrganizationByPid(pid);
				if (cancelled) return;

				if (!res.success) {
					setError(res.error);
					setOrganization(null);
				} else if (!res.data.organization) {
					setError('Organization not found');
					setOrganization(null);
				} else {
					setOrganization(res.data.organization);
					setUserRole(res.data.userRole ?? '');
				}
			} catch (err) {
				if (cancelled) return;
				setError(err instanceof Error ? err.message : 'Failed to load organization');
				setOrganization(null);
			} finally {
				if (!cancelled) {
					setIsLoading(false);
				}
			}
		}

		fetchOrg(currentOrgPid);

		return () => {
			cancelled = true;
		};
	}, [currentOrgPid]);

	if (isLoading) {
		return (
			<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
				<div className="mb-8">
					<div className="h-9 w-64 bg-base-300 rounded animate-pulse" />
					<div className="h-5 w-96 bg-base-300 rounded animate-pulse mt-2" />
				</div>
				<div className="h-64 bg-base-300 rounded animate-pulse" />
			</div>
		);
	}

	if (error) {
		return (
			<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
				<div className="alert alert-error">
					<span>Failed to load organization. {error}</span>
				</div>
			</div>
		);
	}

	if (!organization) {
		return (
			<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
				<div className="flex flex-col items-center justify-center py-16 text-center">
					<Building2 className="h-16 w-16 text-base-content/30 mb-4" />
					<h2 className="text-xl font-semibold text-base-content mb-2">No Organization Selected</h2>
					<p className="text-base-content/70 max-w-md">Select an organization from the sidebar to view its details.</p>
				</div>
			</div>
		);
	}

	const isOwner = userRole === 'owner';
	const isAdmin = isOwner || userRole === 'admin';

	return (
		<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div className="mb-8">
				<h1 className="text-3xl font-bold text-base-content">Organization Details</h1>
				<p className="mt-2 text-base-content/70">View and manage your organization settings.</p>
			</div>
			<OrganizationDetails organization={organization} isAdmin={isAdmin} isOwner={isOwner} />
		</div>
	);
}
