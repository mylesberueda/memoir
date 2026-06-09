//! In-process gRPC harness for memoir-service integration tests.

use std::sync::Arc;
use std::sync::Once;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use hyper_util::rt::TokioIo;
use memoir_core::client::Client as MemoirClient;
use memoir_sdk::memoir::v1::admin_service_client::AdminServiceClient;
use memoir_sdk::memoir::v1::admin_service_server::AdminServiceServer;
use memoir_sdk::memoir::v1::auth_service_client::AuthServiceClient;
use memoir_sdk::memoir::v1::auth_service_server::AuthServiceServer;
use memoir_sdk::memoir::v1::memory_service_client::MemoryServiceClient;
use memoir_sdk::memoir::v1::memory_service_server::MemoryServiceServer;
use memoir_sdk::memoir::v1::{LoginRequest, Scope as ProtoScope};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::DeleteCollectionBuilder;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use tokio::io::DuplexStream;
use tokio::sync::oneshot;
use tokio_stream::StreamExt as _;
use tokio_util::sync::CancellationToken;
use tonic::Request;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tower::service_fn;

use crate::AppContext;
use crate::middleware::auth::Authenticator;
use crate::middleware::jwt::Jwt;
use crate::services::admin::Admin;
use crate::services::auth::{Auth, create_user};
use crate::services::memory::Memory;

/// Alphabet for test identifiers. The schema regex in both migrators rejects
/// hyphens and uppercase, so nanoid's default alphabet is unsafe here.
const TEST_ID_ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ADMIN_USERNAME: &str = "harness_admin";
const ADMIN_PASSWORD: &str = "harness_admin_password_long_enough";
const DUPLEX_BUFFER_BYTES: usize = 1024 * 1024;

/// Per-test harness owning every resource the suite touches.
#[allow(
    dead_code,
    reason = "Several fields are reserved for ticket 0013's AdminService suite \
                             which reuses this harness."
)]
pub struct TestHarness {
    pub admin_jwt: String,
    pub admin_pid: String,
    pub memory: MemoryServiceClient<Channel>,
    pub auth: AuthServiceClient<Channel>,
    pub admin: AdminServiceClient<Channel>,
    /// Shared in-process channel. Use to build additional service clients
    /// when a test needs one beyond the `memory`, `auth`, and `admin` fields.
    pub channel: Channel,
    pub memoir: Arc<MemoirClient>,
    pub service_schema: String,
    pub core_schema: String,
    pub collection: String,

    /// Service pool used to seed users (search_path-pinned to the per-test
    /// schemas so entity-driven inserts hit the right `users` table).
    service_db: DatabaseConnection,

    worker_cancel: CancellationToken,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_task: Option<tokio::task::JoinHandle<Result<(), tonic::transport::Error>>>,
    cleanup_db: Option<DatabaseConnection>,
    cleanup_qdrant: Option<Qdrant>,
}

impl TestHarness {
    pub async fn start() -> Result<Self> {
        init_tracing();

        let database_url =
            std::env::var("DATABASE_URL").context("DATABASE_URL env var must be set for integration tests")?;
        let qdrant_url = std::env::var("QDRANT_URL").context("QDRANT_URL env var must be set for integration tests")?;

        let id = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
        let service_schema = format!("test_{id}_service");
        let core_schema = format!("test_{id}_core");
        let collection = format!("test_{id}");

        // The service pool gets its search_path pinned and is owned by the
        // running handlers; this separate pool is what cleanup uses to
        // DROP SCHEMA cross-search-path at teardown.
        let cleanup_db = Database::connect(&database_url).await.context("connect cleanup pool")?;

        let service_db = build_service_pool(&database_url, &service_schema).await?;
        migration::bootstrap_and_migrate(&service_db, &service_schema)
            .await
            .context("apply memoir-service migrations")?;

        let qdrant = Qdrant::from_url(&qdrant_url).build().context("build Qdrant client")?;
        let memoir = MemoirClient::builder()
            .database_url(database_url.clone())
            .qdrant(qdrant.clone())
            .schema(core_schema.clone())
            .collection(collection.clone())
            .build()
            .await
            .context("build memoir client")?;
        memoir.migrate().await.context("apply memoir-core migrations")?;

        let memoir_worker = memoir
            .spawn_worker()
            .poll_interval(Duration::from_millis(50))
            .lease_duration(Duration::from_secs(10))
            .drain_timeout(Duration::from_secs(5))
            .start()
            .await
            .context("spawn worker")?;
        let worker_cancel = memoir_worker.cancellation_token();
        let memoir = Arc::new(memoir);

        let jwt = Jwt::from_env().context("build Jwt signer from MEMOIR_JWT_SECRET")?;
        let authenticator = Authenticator::new(service_db.clone(), jwt);
        let ctx = Arc::new(AppContext {
            db: Arc::new(service_db.clone()),
            auth: authenticator,
            memoir: memoir.clone(),
            memoir_worker,
        });

        let admin_handler = Admin::new(ctx.clone());
        let auth_handler = Auth::new(ctx.clone());
        let memory_handler = Memory::new(ctx.clone());

        let admin_row = create_user(&service_db, ADMIN_USERNAME.to_owned(), ADMIN_PASSWORD, true)
            .await
            .map_err(|e| anyhow::anyhow!("seed admin user failed: {e:?}"))?;
        let admin_pid = admin_row.pid.clone();

        // Each gRPC request gets its own duplex pair: the connector spawns a
        // fresh duplex on every connect, pushes the server half to the
        // `incoming` channel, and hands the client half back to tonic. This
        // sidesteps tonic's tendency to close the underlying transport
        // after a call completes, which would strand a one-shot duplex.
        let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel::<DuplexStream>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let server_task =
            spawn_in_process_server(admin_handler, auth_handler, memory_handler, incoming_rx, shutdown_rx);

        let channel = build_in_process_channel(incoming_tx).await?;
        let mut auth_client = AuthServiceClient::new(channel.clone());
        let memory_client = MemoryServiceClient::new(channel.clone());
        let admin_client = AdminServiceClient::new(channel.clone());

        let login = auth_client
            .login(Request::new(LoginRequest {
                username: ADMIN_USERNAME.to_owned(),
                password: ADMIN_PASSWORD.to_owned(),
            }))
            .await
            .context("admin login")?
            .into_inner();

        Ok(Self {
            admin_jwt: login.access_token,
            admin_pid,
            memory: memory_client,
            auth: auth_client,
            admin: admin_client,
            channel,
            memoir,
            service_schema,
            core_schema,
            collection,
            service_db: service_db.clone(),
            worker_cancel,
            shutdown_tx: Some(shutdown_tx),
            server_task: Some(server_task),
            cleanup_db: Some(cleanup_db),
            cleanup_qdrant: Some(qdrant),
        })
    }

    pub fn authed<T>(&self, body: T) -> Request<T> {
        let mut req = Request::new(body);
        let value = format!("Bearer {}", self.admin_jwt)
            .parse()
            .expect("bearer header is ascii");
        req.metadata_mut().insert("authorization", value);
        req
    }

    pub fn fresh_scope(&self) -> ProtoScope {
        let suffix = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
        ProtoScope {
            agent_id: format!("agent_{suffix}"),
            org_id: format!("org_{suffix}"),
            user_id: format!("user_{suffix}"),
        }
    }

    /// Creates a fresh non-admin user, logs them in, returns their access token.
    pub async fn login_non_admin(&self) -> Result<String> {
        let suffix = nanoid::nanoid!(8, &TEST_ID_ALPHABET);
        let username = format!("nonadmin_{suffix}");
        let password = "nonadmin_password_long_enough";
        create_user(&self.service_db, username.clone(), password, false)
            .await
            .map_err(|e| anyhow::anyhow!("seed non-admin user failed: {e:?}"))?;
        let mut auth = self.auth.clone();
        let login = auth
            .login(Request::new(LoginRequest {
                username,
                password: password.to_owned(),
            }))
            .await
            .context("non-admin login")?
            .into_inner();
        Ok(login.access_token)
    }

    /// Wraps `body` in a Request carrying `Bearer <token>` instead of the admin JWT.
    pub fn authed_with<T>(&self, body: T, token: &str) -> Request<T> {
        let mut req = Request::new(body);
        let value = format!("Bearer {token}").parse().expect("bearer header is ascii");
        req.metadata_mut().insert("authorization", value);
        req
    }

    /// Inserts a `memory_jobs` row in `failed` state pointing at `source_pid`.
    ///
    /// memoir-core exposes no public API for fabricating job state — production
    /// jobs originate only via the write path. Tests need to short-circuit that
    /// to exercise the admin surface against a known-failed row.
    pub async fn seed_failed_job(&self, source_pid: &str, kind: &str, reason: &str) -> Result<i64> {
        let db = self.cleanup_db.as_ref().context("cleanup pool already dropped")?;
        let stmt = sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            format!(
                r#"INSERT INTO "{schema}".memory_jobs (source_pid, kind, state, attempts, failure_reason)
                   VALUES ($1, $2, 'failed', 3, $3) RETURNING id"#,
                schema = self.core_schema
            ),
            [source_pid.into(), kind.into(), reason.into()],
        );
        let row = db
            .query_one_raw(stmt)
            .await
            .context("seed_failed_job: insert")?
            .context("seed_failed_job: no row returned")?;
        row.try_get::<i64>("", "id").context("seed_failed_job: read id")
    }

    /// Counts rows in the per-test `memory_jobs` table matching `state`.
    pub async fn count_jobs_with_state(&self, state: &str) -> Result<i64> {
        let db = self.cleanup_db.as_ref().context("cleanup pool already dropped")?;
        let stmt = sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            format!(
                r#"SELECT COUNT(*)::BIGINT AS n FROM "{schema}".memory_jobs WHERE state = $1"#,
                schema = self.core_schema
            ),
            [state.into()],
        );
        let row = db
            .query_one_raw(stmt)
            .await
            .context("count_jobs_with_state: query")?
            .context("count_jobs_with_state: no row")?;
        row.try_get::<i64>("", "n").context("count_jobs_with_state: read n")
    }

    pub async fn wait_until_searchable(&mut self, pid: &str, scope: ProtoScope, query: &str) -> Result<()> {
        use memoir_sdk::memoir::v1::SearchRequest;
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut delay = Duration::from_millis(50);
        while Instant::now() < deadline {
            let req = self.authed(SearchRequest {
                scope: Some(scope.clone()),
                query: query.to_owned(),
                limit: 50,
                metadata_filter: None,
                min_similarity: None,
                kinds: None,
                with_graph_enrichment: false,
                graph_depth: 0,
            });
            let resp = self.memory.search(req).await.context("search probe")?.into_inner();
            if resp
                .hits
                .iter()
                .any(|h| h.memory.as_ref().is_some_and(|m| m.pid == pid))
            {
                return Ok(());
            }
            tokio::time::sleep(delay).await;
            delay = (delay * 2).min(Duration::from_millis(500));
        }
        anyhow::bail!("pid {pid} did not become searchable within 10s")
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        let service_schema = self.service_schema.clone();
        let core_schema = self.core_schema.clone();
        let collection = self.collection.clone();
        let Some(db) = self.cleanup_db.take() else { return };
        let Some(qdrant) = self.cleanup_qdrant.take() else {
            return;
        };
        let shutdown_tx = self.shutdown_tx.take();
        let server_task = self.server_task.take();

        // The WorkerHandle is owned by the AppContext held by the handlers,
        // so we can't call shutdown() on it from here. Cancelling the token
        // achieves the same drain semantics; the handle drops when the last
        // Arc<AppContext> clone goes away.
        self.worker_cancel.cancel();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    if let Some(tx) = shutdown_tx {
                        let _ = tx.send(());
                    }
                    if let Some(task) = server_task {
                        let _ = task.await;
                    }
                    if let Err(err) = qdrant
                        .delete_collection(DeleteCollectionBuilder::new(&collection))
                        .await
                    {
                        eprintln!("[TestHarness::drop] qdrant delete_collection({collection}) failed: {err}");
                    }
                    for schema in [&service_schema, &core_schema] {
                        let sql = format!("DROP SCHEMA IF EXISTS \"{schema}\" CASCADE");
                        if let Err(err) = db.execute_unprepared(&sql).await {
                            eprintln!("[TestHarness::drop] drop schema {schema} failed: {err}");
                        }
                    }
                });
            });
        }));

        if let Err(panic) = result {
            eprintln!(
                "[TestHarness::drop] cleanup panicked (service={service_schema} core={core_schema} \
                 collection={collection}): {panic:?}"
            );
        }
    }
}

async fn build_service_pool(database_url: &str, schema: &str) -> Result<DatabaseConnection> {
    let search_path = format!("{schema},public");
    let options = sea_orm::ConnectOptions::new(database_url.to_owned())
        .set_schema_search_path(search_path)
        .to_owned();
    sea_orm::Database::connect(options)
        .await
        .context("connect memoir-service pool")
}

fn spawn_in_process_server(
    admin_handler: Admin,
    auth_handler: Auth,
    memory_handler: Memory,
    incoming_rx: tokio::sync::mpsc::UnboundedReceiver<DuplexStream>,
    shutdown_rx: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<Result<(), tonic::transport::Error>> {
    // DuplexStream natively implements tokio's AsyncRead/Write plus tonic's
    // Connected trait, so it goes through unwrapped. TokioIo would convert
    // it to hyper's IO traits — the wrong direction for the server side.
    let incoming =
        tokio_stream::wrappers::UnboundedReceiverStream::new(incoming_rx).map(Ok::<DuplexStream, std::io::Error>);
    tokio::spawn(async move {
        Server::builder()
            .add_service(AdminServiceServer::new(admin_handler))
            .add_service(AuthServiceServer::new(auth_handler))
            .add_service(MemoryServiceServer::new(memory_handler))
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
    })
}

async fn build_in_process_channel(incoming_tx: tokio::sync::mpsc::UnboundedSender<DuplexStream>) -> Result<Channel> {
    let uri = Uri::from_static("http://in-process.test");
    let channel = Endpoint::from(uri)
        .connect_with_connector(service_fn(move |_: Uri| {
            // Spawn a fresh duplex pair per connection. tonic may reconnect
            // when the server closes the underlying HTTP/2 transport after
            // a call completes; each invocation here yields a brand-new
            // pair so subsequent calls always have a live transport.
            let tx = incoming_tx.clone();
            async move {
                let (client_io, server_io) = tokio::io::duplex(DUPLEX_BUFFER_BYTES);
                tx.send(server_io)
                    .map_err(|_| std::io::Error::other("in-process server gone"))?;
                Ok::<_, std::io::Error>(TokioIo::new(client_io))
            }
        }))
        .await
        .context("connect in-process channel")?;
    Ok(channel)
}

static TRACING_INIT: Once = Once::new();

fn init_tracing() {
    TRACING_INIT.call_once(|| {
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,sqlx=warn,sea_orm=warn,hyper=warn,h2=warn"));
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .try_init();
    });
}
