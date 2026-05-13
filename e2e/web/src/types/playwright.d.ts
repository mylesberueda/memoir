declare global {
	interface Window {
		__playwright_console_messages?: string[];
	}
}

export {};
