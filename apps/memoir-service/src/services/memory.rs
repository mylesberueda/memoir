//! `MemoryService` stub handler.
//!
//! The proto surface is defined in `memoir.v1.MemoryService` to lock in the
//! SDK shape for consumers. The actual memory implementation (Qdrant vector
//! search, write-behind queue, contradiction detection) lands in the memory
//! epic; reference material is at
//! `.tasks/0002-cleanup-and-prep/.reference/rig-memory/`.
//!
//! Every RPC returns [`tonic::Status::unimplemented`] *after* the request
//! has been authenticated by [`crate::middleware::auth::authenticate`].
//! Unauthenticated callers therefore see `Unauthenticated`, not
//! `Unimplemented` — the auth surface is enforced consistently across
//! both services.

use std::sync::Arc;

use memoir_sdk::memoir::v1::memory_service_server::MemoryService;
use memoir_sdk::memoir::v1::{
    ForgetRequest, ForgetResponse, RecallRequest, RecallResponse, RememberRequest,
    RememberResponse, SearchRequest, SearchResponse,
};
use tonic::{Request, Response, Status};

use crate::AppContext;
use crate::middleware::auth::authenticate;

/// Message returned by every `MemoryService` RPC until real handlers land.
const UNIMPLEMENTED_MESSAGE: &str = "memory service not yet implemented";

/// `MemoryService` RPC handler.
///
/// Holds an [`AppContext`] reference so future real handlers have a DB
/// connection at the call site without an API change.
pub struct Memory {
    ctx: Arc<AppContext>,
}

impl Memory {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[tonic::async_trait]
impl MemoryService for Memory {
    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        let _caller = authenticate(self.ctx.db.as_ref(), &request).await?;
        Err(Status::unimplemented(UNIMPLEMENTED_MESSAGE))
    }

    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        let _caller = authenticate(self.ctx.db.as_ref(), &request).await?;
        Err(Status::unimplemented(UNIMPLEMENTED_MESSAGE))
    }

    async fn remember(
        &self,
        request: Request<RememberRequest>,
    ) -> Result<Response<RememberResponse>, Status> {
        let _caller = authenticate(self.ctx.db.as_ref(), &request).await?;
        Err(Status::unimplemented(UNIMPLEMENTED_MESSAGE))
    }

    async fn forget(
        &self,
        request: Request<ForgetRequest>,
    ) -> Result<Response<ForgetResponse>, Status> {
        let _caller = authenticate(self.ctx.db.as_ref(), &request).await?;
        Err(Status::unimplemented(UNIMPLEMENTED_MESSAGE))
    }
}

#[cfg(test)]
mod tests {
    // Integration-level tests (real DB, real auth interceptor) belong in the
    // service's integration suite. Stub-only unit tests against in-memory
    // mocks would test the mocking rig, not the surface — skipped per
    // CLAUDE.md's behavior-first testing rule.
}
