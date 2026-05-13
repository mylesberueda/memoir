import { create } from '@bufbuild/protobuf';
import {
	MessagePartKind,
	MessagePartSchema,
	MessagePartStatus,
} from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import UserMessage from './UserMessage';

const createPart = (id: string, content: string) =>
	create(MessagePartSchema, { kind: MessagePartKind.TEXT, id, content, status: MessagePartStatus.COMPLETE });

const meta: Meta<typeof UserMessage> = {
	title: 'Components/Chat/UserMessage',
	component: UserMessage,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component: 'User message component for displaying user-sent messages with avatar.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		avatar: {
			control: 'text',
			description: 'Avatar URL for the user',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Basic: Story = {
	args: {
		id: 'user-1',
		variant: 'rx',
		timestamp: new Date(),
		parts: [createPart('text-1', 'This is a user message. It appears on the right side of the conversation.')],
	},
};

export const WithAvatar: Story = {
	args: {
		id: 'user-avatar-1',
		variant: 'rx',
		timestamp: new Date(),
		avatar: 'https://avatars.githubusercontent.com/u/12345678?v=4',
		parts: [createPart('text-1', 'This message shows a custom user avatar.')],
	},
};

export const MultiLine: Story = {
	args: {
		id: 'user-multiline-1',
		variant: 'rx',
		timestamp: new Date(),
		parts: [
			createPart('text-1', 'This is a longer message\nthat spans multiple lines\nto test whitespace preservation.'),
		],
	},
};
