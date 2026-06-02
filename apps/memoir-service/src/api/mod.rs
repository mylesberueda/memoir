//! HTTP routes served alongside the gRPC services.
//!
//! Each module owns one route family and is mounted into the axum router
//! built by [`crate::commands::server`]. Auth runs as middleware ahead of
//! every route via [`crate::middleware::auth::Authenticator`] — the same
//! credentials that gate gRPC handlers gate HTTP routes, with the same
//! API-key-wins precedence.

pub(crate) mod playground;

pub(crate) use playground::router as playground_router;
