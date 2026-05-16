/**
 * Placeholder shims for the deleted SDK proto types.
 *
 * These types previously lived in the generated `@polypixel/memoir-sdk/*`
 * proto subtrees for services that no longer exist (api-service). This module
 * re-states just enough of that surface for the memoir-ui to keep
 * type-checking until the memoir-server scaffold lands and real types come
 * back from the new SDK.
 *
 * Scope: only the api-service domain types still consumed by surviving UI
 * surfaces (organizations, users, admin). Rig-service / chat-service /
 * notification-service / billing surfaces were deleted along with their UI;
 * their placeholder types were removed with them.
 */

// ─── users_pb shims ──────────────────────────────────────────────────────────

export type User = {
	id: string;
	email: string;
	name: string;
};

// ─── organizations_pb shims ──────────────────────────────────────────────────

export type Organization = {
	pid: string;
	name: string;
	slug: string;
};

export type OrganizationMember = {
	pid: string;
	userId: string;
	role: string;
	email?: string;
	displayName?: string;
	createdAt?: string;
};

export type ResourcePermission = {
	resource: string;
	read?: boolean;
	write?: boolean;
	execute?: boolean;
};
