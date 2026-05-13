import type { AgentMessageProps } from './AgentMessage';
import type { UserMessageProps } from './UserMessage';

export { type AgentMessageProps, default as AgentMessage } from './AgentMessage';
export { default as UserMessage, type UserMessageProps } from './UserMessage';

export type ChatMessageProps = AgentMessageProps | UserMessageProps;
