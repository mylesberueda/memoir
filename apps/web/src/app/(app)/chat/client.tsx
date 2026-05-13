'use client';

import type { ListModelsResponse } from '@actions/models';
import { create } from '@bufbuild/protobuf';
import { Chat } from '@components';
import type { ChatProps } from '@components/Chat';
import type { PromptBoxForm } from '@components/Chat/PromptInput';
import { MessagePartKind, MessagePartStatus } from '@lib/chat-state';
import { MessagePartSchema } from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import { sub } from 'date-fns';
import { useCallback } from 'react';

const MOCK_MESSAGES: ChatProps['messages'] = [
	{
		id: 'mock-1',
		variant: 'rx',
		timestamp: new Date(sub(new Date(), { minutes: 11 })),
		avatar: 'https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp',
		parts: [
			create(MessagePartSchema, {
				id: 'p1',
				kind: MessagePartKind.TEXT,
				status: MessagePartStatus.COMPLETE,
				content: 'Wake up!',
			}),
		],
	},
	{
		id: 'mock-2',
		variant: 'tx',
		timestamp: new Date(sub(new Date(), { minutes: 10 })),
		parts: [
			create(MessagePartSchema, {
				id: 'p2',
				kind: MessagePartKind.TEXT,
				status: MessagePartStatus.COMPLETE,
				content: 'Hi there',
			}),
		],
	},
	{
		id: 'mock-3',
		variant: 'rx',
		timestamp: new Date(sub(new Date(), { minutes: 9 })),
		avatar: 'https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp',
		parts: [
			create(MessagePartSchema, {
				id: 'p3',
				kind: MessagePartKind.TEXT,
				status: MessagePartStatus.COMPLETE,
				content: 'Why, hello there',
			}),
		],
	},
];

interface ChatPageProps {
	models?: ListModelsResponse;
	modelsError?: Error;
}

export default function ChatPage({ models, modelsError }: ChatPageProps) {
	const handleOnSubmit = useCallback(async (v: PromptBoxForm) => {
		console.log(v);
	}, []);

	return (
		<div className="h-full p-6">
			<Chat
				messages={MOCK_MESSAGES}
				onSubmit={handleOnSubmit}
				models={models?.models}
				modelsLoading={false}
				modelsError={modelsError}
			/>
		</div>
	);
}
