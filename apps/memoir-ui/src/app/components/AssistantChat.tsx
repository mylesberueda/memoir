'use client';

import { getUserAssistant, type UserAssistantResponse } from '@actions/agents';
import { Button, Chat } from '@components';
import { transformMessagesToChatProps, useStreamingChat } from '@components/Chat';
import type { PromptBoxForm } from '@components/Chat/PromptInput';
import { useAssistantChatState } from '@lib/chat-state';
import { useOrganizationsOptional } from '@providers/OrganizationContextProvider';
import cns from 'classnames';
import { Maximize2, Minimize2, Minus, Sparkles } from 'lucide-react';
import { usePathname } from 'next/navigation';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Card } from 'rsc-daisyui';

type ChatSize = 'normal' | 'expanded';

export default function AssistantChat() {
	const pathname = usePathname();
	const detailsRef = useRef<HTMLDetailsElement>(null);
	const [size, setSize] = useState<ChatSize>('normal');
	const [assistant, setAssistant] = useState<UserAssistantResponse | undefined>();
	const [assistantLoading, setAssistantLoading] = useState(false);
	const [assistantError, setAssistantError] = useState<string | null>(null);

	const orgContext = useOrganizationsOptional();
	const currentOrgPid = orgContext?.currentOrgPid;

	const {
		state: { messages, isLoading, sessionId },
		addMessage,
		updateMessage,
		setLoading,
		setError,
		setSessionId,
	} = useAssistantChatState();

	useEffect(() => {
		if (!currentOrgPid) return;

		setAssistantLoading(true);

		const fetchUserAssistant = async () => {
			const result = await getUserAssistant();

			if (result.success) {
				setAssistant(result.data);
			} else {
				console.error('[AssistantChat]', 'Failed to load assistant', result.error);
				setAssistantError(result.error);
			}

			setAssistantLoading(false);
		};

		fetchUserAssistant();
	}, [currentOrgPid]);

	const { submit, stop, isStreaming } = useStreamingChat({
		agentId: assistant?.agent?.identifier.value,
		sessionId,
		addMessage,
		updateMessage,
		setLoading,
		setError,
		setSessionId,
	});

	const canRetry = useCallback(() => false, []);
	const onRetry = useCallback(() => {}, []);

	const chatMessages = useMemo(
		() =>
			transformMessagesToChatProps({
				messages,
				assistantName: assistant?.agent?.name || 'Assistant',
				canRetry,
				onRetry,
			}),
		[messages, assistant?.agent?.name, canRetry, onRetry],
	);

	const handleOnSubmit = useCallback(
		async (form: PromptBoxForm) => {
			await submit({ prompt: form.prompt, files: form.files });
		},
		[submit],
	);

	const handleMinimize = useCallback(() => {
		if (detailsRef.current) {
			detailsRef.current.open = false;
		}
	}, []);

	const handleToggleSize = useCallback(() => {
		setSize((prev) => (prev === 'normal' ? 'expanded' : 'normal'));
	}, []);

	const handleContentClick = useCallback((e: React.MouseEvent) => {
		e.stopPropagation();
	}, []);

	const handleContentKeyDown = useCallback((e: React.KeyboardEvent) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.stopPropagation();
		}
	}, []);

	if (pathname === '/assistant') return null;

	const sizeClasses = size === 'expanded' ? 'w-[700px] h-[764px]' : 'w-lg h-[576px]';

	const isDisabled = isLoading || isStreaming || assistantLoading || !assistant?.agent?.identifier.value;

	return (
		<div id="assistant-chat" className="absolute bottom-5 right-5">
			<details ref={detailsRef} className="dropdown dropdown-top dropdown-left">
				<summary className="btn btn-circle bg-base-content text-base-200 p-2 h-12 w-12 min-h-10">
					<Sparkles size={18} />
				</summary>
				{/** biome-ignore lint/a11y/noStaticElementInteractions: dropdown content */}
				<div className="dropdown-content z-[1] p-0" onClick={handleContentClick} onKeyDown={handleContentKeyDown}>
					<Card.Body
						className={cns(
							'border border-primary rounded-box p-0 flex flex-col bg-base-100 shadow-xl transition-all duration-300 ease-in-out',
							sizeClasses,
						)}>
						<div
							id="assistant-chat__header"
							className="flex-shrink-0 h-10 bg-primary text-primary-content justify-between items-center flex px-3">
							<span className="font-bold uppercase [font-variant:unicase]">Assistant chat</span>
							<div className="flex gap-1 justify-center items-center">
								<Button
									size="xs"
									ghost
									onClick={handleToggleSize}
									aria-label={size === 'normal' ? 'Expand chat' : 'Shrink chat'}>
									{size === 'normal' ? <Maximize2 size={14} /> : <Minimize2 size={14} />}
								</Button>
								<Button size="xs" ghost onClick={handleMinimize} aria-label="Minimize chat">
									<Minus size={14} />
								</Button>
							</div>
						</div>
						<div id="assistant-chat__chat" className="flex-1 overflow-hidden flex flex-col p-4">
							{assistantError ? (
								<div className="flex items-center justify-center h-full">
									<p className="text-error text-sm">{assistantError}</p>
								</div>
							) : (
								<Chat
									id="global_assistant_chat"
									messages={chatMessages}
									onSubmit={handleOnSubmit}
									disabled={isDisabled}
									isLoading={isLoading}
									isStreaming={isStreaming}
									onStopStreaming={stop}
									showLoading
									enableFileUpload={false}
									enableWebSearch={false}
									enableModelSelect={false}
								/>
							)}
						</div>
					</Card.Body>
				</div>
			</details>
		</div>
	);
}
