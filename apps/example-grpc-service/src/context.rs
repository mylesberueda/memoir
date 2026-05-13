use std::sync::Arc;

/// Contains app context such as the db, redis, qdrant clients, in pointers,
/// that can be passed wherever it's needed in the app.
pub(crate) struct AppContext {
    pub(crate) db: Arc<sea_orm::DatabaseConnection>,
}

impl AppContext {
    pub(crate) async fn new() -> Result<Arc<Self>, AppContextError> {
        let db = Db::init().await?;

        Ok(Arc::new(Self { db }))
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
}
