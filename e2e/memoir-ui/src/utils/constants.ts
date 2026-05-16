export const ROUTES = {
	HOME: '/',
	LOGIN: '/auth/login',
	REGISTER: '/auth/registration',
	DASHBOARD: '/dashboard',
	SETTINGS: '/settings',
	ASSISTANT: '/assistant',
	AGENTS: '/agents',
	CONVERSATIONS: '/conversations',
};

export const TIMEOUTS = {
	SHORT: 5000,
	MEDIUM: 10000,
	LONG: 30000,
	LLM: 120000, // Extra time for LLM responses - some models are slow
};

/**
 * Generates a unique test account for the current test run.
 * Uses a timestamp + random suffix to ensure uniqueness across parallel runs.
 */
export function generateTestAccount() {
	const uniqueId = `${Date.now()}-${Math.random().toString(36).substring(2, 8)}`;
	return {
		email: `e2e-${uniqueId}@test.local`,
		password: 'TestPassword123!',
		name: `E2E Test ${uniqueId}`,
	};
}
