'use client';

import type { ActionResult } from '@actions';
import type { Agent } from '@actions/agents';
import type { ListModelsResponse } from '@actions/models';
import type { ListProvidersResponse } from '@actions/providers';
import type { ListToolsResponse } from '@actions/tools';
import { Button, Modal, Select } from '@components';
import { ProviderSource } from '@startup/proto-ts/rig-service/rig/v1/provider_pb';
import { X } from 'lucide-react';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';
import ToolSelector from './ToolSelector';

const _PROVIDER_SOURCE_LABELS: Record<ProviderSource, string> = {
	[ProviderSource.UNSPECIFIED]: 'Unknown',
	[ProviderSource.SYSTEM]: 'System',
	[ProviderSource.USER]: 'User',
};

interface AgentModalProps {
	isOpen: boolean;
	onClose: () => void;
	mode: 'create' | 'edit';
	agent?: Agent;
	onSubmit: (data: AgentFormData) => Promise<void>;
	isSubmitting?: boolean;
	models?: ActionResult<ListModelsResponse>;
	modelsLoading?: boolean;
	providers?: ActionResult<ListProvidersResponse>;
	providersLoading?: boolean;
	tools?: ActionResult<ListToolsResponse>;
	toolsLoading?: boolean;
}

export interface AgentFormData {
	name: string;
	model: string;
	provider_id: string;
	system_prompt: string;
	tools: string[];
}

export default function AgentModal({
	isOpen,
	onClose,
	mode,
	agent,
	onSubmit,
	isSubmitting = false,
	models: modelsResult,
	modelsLoading = false,
	providers: providersResult,
	providersLoading = false,
	tools: toolsResult,
	toolsLoading = false,
}: AgentModalProps) {
	const models = modelsResult?.success ? modelsResult.data.models : [];
	const modelsError = modelsResult && !modelsResult.success ? modelsResult.error : null;
	const providers = providersResult?.success ? providersResult.data.providers : [];
	const providersError = providersResult && !providersResult.success ? providersResult.error : null;
	const tools = toolsResult?.success ? toolsResult.data.tools : [];
	const toolsError = toolsResult && !toolsResult.success ? toolsResult.error : null;

	const [selectedToolPids, setSelectedToolPids] = useState<string[]>([]);
	const defaultValues = useMemo(
		() => ({
			name: agent?.name || '',
			model: agent?.model?.pid || '',
			provider_id: agent?.model?.provider?.pid || '',
			system_prompt: agent?.systemPrompt || '',
			tools: agent?.tools?.map((t) => t.pid) || [],
		}),
		[agent],
	);

	const {
		register,
		handleSubmit,
		formState: { errors },
		reset,
		watch,
		setValue,
	} = useForm<AgentFormData>({
		defaultValues,
	});

	const selectedProviderId = watch('provider_id');
	const selectedModel = watch('model');

	const filteredModels = useMemo(() => {
		if (!selectedProviderId) {
			// No provider selected, show all models
			return models;
		}

		const selectedProvider = providers.find((p) => p.identifier.value === selectedProviderId);
		if (!selectedProvider) return models;

		const providerNameLower = selectedProvider.name.toLowerCase();

		return models.filter((model) => {
			const modelProviderLower = model.providerName?.toLowerCase();
			// Handle different naming conventions
			if (providerNameLower.includes('openai') && modelProviderLower === 'openai') return true;
			if (providerNameLower.includes('anthropic') && modelProviderLower === 'anthropic') return true;
			if (providerNameLower.includes('lm studio') && modelProviderLower === 'lmstudio') return true;
			if (providerNameLower.includes('ollama') && modelProviderLower === 'ollama') return true;
			// For exact matches
			return modelProviderLower === providerNameLower;
		});
	}, [selectedProviderId, providers, models]);

	useEffect(() => {
		if (selectedProviderId && filteredModels.length > 0) {
			// Check if current model is still in filtered list (using PID)
			const currentModelValid = filteredModels.some((m) => m.identifier.value === selectedModel);
			if (!currentModelValid) {
				// Set to first available model for this provider (using PID)
				setValue('model', filteredModels[0].identifier.value);
			}
		}
	}, [selectedProviderId, filteredModels, selectedModel, setValue]);

	useEffect(() => {
		if (isOpen) {
			reset(defaultValues);
			setSelectedToolPids(defaultValues.tools);
		}
	}, [isOpen, reset, defaultValues]);

	const handleClose = useCallback(() => {
		reset();
		setSelectedToolPids([]);
		onClose();
	}, [reset, onClose]);

	const handleFormSubmit = useCallback(
		async (data: AgentFormData) => {
			// Include selected tools in the form data
			await onSubmit({ ...data, tools: selectedToolPids });
			// Don't close here - let the parent handle success/error and closing
		},
		[onSubmit, selectedToolPids],
	);

	const validationRules = useMemo(
		() => ({
			name: {
				required: 'Agent name is required',
				minLength: {
					value: 3,
					message: 'Name must be at least 3 characters',
				},
				maxLength: {
					value: 255,
					message: 'Name must be less than 255 characters',
				},
				pattern: {
					value: /^[a-zA-Z0-9\s\-_]+$/,
					message: 'Name can only contain letters, numbers, spaces, hyphens, and underscores',
				},
			},
			model: {
				required: 'Model selection is required',
			},
			provider_id: {
				required: 'Provider selection is required',
			},
			system_prompt: {
				maxLength: {
					value: 4000,
					message: 'System prompt must be less than 4000 characters',
				},
			},
		}),
		[],
	);

	const systemPromptValue = watch('system_prompt');

	return (
		<Modal open={isOpen}>
			<div className="modal-box max-w-2xl">
				<div id="modal_header" className="flex justify-between items-center mb-4">
					<h3 className="font-bold text-lg">{mode === 'create' ? 'Create Agent' : 'Edit Agent'}</h3>
					<Button ghost size="sm" shape="circle" onClick={handleClose}>
						<X className="w-4 h-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit(handleFormSubmit)} className="space-y-4">
					<div id="name_field__container">
						<label htmlFor="agent-name" className="label">
							<span className="label-text">Name *</span>
						</label>
						<input
							id="agent-name"
							type="text"
							className={`input input-bordered w-full ${errors.name ? 'input-error' : ''}`}
							placeholder="e.g., My Marketing Assistant"
							disabled={isSubmitting}
							{...register('name', validationRules.name)}
						/>
						{errors.name && (
							<div className="label">
								<span className="label-text-alt text-error">{errors.name.message}</span>
							</div>
						)}
					</div>

					<div id="provider_field__container">
						<label htmlFor="agent-provider" className="label">
							<span className="label-text">Provider *</span>
						</label>
						<Select
							id="agent-provider"
							className={`w-full ${errors.provider_id ? 'select-error' : ''}`}
							disabled={providersLoading || providersError !== null || isSubmitting}
							{...register('provider_id', validationRules.provider_id)}>
							{providersLoading ? (
								<option value="" disabled>
									Loading providers...
								</option>
							) : providersError ? (
								<option value="" disabled>
									Failed to load providers
								</option>
							) : (
								<>
									<option value="" disabled>
										Select a provider
									</option>
									{providers.map((provider) => (
										<option key={provider.identifier.value} value={provider.identifier.value}>
											{provider.name}
										</option>
									))}
								</>
							)}
						</Select>
						{errors.provider_id && (
							<div className="label">
								<span className="label-text-alt text-error">{errors.provider_id.message}</span>
							</div>
						)}
					</div>

					<div id="model_field__container">
						<label htmlFor="agent-model" className="label">
							<span className="label-text">Model *</span>
							{selectedProviderId && filteredModels.length > 0 && (
								<span className="label-text-alt">
									{filteredModels.length} model{filteredModels.length !== 1 ? 's' : ''} available
								</span>
							)}
						</label>
						<Select
							id="agent-model"
							className={`w-full ${errors.model ? 'select-error' : ''}`}
							disabled={modelsLoading || modelsError !== null || isSubmitting || !selectedProviderId}
							{...register('model', validationRules.model)}>
							{modelsLoading ? (
								<option value="" disabled>
									Loading models...
								</option>
							) : modelsError ? (
								<option value="" disabled>
									Failed to load models
								</option>
							) : filteredModels.length === 0 ? (
								<option value="" disabled>
									{!selectedProviderId ? 'Select a provider first' : 'No models available'}
								</option>
							) : (
								<>
									<option value="" disabled>
										Select a model
									</option>
									{filteredModels.map((model) => (
										<option key={model.identifier.value} value={model.identifier.value}>
											{model.name}
										</option>
									))}
								</>
							)}
						</Select>
						{errors.model && (
							<div className="label">
								<span className="label-text-alt text-error">{errors.model.message}</span>
							</div>
						)}
					</div>

					<div id="system_prompt_field__container">
						<label htmlFor="agent-system-prompt" className="label">
							<span className="label-text">System Prompt</span>
							<span className="label-text-alt">{systemPromptValue?.length || 0}/4000</span>
						</label>
						<textarea
							id="agent-system-prompt"
							className={`textarea textarea-bordered w-full ${errors.system_prompt ? 'textarea-error' : ''}`}
							placeholder="Describe the role and behavior of this agent..."
							rows={6}
							disabled={isSubmitting}
							{...register('system_prompt', validationRules.system_prompt)}
						/>
						{errors.system_prompt && (
							<div className="label">
								<span className="label-text-alt text-error">{errors.system_prompt.message}</span>
							</div>
						)}
					</div>

					<div id="tools_field__container">
						<ToolSelector
							tools={tools}
							selectedToolPids={selectedToolPids}
							onChange={setSelectedToolPids}
							disabled={isSubmitting}
							error={toolsError}
							loading={toolsLoading}
						/>
					</div>

					<div id="future_features__container" className="border border-base-300 rounded-lg p-4 bg-base-200/30">
						<div className="flex items-center gap-2 text-base-content/50 mb-2">
							<span className="text-sm">More options coming soon:</span>
						</div>
						<ul className="text-xs space-y-1 text-base-content/40">
							<li>File Attachments</li>
							<li>Custom Parameters</li>
						</ul>
					</div>

					<div id="modal_actions__container" className="modal-action">
						<Button type="button" ghost onClick={handleClose} disabled={isSubmitting}>
							Cancel
						</Button>
						<Button type="submit" color="primary" disabled={isSubmitting}>
							{isSubmitting
								? mode === 'create'
									? 'Creating...'
									: 'Updating...'
								: mode === 'create'
									? 'Create Agent'
									: 'Update Agent'}
						</Button>
					</div>
				</form>
			</div>
		</Modal>
	);
}
