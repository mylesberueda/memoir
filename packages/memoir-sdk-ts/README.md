# @polypixel/memoir-sdk

Generated TypeScript SDK for [Memoir](https://github.com/mylesberueda/memoir)'s
`memoir-service`. Use this package when a TypeScript / JavaScript runtime
needs to talk to a running `memoir-service` over the network.

If you'd rather embed Memoir in-process — owning the Postgres connection,
Qdrant client, and embedding model yourself — use the Rust library
[`polypixel-memoir-core`](https://crates.io/crates/polypixel-memoir-core)
instead. The TS SDK and the Rust library are sibling surfaces of the same
product; pick the one that matches your runtime and deployment shape.

## Install

```bash
pnpm add @polypixel/memoir-sdk @connectrpc/connect-node
```

On the browser / edge, swap `@connectrpc/connect-node` for
`@connectrpc/connect-web` and use `createConnectTransport` (memoir-service
speaks both gRPC and Connect protocols).

## Quick start

```ts
import { createClient } from '@connectrpc/connect';
import { createGrpcTransport } from '@connectrpc/connect-node';
import { MemoryService } from '@polypixel/memoir-sdk/memoir/v1/memory_pb';

const transport = createGrpcTransport({
	baseUrl: 'http://localhost:5153',
});

const memory = createClient(MemoryService, transport);

const response = await memory.search(
	{
		scope: {
			agentId: 'my-agent',
			orgId: 'my-org',
			userId: 'user-42',
		},
		query: 'coffee preference',
		limit: 5,
	},
	{
		headers: { authorization: 'Bearer <jwt>' },
	},
);

for (const hit of response.hits) {
	if (hit.memory) {
		console.log(hit.memory.content);
	}
}
```

`memoir-service` requires an `Authorization: Bearer <jwt>` or
`X-Api-Key: mk.<id>.<secret>` header on every authenticated RPC. Attach it
per call via the `headers` option (as above), or via a Connect-RPC
[interceptor](https://connectrpc.com/docs/web/interceptors).

## Rendered prompt context

Every read RPC (`search`, `recall`, `timeline`, `recallAsOf`, `query`) always
returns a `rendered` field: prompt-ready text — a system-prompt preamble
followed by one bullet per memory — produced server-side by memoir-core's own
rendering. The optional `template` on the request chooses the preamble: leave
it unset to use memoir's default phrasing (you never copy the string), or set
it to supply your own (the empty string renders a blank preamble line):

```ts
const response = await memory.query({
	scope: { agentId: 'my-agent', orgId: 'my-org', userId: 'user-42' },
	query: 'what does the user drink?',
	// template omitted → memoir's default preamble; set a string for your own
});

console.log(response.rendered);
```

Query's bullets are dated (`- [YYYY-MM-DD, N units ago] content`); the other
reads render `- content`, mirroring the library.

## What's in the package

- Generated Connect-RPC service definitions: `MemoryService`, `AdminService`,
  `AuthService` under `@polypixel/memoir-sdk/memoir/v1/{memory,admin,auth}_pb`.
- Message types and schemas for every proto in `memoir.v1`.
- Pure ESM. Modern bundlers and Node ≥18 follow the `exports` map natively;
  no CommonJS shim is shipped.

The SDK is regenerated from `.proto` sources by the upstream repository's
`gen:protos` Nx target; consumers do not need `buf` or `protoc` installed.

## License

Dual-licensed under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE),
at your option.
