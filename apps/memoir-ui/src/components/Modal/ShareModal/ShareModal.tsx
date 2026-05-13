'use client';

import {
	listAgentShares,
	listConversationShares,
	shareAgent,
	shareConversation,
	unshareAgent,
	unshareConversation,
} from '@actions/sharing';
import { Modal } from '@components';
import useAuth from '@hooks/useAuth';
import type { OrganizationMember } from '@polypixel/memoir-sdk/api-service/api/v1/organizations_pb';
import type { AgentShare } from '@polypixel/memoir-sdk/rig-service/rig/v1/agent_pb';
import type { ConversationShare } from '@polypixel/memoir-sdk/rig-service/rig/v1/inference_pb';
import { Loader2, Share2, Trash2, UserPlus, X } from 'lucide-react';
import { useCallback, useEffect, useState, useTransition } from 'react';
import PermissionSelector, { PERM_EXECUTE, PERM_READ, PermissionBadges } from './PermissionSelector';

type ShareRecord = AgentShare | ConversationShare;

type ResourceType = 'agent' | 'conversation';

interface ShareModalProps {
	isOpen: boolean;
	onClose: () => void;
	resourceType: ResourceType;
	resourcePid: string;
	ownerUserId: string;
	members: OrganizationMember[];
}

const DEFAULT_PERMS: Record<ResourceType, number> = {
	agent: PERM_READ | PERM_EXECUTE,
	conversation: PERM_READ,
};

function formatUserDisplay(email: string, displayName?: string | null): string {
	const name = displayName?.trim();
	if (name) {
		return `${email} (${name})`;
	}
	return email;
}

export default function ShareModal({
	isOpen,
	onClose,
	resourceType,
	resourcePid,
	ownerUserId,
	members,
}: ShareModalProps) {
	const { user } = useAuth();
	const [shares, setShares] = useState<ShareRecord[]>([]);
	const [isLoadingShares, setIsLoadingShares] = useState(false);
	const [selectedUserId, setSelectedUserId] = useState('');
	const [newPermissions, setNewPermissions] = useState(DEFAULT_PERMS[resourceType]);
	const [error, setError] = useState<string | null>(null);
	const [isSharing, startSharingTransition] = useTransition();
	const [isRemoving, startRemovingTransition] = useTransition();

	const loadShares = useCallback(async () => {
		setIsLoadingShares(true);
		setError(null);
		const result =
			resourceType === 'agent' ? await listAgentShares(resourcePid) : await listConversationShares(resourcePid);

		if (result.success) {
			setShares(result.data.shares);
		} else {
			setError(result.error);
		}
		setIsLoadingShares(false);
	}, [resourceType, resourcePid]);

	useEffect(() => {
		if (isOpen) {
			loadShares();
			setSelectedUserId('');
			setNewPermissions(DEFAULT_PERMS[resourceType]);
			setError(null);
		}
	}, [isOpen, loadShares, resourceType]);

	const sharedUserIds = new Set(shares.map((s) => s.userId));
	const availableMembers = members.filter((m) => m.userId !== ownerUserId && !sharedUserIds.has(m.userId));

	// Resolve owner display from current user context
	const ownerEmail = user && user.id === ownerUserId ? user.email : ownerUserId;
	const ownerName = user && user.id === ownerUserId ? user.name : undefined;

	const handleShare = () => {
		if (!selectedUserId || newPermissions === 0) return;

		startSharingTransition(async () => {
			setError(null);
			const result =
				resourceType === 'agent'
					? await shareAgent(resourcePid, selectedUserId, newPermissions)
					: await shareConversation(resourcePid, selectedUserId, newPermissions);

			if (result.success) {
				setSelectedUserId('');
				setNewPermissions(DEFAULT_PERMS[resourceType]);
				await loadShares();
			} else {
				setError(result.error);
			}
		});
	};

	const handleUnshare = (userId: string) => {
		startRemovingTransition(async () => {
			setError(null);
			const result =
				resourceType === 'agent'
					? await unshareAgent(resourcePid, userId)
					: await unshareConversation(resourcePid, userId);

			if (result.success) {
				await loadShares();
			} else {
				setError(result.error);
			}
		});
	};

	const resourceLabel = resourceType === 'agent' ? 'Agent' : 'Conversation';

	return (
		<Modal open={isOpen}>
			<div id="share_modal__container" className="modal-box max-w-lg">
				<div id="share_modal__header" className="flex items-center justify-between mb-4">
					<div className="flex items-center gap-2">
						<Share2 className="h-5 w-5" />
						<h3 className="font-bold text-lg">Share {resourceLabel}</h3>
					</div>
					<button type="button" className="btn btn-ghost btn-sm btn-circle" onClick={onClose}>
						<X className="h-4 w-4" />
					</button>
				</div>

				{error && (
					<div className="alert alert-error mb-4">
						<span className="text-sm">{error}</span>
					</div>
				)}

				<div id="share_modal__add_member" className="mb-6">
					<label htmlFor="share-member-select" className="label">
						<span className="label-text font-medium">Add member</span>
					</label>
					<div className="flex items-center gap-2">
						<select
							id="share-member-select"
							className="select select-bordered select-sm flex-1 min-w-0"
							value={selectedUserId}
							onChange={(e) => setSelectedUserId(e.target.value)}
							disabled={availableMembers.length === 0 || isSharing}>
							<option value="">{availableMembers.length === 0 ? 'No members available' : 'Select a member...'}</option>
							{availableMembers.map((m) => (
								<option key={m.userId} value={m.userId}>
									{formatUserDisplay(m.email || m.userId, m.displayName)}
								</option>
							))}
						</select>
						<PermissionSelector value={newPermissions} onChange={setNewPermissions} disabled={isSharing} />
						<button
							type="button"
							className="btn btn-primary btn-sm"
							disabled={!selectedUserId || newPermissions === 0 || isSharing}
							onClick={handleShare}>
							{isSharing ? <Loader2 className="h-4 w-4 animate-spin" /> : <UserPlus className="h-4 w-4" />}
						</button>
					</div>
				</div>

				<div id="share_modal__shares_list">
					<div className="text-sm font-medium text-base-content/70 mb-2">Shared with</div>

					<div className="space-y-1">
						<div
							id="share_modal__owner"
							className="grid grid-cols-[1fr_auto_auto] items-center gap-3 py-2 px-3 bg-base-200 rounded-lg">
							<div className="min-w-0 truncate">
								<span className="text-sm">{formatUserDisplay(ownerEmail, ownerName)}</span>
								<span className="badge badge-ghost badge-xs ml-2">Owner</span>
							</div>
							<span className="text-xs text-base-content/50">Full access</span>
							<div className="w-6" />
						</div>

						{isLoadingShares ? (
							<div className="flex justify-center py-4">
								<Loader2 className="h-5 w-5 animate-spin text-base-content/50" />
							</div>
						) : (
							shares.map((share) => (
								<div
									key={share.userId}
									className="grid grid-cols-[1fr_auto_auto] items-center gap-3 py-2 px-3 rounded-lg hover:bg-base-200">
									<span className="text-sm truncate" title={share.email || share.userId}>
										{formatUserDisplay(share.email || share.userId, share.displayName)}
									</span>
									<PermissionBadges permissions={share.permissions} />
									<button
										type="button"
										className="btn btn-ghost btn-xs text-error hover:bg-error hover:text-error-content"
										onClick={() => handleUnshare(share.userId)}
										disabled={isRemoving}
										title="Remove share">
										<Trash2 className="h-3 w-3" />
									</button>
								</div>
							))
						)}
					</div>
				</div>

				<div id="share_modal__actions" className="modal-action">
					<button type="button" className="btn btn-ghost" onClick={onClose}>
						Done
					</button>
				</div>
			</div>
		</Modal>
	);
}
