# CLAUDE.md - Project-Specific Guidelines

**Applies to**: Startup AI monorepo
**Architecture docs**: See `.agents/` directory (ARCHITECTURE.md, AUTH.md, TESTING.md, COMMANDS.md, MCP.md)

---

## 🚨 WORKSPACE COMMANDS

**ALWAYS run commands from workspace root using `pnpm nx`:**

```bash
# Testing
pnpm nx run api:test              # Unit tests
pnpm nx run api:test:integration  # Integration tests
pnpm nx run-many --target test --all

# Building
pnpm nx run-many --target build --all

# Linting
pnpm nx run-many --target lint --parallel
```

**WHY**: `pnpm nx` from workspace root loads project `.env` files automatically. Running commands in project directories (e.g., `cd apps/api && cargo test`) will FAIL.

**EXCEPTION**: Database migrations must run in project directories. Developer will specify.

---

## 🚨 SERVICE MANAGEMENT

**YOU NEVER start/stop/restart services or databases.** Developer manages all services.

**When services need attention:**

```
"Service X appears down. Please restart it:
  pnpm nx run api:dev
in a separate terminal."
```

**NO docker starts/builds/restarts. NO database creation.**

---

## 🚨 ENVIRONMENT CONFIGURATION

| File Type | Loaded By | Purpose |
|-----------|-----------|---------|
| `apps/<service>/.env` | `pnpm nx` commands | Service configuration (FUNCTIONAL) |
| Root `.env` | Docker Compose | Container configuration (PostgreSQL, Redis, Zitadel) |
| `.env.zitadel` files | Nothing | Documentation examples only |

---

## 🚨 MONOREPO STRUCTURE

### Apps (Services)

| Project | Tech | Purpose |
|---------|------|---------|
| `apps/api-service/` | Rust/Loco | Main REST API - auth, users, billing (Stripe), sessions |
| `apps/chat-service/` | Rust/gRPC | Chat microservice - real-time messaging, Redis pub/sub |
| `apps/notification-service/` | Rust/gRPC | Notification microservice - alerts, emails |
| `apps/rig-service/` | Rust/gRPC | AI agent microservice - agent orchestration (Rig/Kameo) |
| `apps/example-grpc-service/` | Rust/gRPC | Template/example for new gRPC services |
| `apps/web/` | Next.js 14 | User-facing web application |

### Packages (Shared Libraries)

| Package | Tech | Purpose |
|---------|------|---------|
| `packages/common-rs/` | Rust | Shared utilities - encryption (AES-GCM), hashing (Argon2) |
| `packages/proto-rs/` | Rust | Generated gRPC/protobuf types for Rust services |
| `packages/proto-ts/` | TypeScript | Generated gRPC/protobuf types for TS clients |

### Other Directories

| Directory | Purpose |
|-----------|---------|
| `cli/` | Rust CLI tool for startup.ai management |
| `infrastructure/` | Kubernetes manifests and Terraform configs. **Before any IaC change, read `infrastructure/IAC_RULES.md`** — it documents the forward-compatibility rules every infra change must satisfy (compute is commodity, state lives off-cluster, edge is fourth-party, etc.) and the bar for exceptions. |
| `e2e/` | End-to-end tests (Playwright for `apps/web`) |

**Authentication**: Users authenticate via API's `/api/v1/auth/login` (email/password → HS256 JWT). Service-to-service auth uses Zitadel RS256 JWT. See `.agents/AUTH.md`.

---

## 🚨 TESTING RULES (Project-Specific)

**Unit tests:**

- NO external services (database, APIs, network)
- Must pass without services running
- Fast (milliseconds)

**Integration tests:**

- Real services allowed
- Requires services running
- Slower (seconds acceptable)

**E2E tests:**

- NO mocks (real system integration)
- Browser automation
- Only in `apps/web`

**Tests MUST fail (not skip) when something is wrong.**
