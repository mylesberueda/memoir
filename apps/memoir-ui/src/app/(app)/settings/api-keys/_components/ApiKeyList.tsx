'use client';

import { timestampDate } from '@bufbuild/protobuf/wkt';
import { Badge } from '@components';
import { type ApiKey, ApiKeyRole, ApiKeyStatus } from '@polypixel/memoir-sdk/memoir/v1/auth_pb';
import { Ban, KeyRound, RefreshCw } from 'lucide-react';

interface ApiKeyListProps {
	keys: ApiKey[];
	busyPid: string | null;
	onRotate: (key: ApiKey) => void;
	onRevoke: (key: ApiKey) => void;
}

function formatDate(ts: ApiKey['createdAt']): string {
	if (!ts) return '—';
	return timestampDate(ts).toLocaleDateString(undefined, {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
	});
}

export default function ApiKeyList({ keys, busyPid, onRotate, onRevoke }: ApiKeyListProps) {
	if (keys.length === 0) {
		return (
			<div id="api_key_list__empty" className="rounded-lg border border-base-300 border-dashed p-10 text-center">
				<KeyRound className="mx-auto h-8 w-8 text-base-content/40" />
				<p className="mt-3 font-medium">No API keys yet</p>
				<p className="mt-1 text-base-content/70 text-sm">
					Create a key to authenticate a downstream service against Memoir.
				</p>
			</div>
		);
	}

	return (
		<div id="api_key_list__container" className="overflow-x-auto">
			<table className="table">
				<thead>
					<tr>
						<th>Name</th>
						<th>Key id</th>
						<th>Role</th>
						<th>Status</th>
						<th>Created</th>
						<th>Last used</th>
						<th />
					</tr>
				</thead>
				<tbody>
					{keys.map((key) => {
						const revoked = key.status === ApiKeyStatus.REVOKED;
						const busy = busyPid === key.pid;
						return (
							<tr key={key.pid} className={revoked ? 'opacity-60' : undefined}>
								<td className="font-medium">
									{key.name}
									{key.orgId && <span className="ml-2 text-base-content/60 text-xs">org: {key.orgId}</span>}
								</td>
								<td>
									<code className="font-mono text-xs">mk.{key.keyId}</code>
								</td>
								<td>
									<Badge color={key.role === ApiKeyRole.ADMIN ? 'warning' : 'neutral'} size="sm">
										{key.role === ApiKeyRole.ADMIN ? 'admin' : 'integration'}
									</Badge>
								</td>
								<td>
									<Badge color={revoked ? 'error' : 'success'} size="sm">
										{revoked ? 'revoked' : 'active'}
									</Badge>
								</td>
								<td className="text-sm">{formatDate(key.createdAt)}</td>
								<td className="text-sm">{key.lastUsedAt ? formatDate(key.lastUsedAt) : 'never'}</td>
								<td className="text-right">
									{!revoked && (
										<div className="flex justify-end gap-1">
											<button
												type="button"
												className="btn btn-ghost btn-xs gap-1 tooltip"
												data-tip="Regenerate"
												aria-label={`Regenerate ${key.name}`}
												disabled={busy}
												onClick={() => onRotate(key)}>
												<RefreshCw className="h-3 w-3" />
											</button>
											<button
												type="button"
												className="btn btn-ghost btn-xs gap-1 text-error tooltip"
												data-tip="Revoke"
												aria-label={`Revoke ${key.name}`}
												disabled={busy}
												onClick={() => onRevoke(key)}>
												<Ban className="h-3 w-3" />
											</button>
										</div>
									)}
								</td>
							</tr>
						);
					})}
				</tbody>
			</table>
		</div>
	);
}
