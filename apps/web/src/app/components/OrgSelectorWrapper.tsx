'use client';

import OrgSelector from '@components/OrgSelector';
import { useOrganizationsOptional } from '@providers/OrganizationContextProvider';

export default function OrgSelectorWrapper() {
	const context = useOrganizationsOptional();

	if (!context) {
		return null;
	}

	return (
		<OrgSelector
			organizations={context.organizations}
			currentOrgPid={context.currentOrgPid}
			onOrgChange={context.setCurrentOrg}
		/>
	);
}
