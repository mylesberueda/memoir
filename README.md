# Memoir

Memory for AI agents. Memoir stores what an agent is told, derives durable facts from it, and serves those facts back on demand.

You write conversation turns. A background worker runs LLM extraction over them to produce semantic facts, and recall reads that fact layer. The raw turns are kept as an audit trail and as the source facts get re-derived from when something needs correcting. Postgres holds the truth, Qdrant indexes the vectors, and Memoir owns the write-behind queue that keeps the two in agreement.

## Two surfaces

Memoir ships as a library or as a service, running the same engine behind different boundaries.

**`memoir-core`** is an embeddable Rust library — `cargo add polypixel-memoir-core`, bring your own Postgres and Qdrant, call it in-process. It contains everything: memory, embedding, extraction, the worker. There's no auth layer, because the host process is already the trust boundary.

**`memoir-service`** wraps that library in gRPC and ships as a Docker image. It adds local auth (JWT and API keys) and puts the surface on the wire; the handlers themselves are thin, mostly unwrapping a request and re-wrapping the library's answer. Network clients talk to it through the generated SDKs — `polypixel-memoir-sdk` on crates.io, `@polypixel/memoir-sdk` on npm.

Use the library if you're writing a Rust agent, and the service if other processes or languages need to reach it.

## What each component needs

Memoir runs against a Postgres database (source of truth) and a Qdrant instance (vector index), plus an optional FalkorDB if you turn on the knowledge graph. Redis appears only under the admin UI, which uses it for its own sessions — the memory engine itself never touches it.

| Component | Postgres | Qdrant | FalkorDB | Redis | Also needs |
|---|:---:|:---:|:---:|:---:|---|
| `memoir-core` (library) | required | required | optional (graph) | — | — |
| `memoir-service` | required | required | optional (graph) | — | — |
| `memoir-ui` (admin) | — | — | — | required (sessions) | `memoir-service` |

None of these need to be dedicated instances. All three can be co-tenanted with a host app's existing infrastructure — Postgres by schema, Qdrant by collection, FalkorDB by named graph. The `redis:alpine` you'll see in the repo's compose file belongs to `memoir-ui`; a service-only deployment can drop it.

## What it does

Every write and read is partitioned by an `(agent, org, user)` tuple, so one tenant never sees another's memories.

Writes go in as raw conversation turns. The worker extracts facts from them asynchronously, and it's those facts you query. Two read paths exist: `search` is a raw nearest-neighbor lookup, while `query` re-ranks by a tunable blend of cosine distance, confidence, recency, and category, then hands back prompt-shaped context. Extracted facts carry both a confidence score and an optional NLI category label, and either can serve as a ranking signal or a hard filter.

Facts also carry an event-time that's distinct from when Memoir was told about them. That distinction is what makes `timeline` and `recall_as_of` work — the latter reconstructs the state of knowledge as it stood at some past instant, rather than what you now know about that instant.

Correction works by teaching rather than editing. You don't hand-edit a semantic fact; you send `feedback` and Memoir re-derives it from the source. Editing the source cascades to everything derived from it. Retirements are recorded as either `rejected` (the extraction was wrong) or `stale` (the source moved on), and `extraction_stats` reports per-model accuracy from that split.

The write-behind queue lives in Postgres and survives crashes. Failed jobs surface in an admin view, and `reconcile` retries them while sweeping up orphaned vectors.

Models are pluggable. Extraction runs against Ollama, OpenAI, or Anthropic through `LlmConfig`; the categorizer accepts any zero-shot NLI model through `NliConfig`. Both are optional, and leaving them out just skips those stages. The same is true of the [FalkorDB](https://falkordb.com)-backed knowledge graph — opt in to derive an entity/relationship graph from your memories and enrich reads with it, or leave it off and run the vector tier alone.

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

Extraction and categorization are opt-in on the builder. `.extraction_llm(LlmConfig::ollama(url, model))` turns episodic turns into semantic facts, and `.categorize_model(NliConfig::default())` labels them; without either, what you have is a scoped vector store. [`examples/library-quickstart.rs`](packages/memoir-core/examples/library-quickstart.rs) walks the full lifecycle, and [`packages/memoir-core/README.md`](packages/memoir-core/README.md) documents the API surface.

## Service quick start

Run the service when you want a memory backend other processes talk to over gRPC.

The fastest path is the copy-paste deployment in [`docker/`](docker/), which brings up the service, the admin console, and the backing stores in a single `docker compose up`. Copy that directory into your project and follow its [README](docker/README.md).

To run just the service container by hand against your own Postgres + Qdrant:

```bash
docker compose --profile dbs up -d   # Postgres + Qdrant

docker run --rm -p 5153:5153 \
  -e DATABASE_URL=postgres://postgres:postgres@host.docker.internal:54321/memoir_service \
  -e QDRANT_URL=http://host.docker.internal:6334 \
  -e JWT_SECRET=$(openssl rand -base64 32) \
  ghcr.io/mylesberueda/memoir/memoir-service:latest
```

Migrations run at startup, but the database named in `DATABASE_URL` has to exist already — Memoir creates schemas and tables, never the database itself. The `--profile dbs` compose above seeds one called `memoir`, so either point `DATABASE_URL` at `…/memoir` or run `createdb -h localhost -p 54321 -U postgres memoir_service` first.

Port 5153 carries three gRPC services:

| Service | Covers |
|---|---|
| `MemoryService` | remember, search, query, recall, timeline, recall-as-of, edit, feedback, forget, supersession-history, list-agents |
| `AdminService` | failed-job triage, reconcile, unsupersede, extraction stats, inspect-graph |
| `AuthService` | bootstrap, login, users, API keys |

Auth is local. A bootstrap token creates the first admin; after that, JWTs and `mk.*` API keys gate every RPC. Issue and regenerate those keys from the admin console under Settings → API Keys, or over `AuthService` directly. A key's secret is displayed once at creation and stored only as a hash, so a lost key gets regenerated rather than recovered.

Configuration is environment-driven. `DATABASE_URL`, `QDRANT_URL`, and `JWT_SECRET` are required; `SERVICE_SCHEMA` and `CORE_SCHEMA` isolate the auth and memory tables; `EXTRACTION_*` wires up the extraction LLM. Full list in [`apps/memoir-service/.env.example`](apps/memoir-service/.env.example).

## Releases

Releases are tag-driven from `main`. Pushing a `v*` tag publishes `polypixel-memoir-core` and `polypixel-memoir-sdk` to crates.io, `@polypixel/memoir-sdk` to npm, and the service image to GHCR. Bump the version in all three manifests to match the tag before tagging. [`RELEASE.md`](RELEASE.md) has the cutoff procedure; [`.tasks/1000-release-operator-runbook.md`](.tasks/1000-release-operator-runbook.md) explains why each step is what it is.

## Contributing

Fork, branch off `main`, open a PR against it. Read [`infrastructure/IAC_RULES.md`](infrastructure/IAC_RULES.md) before any infrastructure change and [`infrastructure/DEPLOY.md`](infrastructure/DEPLOY.md) for the deploy model and rollback runbook.

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

## Contact

Myles Berueda — [GitHub](https://github.com/mylesberueda) · [LinkedIn](https://linkedin.com/in/myles-berueda) · [Mastodon](https://mstdn.social/@mylesberueda)
