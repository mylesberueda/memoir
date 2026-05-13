'use client';

import type { Provider } from '@actions/providers';
import { Modal, Select } from '@components';
import { Eye, EyeOff, X } from 'lucide-react';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';

interface ProviderModalProps {
	isOpen: boolean;
	onClose: () => void;
	mode: 'create' | 'edit';
	provider?: Provider;
	onSubmit: (data: ProviderFormData) => Promise<void>;
	isSubmitting?: boolean;
}

export interface ProviderFormData {
	name: string;
	provider_type: string;
	credentials: string;
	endpoint_url: string;
}

const PROVIDER_TYPES = [
	{ value: 'openai', label: 'OpenAI', needsApiKey: true, needsEndpoint: false },
	{ value: 'anthropic', label: 'Anthropic', needsApiKey: true, needsEndpoint: false },
	{ value: 'ollama', label: 'Ollama', needsApiKey: false, needsEndpoint: true },
	{ value: 'lmstudio', label: 'LM Studio', needsApiKey: false, needsEndpoint: true },
];

export default function ProviderModal({
	isOpen,
	onClose,
	mode,
	provider,
	onSubmit,
	isSubmitting = false,
}: ProviderModalProps) {
	const [showCredentials, setShowCredentials] = useState(false);

	const defaultValues = useMemo(
		() => ({
			name: provider?.name || '',
			provider_type: provider?.providerType || '',
			credentials: '',
			endpoint_url: provider?.endpointUrl || '',
		}),
		[provider],
	);

	const {
		register,
		handleSubmit,
		formState: { errors },
		reset,
		watch,
	} = useForm<ProviderFormData>({
		defaultValues,
	});

	const selectedProviderType = watch('provider_type');

	const providerConfig = useMemo(() => {
		return PROVIDER_TYPES.find((p) => p.value === selectedProviderType);
	}, [selectedProviderType]);

	useEffect(() => {
		if (isOpen) {
			reset(defaultValues);
			setShowCredentials(false);
		}
	}, [isOpen, reset, defaultValues]);

	const handleClose = useCallback(() => {
		reset();
		setShowCredentials(false);
		onClose();
	}, [reset, onClose]);

	const handleFormSubmit = useCallback(
		async (data: ProviderFormData) => {
			await onSubmit(data);
		},
		[onSubmit],
	);

	const validationRules = useMemo(
		() => ({
			name: {
				required: 'Provider name is required',
				minLength: {
					value: 2,
					message: 'Name must be at least 2 characters',
				},
				maxLength: {
					value: 100,
					message: 'Name must be less than 100 characters',
				},
			},
			provider_type: {
				required: 'Provider type is required',
			},
			credentials: {
				validate: (value: string) => {
					if (mode === 'create' && providerConfig?.needsApiKey && !value) {
						return 'API key is required for this provider';
					}
					return true;
				},
			},
			endpoint_url: {
				validate: (value: string) => {
					if (providerConfig?.needsEndpoint && !value) {
						return 'Endpoint URL is required for this provider';
					}
					if (value && !value.startsWith('http://') && !value.startsWith('https://')) {
						return 'Endpoint URL must start with http:// or https://';
					}
					return true;
				},
			},
		}),
		[mode, providerConfig],
	);

	return (
		<Modal open={isOpen}>
			<div className="modal-box max-w-lg">
				<div className="mb-4 flex items-center justify-between">
					<h3 className="text-lg font-bold">{mode === 'create' ? 'Add Provider' : 'Edit Provider'}</h3>
					<button type="button" className="btn btn-circle btn-ghost btn-sm" onClick={handleClose}>
						<X className="h-4 w-4" />
					</button>
				</div>

				<form onSubmit={handleSubmit(handleFormSubmit)} className="space-y-4">
					{/* Name Field */}
					<div>
						<label htmlFor="provider-name" className="label">
							<span className="label-text">Name *</span>
						</label>
						<input
							id="provider-name"
							type="text"
							className={`input input-bordered w-full ${errors.name ? 'input-error' : ''}`}
							placeholder="e.g., My OpenAI Account"
							disabled={isSubmitting}
							{...register('name', validationRules.name)}
						/>
						{errors.name && (
							<div className="label">
								<span className="label-text-alt text-error">{errors.name.message}</span>
							</div>
						)}
					</div>

					{/* Provider Type */}
					<div>
						<label htmlFor="provider-type" className="label">
							<span className="label-text">Provider Type *</span>
						</label>
						<Select
							id="provider-type"
							className={`w-full ${errors.provider_type ? 'select-error' : ''}`}
							disabled={isSubmitting || mode === 'edit'}
							{...register('provider_type', validationRules.provider_type)}>
							<option value="" disabled>
								Select a provider type
							</option>
							{PROVIDER_TYPES.map((type) => (
								<option key={type.value} value={type.value}>
									{type.label}
								</option>
							))}
						</Select>
						{errors.provider_type && (
							<div className="label">
								<span className="label-text-alt text-error">{errors.provider_type.message}</span>
							</div>
						)}
						{mode === 'edit' && (
							<div className="label">
								<span className="label-text-alt text-base-content/50">
									Provider type cannot be changed after creation
								</span>
							</div>
						)}
					</div>

					{/* API Key / Credentials (for OpenAI, Anthropic) */}
					{providerConfig?.needsApiKey && (
						<div>
							<label htmlFor="provider-credentials" className="label">
								<span className="label-text">
									API Key {mode === 'create' ? '*' : ''}
									{mode === 'edit' && <span className="text-base-content/50"> (leave blank to keep current)</span>}
								</span>
							</label>
							<div className="relative">
								<input
									id="provider-credentials"
									type={showCredentials ? 'text' : 'password'}
									className={`input input-bordered w-full pr-10 ${errors.credentials ? 'input-error' : ''}`}
									placeholder={mode === 'edit' ? '••••••••••••••••' : 'sk-...'}
									disabled={isSubmitting}
									autoComplete="off"
									{...register('credentials', validationRules.credentials)}
								/>
								<button
									type="button"
									className="btn btn-ghost btn-sm absolute right-1 top-1/2 -translate-y-1/2"
									onClick={() => setShowCredentials(!showCredentials)}
									tabIndex={-1}>
									{showCredentials ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
								</button>
							</div>
							{errors.credentials && (
								<div className="label">
									<span className="label-text-alt text-error">{errors.credentials.message}</span>
								</div>
							)}
						</div>
					)}

					{/* Endpoint URL (for Ollama, LM Studio) */}
					{providerConfig?.needsEndpoint && (
						<div>
							<label htmlFor="provider-endpoint" className="label">
								<span className="label-text">Endpoint URL *</span>
							</label>
							<input
								id="provider-endpoint"
								type="url"
								className={`input input-bordered w-full ${errors.endpoint_url ? 'input-error' : ''}`}
								placeholder="http://localhost:11434"
								disabled={isSubmitting}
								{...register('endpoint_url', validationRules.endpoint_url)}
							/>
							{errors.endpoint_url && (
								<div className="label">
									<span className="label-text-alt text-error">{errors.endpoint_url.message}</span>
								</div>
							)}
							<div className="label">
								<span className="label-text-alt text-base-content/50">
									{selectedProviderType === 'ollama' && 'Default: http://localhost:11434'}
									{selectedProviderType === 'lmstudio' && 'Default: http://localhost:1234'}
								</span>
							</div>
						</div>
					)}

					{/* Info box for provider-specific guidance */}
					{selectedProviderType && (
						<div className="rounded-lg border bg-base-200/50 p-4">
							<div className="text-sm text-base-content/70">
								{selectedProviderType === 'openai' && (
									<p>
										Get your API key from{' '}
										<a
											href="https://platform.openai.com/api-keys"
											target="_blank"
											rel="noopener noreferrer"
											className="link link-primary">
											OpenAI Dashboard
										</a>
									</p>
								)}
								{selectedProviderType === 'anthropic' && (
									<p>
										Get your API key from{' '}
										<a
											href="https://console.anthropic.com/settings/keys"
											target="_blank"
											rel="noopener noreferrer"
											className="link link-primary">
											Anthropic Console
										</a>
									</p>
								)}
								{selectedProviderType === 'ollama' && (
									<p>
										Make sure Ollama is running locally. Visit{' '}
										<a href="https://ollama.ai" target="_blank" rel="noopener noreferrer" className="link link-primary">
											ollama.ai
										</a>{' '}
										for installation instructions.
									</p>
								)}
								{selectedProviderType === 'lmstudio' && (
									<p>
										Start the LM Studio server with the API enabled. Visit{' '}
										<a
											href="https://lmstudio.ai"
											target="_blank"
											rel="noopener noreferrer"
											className="link link-primary">
											lmstudio.ai
										</a>{' '}
										for more info.
									</p>
								)}
							</div>
						</div>
					)}

					<div className="modal-action">
						<button type="button" className="btn btn-ghost" onClick={handleClose} disabled={isSubmitting}>
							Cancel
						</button>
						<button type="submit" className="btn btn-primary" disabled={isSubmitting || !selectedProviderType}>
							{isSubmitting ? (
								<>
									<span className="loading loading-spinner loading-sm" />
									{mode === 'create' ? 'Adding...' : 'Updating...'}
								</>
							) : mode === 'create' ? (
								'Add Provider'
							) : (
								'Update Provider'
							)}
						</button>
					</div>
				</form>
			</div>
		</Modal>
	);
}
