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
qdrant-client = "1.18"
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
    let qdrant = qdrant_client::Qdrant::from_url("http://localhost:6334").build()?;

    let client = Client::builder()
        .database_url("postgres://postgres:postgres@localhost:54321/memoir")
        .qdrant(qdrant)
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
- **`Client::recall(pid)`** — direct lookup by memoir pid.
- **`Client::forget(target)`** — delete by pid or by scope.
- **`Client::reconcile()`** — retry failed jobs and clean orphan vectors.
- **`Client::spawn_worker()`** — background queue drain.
- Admin: `Client::failed_jobs`, `retry_job`, `delete_failed_job`, `pending_jobs_count`, `unsupersede`, `retry_failed_jobs`.

## Companion crates

- [`polypixel-memoir-sdk`](https://crates.io/crates/polypixel-memoir-sdk) — generated gRPC client for callers talking to `memoir-service` over the network instead of embedding the library.
- `memoir-service` — the gRPC adapter, distributed as a Docker image at `ghcr.io/mylesberueda/memoir/memoir-service`.

## Documentation

API docs: [docs.rs/polypixel-memoir-core](https://docs.rs/polypixel-memoir-core).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
