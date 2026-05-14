'use client';

import { deleteOrg, updateOrg } from '@actions/organizations';
import { Trash2 } from 'lucide-react';
import { useRouter } from 'next/navigation';
import { useState, useTransition } from 'react';
import { useForm } from 'react-hook-form';
import type { Organization } from '@/lib/proto-shims';

interface OrganizationDetailsProps {
	organization: Organization;
	isAdmin: boolean;
	isOwner: boolean;
}

interface OrgFormData {
	name: string;
	slug: string;
}

export default function OrganizationDetails({ organization, isAdmin, isOwner }: OrganizationDetailsProps) {
	const router = useRouter();
	const [isPending, startTransition] = useTransition();
	const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

	const {
		register,
		handleSubmit,
		formState: { errors, isDirty },
	} = useForm<OrgFormData>({
		defaultValues: {
			name: organization.name,
			slug: organization.slug,
		},
	});

	const handleUpdateOrg = (data: OrgFormData) => {
		startTransition(async () => {
			const result = await updateOrg(organization.pid, { name: data.name, slug: data.slug });
			if (!result.success) {
				console.error('Failed to update organization:', result.error);
				// TODO: Show error toast
				return;
			}
			router.refresh();
		});
	};

	const handleDelete = () => {
		if (!showDeleteConfirm) {
			setShowDeleteConfirm(true);
			return;
		}

		startTransition(async () => {
			const result = await deleteOrg(organization.pid);
			if (!result.success) {
				console.error('Failed to delete organization:', result.error);
				// TODO: Show error toast
				return;
			}
			router.push('/dashboard');
		});
	};

	return (
		<div className="space-y-6">
			<div className="card bg-base-100 shadow-sm">
				<div className="card-body">
					<h2 className="card-title">Organization Details</h2>
					<form onSubmit={handleSubmit(handleUpdateOrg)} className="space-y-4">
						<div>
							<label htmlFor="org-name" className="label">
								<span className="label-text">Organization Name</span>
							</label>
							<input
								id="org-name"
								type="text"
								className={`input input-bordered w-full ${errors.name ? 'input-error' : ''}`}
								disabled={!isAdmin || isPending}
								{...register('name', {
									required: 'Name is required',
									minLength: { value: 3, message: 'Name must be at least 3 characters' },
								})}
							/>
							{errors.name && (
								<div className="label">
									<span className="label-text-alt text-error">{errors.name.message}</span>
								</div>
							)}
						</div>

						<div>
							<label htmlFor="org-slug" className="label">
								<span className="label-text">URL Slug</span>
							</label>
							<input
								id="org-slug"
								type="text"
								className={`input input-bordered w-full ${errors.slug ? 'input-error' : ''}`}
								disabled={!isAdmin || isPending}
								{...register('slug', {
									required: 'Slug is required',
									pattern: {
										value: /^[a-z0-9-]+$/,
										message: 'Slug can only contain lowercase letters, numbers, and hyphens',
									},
								})}
							/>
							{errors.slug && (
								<div className="label">
									<span className="label-text-alt text-error">{errors.slug.message}</span>
								</div>
							)}
							<div className="label">
								<span className="label-text-alt">Used in URLs: /organizations/{organization.slug}</span>
							</div>
						</div>

						{isAdmin && (
							<div className="card-actions justify-end">
								<button type="submit" className="btn btn-primary" disabled={!isDirty || isPending}>
									{isPending ? (
										<>
											<span className="loading loading-spinner loading-sm" />
											Saving...
										</>
									) : (
										'Save Changes'
									)}
								</button>
							</div>
						)}
					</form>
				</div>
			</div>
			{isOwner && (
				<div className="card bg-base-100 shadow-sm border-2 border-error">
					<div className="card-body">
						<h2 className="card-title text-error">Danger Zone</h2>
						<p className="text-sm text-base-content/70">
							Deleting this organization will remove all associated data and cannot be undone.
						</p>
						<div className="card-actions justify-end">
							{showDeleteConfirm ? (
								<div className="flex gap-2">
									<button
										type="button"
										className="btn btn-ghost"
										onClick={() => setShowDeleteConfirm(false)}
										disabled={isPending}>
										Cancel
									</button>
									<button type="button" className="btn btn-error" onClick={handleDelete} disabled={isPending}>
										{isPending ? (
											<>
												<span className="loading loading-spinner loading-sm" />
												Deleting...
											</>
										) : (
											<>
												<Trash2 className="h-4 w-4 mr-2" />
												Confirm Delete
											</>
										)}
									</button>
								</div>
							) : (
								<button type="button" className="btn btn-error btn-outline" onClick={handleDelete}>
									<Trash2 className="h-4 w-4 mr-2" />
									Delete Organization
								</button>
							)}
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
