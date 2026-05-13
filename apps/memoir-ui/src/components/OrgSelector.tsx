'use client';

import type { Organization } from '@polypixel/proto-ts/api-service/api/v1/organizations_pb';
import { Building2, Check, Plus } from 'lucide-react';
import Link from 'next/link';
import { Dropdown } from 'rsc-daisyui';

interface OrgSelectorProps {
	organizations: Organization[];
	currentOrgPid?: string;
	onOrgChange?: (pid: string) => void;
}

export default function OrgSelector({ organizations, currentOrgPid, onOrgChange }: OrgSelectorProps) {
	const currentOrg = currentOrgPid ? organizations.find((org) => org.pid === currentOrgPid) : undefined;

	return (
		<>
			<Dropdown.Popover
				className="w-96 border border-base-300 bg-base-100 shadow-md"
				id="org-selector-popover"
				style={
					{
						positionAnchor: '--org-selector-anchor',
						positionArea: 'block-end span-inline-end',
					} as React.CSSProperties
				}>
				<div className="px-4 py-2 border-b border-base-300">
					<span className="text-xs font-semibold text-base-content/70 uppercase">Context</span>
				</div>
				<div className="max-h-64 overflow-y-auto">
					{organizations.map((org) => {
						const isSelected = currentOrgPid === org.pid;
						return (
							<button
								key={org.pid}
								type="button"
								onClick={() => onOrgChange?.(org.pid)}
								className="flex items-center justify-between gap-2 w-full px-4 py-2 hover:bg-base-200 transition-colors">
								<div className="flex items-center gap-2">
									<Building2 className="h-4 w-4 text-base-content/60" />
									<div className="flex flex-col items-start">
										<span className="text-sm font-medium">{org.name}</span>
										<span className="text-xs text-base-content/60">@{org.slug}</span>
									</div>
								</div>
								{isSelected && <Check className="h-4 w-4 text-primary" />}
							</button>
						);
					})}
				</div>
				<div className="border-t border-base-300">
					<Link href="/organizations/new" className="block w-full">
						<button
							type="button"
							className="flex items-center gap-2 w-full px-4 py-2 hover:bg-base-200 transition-colors">
							<Plus className="h-4 w-4" />
							<span className="text-sm font-medium">Create Organization</span>
						</button>
					</Link>
				</div>
			</Dropdown.Popover>
			<button
				type="button"
				popoverTarget="org-selector-popover"
				className="flex items-center gap-2 rounded-lg px-3 py-2 transition-colors hover:bg-base-100 cursor-pointer"
				style={{ anchorName: '--org-selector-anchor' } as React.CSSProperties}>
				<Building2 className="h-4 w-4 text-base-content" />
				<span className="text-sm font-medium text-base-content max-w-[150px] truncate">
					{currentOrg?.name ?? 'Select Organization'}
				</span>
			</button>
		</>
	);
}
