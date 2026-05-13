'use client';

import { createOrg } from '@actions/organizations';
import { useOrganizations } from '@providers/OrganizationContextProvider';
import { Building2 } from 'lucide-react';
import { useRouter } from 'next/navigation';
import { useState, useTransition } from 'react';
import { useForm } from 'react-hook-form';

interface OrgFormData {
	name: string;
	slug: string;
}

function generateSlug(name: string): string {
	return name
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, '-')
		.replace(/^-|-$/g, '');
}

export default function NewOrganizationClient() {
	const router = useRouter();
	const { setCurrentOrg } = useOrganizations();
	const [isPending, startTransition] = useTransition();
	const [slugManuallyEdited, setSlugManuallyEdited] = useState(false);

	const {
		register,
		handleSubmit,
		formState: { errors },
		watch,
		setValue,
	} = useForm<OrgFormData>({
		defaultValues: {
			name: '',
			slug: '',
		},
	});

	// Auto-generate slug from name if not manually edited
	const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		const newName = e.target.value;
		if (!slugManuallyEdited) {
			setValue('slug', generateSlug(newName));
		}
	};

	const handleSlugChange = () => {
		setSlugManuallyEdited(true);
	};

	const handleCreateOrg = (data: OrgFormData) => {
		startTransition(async () => {
			const result = await createOrg(data.name, data.slug);
			if (!result.success) {
				console.error('Failed to create organization:', result.error);
				// TODO: Show error toast
				return;
			}
			// Auto-switch context to the newly created org
			if (result.data.organization?.pid) {
				setCurrentOrg(result.data.organization.pid);
			}
			router.push(`/settings/organization`);
		});
	};

	return (
		<div className="mx-auto max-w-2xl px-4 py-6 sm:px-6 lg:px-8">
			<div className="mb-8">
				<div className="flex items-center gap-3 mb-2">
					<Building2 className="h-8 w-8 text-primary" />
					<h1 className="text-3xl font-bold text-base-content">Create Organization</h1>
				</div>
				<p className="mt-2 text-base-content/70">
					Organizations allow you to collaborate with team members and manage resources together.
				</p>
			</div>

			<div className="card bg-base-100 shadow-md">
				<div className="card-body">
					<form onSubmit={handleSubmit(handleCreateOrg)} className="space-y-6">
						<div>
							<label htmlFor="org-name" className="label">
								<span className="label-text font-medium">Organization Name</span>
								<span className="label-text-alt text-error">*</span>
							</label>
							<input
								id="org-name"
								type="text"
								className={`input input-bordered w-full ${errors.name ? 'input-error' : ''}`}
								placeholder="Acme Corp"
								disabled={isPending}
								{...register('name', {
									required: 'Organization name is required',
									minLength: { value: 3, message: 'Name must be at least 3 characters' },
									maxLength: { value: 100, message: 'Name must be less than 100 characters' },
								})}
								onChange={(e) => {
									register('name').onChange(e);
									handleNameChange(e);
								}}
							/>
							{errors.name && (
								<div className="label">
									<span className="label-text-alt text-error">{errors.name.message}</span>
								</div>
							)}
						</div>

						<div>
							<label htmlFor="org-slug" className="label">
								<span className="label-text font-medium">URL Slug</span>
								<span className="label-text-alt text-error">*</span>
							</label>
							<input
								id="org-slug"
								type="text"
								className={`input input-bordered w-full font-mono ${errors.slug ? 'input-error' : ''}`}
								placeholder="acme-corp"
								disabled={isPending}
								{...register('slug', {
									required: 'Slug is required',
									minLength: { value: 3, message: 'Slug must be at least 3 characters' },
									maxLength: { value: 50, message: 'Slug must be less than 50 characters' },
									pattern: {
										value: /^[a-z0-9-]+$/,
										message: 'Slug can only contain lowercase letters, numbers, and hyphens',
									},
								})}
								onChange={(e) => {
									register('slug').onChange(e);
									handleSlugChange();
								}}
							/>
							{errors.slug ? (
								<div className="label">
									<span className="label-text-alt text-error">{errors.slug.message}</span>
								</div>
							) : (
								<div className="label">
									<span className="label-text-alt">Used in URLs: /organizations/{watch('slug') || 'your-slug'}</span>
								</div>
							)}
						</div>

						<div className="alert alert-info">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								className="stroke-current shrink-0 w-6 h-6"
								role="img"
								aria-label="Information">
								<path
									strokeLinecap="round"
									strokeLinejoin="round"
									strokeWidth="2"
									d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
							<span className="text-sm">You will be automatically assigned as the owner of this organization.</span>
						</div>

						<div className="card-actions justify-end gap-2">
							<button type="button" className="btn btn-ghost" onClick={() => router.back()} disabled={isPending}>
								Cancel
							</button>
							<button type="submit" className="btn btn-primary" disabled={isPending}>
								{isPending ? (
									<>
										<span className="loading loading-spinner loading-sm" />
										Creating...
									</>
								) : (
									<>
										<Building2 className="h-4 w-4 mr-2" />
										Create Organization
									</>
								)}
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	);
}
