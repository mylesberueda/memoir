use axum::{Router, routing::get};
use fred::clients::Client as RedisClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod webhooks;

pub async fn router(db: Arc<DatabaseConnection>, redis: Arc<RedisClient>) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .nest("/webhooks", webhooks::router(db, redis).await)
}

async fn ping() -> &'static str {
    "pong"
}
