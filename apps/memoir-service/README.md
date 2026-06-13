# memoir-service

A gRPC server over [`memoir-core`](../../packages/memoir-core/README.md), adding local auth (JWT + API keys). Run it when you want a memory backend other processes or languages talk to over the network. If you're writing a Rust agent, embed [`memoir-core`](../../packages/memoir-core/README.md) in-process instead.

## What it serves

Three gRPC services on one port, plus gRPC health and reflection. Run `grpcurl <host>:<port> list` to enumerate everything (reflection is registered, so no local `.proto` files needed). Network clients use the generated SDKs: [`polypixel-memoir-sdk`](https://crates.io/crates/polypixel-memoir-sdk) (crates.io) or `@polypixel/memoir-sdk` (npm).

| Service | RPCs |
|---|---|
| `MemoryService` | `remember`, `search`, `query`, `recall`, `timeline`, `recall_as_of`, `edit`, `feedback`, `forget`, `supersession_history`, `list_agents` |
| `AdminService` | failed-job triage, `reconcile`, `unsupersede`, `extraction_stats`, `inspect_graph` |
| `AuthService` | bootstrap, login, refresh, users, API keys |

## Running it

The image is published to `ghcr.io/mylesberueda/memoir/memoir-service`. It needs a Postgres database and a Qdrant instance; bring both up locally with the compose stack at the repo root:

```bash
docker compose --profile dbs up -d        # Postgres + Qdrant
```

Then run the service against them:

```bash
docker run --rm -p 5153:5153 \
  -e DATABASE_URL=postgres://postgres:postgres@host.docker.internal:54321/memoir_service \
  -e QDRANT_URL=http://host.docker.internal:6334 \
  -e JWT_SECRET=$(openssl rand -base64 32) \
  ghcr.io/mylesberueda/memoir/memoir-service:latest
```

On first start, with no admin in the database, the service prints a one-time **bootstrap token** to stdout — exchange it for the first admin (see [Authentication](#authentication)).

> **Create the database first.** Migrations create Memoir's schemas and tables, never the database itself; the server won't start if it can't connect. The compose Postgres seeds only a database called `memoir`, so if `DATABASE_URL` points at `memoir_service` (as `.env.example` does), create it yourself first: `createdb -h localhost -p 54321 -U postgres memoir_service`.

### Ports

| Port | Default | Purpose |
|---|---|---|
| gRPC | `5153` | All three services, health, reflection. Override with `-e PORT=…`. |
| HTTP | `5154` | Playground chat UI. Don't expose it in production. Override with `-e HTTP_PORT=…`. |

## Configuration

Environment-driven; the canonical list with dev defaults is in [`.env.example`](.env.example).

### Required

| Variable | Purpose |
|---|---|
| `DATABASE_URL` | Postgres connection string. The database must pre-exist. |
| `QDRANT_URL` | Qdrant endpoint. Include the scheme (`http://host:6334`) — it's passed to the Qdrant client verbatim. |
| `JWT_SECRET` | Signing secret for access/refresh JWTs. Must be base64 that decodes to ≥32 bytes (`openssl rand -base64 32`) or startup fails. Rotating it invalidates all live sessions. |

### Optional

| Variable | Default | Purpose |
|---|---|---|
| `HOST` | `0.0.0.0` | Bind address. |
| `PORT` | `5153` | gRPC port. |
| `HTTP_PORT` | `5154` | Playground HTTP port. |
| `SERVICE_SCHEMA` | `memoir_service` | Postgres schema for the auth/tenant tables (co-tenants a shared Postgres). |
| `CORE_SCHEMA` | `memoir` | Postgres schema for the memory tables. |

### Extraction (optional)

Without these, the service is a scoped vector store — it stores and searches what you write but derives no semantic facts. Set the provider to enable the extraction worker.

| Variable | Purpose |
|---|---|
| `EXTRACTION_PROVIDER` | `ollama`, `openai`, or `anthropic`. Unset → embed-only. An unrecognized value fails startup. |
| `EXTRACTION_URL` | Provider endpoint (e.g. `http://localhost:11434` for Ollama). |
| `EXTRACTION_MODEL` | Model id; provider default if unset. |
| `EXTRACTION_API_KEY` | Required for `openai`/`anthropic`; startup fails if missing. |

### Knowledge graph (optional)

Writes entities and relationships extracted from memories into a [FalkorDB](https://falkordb.com) instance and enriches reads with graph context.

| Variable | Default | Purpose |
|---|---|---|
| `FALKOR_URL` | — | FalkorDB endpoint, Redis-protocol (`redis://host:6379`). Set → validated at startup; an unreachable endpoint fails the boot. |
| `GRAPH_NAME` | `memoir` | Named graph to write to (isolates co-tenants). |

**`FALKOR_URL` alone won't populate the graph** — set a relational provider too, or the graph stays empty. Same variable shape as extraction, separate so triples can use a different model:

| Variable | Purpose |
|---|---|
| `RELATIONAL_PROVIDER` | `ollama`, `openai`, or `anthropic`. Unset → no triples are extracted. |
| `RELATIONAL_URL` | Provider endpoint. |
| `RELATIONAL_MODEL` | Model id; provider default if unset. |
| `RELATIONAL_API_KEY` | Required for `openai`/`anthropic`. |

### Dev-mode bootstrap (local only)

Skips the bootstrap-token step and creates an admin from env vars on first start. Don't use in production — these env vars leak to `docker inspect` and logs.

| Variable | Purpose |
|---|---|
| `DEV_MODE` | `true` enables it. |
| `DEV_ADMIN_USERNAME` | Admin username. Must be email-shaped — the `memoir-ui` login form validates email format. |
| `DEV_ADMIN_PASSWORD` | Admin password. |

## Authentication

Every RPC except the auth-path RPCs requires `authorization: Bearer <token>`. Two credential types:

- **User JWTs** — log in with username + password for an access/refresh pair. The access token authenticates calls; the refresh token mints a new one when it expires.
- **API keys** — long-lived tokens of shape `mk.<key_id>.<secret>` for service-to-service callers. Each key has a role (`admin` or `integration`) and can be scope-bound to an `org_id`.

### First admin (bootstrap)

The bootstrap token (24h TTL) is logged to stdout on first start. Consume it:

```bash
grpcurl -plaintext -d '{
  "token": "<token-from-stdout>",
  "username": "admin@example.com",
  "password": "<password>"
}' localhost:5153 memoir.v1.AuthService/ConsumeBootstrapToken
```

Single-use; invalidated once an admin exists. (In dev, `DEV_MODE` replaces this step.)

### Integration key

Log in as the admin, then mint a key with that access token — the plaintext is returned once.

```bash
grpcurl -plaintext -d '{"username":"admin@example.com","password":"<password>"}' \
  localhost:5153 memoir.v1.AuthService/Login

grpcurl -plaintext \
  -H 'authorization: Bearer <access_token>' \
  -d '{"name":"agent-backend","role":2}' \
  localhost:5153 memoir.v1.AuthService/CreateApiKey
```

`role:2` is integration (`1` is admin). Store the returned `plaintext` — it's never shown again. Rotate with `RotateApiKey`, disable with `RevokeApiKey`. Full contract in [`proto/memoir/v1/auth.proto`](proto/memoir/v1/auth.proto).

## Deployment

Stateless binary; all state lives in Postgres, Qdrant, and (optionally) FalkorDB.

- **Health probes.** The gRPC health service reports `SERVING` for all three services — target it with a gRPC health-check probe.
- **Writable HOME.** On first use, `fastembed` downloads the embedding model (BGE-small-en-v1.5, ~50 MB) and caches it under `$HOME`. A read-only-root deployment must mount a writable HOME (or model-cache) volume, or every start re-downloads.
- **Fail-fast startup.** A missing/unreachable database, a missing extraction key, or an unknown provider fails startup. Size the liveness probe's initial delay to allow first-start migrations.

For the production deploy model (GitOps, ArgoCD), see [`infrastructure/DEPLOY.md`](../../infrastructure/DEPLOY.md).

### Deploying FalkorDB

- Use the **`falkordb/falkordb-server`** image (server only), not `falkordb/falkordb` (bundles the browser UI). The dev `docker-compose.yml` uses the bundled image for convenience.
- FalkorDB takes its password as `REDIS_ARGS=--requirepass <password>`; reach it via the `redis://` scheme in `FALKOR_URL`.

## Building from source

```bash
cargo run -p memoir-service -- server start   # against the local compose stores
docker build --target runtime -t memoir-service:local -f apps/memoir-service/Dockerfile .
```

## License

Licensed under either of [Apache License 2.0](../../LICENSE-APACHE) or [MIT](../../LICENSE-MIT) at your option.
