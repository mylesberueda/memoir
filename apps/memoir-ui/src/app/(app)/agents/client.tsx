'use client';

import type { ActionResult } from '@actions';
import type { Agent, ListAgentsResponse } from '@actions/agents';
import { createAgent, deleteAgent, updateAgent } from '@actions/agents';
import type { ListModelsResponse } from '@actions/models';
import { getOrgMembers } from '@actions/organizations';
import type { ListProvidersResponse } from '@actions/providers';
import type { ListToolsResponse } from '@actions/tools';
import { Button, Modal } from '@components';
import AgentModal, { type AgentFormData } from '@components/Modal/AgentModal';
import ShareModal from '@components/Modal/ShareModal/ShareModal';
import useToast from '@hooks/useToast';
import { useOrganizations } from '@providers/OrganizationContextProvider';
import type { OrganizationMember } from '@polypixel/memoir-sdk/api-service/api/v1/organizations_pb';
import { Plus } from 'lucide-react';
import { useState, useTransition } from 'react';
import AgentGrid from './_components/AgentGrid';

interface AgentsClientProps {
	agents?: ActionResult<ListAgentsResponse>;
	models?: ActionResult<ListModelsResponse>;
	providers?: ActionResult<ListProvidersResponse>;
	tools?: ActionResult<ListToolsResponse>;
}

export default function AgentsClient({ agents, models, providers, tools }: AgentsClientProps) {
	const { currentOrgPid, can } = useOrganizations();
	const [modalState, setModalState] = useState<{
		isOpen: boolean;
		mode: 'create' | 'edit';
		agent?: Agent;
	}>({ isOpen: false, mode: 'create' });
	const [deleteState, setDeleteState] = useState<{
		isOpen: boolean;
		agent?: Agent;
	}>({ isOpen: false });
	const [shareState, setShareState] = useState<{
		isOpen: boolean;
		agent?: Agent;
	}>({ isOpen: false });
	const [orgMembers, setOrgMembers] = useState<OrganizationMember[]>([]);
	const [isPending, startTransition] = useTransition();
	const [isDeleting, startDeleteTransition] = useTransition();
	const toast = useToast();

	const handleShareAgent = async (agent: Agent) => {
		if (!currentOrgPid) return;
		const result = await getOrgMembers(currentOrgPid);
		if (result.success) {
			setOrgMembers(result.data.members);
		}
		setShareState({ isOpen: true, agent });
	};

	const handleCreateAgent = () => {
		setModalState({ isOpen: true, mode: 'create' });
	};

	const handleEditAgent = (agent: Agent) => {
		setModalState({ isOpen: true, mode: 'edit', agent });
	};

	const handleAgentSubmit = async (data: AgentFormData) => {
		startTransition(async () => {
			let result: ActionResult<Agent> | undefined;

			if (modalState.mode === 'create') {
				result = await createAgent({
					name: data.name,
					modelPid: data.model,
					systemPrompt: data.system_prompt || undefined,
					providerPid: data.provider_id || undefined,
					toolPids: data.tools,
				});
			} else if (modalState.mode === 'edit' && modalState.agent) {
				result = await updateAgent({
					pid: modalState.agent.identifier.value,
					name: data.name,
					modelPid: data.model,
					systemPrompt: data.system_prompt || '',
					tools: data.tools?.map((pid) => ({ pid, isActive: true })),
					providerPid: data.provider_id || undefined,
				});
			}

			if (result && !result.success) {
				toast.error(`Failed to ${modalState.mode} agent: ${result.error}`);
				return;
			}

			toast.success(`Agent ${modalState.mode === 'create' ? 'created' : 'updated'} successfully`);
			setModalState({ isOpen: false, mode: 'create' });
		});
	};

	const handleDeleteAgent = (agent: Agent) => {
		setDeleteState({ isOpen: true, agent });
	};

	const handleConfirmDelete = () => {
		const agentToDelete = deleteState.agent;
		if (!agentToDelete) return;

		startDeleteTransition(async () => {
			const result = await deleteAgent(agentToDelete.identifier.value);

			if (!result.success) {
				toast.error(`Failed to delete agent: ${result.error}`);
				return;
			}

			toast.success('Agent deleted successfully');
			setDeleteState({ isOpen: false });
		});
	};

	return (
		<div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div id="agents_page__header" className="mb-8 flex items-center justify-between">
				<div>
					<h1 className="text-3xl font-bold text-base-content">Agents</h1>
					<p className="mt-2 text-base-content/70">Manage your AI agents and their configurations.</p>
				</div>
				{can('agents', 'write') && (
					<Button color="primary" onClick={handleCreateAgent}>
						<Plus className="mr-2 h-4 w-4" />
						Create Agent
					</Button>
				)}
			</div>
			<AgentGrid
				agents={agents}
				isLoading={false}
				onRefresh={() => window.location.reload()}
				onEdit={can('agents', 'write') ? handleEditAgent : undefined}
				onCreate={can('agents', 'write') ? handleCreateAgent : undefined}
				onDelete={can('agents', 'write') ? handleDeleteAgent : undefined}
				onShare={can('agents', 'write') ? handleShareAgent : undefined}
			/>
			<AgentModal
				isOpen={modalState.isOpen}
				onClose={() => setModalState({ isOpen: false, mode: 'create' })}
				mode={modalState.mode}
				agent={modalState.agent}
				onSubmit={handleAgentSubmit}
				isSubmitting={isPending}
				models={models}
				providers={providers}
				tools={tools}
			/>

			<Modal open={deleteState.isOpen}>
				<div id="delete_modal__container" className="modal-box">
					<h3 className="font-bold text-lg">Delete Agent</h3>
					<p className="py-4">
						Are you sure you want to delete <span className="font-semibold">{deleteState.agent?.name}</span>? This
						action cannot be undone.
					</p>
					<div id="delete_modal__actions" className="modal-action">
						<Button ghost onClick={() => setDeleteState({ isOpen: false })} disabled={isDeleting}>
							Cancel
						</Button>
						<Button color="error" onClick={handleConfirmDelete} disabled={isDeleting}>
							{isDeleting ? 'Deleting...' : 'Delete'}
						</Button>
					</div>
				</div>
			</Modal>

			{shareState.agent && (
				<ShareModal
					isOpen={shareState.isOpen}
					onClose={() => setShareState({ isOpen: false })}
					resourceType="agent"
					resourcePid={shareState.agent.identifier.value}
					ownerUserId={shareState.agent.createdByUserId}
					members={orgMembers}
				/>
			)}
		</div>
	);
}
