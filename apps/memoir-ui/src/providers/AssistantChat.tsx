'use client';

import { createContext, useContext, useState } from 'react';

interface AssistantChatContextType {
	isSidebarOpen: boolean;
	openSidebar: () => void;
	closeSidebar: () => void;
	toggleSidebar: () => void;
}

const AssistantChatContext = createContext<AssistantChatContextType | undefined>(undefined);

interface AssistantChatProviderProps {
	children: React.ReactNode;
}

export function AssistantChatProvider({ children }: AssistantChatProviderProps) {
	const [isSidebarOpen, setIsSidebarOpen] = useState<boolean>(false);

	const openSidebar = () => setIsSidebarOpen(true);
	const closeSidebar = () => setIsSidebarOpen(false);
	const toggleSidebar = () => setIsSidebarOpen((prev) => !prev);

	return (
		<AssistantChatContext.Provider value={{ isSidebarOpen, openSidebar, closeSidebar, toggleSidebar }}>
			{children}
		</AssistantChatContext.Provider>
	);
}

export function useAssistantChatContext(): AssistantChatContextType {
	const context = useContext(AssistantChatContext);
	if (!context) {
		throw new Error('useAssistantChatContext must be used within a AssistantChatProvider');
	}
	return context;
}
