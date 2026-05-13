//! Server lifecycle management.
//!
//! Provides traits for managing startup and shutdown phases.
//! Each concern (seeding, caching, connections) implements [`Lifecycle`] to
//! participate in the server's lifecycle events.

mod crypto;
mod jwt;

pub(crate) mod seed;
pub(crate) mod session_registry;
pub(crate) mod tool_registry;

pub(crate) use crypto::Crypto;
pub(crate) use jwt::Auth;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// Errors that can occur during lifecycle events.
#[derive(Debug, thiserror::Error)]
pub(crate) enum HooksError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Provider error: {0}")]
    Provider(#[from] crate::clients::ProviderError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[allow(dead_code)]
/// Context available during startup.
#[derive(Clone)]
pub(crate) struct StartupContext {
    pub(crate) db: Arc<DatabaseConnection>,
}

#[allow(dead_code)]
/// Context available during shutdown.
#[derive(Clone)]
pub(crate) struct ShutdownContext {
    pub(crate) db: Arc<DatabaseConnection>,
}

pub(crate) trait Hooks<T = ()>: Send + Sync {
    fn on_startup(&self) -> impl std::future::Future<Output = Result<T, HooksError>> + Send;

    /// Called during graceful shutdown.
    #[allow(dead_code)]
    fn on_shutdown(&self, _ctx: &ShutdownContext) -> impl std::future::Future<Output = Result<(), HooksError>> + Send {
        async { Ok(()) }
    }
}
