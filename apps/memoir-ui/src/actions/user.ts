'use server';

import { getAuthProvider } from '@/lib/auth';
import type { User } from '@/lib/proto-shims';
import { deleteAllUserSessions, getSession } from '@/lib/session';

export type { User };

export async function getMe(): Promise<User | null> {
	return null;
}

export interface ChangePasswordResult {
	success: boolean;
	error?: string;
}

export async function changePassword(currentPassword: string, newPassword: string): Promise<ChangePasswordResult> {
	const session = await getSession();
	if (!session) {
		return { success: false, error: 'Not authenticated' };
	}

	const provider = getAuthProvider();
	const result = await provider.changePassword(session.userId, currentPassword, newPassword);

	if (!result.success) {
		return { success: false, error: result.error };
	}

	await deleteAllUserSessions(session.userId);

	return { success: true };
}
