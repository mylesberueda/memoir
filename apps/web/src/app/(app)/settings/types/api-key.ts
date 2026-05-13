export interface ApiKey {
	id: string;
	name: string;
	is_active: boolean;
	user_id: string;
	created_at: string;
	last_used_at: string | null;
}

export interface ApiKeyCreateRequest {
	name: string;
}

export interface ApiKeyCreateResponse extends ApiKey {
	key: string; // Only returned once upon creation
}

export interface ApiKeysListResponse {
	data: ApiKey[];
	count: number;
}
