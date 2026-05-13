'use server';

import { create } from '@bufbuild/protobuf';
import { MeRequestSchema, type User } from '@startup/proto-ts/api-service/api/v1/users_pb';
import { getAuthProvider } from '@/lib/auth';
import { userServiceClient } from '@/lib/grpc/clients';
import { deleteAllUserSessions, getSession } from '@/lib/session';

/**
 * Get the current user's profile from the api-service.
 * This is an example of how to use gRPC clients in server actions.
 */
export async function getMe(): Promise<User | null> {
	const client = await userServiceClient();
	if (!client) {
		// User is not authenticated
		return null;
	}

	try {
		const request = create(MeRequestSchema, {});
		const response = await client.me(request);
		return response.user ?? null;
	} catch (error) {
		console.error('Failed to get user profile:', error);
		return null;
	}
}

export interface ChangePasswordResult {
	success: boolean;
	error?: string;
}

/**
 * Change the current user's password.
 * On success, all user sessions are revoked (user must log in again).
 */
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

	// Revoke all sessions - user must log in again with new password
	await deleteAllUserSessions(session.userId);

	return { success: true };
}
