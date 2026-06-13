# Memoir — Docker deployment

Copy this directory into your project and run the full Memoir stack — the gRPC
memory service, the admin console, and their backing stores — with one command.

```
docker/
├── docker-compose.yml
├── .env.example
└── README.md
```

## What it runs

| Container | Image | Purpose | Host port (default) |
|---|---|---|---|
| `memoir-service` | `ghcr.io/mylesberueda/memoir/memoir-service` | gRPC memory API | `5153` |
| `memoir-ui` | `ghcr.io/mylesberueda/memoir/memoir-ui` | Admin web console | `3000` |
| `postgres` | `pgvector/pgvector:pg17` | Source of truth | `54321` |
| `qdrant` | `qdrant/qdrant` | Vector index | `6333` / `6334` |
| `redis` | `redis:alpine` | UI sessions | `63791` |
| `falkordb` | `falkordb/falkordb` | Knowledge graph (opt-in) | `63792` |

`falkordb` only starts with the `knowledge-graph` profile; everything else starts by default.

## Quick start

```bash
cp .env.example .env

# Fill in the two secrets — both must be ≥32 bytes.
# MEMOIR_SERVICE_JWT_SECRET must be base64 (the service rejects a short/non-base64 value at startup).
openssl rand -base64 32   # → MEMOIR_SERVICE_JWT_SECRET
openssl rand -base64 32   # → MEMOIR_UI_SESSION_SECRET

docker compose up -d
```

The service migrates the database on startup and prints a one-time **bootstrap token** to its logs:

```bash
docker compose logs memoir-service | grep -A2 BOOTSTRAP
```

Exchange it for the first admin (see [Creating the first admin](#creating-the-first-admin)), then open the console at `http://localhost:3000`.

To run with the knowledge graph:

```bash
docker compose --profile knowledge-graph up -d
```

…and set `MEMOIR_SERVICE_FALKOR_URL=redis://falkordb:6379` plus a `MEMOIR_SERVICE_RELATIONAL_PROVIDER` in `.env` (see [Knowledge graph](#knowledge-graph)).

## Configuration

Every setting is an environment variable in `.env`, prefixed by the container it
configures: `MEMOIR_SERVICE_*` for the service, `MEMOIR_UI_*` for the console.
The compose file maps each prefixed var to the bare name the app inside the
container actually reads (e.g. `MEMOIR_SERVICE_JWT_SECRET` → `JWT_SECRET`), so
the two services never collide on a shared name. You only ever edit `.env`.

### Internal vs. host ports

Containers reach each other over the compose network by **service name on the
container port** — `postgres:5432`, `qdrant:6334`, `memoir-service:5153` — not
`localhost`. The `*_PORT` vars only control what's **published to your host**;
changing them does not change the internal URLs.

### What you must change before exposing this

- `MEMOIR_SERVICE_JWT_SECRET` and `MEMOIR_UI_SESSION_SECRET` — replace the
  placeholders with real random secrets.
- `MEMOIR_SERVICE_DB_PASS` — change off `postgres`.
- `MEMOIR_SERVICE_DEV_MODE` — leave `false`. `true` creates an admin from
  plaintext env vars and is for local development only.

## Creating the first admin

With no admin in the database, the service logs a one-time bootstrap token (24h
TTL). Consume it with [`grpcurl`](https://github.com/fullstorydev/grpcurl):

```bash
grpcurl -plaintext -d '{
  "token": "<token-from-logs>",
  "username": "admin@example.com",
  "password": "<password>"
}' localhost:5153 memoir.v1.AuthService/ConsumeBootstrapToken
```

Then log in at `http://localhost:3000`. (For local development only, set
`MEMOIR_SERVICE_DEV_MODE=true` with `MEMOIR_SERVICE_DEV_ADMIN_USERNAME` /
`_PASSWORD` to skip this and have the admin created on first start.)

## Extraction (turning conversations into facts)

Out of the box the service stores and vector-searches what you write but derives
no semantic facts. Point it at an LLM to enable the extraction worker:

```bash
MEMOIR_SERVICE_EXTRACTION_PROVIDER=openai      # or ollama, anthropic
MEMOIR_SERVICE_EXTRACTION_MODEL=gpt-4o-mini
MEMOIR_SERVICE_EXTRACTION_API_KEY=sk-…
```

For Ollama, set `MEMOIR_SERVICE_EXTRACTION_URL` to a reachable host (from inside
the container, your host is `http://host.docker.internal:11434`).

## Knowledge graph

Opt-in entity/relationship enrichment backed by FalkorDB. Start the profile and
set the connection:

```bash
docker compose --profile knowledge-graph up -d
```

```bash
MEMOIR_SERVICE_FALKOR_URL=redis://falkordb:6379
MEMOIR_SERVICE_RELATIONAL_PROVIDER=openai      # or ollama, anthropic
MEMOIR_SERVICE_RELATIONAL_MODEL=gpt-4o-mini
MEMOIR_SERVICE_RELATIONAL_API_KEY=sk-…
```

`FALKOR_URL` alone wires the connection but the graph stays empty — it needs a
relational provider to extract triples.

## Operating

```bash
docker compose logs -f memoir-service     # follow service logs
docker compose pull && docker compose up -d   # upgrade to latest images
docker compose down                       # stop (volumes persist)
docker compose down -v                    # stop and delete all data
```

State lives in the `postgres`, `qdrant`, `redis`, and `falkordb` named volumes;
`down` without `-v` keeps it.
