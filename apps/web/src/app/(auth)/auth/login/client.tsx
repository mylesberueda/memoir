'use client';

import { login } from '@actions/auth';
import { Button, FormInput } from '@components';
import { useRouter } from 'next/navigation';
import { useState } from 'react';
import { type FieldError, useForm } from 'react-hook-form';

interface LoginFormData {
	email: string;
	password: string;
	name?: string;
	confirmPassword?: string;
}

interface LoginClientProps {
	authRequest?: string;
	initialMode: 'login' | 'register';
	verified: boolean;
	error?: string;
}

export default function LoginClient({ authRequest, initialMode, verified, error: initialError }: LoginClientProps) {
	const router = useRouter();
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(initialError || null);
	const [successMessage, setSuccessMessage] = useState<string | null>(
		verified ? 'Your email has been successfully verified. You may now sign in to your account.' : null,
	);
	const [isRegistrationMode, setIsRegistrationMode] = useState(initialMode === 'register');

	const {
		register,
		handleSubmit,
		watch,
		formState: { errors },
	} = useForm<LoginFormData>({
		mode: 'onBlur',
	});

	const password = watch('password');
	const confirmPassword = watch('confirmPassword');

	const onSubmit = async (data: LoginFormData) => {
		setIsLoading(true);
		setError(null);

		try {
			if (isRegistrationMode) {
				const { register: registerAction } = await import('@/actions/auth');

				if (!data.name || !data.confirmPassword) {
					setError('Please fill in all fields');
					setIsLoading(false);
					return;
				}

				if (data.password !== data.confirmPassword) {
					setError('Passwords do not match');
					setIsLoading(false);
					return;
				}

				const result = await registerAction(data.email, data.password, data.name);

				if (result.success) {
					setSuccessMessage('Registration successful! Please check your email to verify your account.');
					setIsRegistrationMode(false);
				} else {
					setError(result.error || 'Registration failed');
				}
			} else {
				if (!authRequest) {
					setError('Authentication session expired. Please try again.');
					// Navigate to login to restart OIDC flow
					router.push('/auth/login');
					return;
				}

				const result = await login(data.email, data.password, authRequest);

				if (result.success && result.callbackUrl) {
					window.location.href = result.callbackUrl;
				} else {
					setError(result.error || 'Login failed');
				}
			}
		} catch (err) {
			console.error('Login error:', err);
			setError('An unexpected error occurred. Please try again.');
		} finally {
			setIsLoading(false);
		}
	};

	const handleModeSwitch = (newMode: 'login' | 'register') => {
		setError(null);
		setSuccessMessage(null);
		if (newMode === 'register') {
			setIsRegistrationMode(true);
		} else {
			setIsRegistrationMode(false);
		}
	};

	return (
		<div id="login_page__container" className="flex items-center justify-center min-h-full p-4">
			<div id="login_page__content" className="w-full max-w-md">
				<div id="login_page__header" className="text-center mb-8">
					<h1 className="text-3xl font-bold transition-all duration-300">
						{isRegistrationMode ? 'Create Account' : 'Welcome Back'}
					</h1>
					<p className="text-base-content/70 mt-2 transition-all duration-300">
						{isRegistrationMode ? 'Sign up to get started with Memoir' : 'Sign in to your Memoir account'}
					</p>
				</div>

				<div id="login_page__card" className="card bg-base-100 shadow-xl">
					<div className="card-body">
						<form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
							{successMessage && (
								<div role="alert" className={`alert ${isRegistrationMode ? 'alert-info' : 'alert-success'}`}>
									<span>{successMessage}</span>
								</div>
							)}
							{error && (
								<div role="alert" className="alert alert-error">
									<span>{error}</span>
								</div>
							)}
							<fieldset id="login_form__fields" className="fieldset w-full max-w-md">
								<div
									id="login_form__name_field"
									className="overflow-hidden transition-all duration-300 ease-in-out"
									style={{
										maxHeight: isRegistrationMode ? '100px' : '0px',
										opacity: isRegistrationMode ? 1 : 0,
									}}>
									<FormInput
										name="name"
										label="Full Name"
										type="text"
										placeholder="Full name"
										error={errors.name}
										register={register('name', {
											required: isRegistrationMode ? 'Required' : false,
											minLength: {
												value: 2,
												message: 'Name must be at least 2 characters',
											},
										})}
									/>
								</div>

								<FormInput
									name="email"
									label="Email"
									type="email"
									placeholder="Email"
									error={errors.email}
									register={register('email', {
										required: 'Required',
										pattern: {
											value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
											message: 'Invalid email address',
										},
									})}
								/>

								<FormInput
									name="password"
									label="Password"
									type="password"
									placeholder="Password"
									error={errors.password}
									register={register('password', {
										required: 'Required',
										minLength: isRegistrationMode
											? {
													value: 8,
													message: 'Password must be at least 8 characters',
												}
											: undefined,
										pattern: isRegistrationMode
											? {
													value: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
													message: 'Weak password',
												}
											: undefined,
									})}
								/>

								<div
									id="login_form__confirm_password_field"
									className="overflow-hidden transition-all duration-300 ease-in-out"
									style={{
										maxHeight: isRegistrationMode ? '100px' : '0px',
										opacity: isRegistrationMode ? 1 : 0,
									}}>
									<FormInput
										name="confirmPassword"
										label="Confirm password"
										type="password"
										placeholder="Confirm password"
										error={
											errors.confirmPassword ||
											(isRegistrationMode && password && confirmPassword && password !== confirmPassword
												? ({
														type: 'manual',
														message: 'Passwords do not match',
													} as FieldError)
												: undefined)
										}
										register={register('confirmPassword', {
											required: isRegistrationMode ? 'Required' : false,
											validate: isRegistrationMode
												? (value) => {
														return value === password || 'Passwords do not match';
													}
												: undefined,
										})}
									/>
								</div>
							</fieldset>
							<div id="login_form__actions" className="form-control mt-6">
								<Button type="submit" color="primary" className="btn-block" loading={isLoading}>
									{isLoading
										? isRegistrationMode
											? 'Creating Account...'
											: 'Signing In...'
										: isRegistrationMode
											? 'Create Account'
											: 'Sign In'}
								</Button>
							</div>
						</form>
					</div>
				</div>

				<div id="login_page__footer" className="text-center mt-6 transition-all duration-300">
					<p className="text-base-content/70">
						{isRegistrationMode ? (
							<>
								Already have an account?{' '}
								<button type="button" onClick={() => handleModeSwitch('login')} className="link link-primary">
									Sign in
								</button>
							</>
						) : (
							<>
								Don&apos;t have an account?{' '}
								<button type="button" onClick={() => handleModeSwitch('register')} className="link link-primary">
									Sign up
								</button>
							</>
						)}
					</p>
				</div>
			</div>
		</div>
	);
}
