use fred::prelude::ClientLike as _;
use std::sync::Arc;

#[derive(Debug)]
/// Contains app context such as the db, redis, qdrant clients, in pointers,
/// that can be passed wherever it's needed in the app.
pub(crate) struct AppContext {
    pub(crate) db: Arc<sea_orm::DatabaseConnection>,
    pub(crate) redis: Arc<fred::prelude::Client>,
}

impl AppContext {
    pub(crate) async fn new() -> Result<Arc<Self>, AppContextError> {
        let db = Db::init().await?;
        let redis = Redis::init().await?;

        Ok(Arc::new(Self { db, redis }))
    }
}

struct Db;

impl Db {
    async fn init() -> Result<Arc<sea_orm::DatabaseConnection>, AppContextError> {
        let db_url = AppEnv::get("DATABASE_URL")?;

        tracing::info!("Connecting to database...");
        let db = Arc::new(sea_orm::Database::connect(&db_url).await?);
        tracing::info!("Database connected!");

        Ok(db)
    }
}

struct Redis;

impl Redis {
    async fn init() -> Result<Arc<fred::prelude::Client>, AppContextError> {
        let redis_url = AppEnv::get("REDIS_URL")?;

        tracing::info!("Initializing Redis...");
        let redis_config = fred::prelude::Config::from_url(&redis_url)?;
        let redis = fred::prelude::Client::new(redis_config, None, None, None);
        redis.init().await?;
        let redis = Arc::new(redis);
        tracing::info!("Redis connected!");

        Ok(redis)
    }
}

struct AppEnv;

impl AppEnv {
    fn get(key: &'static str) -> Result<String, AppContextError> {
        std::env::var(key).map_err(|_| AppContextError::EnvironmentVariableMissing(key))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppContextError {
    #[error("db error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("redis error: {0}")]
    Redis(#[from] fred::error::Error),
    #[error("environment variable missing: {0}")]
    EnvironmentVariableMissing(&'static str),
}
