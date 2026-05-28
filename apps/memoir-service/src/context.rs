use std::sync::Arc;

use memoir_core::client::{Client as MemoirClient, ClientError as MemoirClientError};
use qdrant_client::{Qdrant, QdrantError};

/// Contains app context such as the db and memoir handles, in pointers, that
/// can be passed wherever it's needed in the app.
pub(crate) struct AppContext {
    pub(crate) db: Arc<sea_orm::DatabaseConnection>,
    #[expect(
        dead_code,
        reason = "Wired into MemoryService handlers in epic 0007; held here so startup\
                  validates Qdrant connectivity + memoir migrations before serving traffic."
    )]
    pub(crate) memoir: Arc<MemoirClient>,
}

impl AppContext {
    pub(crate) async fn new() -> Result<Arc<Self>, AppContextError> {
        let db = Db::init().await?;
        let qdrant = QdrantBootstrap::init()?;
        let memoir = Memoir::init(&db, qdrant).await?;

        Ok(Arc::new(Self { db, memoir }))
    }
}

struct Db;

impl Db {
    async fn init() -> Result<Arc<sea_orm::DatabaseConnection>, AppContextError> {
        let db_url = Env::get("DATABASE_URL")?;

        tracing::info!("Connecting to database...");
        let db = Arc::new(sea_orm::Database::connect(&db_url).await.map_err(AppContextError::Db)?);
        tracing::info!("Database connected!");

        Ok(db)
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
    async fn init(db: &Arc<sea_orm::DatabaseConnection>, qdrant: Qdrant) -> Result<Arc<MemoirClient>, AppContextError> {
        tracing::info!("Building memoir client...");
        let client = MemoirClient::builder()
            .db((**db).clone())
            .qdrant(qdrant)
            .build()
            .await
            .map_err(AppContextError::Memoir)?;

        tracing::info!("Applying memoir migrations...");
        client.migrate().await.map_err(AppContextError::Memoir)?;
        tracing::info!("Memoir client ready!");

        Ok(Arc::new(client))
    }
}

struct Env;

impl Env {
    fn get(key: &'static str) -> Result<String, AppContextError> {
        std::env::var(key).map_err(|_| AppContextError::EnvironmentVariableMissing(key))
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
}
