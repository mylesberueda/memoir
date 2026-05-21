import 'server-only';

import { type Client, createClient } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import { AuthService } from '@polypixel/memoir-sdk/memoir/v1/auth_pb';

/**
 * Server-to-server transport pointed at memoir-service's gRPC endpoint.
 *
 * Configured once at module load. `MEMOIR_SERVICE_URL` must be set; the
 * Next.js server-side routes that call into this transport panic loudly at
 * import time if it's missing, which is preferable to silent fall-through
 * to a default localhost that may not match the running container.
 */
function transport() {
	const baseUrl = process.env.MEMOIR_SERVICE_URL;
	if (!baseUrl) {
		throw new Error('MEMOIR_SERVICE_URL must be set for the memoir-ui server to reach memoir-service');
	}
	return createGrpcTransport({ baseUrl });
}

/**
 * Connect-RPC client for memoir-service's AuthService.
 *
 * Used by the AuthServiceAuthProvider and the /login API route. The client
 * itself carries no credentials — callers attach `authorization: Bearer
 * <jwt>` or `x-api-key: mk.<id>.<secret>` headers per call via the
 * `headers` option on each RPC.
 */
export function authClient(): Client<typeof AuthService> {
	return createClient(AuthService, transport());
}
