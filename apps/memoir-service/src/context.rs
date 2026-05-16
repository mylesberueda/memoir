use std::sync::Arc;

use memoir_core::client::{
    Client as MemoirClient, ClientError as MemoirClientError, WorkerHandle as MemoirWorkerHandle,
};
use migration::{MigrationError as ServiceMigrationError, bootstrap_and_migrate};
use qdrant_client::{Qdrant, QdrantError};
use sea_orm::ConnectOptions;

use crate::middleware::auth::Authenticator;
use crate::middleware::jwt::{Jwt, JwtError};

/// Contains app context such as the db and memoir handles, in pointers, that
/// can be passed wherever it's needed in the app.
pub(crate) struct AppContext {
    pub(crate) db: Arc<sea_orm::DatabaseConnection>,
    /// Per-process credential verifier shared by every AuthService handler.
    /// Owns the JWT signer + the DB handle used for API-key verification.
    pub(crate) auth: Authenticator,
    #[expect(
        dead_code,
        reason = "Wired into MemoryService handlers in a follow-up; held here so startup \
                  validates Qdrant connectivity + memoir migrations before serving traffic."
    )]
    pub(crate) memoir: Arc<MemoirClient>,
    /// Memoir's queue worker. Drains `memory_jobs` for the lifetime of the
    /// service. Held here so the worker stays alive — dropping the handle
    /// does NOT shut the worker down; explicit shutdown belongs in the
    /// server's graceful-stop path.
    #[expect(
        dead_code,
        reason = "Worker drains the queue in the background; the handle is held so it isn't \
                  reclaimed early. Graceful shutdown via `worker.shutdown()` belongs in the \
                  server's stop path (follow-up)."
    )]
    pub(crate) memoir_worker: MemoirWorkerHandle,
}

impl AppContext {
    pub(crate) async fn new() -> Result<Arc<Self>, AppContextError> {
        let service_schema = Env::get_or("MEMOIR_SERVICE_SCHEMA", migration::DEFAULT_SCHEMA);
        let memoir_schema = Env::get_or("MEMOIR_SCHEMA", memoir_core::DEFAULT_SCHEMA);
        let database_url = Env::get("DATABASE_URL")?;

        // memoir-service owns its own pool, pinned to its own schema. memoir-core
        // builds a separate pool inside `Client::builder` (see below) pinned to
        // its own schema. The two pools share a Postgres backend but have
        // disjoint search_paths, so each crate's `seaql_migrations` ledger
        // lands in its own schema without collision.
        let db = Db::init_service(&database_url, &service_schema).await?;
        Db::apply_service_migrations(&db, &service_schema).await?;

        let qdrant = QdrantBootstrap::init()?;
        let (memoir, memoir_worker) = Memoir::init(&database_url, qdrant, &memoir_schema).await?;

        // JWT signer is constructed before any handler runs so misconfigured
        // secrets fail loudly at startup rather than on the first Login.
        let jwt = Jwt::from_env().map_err(AppContextError::Jwt)?;
        let auth = Authenticator::new((*db).clone(), jwt);

        Ok(Arc::new(Self {
            db,
            auth,
            memoir,
            memoir_worker,
        }))
    }
}

struct Db;

impl Db {
    async fn init_service(
        database_url: &str,
        service_schema: &str,
    ) -> Result<Arc<sea_orm::DatabaseConnection>, AppContextError> {
        // search_path holds *only* memoir-service's schema. memoir-core uses
        // a separate pool with its own search_path; the two never see each
        // other's tables.
        let search_path = format!("{service_schema},public");
        tracing::info!(search_path = %search_path, "Connecting to database (service pool)...");
        let options = ConnectOptions::new(database_url.to_owned())
            .set_schema_search_path(search_path)
            .to_owned();
        let db = Arc::new(sea_orm::Database::connect(options).await.map_err(AppContextError::Db)?);
        tracing::info!("Service database connected!");

        Ok(db)
    }

    async fn apply_service_migrations(
        db: &sea_orm::DatabaseConnection,
        service_schema: &str,
    ) -> Result<(), AppContextError> {
        tracing::info!("Applying memoir-service migrations...");
        bootstrap_and_migrate(db, service_schema)
            .await
            .map_err(AppContextError::ServiceMigration)?;
        tracing::info!("memoir-service migrations applied!");
        Ok(())
    }
}

struct QdrantBootstrap;

impl QdrantBootstrap {
    fn init() -> Result<Qdrant, AppContextError> {
        let url = Env::get("QDRANT_URL")?;

        tracing::info!("Connecting to Qdrant...");
        let qdrant = Qdrant::from_url(&url).build().map_err(Box::new)?;
        tracing::info!("Qdrant connected!");

        Ok(qdrant)
    }
}

struct Memoir;

impl Memoir {
    async fn init(
        database_url: &str,
        qdrant: Qdrant,
        schema: &str,
    ) -> Result<(Arc<MemoirClient>, MemoirWorkerHandle), AppContextError> {
        tracing::info!("Building memoir client...");
        let client = MemoirClient::builder()
            .database_url(database_url.to_owned())
            .qdrant(qdrant)
            .schema(schema.to_owned())
            .build()
            .await
            .map_err(AppContextError::Memoir)?;

        tracing::info!("Applying memoir-core migrations...");
        client.migrate().await.map_err(AppContextError::Memoir)?;

        // Memoir's write path is persistent: every Remember enqueues an
        // embed job into `memory_jobs`. The worker drains that queue. If
        // we don't spawn it, writes land in Postgres but never reach
        // Qdrant — every search would return nothing.
        tracing::info!("Spawning memoir worker...");
        let worker = client
            .spawn_worker()
            .start()
            .await
            .map_err(AppContextError::Memoir)?;

        tracing::info!("Memoir client + worker ready!");

        Ok((Arc::new(client), worker))
    }
}

struct Env;

impl Env {
    fn get(key: &'static str) -> Result<String, AppContextError> {
        std::env::var(key).map_err(|_| AppContextError::EnvironmentVariableMissing(key))
    }

    fn get_or(key: &'static str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_owned())
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppContextError {
    #[error("db error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("environment variable missing: {0}")]
    EnvironmentVariableMissing(&'static str),
    #[error("qdrant error: {0}")]
    Qdrant(#[from] Box<QdrantError>),
    #[error("memoir error: {0}")]
    Memoir(#[from] MemoirClientError),
    #[error("memoir-service migration error: {0}")]
    ServiceMigration(#[from] ServiceMigrationError),
    #[error("jwt configuration error: {0}")]
    Jwt(JwtError),
}
