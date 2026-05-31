//! `MemoryService` gRPC handler.
//!
//! Thin adapter over `memoir_core::Client`. Each RPC follows the same
//! five-step skeleton:
//!
//! 1. Authenticate the request via [`Authenticator::authenticate`] —
//!    surfaces `Unauthenticated` for missing/invalid credentials.
//! 2. Unwrap the proto request into library types via
//!    [`crate::services::conversions`] — surfaces `InvalidArgument`
//!    for malformed input (empty scope fields, missing oneof target,
//!    metadata that can't round-trip through JSON).
//! 3. Call the corresponding `ctx.memoir.<method>` per ticket 0009's
//!    library-method-to-RPC mapping.
//! 4. Map any [`memoir_core::client::ClientError`] to a `tonic::Status`
//!    via the `From` impl in memoir-core (behind the `grpc` feature).
//! 5. Wrap the library return value in the proto response shape.
//!
//! ## Caller identity & scope
//!
//! v0.1 trusts an authenticated caller to choose any scope they write to
//! or read from. Per-scope role / scope-binding policy is an
//! auth-hardening concern and lives in a future epic. Today the caller's
//! pid is logged for audit but not threaded into memoir-core (which has
//! no caller concept by design — the host process is the trust boundary
//! for library users).

use std::sync::Arc;

use memoir_core::store::TimelineDirection;
use memoir_sdk::memoir::v1::memory_service_server::MemoryService;
use memoir_sdk::memoir::v1::{
    EditRequest, EditResponse, ForgetRequest, ForgetResponse, ListAgentsRequest, ListAgentsResponse, QueryRequest,
    QueryResponse, RecallAsOfRequest, RecallAsOfResponse, RecallRequest, RecallResponse, RememberRequest,
    RememberResponse, SearchHit, SearchRequest, SearchResponse, SupersessionHistoryRequest,
    SupersessionHistoryResponse, TimelineRequest, TimelineResponse,
};
use tonic::{Request, Response, Status};

use crate::AppContext;
use crate::middleware::auth::{Authenticator, Principal};
use crate::services::conversions::{
    EditArgs, QueryArgs, RecallAsOfArgs, SupersessionHistoryArgs, TimelineArgs, WireSupersessionEvent,
    forget_target_from_proto, metadata_filter_from_proto, metadata_from_proto, query_response, recall_as_of_response,
    scope_from_proto, timeline_response,
};
use crate::services::wire::{WireError, WireMemory};

/// `MemoryService` RPC handler.
///
/// Holds an [`AppContext`] reference so each handler can reach
/// `ctx.memoir` (the library [`memoir_core::client::Client`]) and
/// `ctx.auth` (the [`Authenticator`]).
pub struct Memory {
    ctx: Arc<AppContext>,
}

impl Memory {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }

    fn auth(&self) -> &Authenticator {
        &self.ctx.auth
    }
}

/// Returns the pid of the principal as a borrowed string for tracing.
fn principal_pid(principal: &Principal) -> &str {
    match principal {
        Principal::User { pid } => pid,
        Principal::ApiKey { pid } => pid,
    }
}

#[tonic::async_trait]
impl MemoryService for Memory {
    /// Searches indexed memories under a scope by vector similarity.
    ///
    /// `limit = 0` falls back to the library default (10). The optional
    /// `metadata_filter` is AND-joined with the scope+kind conditions
    /// enforced by the library — caller-supplied conditions cannot widen
    /// scope. The optional `min_similarity` sets a score floor; hits below
    /// it are dropped by the vector backend before they reach the response.
    async fn search(&self, request: Request<SearchRequest>) -> Result<Response<SearchResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let SearchRequest {
            scope,
            query,
            limit,
            metadata_filter,
            min_similarity,
            kinds,
        } = request.into_inner();

        let scope = scope_from_proto(scope)?;
        let metadata_filter = metadata_filter_from_proto(metadata_filter)?;

        tracing::event!(
            name: "memoir.service.memory.search.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            scope.agent_id = %scope.agent_id,
            scope.org_id = %scope.org_id,
            scope.user_id = %scope.user_id,
            query.len = query.len(),
            limit = limit,
            metadata_filter.present = metadata_filter.is_some(),
            min_similarity = ?min_similarity,
            "MemoryService.Search invoked",
        );

        let mut builder = self.ctx.memoir.search(query, scope);
        if limit > 0 {
            builder = builder.limit(limit as usize);
        }
        if let Some(filter) = metadata_filter {
            builder = builder.metadata_filter(filter);
        }
        if let Some(threshold) = min_similarity {
            builder = builder.min_similarity(threshold);
        }
        // Builder treats neither-toggled as both kinds, so only toggle to filter down.
        if let Some(k) = kinds {
            if k.episodic {
                builder = builder.episodic();
            }
            if k.semantic {
                builder = builder.semantic();
            }
        }
        let memories = builder.await.map_err(WireError::into_status)?;

        let hits = memories
            .list()
            .iter()
            .cloned()
            .map(|memory| {
                let score = memory.score.unwrap_or(0.0);
                SearchHit {
                    memory: Some(WireMemory::from(memory).0),
                    score,
                }
            })
            .collect();

        Ok(Response::new(SearchResponse { hits }))
    }

    /// Looks up a memory by pid at any lifecycle state.
    async fn recall(&self, request: Request<RecallRequest>) -> Result<Response<RecallResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let RecallRequest { pid: memory_pid } = request.into_inner();

        if memory_pid.is_empty() {
            return Err(Status::invalid_argument("pid: required"));
        }

        tracing::event!(
            name: "memoir.service.memory.recall.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            memory.pid = %memory_pid,
            "MemoryService.Recall invoked",
        );

        let memory = self
            .ctx
            .memoir
            .recall(&memory_pid)
            .await
            .map_err(WireError::into_status)?;

        Ok(Response::new(RecallResponse {
            memory: Some(WireMemory::from(memory).0),
        }))
    }

    /// Writes content as an episodic memory; returns the persisted row with `status = PENDING`.
    ///
    /// The write is queue-backed — the embed (and, if extraction is
    /// configured, extract) job is enqueued before the response returns.
    /// The worker drains the queue asynchronously; the returned memory's
    /// vector index entry remains in `pending` state until processing
    /// completes.
    async fn remember(&self, request: Request<RememberRequest>) -> Result<Response<RememberResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let RememberRequest {
            scope,
            content,
            metadata,
        } = request.into_inner();

        if content.is_empty() {
            return Err(Status::invalid_argument("content: required"));
        }
        let scope = scope_from_proto(scope)?;
        let metadata = metadata_from_proto(metadata)?;

        tracing::event!(
            name: "memoir.service.memory.remember.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            scope.agent_id = %scope.agent_id,
            scope.org_id = %scope.org_id,
            scope.user_id = %scope.user_id,
            content.len = content.len(),
            "MemoryService.Remember invoked",
        );

        let written = self
            .ctx
            .memoir
            .remember(content, scope)
            .metadata(metadata)
            .await
            .map_err(WireError::into_status)?;

        Ok(Response::new(RememberResponse {
            memory: Some(WireMemory::from(written).0),
        }))
    }

    /// Deletes one memory by pid or every memory matching a scope tuple.
    ///
    /// `hard_delete` is accepted on the wire for forward compatibility
    /// but **ignored** — memoir-core's `forget` is currently always a
    /// hard delete (no soft-delete substrate exists). When the library
    /// gains soft-delete support, this handler will start respecting the
    /// flag.
    async fn forget(&self, request: Request<ForgetRequest>) -> Result<Response<ForgetResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let request = request.into_inner();
        let hard_delete = request.hard_delete;
        let target = forget_target_from_proto(request)?;

        tracing::event!(
            name: "memoir.service.memory.forget.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            target = ?target,
            hard_delete = hard_delete,
            "MemoryService.Forget invoked",
        );

        let deleted_pids = self.ctx.memoir.forget(target).await.map_err(WireError::into_status)?;

        Ok(Response::new(ForgetResponse { deleted_pids }))
    }

    /// Chronological event-log read. Postgres-only; no vector search, no LLM.
    ///
    /// Includes superseded rows by default (audit view). Read-tier — same
    /// auth posture as `Search` / `Recall`.
    async fn timeline(&self, request: Request<TimelineRequest>) -> Result<Response<TimelineResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let TimelineArgs { scope, params } = request.into_inner().try_into()?;

        tracing::event!(
            name: "memoir.service.memory.timeline.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            scope.agent_id = %scope.agent_id,
            scope.org_id = %scope.org_id,
            scope.user_id = %scope.user_id,
            limit = params.limit,
            include_superseded = params.include_superseded,
            "MemoryService.Timeline invoked",
        );

        let mut builder = self.ctx.memoir.timeline(scope).limit(params.limit);
        if params.kinds.episodic && !params.kinds.semantic {
            builder = builder.episodic();
        }
        if params.kinds.semantic && !params.kinds.episodic {
            builder = builder.semantic();
        }
        if !params.include_superseded {
            builder = builder.exclude_superseded();
        }
        if matches!(params.direction, TimelineDirection::Ascending) {
            builder = builder.ascending();
        }
        if let Some(t) = params.created_after {
            builder = builder.created_after(t);
        }
        if let Some(t) = params.created_before {
            builder = builder.created_before(t);
        }
        if let Some(t) = params.event_at_after {
            builder = builder.event_at_after(t);
        }
        if let Some(t) = params.event_at_before {
            builder = builder.event_at_before(t);
        }

        let memories = builder.await.map_err(WireError::into_status)?;
        Ok(Response::new(timeline_response(memories)))
    }

    /// Point-in-time read: what memoir knew as of a timestamp. Postgres-only.
    ///
    /// Read-tier auth. Logs the `as_of` value and result count so the
    /// point-in-time read is auditable ("who checked what memoir knew, when").
    async fn recall_as_of(&self, request: Request<RecallAsOfRequest>) -> Result<Response<RecallAsOfResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let RecallAsOfArgs { scope, params } = request.into_inner().try_into()?;

        tracing::event!(
            name: "memoir.service.memory.recall_as_of.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            scope.agent_id = %scope.agent_id,
            scope.org_id = %scope.org_id,
            scope.user_id = %scope.user_id,
            memory.as_of = %params.as_of,
            limit = params.limit,
            "MemoryService.RecallAsOf invoked",
        );

        let mut builder = self.ctx.memoir.recall_as_of(scope, params.as_of).limit(params.limit);
        if params.kinds.episodic && !params.kinds.semantic {
            builder = builder.episodic();
        }
        if params.kinds.semantic && !params.kinds.episodic {
            builder = builder.semantic();
        }

        let memories = builder.await.map_err(WireError::into_status)?;
        Ok(Response::new(recall_as_of_response(memories)))
    }

    /// Hybrid-ranked retrieval returning prompt-shaped context.
    ///
    /// Read-tier auth. The response carries raw memories + hybrid score +
    /// the strategy used; the SDK renders the prompt string, not the wire.
    async fn query(&self, request: Request<QueryRequest>) -> Result<Response<QueryResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let args: QueryArgs = request.into_inner().try_into()?;

        tracing::event!(
            name: "memoir.service.memory.query.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            scope.agent_id = %args.scope.agent_id,
            scope.org_id = %args.scope.org_id,
            scope.user_id = %args.scope.user_id,
            query.len = args.query.len(),
            limit = args.limit,
            "MemoryService.Query invoked",
        );

        let mut builder = self
            .ctx
            .memoir
            .query(args.query, args.scope)
            .limit(args.limit)
            .ranking(args.ranking);
        if args.kinds.episodic && !args.kinds.semantic {
            builder = builder.episodic();
        }
        if args.kinds.semantic && !args.kinds.episodic {
            builder = builder.semantic();
        }
        if let Some(filter) = args.metadata_filter {
            builder = builder.metadata_filter(filter);
        }
        if let Some(threshold) = args.min_similarity {
            builder = builder.min_similarity(threshold);
        }
        if let Some(t) = args.created_after {
            builder = builder.created_after(t);
        }
        if let Some(t) = args.created_before {
            builder = builder.created_before(t);
        }
        if let Some(t) = args.event_at_after {
            builder = builder.event_at_after(t);
        }
        if let Some(t) = args.event_at_before {
            builder = builder.event_at_before(t);
        }

        let context = builder.await.map_err(WireError::into_status)?;
        Ok(Response::new(query_response(context)))
    }

    /// In-place correction of an existing memory. Distinct from supersession.
    ///
    /// Write-tier intent: this is the one mutating RPC of the temporality
    /// set. v0.1 enforces no per-tier policy (Remember authenticates the
    /// same way), so this authenticates like every handler; when role-tiers
    /// land, Edit needs write authorization. Reserved-metadata-key and
    /// unsupported-kind (non-episodic) rejections are mapped from the
    /// library error via `WireError`.
    async fn edit(&self, request: Request<EditRequest>) -> Result<Response<EditResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let args: EditArgs = request.into_inner().try_into()?;

        tracing::event!(
            name: "memoir.service.memory.edit.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            memory.pid = %args.pid,
            content.set = args.content.is_some(),
            metadata.set = args.metadata.is_some(),
            event_at.set = args.event_at.is_some(),
            "MemoryService.Edit invoked",
        );

        let mut builder = self.ctx.memoir.edit(args.pid);
        if let Some(content) = args.content {
            builder = builder.content(content);
        }
        if let Some(metadata) = args.metadata {
            builder = builder.metadata(metadata);
        }
        if let Some(event_at) = args.event_at {
            builder = builder.event_at(event_at);
        }

        let updated = builder.await.map_err(WireError::into_status)?;
        Ok(Response::new(EditResponse {
            memory: Some(WireMemory::from(updated).0),
        }))
    }

    async fn supersession_history(
        &self,
        request: Request<SupersessionHistoryRequest>,
    ) -> Result<Response<SupersessionHistoryResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let args: SupersessionHistoryArgs = request.into_inner().try_into()?;

        let events = self
            .ctx
            .memoir
            .supersession_history(&args.pid)
            .await
            .map_err(WireError::into_status)?;

        tracing::event!(
            name: "memoir.service.memory.supersession_history.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            memory.pid = %args.pid,
            events.count = events.len(),
            "MemoryService.SupersessionHistory invoked",
        );

        Ok(Response::new(SupersessionHistoryResponse {
            events: events.into_iter().map(|e| WireSupersessionEvent::from(e).0).collect(),
        }))
    }

    async fn list_agents(&self, request: Request<ListAgentsRequest>) -> Result<Response<ListAgentsResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        let pid = principal_pid(&caller.principal).to_owned();
        let req = request.into_inner();

        if req.org_id.is_empty() || req.user_id.is_empty() {
            return Err(Status::invalid_argument("org_id and user_id must both be non-empty"));
        }

        let agent_ids = self
            .ctx
            .memoir
            .list_agents(&req.org_id, &req.user_id)
            .await
            .map_err(WireError::into_status)?;

        tracing::event!(
            name: "memoir.service.memory.list_agents.invoked",
            tracing::Level::INFO,
            caller.pid = %pid,
            agents.count = agent_ids.len(),
            "MemoryService.ListAgents invoked",
        );

        Ok(Response::new(ListAgentsResponse { agent_ids }))
    }
}

#[cfg(test)]
mod tests {
    // Integration-level tests (real DB, real auth, real Qdrant) belong in
    // the service's integration suite — ticket 0012. Per CLAUDE.md's
    // behavior-first testing rule, the conversion shims in
    // `services/conversions.rs` carry the unit-test burden for boundary
    // logic; the handler bodies are too thin to mock-test meaningfully.

    use super::*;

    #[test]
    fn should_extract_user_pid_from_principal() {
        let principal = Principal::User { pid: "user-abc".into() };
        assert_eq!(principal_pid(&principal), "user-abc");
    }

    #[test]
    fn should_extract_api_key_pid_from_principal() {
        let principal = Principal::ApiKey { pid: "key-xyz".into() };
        assert_eq!(principal_pid(&principal), "key-xyz");
    }
}
