'use client';

import { Button } from '@components';
import { useRouter } from 'next/navigation';
import { useTransition } from 'react';
import { useForm } from 'react-hook-form';
import { changePassword } from '@/actions/user';
import FormInput from '@/components/FormInput/FormInput';
import useToast from '@/hooks/useToast';

interface PasswordFormData {
	currentPassword: string;
	newPassword: string;
	confirmPassword: string;
}

export default function PasswordChangeForm() {
	const [isPending, startTransition] = useTransition();
	const router = useRouter();
	const { success, error: showError } = useToast();

	const {
		register,
		handleSubmit,
		formState: { errors },
		watch,
		reset,
	} = useForm<PasswordFormData>({
		mode: 'onBlur',
	});

	const newPassword = watch('newPassword');

	const onSubmit = (data: PasswordFormData) => {
		startTransition(async () => {
			const result = await changePassword(data.currentPassword, data.newPassword);

			if (result.success) {
				success('Password changed successfully. Please log in again.');
				reset();
				// Small delay to show the toast before redirect
				setTimeout(() => {
					router.push('/auth/login');
				}, 1500);
			} else {
				showError(result.error || 'Failed to change password');
			}
		});
	};

	return (
		<div>
			<h2 className="card-title">Password</h2>
			<p className="text-base-content/70 text-sm mb-4">
				Update your password. You will be logged out of all devices after changing your password.
			</p>

			<form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
				<div>
					<FormInput
						name="currentPassword"
						label="Current Password"
						type="password"
						placeholder="Enter current password"
						error={errors.currentPassword}
						register={register('currentPassword', {
							required: 'Current password is required',
						})}
					/>
				</div>

				<div>
					<FormInput
						name="newPassword"
						label="New Password"
						type="password"
						placeholder="Enter new password"
						error={errors.newPassword}
						register={register('newPassword', {
							required: 'New password is required',
							minLength: {
								value: 8,
								message: 'Password must be at least 8 characters',
							},
							pattern: {
								value: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
								message: 'Must include uppercase, lowercase, and a number',
							},
						})}
					/>
				</div>

				<div>
					<FormInput
						name="confirmPassword"
						label="Confirm New Password"
						type="password"
						placeholder="Confirm new password"
						error={errors.confirmPassword}
						register={register('confirmPassword', {
							required: 'Please confirm your password',
							validate: (value) => value === newPassword || 'Passwords do not match',
						})}
					/>
				</div>

				<div className="card-actions justify-end pt-4">
					<Button type="submit" color="primary" loading={isPending}>
						{isPending ? 'Changing Password...' : 'Change Password'}
					</Button>
				</div>
			</form>
		</div>
	);
}
