use axum::Router;
use fred::clients::Client as RedisClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod stripe;

pub async fn router(db: Arc<DatabaseConnection>, redis: Arc<RedisClient>) -> Router {
    Router::new().nest("/stripe", stripe::router(db, redis).await)
}
