# Memoir

A memory substrate for AI agents. Memoir stores what an agent is told, derives durable facts from it, and serves them back — ranked, correctable, and scoped per tenant.

You write conversation turns; a background worker runs LLM extraction over them to produce semantic facts. Recall reads the fact layer; the raw turns stay as the audit trail and the source those facts are re-derived from. Postgres is the source of truth, Qdrant is the vector index, and Memoir owns the schema, the embedding model, and the write-behind queue that keeps the two consistent.

## Two surfaces

Memoir ships as a library or a service. Same engine, different boundary.

- **`memoir-core`** — an embeddable Rust library. `cargo add polypixel-memoir-core`, bring your own Postgres + Qdrant, and call it in-process. No auth: the host process is the trust boundary. This is everything — memory, embedding, extraction, the worker.
- **`memoir-service`** — a gRPC adapter over the library, shipped as a Docker image. Adds local auth (JWT + API keys) and exposes the surface over the wire. A thin wrapper: every handler unwraps the request, calls the library, wraps the response. Network clients use the generated SDKs (`polypixel-memoir-sdk` on crates.io, `@polypixel/memoir-sdk` on npm).

Pick the library if you're writing a Rust agent. Pick the service if you want a memory backend other processes or languages talk to.

## Features

- **Scoped memory.** Every write and read is partitioned by an `(agent, org, user)` tuple. One tenant never sees another's memories.
- **Episodic capture, semantic recall.** Writes are raw turns; the worker extracts facts from them asynchronously. You query the facts.
- **Vector search and ranked query.** `search` is raw nearest-neighbor. `query` re-ranks by a tunable blend of cosine, confidence, recency, and category, and returns prompt-shaped context.
- **Temporality.** Facts carry an event-time distinct from when Memoir was told. Read the chronological `timeline`, or `recall_as_of` a past instant to get the state of knowledge as it stood then.
- **Categorization and confidence.** Extracted facts carry a confidence score and an opt-in NLI category label, both usable as ranking signals or hard filters.
- **Correction by teaching.** Semantic facts are never hand-edited. A wrong fact is corrected with `feedback` — Memoir re-derives from the source. Edit the source itself and the derived facts cascade. Retirements are tracked as `rejected` (a wrong extraction) or `stale` (the source changed); `extraction_stats` reports accuracy per model.
- **Durable by construction.** The write-behind queue is Postgres-backed and survives crashes. Failed jobs surface to an admin view; `reconcile` retries them and sweeps orphaned vectors.
- **Pluggable models.** Extraction runs against Ollama, OpenAI, or Anthropic via `LlmConfig`. The categorizer is any zero-shot NLI model via `NliConfig`. Both are optional — leave them out and those stages simply skip.

## Library quick start

```toml
[dependencies]
polypixel-memoir-core = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

You need a Postgres database (with pgvector) and a Qdrant instance. `docker compose --profile dbs up -d` brings both up locally.

```rust
use memoir_core::client::Client;
use memoir_core::memory::Scope;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .database_url("postgres://postgres:postgres@localhost:54321/memoir")
        .qdrant("http://localhost:6334")
        .build()
        .await?;

    client.migrate().await?;
    let worker = client.spawn_worker().start().await?;

    let scope = Scope {
        agent_id: "my-agent".into(),
        org_id: "my-org".into(),
        user_id: "user-42".into(),
    };

    client.remember("the user prefers dark roast coffee", scope.clone()).await?;

    let hits = client.search("coffee preference", scope).limit(5).await?;
    for m in hits.list() {
        println!("{}", m.content);
    }

    worker.shutdown().await;
    Ok(())
}
```

Extraction and categorization are opt-in on the builder: `.extraction_llm(LlmConfig::ollama(url, model))` turns episodic turns into semantic facts; `.categorize_model(NliConfig::default())` labels them. Without them, Memoir is a scoped vector store. See [`examples/library-quickstart.rs`](packages/memoir-core/examples/library-quickstart.rs) for the full lifecycle and [`packages/memoir-core/README.md`](packages/memoir-core/README.md) for the API surface.

## Service quick start

Run the service when you want a memory backend other processes talk to over gRPC.

```bash
docker compose --profile dbs up -d   # Postgres + Qdrant

docker run --rm -p 5153:5153 \
  -e DATABASE_URL=postgres://postgres:postgres@host.docker.internal:54321/memoir_service \
  -e QDRANT_URL=http://host.docker.internal:6334 \
  -e MEMOIR_JWT_SECRET=$(openssl rand -base64 32) \
  ghcr.io/mylesberueda/memoir/memoir-service:latest
```

Migrations run at startup. The service exposes three gRPC services on port 5153 — `MemoryService` (remember, search, query, recall, timeline, recall-as-of, edit, feedback, forget), `AdminService` (failed-job triage, reconcile, extraction stats), and `AuthService` (bootstrap, login, users, API keys). Auth is local: a bootstrap token creates the first admin, then JWTs and `mk.*` API keys gate every RPC.

Configuration is environment-driven — `DATABASE_URL`, `QDRANT_URL`, `MEMOIR_JWT_SECRET` are required; `SERVICE_SCHEMA`/`CORE_SCHEMA` isolate the auth and memory tables; `MEMOIR_EXTRACTION_*` wires the extraction LLM. See [`apps/memoir-service/.env.example`](apps/memoir-service/.env.example).

## Releases

Releases are tag-driven from `main`. Pushing a `v*` tag publishes `polypixel-memoir-core` and `polypixel-memoir-sdk` to crates.io, `@polypixel/memoir-sdk` to npm, and the service image to GHCR. Bump the version in the three manifests to match the tag first; see [`RELEASE.md`](RELEASE.md) for the cutoff procedure and [`.tasks/1000-release-operator-runbook.md`](.tasks/1000-release-operator-runbook.md) for the rationale.

## Contributing

Fork, branch off `dev`, open a PR against it. Work lands on `dev` (staging) and promotes to `main` (production); CI commits image digests to parallel `deploy/*` branches that ArgoCD watches, so branch history stays clean. Read [`infrastructure/IAC_RULES.md`](infrastructure/IAC_RULES.md) before any infrastructure change and [`infrastructure/DEPLOY.md`](infrastructure/DEPLOY.md) for the deploy model and rollback runbook.

The repo is maintained with [Jujutsu](https://jj-vcs.github.io/jj/), but it's a git repo underneath — use whatever you like. If you do use jj, it has its own remote/bookmark workflow for pushing branches.

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

## Contact

Myles Berueda — [GitHub](https://github.com/mylesberueda) · [LinkedIn](https://linkedin.com/in/myles-berueda) · [Mastodon](https://mstdn.social/@mylesberueda)
