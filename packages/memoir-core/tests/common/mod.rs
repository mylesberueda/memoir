//! Shared test harness for memoir-core integration tests.
//!
//! Builds a `Client` against the live Postgres + Qdrant containers configured
//! via `DATABASE_URL` and `QDRANT_URL` env vars. Each `fresh_client()` call
//! generates a unique Postgres schema and Qdrant collection name so tests do
//! not contaminate each other. The returned `TestClient` cleans up its
//! partitions on `Drop` (best-effort — failures during cleanup log a warning
//! and do not mask test assertion failures).
//!
//! Used only when the `integration` feature is enabled — gated at the test
//! file level via `#[cfg(feature = "integration")]`.

#![cfg(feature = "integration")]
#![allow(dead_code)] // Helpers used across integration test files; some test
                    // binaries will not exercise every helper.

use std::ops::Deref;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use memoir_core::client::{Client, WorkerHandle};
use memoir_core::memory::Scope;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::DeleteCollectionBuilder;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

/// Lowercase + digit alphabet for test identifiers.
///
/// Hyphens and underscores are intentionally excluded: schema names must
/// match `[a-z_][a-z0-9_]*` per memoir-core-migration's regex, and we want
/// the leading `test_` prefix to be the only separator.
const TEST_ID_ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Builds a `Client` against the live Postgres + Qdrant under a unique partition.
///
/// The returned `TestClient` derefs to `Client` so tests can call any
/// `Client` method directly. Drop cleans up the Postgres schema and the
/// Qdrant collection.
pub async fn fresh_client() -> Result<TestClient> {
    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL env var must be set for integration tests")?;
    let qdrant_url =
        std::env::var("QDRANT_URL").context("QDRANT_URL env var must be set for integration tests")?;

    let suffix = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
    let schema = format!("test_{suffix}");
    let collection = format!("test_{suffix}");

    let db = Database::connect(&database_url)
        .await
        .context("connect to Postgres")?;
    let qdrant = Qdrant::from_url(&qdrant_url).build().context("build Qdrant client")?;

    let client = Client::builder()
        .db(db.clone())
        .qdrant(qdrant.clone())
        .schema(schema.clone())
        .collection(collection.clone())
        .build()
        .await
        .context("build memoir Client")?;
    client.migrate().await.context("apply memoir migrations")?;

    // Spawn a worker so the queue actually drains. Without this, every
    // `Client::remember` would enqueue an embed job and tests would hang
    // waiting for `wait_until_indexed` to never succeed.
    //
    // Short poll interval is appropriate for tests — production deployments
    // use the default 1-second interval. Short lease so a misbehaving test
    // doesn't pin a job for a minute.
    let worker = client
        .spawn_worker()
        .poll_interval(Duration::from_millis(50))
        .lease_duration(Duration::from_secs(10))
        .drain_timeout(Duration::from_secs(5))
        .start()
        .await
        .context("spawn worker")?;

    Ok(TestClient {
        client,
        worker: Some(worker),
        cleanup_db: Some(db),
        cleanup_qdrant: Some(qdrant),
        schema,
        collection,
    })
}

/// Test-scoped wrapper that owns the partition resources and cleans them up on drop.
pub struct TestClient {
    client: Client,
    worker: Option<WorkerHandle>,
    cleanup_db: Option<DatabaseConnection>,
    cleanup_qdrant: Option<Qdrant>,
    pub schema: String,
    pub collection: String,
}

impl Deref for TestClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        let schema = self.schema.clone();
        let collection = self.collection.clone();
        let Some(db) = self.cleanup_db.take() else { return };
        let Some(qdrant) = self.cleanup_qdrant.take() else { return };
        let worker = self.worker.take();

        // Cleanup needs async; we synchronously block the current thread on a
        // fresh future. `block_in_place` is only safe inside a multi-thread
        // runtime — which is what `#[tokio::test(flavor = "multi_thread")]`
        // gives us. Tests that use the default runtime will see this no-op
        // (the inner block returns gracefully).
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    if let Some(worker) = worker {
                        worker.shutdown().await;
                    }

                    if let Err(err) = qdrant
                        .delete_collection(DeleteCollectionBuilder::new(&collection))
                        .await
                    {
                        eprintln!("[TestClient::drop] qdrant delete_collection({collection}) failed: {err}");
                    }

                    let sql = format!("DROP SCHEMA IF EXISTS \"{schema}\" CASCADE");
                    if let Err(err) = db.execute_unprepared(&sql).await {
                        eprintln!("[TestClient::drop] drop schema {schema} failed: {err}");
                    }
                });
            });
        }));

        if let Err(panic) = result {
            eprintln!(
                "[TestClient::drop] cleanup panicked (schema={schema} collection={collection}): {panic:?}"
            );
        }
    }
}

/// Builds a fresh, deterministic scope tuple for use within a test.
pub fn fresh_scope() -> Scope {
    let suffix = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
    Scope {
        agent_id: format!("agent_{suffix}"),
        org_id: format!("org_{suffix}"),
        user_id: format!("user_{suffix}"),
    }
}

/// Polls until a `pid` is observable via `search`, or returns an error on timeout.
///
/// Used to wait for the async embed-on-write substrate to flip a row from
/// `pending` to `indexed`. The query is the row's own content, ensuring a
/// strong vector match.
pub async fn wait_until_indexed(
    client: &Client,
    pid: &str,
    scope: &Scope,
    query: &str,
    timeout: Duration,
) -> Result<()> {
    let deadline = Instant::now() + timeout;
    let mut delay = Duration::from_millis(50);

    while Instant::now() < deadline {
        let hits = client
            .remember(query, scope.clone())
            .limit(50)
            .await
            .context("search probe failed")?;
        if hits.list().iter().any(|m| m.pid == pid) {
            return Ok(());
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_millis(500));
    }

    anyhow::bail!("pid {pid} did not become searchable within {timeout:?}")
}

