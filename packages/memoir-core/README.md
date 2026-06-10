# polypixel-memoir-core

Memory substrate for AI agents, as an embeddable Rust library.

Memoir stores agent memories in Postgres (source of truth) and indexes them
in Qdrant (vector similarity search). Consumers bring their own Postgres
connection string and Qdrant client; `memoir-core` owns the schema, the
embedding model, the write-behind queue, and the background worker that
keeps the two stores consistent.

## Install

```toml
[dependencies]
polypixel-memoir-core = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

You also need a Postgres database and a running Qdrant instance. The
[`docker-compose.yml`](https://github.com/mylesberueda/memoir) at the
project root brings both up locally.

## Quick start

```rust,no_run
use memoir_core::client::{Client, DEFAULT_SYSTEM_PROMPT};
use memoir_core::memory::Scope;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .database_url("postgres://postgres:postgres@localhost:54321/memoir")
        .qdrant("http://localhost:6334")
        .system_prompt(DEFAULT_SYSTEM_PROMPT)
        .build()
        .await?;

    client.migrate().await?;
    let worker = client.spawn_worker().start().await?;

    let scope = Scope {
        agent_id: "my-agent".into(),
        org_id: "my-org".into(),
        user_id: "user-42".into(),
    };

    // Write — queued, embedded asynchronously by the worker.
    client
        .remember("the user prefers dark roast coffee", scope.clone())
        .await?;

    // Search — vector similarity within the scope.
    let memories = client.search("coffee preference", scope).limit(5).await?;
    for m in memories.list() {
        println!("{}", m.content);
    }

    worker.shutdown().await;
    Ok(())
}
```

See [`examples/library-quickstart.rs`](https://github.com/mylesberueda/memoir/blob/main/packages/memoir-core/examples/library-quickstart.rs)
for the full lifecycle — remember, search, recall, forget, reconcile.

## Surfaces

- **`Client::remember(content, scope)`** — write-only, returns the persisted row at `PENDING`.
- **`Client::search(query, scope)`** — vector similarity search with `.limit()`, `.episodic()`, `.semantic()`, `.metadata_filter()`, `.min_similarity()` builders.
- **`Client::query(query, scope)`** — ranked, prompt-shaped retrieval: re-ranks candidates by a blend of cosine, confidence, recency, and category. See **Selection** below.
- **`Client::recall(pid)`** — direct lookup by memoir pid.
- **`Client::feedback(pid)`** / **`Client::edit(pid)`** — the correction surface. See **Correction** below.
- **`Client::forget(target)`** — delete by pid or by scope.
- **`Client::reconcile()`** — retry failed jobs and clean orphan vectors.
- **`Client::spawn_worker()`** — background queue drain.
- Admin: `Client::failed_jobs`, `retry_job`, `delete_failed_job`, `pending_jobs_count`, `unsupersede`, `retry_failed_jobs`, `extraction_stats`.

## Selection

`search` returns raw nearest-neighbor hits by cosine similarity. `query`
re-ranks candidates by a blend of signals — cosine, the memory's confidence,
recency, and a bonus for preferred categories — and returns prompt-shaped
context. Pass a `RankingStrategy` to tune it: `Hybrid` (cosine + recency) when
those are the only signals that matter, or `Blended` (with `BlendWeights`) to
also reward high-confidence, preferred-category facts. `min_confidence` and
`category` on `search`/`query` are *hard filters* (they exclude rows); the
blend *weights* the same signals softly — distinct mechanisms.

## Correction

Semantic memories (the facts memoir extracts) are always derived, never
hand-edited. To fix a wrong fact, teach memoir rather than overwrite it:

- **`Client::feedback(wrong_pid).correction(text)`** — the extraction was wrong.
  memoir retires the derived row as `Rejected` and re-derives from the episodic
  source with the correction in context.
- **`Client::edit(episodic_pid)`** — the source itself changed. Editing the
  content (or event-time) cascades: derived facts are retired as `Stale` and
  re-extracted.

`Rejected` counts against extraction accuracy; `Stale` does not (the model
didn't err — the source changed). Retired rows are kept, not deleted, so
**`Client::extraction_stats()`** can report accuracy per provider/model.

## Categorization

`query`'s category signal is populated by an opt-in zero-shot NLI classifier.
Enable it on the builder with `.categorize_model(NliConfig::default())` (or a
custom `NliConfig` for a different HuggingFace model). Without it,
categorization is skipped and the category-bonus blend term is inert.

## Companion crates

- [`polypixel-memoir-sdk`](https://crates.io/crates/polypixel-memoir-sdk) — generated gRPC client for callers talking to `memoir-service` over the network instead of embedding the library.
- `memoir-service` — the gRPC adapter, distributed as a Docker image at `ghcr.io/mylesberueda/memoir/memoir-service`.

## Documentation

API docs: [docs.rs/polypixel-memoir-core](https://docs.rs/polypixel-memoir-core).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
