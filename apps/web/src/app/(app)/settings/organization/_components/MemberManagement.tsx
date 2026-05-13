'use client';

import { addOrgMemberByEmail, getOrgMembers, removeOrgMember, updateOrgMemberRole } from '@actions/organizations';
import { Select } from '@components';
import type { Organization, OrganizationMember } from '@startup/proto-ts/api-service/api/v1/organizations_pb';
import { Mail, Trash2, UserPlus } from 'lucide-react';
import { useRouter } from 'next/navigation';
import { useEffect, useState, useTransition } from 'react';
import { useForm } from 'react-hook-form';

interface MemberManagementProps {
	organization: Organization;
	isAdmin: boolean;
	isOwner?: boolean;
}

interface InviteFormData {
	email: string;
	role: string;
}

function formatDate(timestamp: string) {
	return new Date(timestamp).toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
	});
}

export default function MemberManagement({ organization, isAdmin, isOwner = false }: MemberManagementProps) {
	const router = useRouter();
	const [isPending, startTransition] = useTransition();
	const [members, setMembers] = useState<OrganizationMember[]>([]);
	const [isLoadingMembers, setIsLoadingMembers] = useState(true);
	const [showInviteForm, setShowInviteForm] = useState(false);

	const {
		register,
		handleSubmit,
		formState: { errors },
		reset,
	} = useForm<InviteFormData>({
		defaultValues: {
			email: '',
			role: 'member',
		},
	});

	useEffect(() => {
		async function loadMembers() {
			const result = await getOrgMembers(organization.pid);
			if (!result.success) {
				console.error('Failed to load members:', result.error);
			} else {
				setMembers(result.data.members);
			}
			setIsLoadingMembers(false);
		}
		loadMembers();
	}, [organization.pid]);

	const handleInvite = (data: InviteFormData) => {
		startTransition(async () => {
			const result = await addOrgMemberByEmail(organization.pid, data.email, data.role);
			if (!result.success) {
				console.error('Failed to invite member:', result.error);
				return;
			}
			reset();
			setShowInviteForm(false);
			router.refresh();

			const updatedResult = await getOrgMembers(organization.pid);
			if (updatedResult.success) {
				setMembers(updatedResult.data.members);
			}
		});
	};

	const handleRemoveMember = (userId: string) => {
		if (!confirm('Are you sure you want to remove this member?')) {
			return;
		}

		startTransition(async () => {
			const result = await removeOrgMember(organization.pid, userId);
			if (!result.success) {
				console.error('Failed to remove member:', result.error);
				return;
			}
			router.refresh();

			const updatedResult = await getOrgMembers(organization.pid);
			if (updatedResult.success) {
				setMembers(updatedResult.data.members);
			}
		});
	};

	const handleRoleChange = (userId: string, newRole: string) => {
		startTransition(async () => {
			const result = await updateOrgMemberRole(organization.pid, userId, newRole);
			if (!result.success) {
				console.error('Failed to update member role:', result.error);
				return;
			}
			router.refresh();

			const updatedResult = await getOrgMembers(organization.pid);
			if (updatedResult.success) {
				setMembers(updatedResult.data.members);
			}
		});
	};

	return (
		<div className="space-y-6">
			{isAdmin && (
				<div className="card bg-base-100 shadow-sm">
					<div className="card-body">
						<div className="flex items-center justify-between">
							<h2 className="card-title">Invite Member</h2>
							<button
								type="button"
								className="btn btn-primary btn-sm"
								onClick={() => setShowInviteForm(!showInviteForm)}>
								<UserPlus className="h-4 w-4 mr-2" />
								{showInviteForm ? 'Cancel' : 'Invite'}
							</button>
						</div>

						{showInviteForm && (
							<form onSubmit={handleSubmit(handleInvite)} className="mt-4 space-y-4">
								<div>
									<label htmlFor="member-email" className="label">
										<span className="label-text">Email address</span>
									</label>
									<input
										id="member-email"
										type="email"
										className={`input input-bordered w-full ${errors.email ? 'input-error' : ''}`}
										placeholder="name@example.com"
										disabled={isPending}
										{...register('email', {
											required: 'Email is required',
											pattern: {
												value: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
												message: 'Enter a valid email address',
											},
										})}
									/>
									{errors.email && (
										<div className="label">
											<span className="label-text-alt text-error">{errors.email.message}</span>
										</div>
									)}
								</div>

								<div>
									<label htmlFor="member-role" className="label">
										<span className="label-text">Role</span>
									</label>
									<Select id="member-role" className="w-full" disabled={isPending} {...register('role')}>
										<option value="member">Member</option>
										<option value="admin">Admin</option>
										{isOwner && <option value="owner">Owner</option>}
									</Select>
								</div>

								<div className="card-actions justify-end">
									<button type="submit" className="btn btn-primary" disabled={isPending}>
										{isPending ? (
											<>
												<span className="loading loading-spinner loading-sm" />
												Adding...
											</>
										) : (
											<>
												<Mail className="h-4 w-4 mr-2" />
												Add Member
											</>
										)}
									</button>
								</div>
							</form>
						)}
					</div>
				</div>
			)}

			<div className="card bg-base-100 shadow-sm overflow-hidden">
				<div className="card-body pb-0">
					<h2 className="card-title">Members ({members.length})</h2>
				</div>

				{isLoadingMembers ? (
					<div className="flex justify-center py-8">
						<span className="loading loading-spinner loading-lg" />
					</div>
				) : members.length === 0 ? (
					<div className="text-center py-8 text-base-content/70">No members found</div>
				) : (
					<div>
						<div
							id="member_list__header"
							className={[
								'px-6 py-3 border-b border-base-200 bg-base-200/30',
								'hidden sm:grid items-center gap-4',
								'sm:grid-cols-[1fr_6rem_7rem_2rem]',
							].join(' ')}>
							<span className="text-xs font-medium uppercase tracking-wider text-base-content/50">Member</span>
							<span className="text-xs font-medium uppercase tracking-wider text-base-content/50">Role</span>
							<span className="text-xs font-medium uppercase tracking-wider text-base-content/50">Joined</span>
							{isAdmin && <span />}
						</div>

						<div>
							{members.map((member) => (
								<MemberRow
									key={member.pid}
									member={member}
									isAdmin={isAdmin}
									isPending={isPending}
									onRoleChange={handleRoleChange}
									onRemove={handleRemoveMember}
								/>
							))}
						</div>

						<div className="px-6 py-3 border-t border-base-200 bg-base-200/30">
							<p className="text-xs text-base-content/50">
								{members.length} member{members.length !== 1 ? 's' : ''}
							</p>
						</div>
					</div>
				)}
			</div>
		</div>
	);
}

function MemberRow({
	member,
	isAdmin,
	isPending,
	onRoleChange,
	onRemove,
}: {
	member: OrganizationMember;
	isAdmin: boolean;
	isPending: boolean;
	onRoleChange: (userId: string, role: string) => void;
	onRemove: (userId: string) => void;
}) {
	const displayName = member.displayName?.trim();

	return (
		<div
			className={[
				'px-6 py-3 border-b border-base-200 last:border-b-0',
				'grid items-center gap-x-4 gap-y-2',
				'grid-cols-[1fr_auto] sm:grid-cols-[1fr_6rem_7rem_2rem]',
			].join(' ')}>
			<div id="member_row__info" className="min-w-0">
				<div className="text-sm truncate">{member.email || member.userId}</div>
				{displayName && <div className="text-xs text-base-content/50 truncate">{displayName}</div>}
			</div>

			<div id="member_row__role" className="sm:col-auto">
				{isAdmin && member.role !== 'owner' ? (
					<Select
						value={member.role}
						onChange={(e) => onRoleChange(member.userId, e.target.value)}
						disabled={isPending}
						className="select-xs sm:select-sm w-auto">
						<option value="member">Member</option>
						<option value="admin">Admin</option>
					</Select>
				) : (
					<span className="badge badge-primary badge-sm">{member.role}</span>
				)}
			</div>

			<div id="member_row__joined" className="hidden sm:block text-xs text-base-content/50">
				{formatDate(member.createdAt)}
			</div>

			<div id="member_row__actions" className="hidden sm:flex justify-end">
				{isAdmin && member.role !== 'owner' && (
					<button
						type="button"
						className="btn btn-ghost btn-xs btn-square"
						onClick={() => onRemove(member.userId)}
						disabled={isPending}>
						<Trash2 className="h-3.5 w-3.5 text-error" />
					</button>
				)}
			</div>
		</div>
	);
}
