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
use std::sync::Once;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use memoir_core::client::{Client, WorkerHandle};
use memoir_core::llm::LlmConfig;
use memoir_core::memory::{MemoryKind, Scope};
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
/// Qdrant collection. No LLM is configured; extract jobs (if any) skip
/// gracefully with a WARN per `client/extract.rs:96`.
pub async fn fresh_client() -> Result<TestClient> {
    build_test_client(None, None).await
}

/// Like [`fresh_client`] but wires a real Ollama extraction LLM.
///
/// Reads `OLLAMA_URL` and `OLLAMA_MODEL` from the environment; both must be
/// set. The returned client's worker will dispatch extract jobs to that
/// Ollama instance — tests that exercise the extraction stage use this
/// constructor.
pub async fn fresh_client_with_extraction() -> Result<TestClient> {
    let ollama_url =
        std::env::var("OLLAMA_URL").context("OLLAMA_URL env var must be set for extraction tests")?;
    let ollama_model = std::env::var("OLLAMA_MODEL")
        .context("OLLAMA_MODEL env var must be set for extraction tests")?;
    build_test_client(Some(LlmConfig::ollama(ollama_url, ollama_model)), None).await
}

/// Builds a `Client` wired to live Postgres + Qdrant + FalkorDB, with extraction.
///
/// Reads `FALKOR_URL`, `OLLAMA_URL`, and `OLLAMA_MODEL` from the environment;
/// all must be set. The Ollama model drives both semantic extraction and
/// relational triple extraction (the graph write path). The returned client
/// writes to a per-test graph name under the shared FalkorDB instance, and its
/// [`TestClient::fresh_scope`] hands out scopes that are wiped from the graph on
/// drop — see [`TestClient`]'s cleanup contract.
#[cfg(feature = "knowledge-graph")]
pub async fn fresh_graph_client() -> Result<TestClient> {
    let falkor_url =
        std::env::var("FALKOR_URL").context("FALKOR_URL env var must be set for graph integration tests")?;
    let ollama_url =
        std::env::var("OLLAMA_URL").context("OLLAMA_URL env var must be set for graph integration tests")?;
    let ollama_model = std::env::var("OLLAMA_MODEL")
        .context("OLLAMA_MODEL env var must be set for graph integration tests")?;
    let llm = LlmConfig::ollama(ollama_url, ollama_model);
    build_test_client(Some(llm.clone()), Some(GraphConfig { falkor_url, llm })).await
}

/// Live-FalkorDB wiring for a graph-backed test client.
///
/// The graph name is minted per test inside [`build_test_client`] (a unique
/// `test_<suffix>`) so suites share one FalkorDB instance without colliding;
/// `llm` is the relational triple extractor the graph write path needs. Defined
/// un-gated so `build_test_client`'s signature is valid in every feature state;
/// only the wiring that consumes it is `knowledge-graph`-gated.
struct GraphConfig {
    falkor_url: String,
    llm: LlmConfig,
}

static TRACING_INIT: Once = Once::new();

fn init_tracing() {
    TRACING_INIT.call_once(|| {
        // RUST_LOG controls verbosity. Default is INFO+ for memoir-core, off
        // for the noisy sqlx/hyper crates. Operators can override via env.
        let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new("info,sqlx=warn,sea_orm=warn,hyper=warn")
        });
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .try_init();
    });
}

#[cfg_attr(not(feature = "knowledge-graph"), allow(unused_variables))]
async fn build_test_client(extraction: Option<LlmConfig>, graph: Option<GraphConfig>) -> Result<TestClient> {
    init_tracing();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL env var must be set for integration tests")?;
    let qdrant_url =
        std::env::var("QDRANT_URL").context("QDRANT_URL env var must be set for integration tests")?;

    let suffix = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
    let schema = format!("test_{suffix}");
    let collection = format!("test_{suffix}");

    // memoir-core builds its own pool internally from the URL we pass it,
    // pinned to the per-test schema. The harness keeps a separate, plain
    // pool here just so `TestClient::drop` can issue `DROP SCHEMA ...
    // CASCADE` at teardown — the Client's own pool isn't reachable for
    // that.
    let db = Database::connect(&database_url)
        .await
        .context("connect to Postgres (cleanup pool)")?;
    // The Client builds its own Qdrant client from the URL; this separate handle
    // exists only so `TestClient::drop` can delete the per-test collection at
    // teardown (the Client's internal client isn't reachable for that).
    let qdrant = Qdrant::from_url(&qdrant_url).build().context("build Qdrant cleanup client")?;

    let builder = Client::builder()
        .database_url(database_url.clone())
        .qdrant(qdrant_url.clone())
        .schema(schema.clone())
        .collection(collection.clone())
        .maybe_extraction_llm(extraction);

    // `maybe_*` setters keep the builder on one typestate path whether or not a
    // graph is wired (a `match` that calls `.falkor()` on one arm only would
    // give the arms different bon builder types). A per-test graph name keeps
    // suites sharing one FalkorDB instance from colliding — isolation by name,
    // mirroring the per-test schema/collection.
    #[cfg(feature = "knowledge-graph")]
    let builder = builder
        .maybe_falkor(graph.as_ref().map(|cfg| cfg.falkor_url.clone()))
        .maybe_graph_name(graph.as_ref().map(|_| format!("test_{suffix}")))
        .maybe_relational_llm(graph.as_ref().map(|cfg| cfg.llm.clone()));

    let client = builder.build().await.context("build memoir Client")?;
    client.migrate().await.context("apply memoir migrations")?;

    // Spawn a worker so the queue actually drains. Without this, every
    // `Client::remember` would enqueue an embed job and tests would hang
    // waiting for `wait_until_indexed` to never succeed (the row stays
    // `pending` and `Client::search` filters it out).
    //
    // The lease must exceed a single extraction inference. Capable local
    // models (qwen3:14b and up) take 10-30s per call on consumer hardware,
    // especially on a cold load. A lease shorter than that expires
    // mid-inference, the reconciler assumes the worker died and re-leases
    // the job, and the same call restarts until max_attempts — the job
    // never completes. 60s clears a cold large-model inference.
    let worker = client
        .spawn_worker()
        .poll_interval(Duration::from_millis(50))
        .lease_duration(Duration::from_secs(60))
        .drain_timeout(Duration::from_secs(5))
        .start()
        .await
        .context("spawn worker")?;

    Ok(TestClient {
        client,
        worker: Some(worker),
        cleanup_db: Some(db),
        cleanup_qdrant: Some(qdrant),
        cleanup_scopes: Vec::new(),
        schema,
        collection,
    })
}

/// Test-scoped wrapper that owns the partition resources and cleans them up on drop.
///
/// Beyond the per-test Postgres schema and Qdrant collection (uniquely named, so
/// a leak is inert), graph tests share one FalkorDB graph name. A scope minted
/// via [`TestClient::fresh_scope`] is recorded here and wiped from that shared
/// graph on drop — a leaked scope would otherwise pollute sibling tests.
pub struct TestClient {
    client: Client,
    worker: Option<WorkerHandle>,
    cleanup_db: Option<DatabaseConnection>,
    cleanup_qdrant: Option<Qdrant>,
    cleanup_scopes: Vec<Scope>,
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
        #[cfg(feature = "knowledge-graph")]
        let client = self.client.clone();
        #[cfg(feature = "knowledge-graph")]
        let scopes = std::mem::take(&mut self.cleanup_scopes);

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

                    // Graph scopes first: forget_scope routes through the
                    // Client's own pool, which the schema drop below tears
                    // down. A leak here pollutes the shared test graph, so the
                    // failure is loud (eprintln per scope), not swallowed.
                    #[cfg(feature = "knowledge-graph")]
                    for scope in scopes {
                        if let Err(err) = client
                            .forget(memoir_core::memory::ForgetTarget::Scope(scope.clone()))
                            .await
                        {
                            eprintln!("[TestClient::drop] forget graph scope {scope:?} failed: {err}");
                        }
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

impl TestClient {
    /// Mints a fresh scope and records it for graph cleanup on drop.
    ///
    /// Graph tests must use this instead of the free [`fresh_scope`]: the
    /// returned scope is wiped from the shared test graph when the
    /// [`TestClient`] drops, so a test cannot leak nodes into a sibling's run.
    /// Minting *is* registration, so cleanup cannot be forgotten. Requires
    /// `&mut` to record the scope; bind the client `let mut`.
    #[cfg(feature = "knowledge-graph")]
    pub fn fresh_scope(&mut self) -> Scope {
        let scope = fresh_scope();
        self.cleanup_scopes.push(scope.clone());
        scope
    }

    /// Mints a tracked scope under a caller-supplied `org_id`.
    ///
    /// Same cleanup contract as [`Self::fresh_scope`]; the fixed `org_id` lets a
    /// test put two scopes in one org (e.g. to exercise a partial-scope,
    /// org-wide graph inspect that must span both).
    #[cfg(feature = "knowledge-graph")]
    pub fn fresh_scope_in_org(&mut self, org_id: &str) -> Scope {
        let suffix = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
        let scope = Scope {
            agent_id: format!("agent_{suffix}"),
            org_id: org_id.to_string(),
            user_id: format!("user_{suffix}"),
        };
        self.cleanup_scopes.push(scope.clone());
        scope
    }

    /// Opens a fresh `DatabaseConnection` pinned to this test's schema.
    ///
    /// Used by migration-layer tests that need to issue raw SQL against
    /// memoir-core's tables — e.g. verifying a trigger's side-effect or
    /// asserting a CHECK constraint rejects bad inserts. The returned
    /// connection's `search_path` is set to the test's schema first then
    /// `public`, matching the convention `Client::new` uses internally.
    pub async fn raw_db(&self) -> Result<DatabaseConnection> {
        let database_url =
            std::env::var("DATABASE_URL").context("DATABASE_URL env var must be set")?;
        let search_path = format!("{},public", self.schema);
        let options = sea_orm::ConnectOptions::new(database_url)
            .set_schema_search_path(search_path)
            .to_owned();
        Database::connect(options).await.context("connect raw_db")
    }

    /// Opens a fresh `Qdrant` client targeting the configured QDRANT_URL.
    ///
    /// Used by payload-layer tests that need to inspect raw Qdrant point
    /// payloads — e.g. verifying that the upsert path wrote the expected
    /// `created_at` / `event_at` / metadata-flattened fields. The
    /// collection name is `self.collection`; callers issue scrolls/queries
    /// against that name.
    pub fn raw_qdrant(&self) -> Result<Qdrant> {
        let qdrant_url = std::env::var("QDRANT_URL").context("QDRANT_URL env var must be set")?;
        Qdrant::from_url(&qdrant_url).build().context("build raw_qdrant client")
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
            .search(query, scope.clone())
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

/// Polls the scope until at least one indexed row is observable, returning its pid.
///
/// Pure observation: each poll iteration issues a `Client::search`, which
/// reads only. Tests that need the pid of a prior write use this after the
/// write completes, accepting that the embed substrate may take a moment
/// to flip the row from `pending` to `indexed`.
pub async fn wait_for_first_pid(
    client: &Client,
    scope: &Scope,
    query: &str,
    timeout: Duration,
) -> Result<String> {
    let deadline = Instant::now() + timeout;
    let mut delay = Duration::from_millis(50);

    while Instant::now() < deadline {
        let hits = client.search(query, scope.clone()).limit(50).await?;
        if let Some(first) = hits.list().first() {
            return Ok(first.pid.clone());
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_millis(500));
    }

    anyhow::bail!("no indexed row appeared in scope within {timeout:?}")
}

/// Polls until at least one semantic memory exists with `source_pid = source`.
///
/// Used to wait for the async extract worker stage to write derived semantic
/// rows for a given episodic source pid. Inspects the store directly rather
/// than going through search, so polling has no side effects on the test's
/// scope (no extra episodic writes per iteration).
///
/// Returns the list of matching semantic memories observed on the first
/// successful poll. Returns an error on timeout.
pub async fn wait_until_extracted(
    client: &Client,
    scope: &Scope,
    source_pid: &str,
    timeout: Duration,
) -> Result<Vec<memoir_core::memory::Memory>> {
    use memoir_core::store::MemoryStore;

    let deadline = Instant::now() + timeout;
    let mut delay = Duration::from_millis(200);

    while Instant::now() < deadline {
        let pids = client
            .store()
            .indexed_pids_in_scope(scope)
            .await
            .context("indexed_pids_in_scope probe failed")?;
        let pid_refs: Vec<&str> = pids.iter().map(String::as_str).collect();
        let rows = client
            .store()
            .find_by_pids(&pid_refs)
            .await
            .context("find_by_pids probe failed")?;
        let semantics: Vec<_> = rows
            .into_iter()
            .filter(|m| m.kind == MemoryKind::Semantic && m.source_pid.as_deref() == Some(source_pid))
            .collect();
        if !semantics.is_empty() {
            return Ok(semantics);
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(2));
    }

    anyhow::bail!("no semantic memories observed for source_pid {source_pid} within {timeout:?}")
}

/// Polls the graph for `scope` until `ready` accepts the snapshot, or times out.
///
/// Graph commit is asynchronous: the worker drains `RelationalExtract` → stage →
/// `Synthesize` → commit. A test that writes then immediately reads races the
/// worker, so it waits on the end state instead — reading the live graph via
/// [`Client::inspect_graph`] each iteration and handing the snapshot to `ready`
/// (e.g. "an `Alice -WORKS_AT-> Acme` edge exists"). Returns the accepted
/// snapshot for follow-up assertions, or an error on timeout.
#[cfg(feature = "knowledge-graph")]
pub async fn wait_until_graph_committed(
    client: &Client,
    scope: &Scope,
    timeout: Duration,
    ready: impl Fn(&memoir_core::graph::GraphSnapshot) -> bool,
) -> Result<memoir_core::graph::GraphSnapshot> {
    let deadline = Instant::now() + timeout;
    let mut delay = Duration::from_millis(200);

    while Instant::now() < deadline {
        let snapshot = client
            .inspect_graph()
            .agent(scope.agent_id.clone())
            .org(scope.org_id.clone())
            .user(scope.user_id.clone())
            .await
            .context("inspect_graph probe failed")?;
        if ready(&snapshot) {
            return Ok(snapshot);
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(2));
    }

    anyhow::bail!("graph did not reach the expected state for scope {scope:?} within {timeout:?}")
}

