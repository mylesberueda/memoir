//! `AdminService` gRPC handler — operator-only triage of the write-behind queue.
//!
//! Thin adapter over `memoir_core::Client`'s admin methods (`failed_jobs`,
//! `retry_job`, `delete_failed_job`, `pending_jobs_count`, `unsupersede`,
//! `reconcile`, `retry_failed_jobs`). Each RPC follows the same skeleton:
//!
//! 1. Authenticate via [`Authenticator::authenticate`] — surfaces
//!    `Unauthenticated` for missing/invalid credentials.
//! 2. Gate on `caller.is_admin` via
//!    [`crate::middleware::auth::CallerIdentity::require_admin`] — surfaces
//!    `PermissionDenied` for non-admin callers.
//! 3. **Audit log** the invocation at INFO with caller pid + key arguments
//!    BEFORE the library call. Admin actions are audit-relevant — if the
//!    library call panics mid-flight, the audit trail still records that
//!    the operation was attempted.
//! 4. Unwrap proto request into library types via
//!    [`crate::services::conversions`] — surfaces `InvalidArgument` for
//!    malformed input.
//! 5. Call the corresponding `ctx.memoir.<admin_method>`; map any
//!    [`memoir_core::client::ClientError`] to a `tonic::Status` via the
//!    `From` impl in memoir-core (behind the `grpc` feature).
//! 6. Wrap the library return value in the proto response shape.
//!
//! ## Admin-only enforcement
//!
//! Admin-only enforcement is **per-RPC explicit**, not via a tonic
//! interceptor — matching the codebase's existing pattern in
//! `services/auth.rs`. If an interceptor pattern is ever introduced, this
//! handler still checks `is_admin` directly so the gate is impossible to
//! forget on a new RPC.
//!
//! ## PII boundary
//!
//! Per the proto's design (see `admin.proto`'s service-level comment),
//! admin responses contain **metadata only** — no job payloads, no memory
//! content. The library's `FailedJob` type does not carry that data; this
//! handler simply preserves the boundary.

use std::sync::Arc;

use memoir_sdk::memoir::v1::admin_service_server::AdminService;
use memoir_sdk::memoir::v1::{
    DeleteFailedJobRequest, DeleteFailedJobResponse, ExtractionStatsRequest, ExtractionStatsResponse,
    InspectGraphRequest, InspectGraphResponse, ListFailedJobsRequest, ListFailedJobsResponse, PendingJobsCountRequest,
    PendingJobsCountResponse, ReconcileRequest, ReconcileResponse, RetryFailedJobsRequest, RetryFailedJobsResponse,
    RetryJobRequest, RetryJobResponse, UnsupersedeRequest, UnsupersedeResponse,
};
use tonic::{Request, Response, Status};

use crate::AppContext;
use crate::middleware::auth::{Authenticator, Principal};
use crate::services::conversions::{
    WireExtractionStat, WireFailedJob, WireInspectGraphResponse, WireReconcileResponse, WireRetryArgs, WireStatsFilter,
    u64_count_to_proto,
};
use crate::services::wire::WireError;

/// Default cap when `ListFailedJobsRequest.limit == 0`.
///
/// Matches the convention in `services/auth.rs` and prevents a stray
/// caller from requesting an unbounded result set. Operators paginate by
/// shrinking the limit and walking via repeated calls (no cursor in v0.1
/// per ticket 0008).
const DEFAULT_FAILED_JOBS_LIMIT: usize = 50;

/// Hard cap for `ListFailedJobsRequest.limit`.
///
/// Protects the server from a misconfigured client requesting a giant
/// page. Operator-facing UIs paginate well below this in practice.
const MAX_FAILED_JOBS_LIMIT: usize = 500;

/// `AdminService` RPC handler.
///
/// Holds an [`AppContext`] reference so each handler can reach
/// `ctx.memoir` (the library [`memoir_core::client::Client`]) and
/// `ctx.auth` (the [`Authenticator`]). Constructed once at process start;
/// cloned cheaply behind `Arc` for each incoming request.
pub struct Admin {
    ctx: Arc<AppContext>,
}

impl Admin {
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
impl AdminService for Admin {
    /// Lists failed jobs newest-first, capped at `limit`.
    ///
    /// `limit = 0` falls back to [`DEFAULT_FAILED_JOBS_LIMIT`]; values above
    /// [`MAX_FAILED_JOBS_LIMIT`] are clamped. Metadata only — no payloads
    /// or memory content per the proto's PII boundary.
    async fn list_failed_jobs(
        &self,
        request: Request<ListFailedJobsRequest>,
    ) -> Result<Response<ListFailedJobsResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let ListFailedJobsRequest { limit } = request.into_inner();

        let resolved_limit = resolve_failed_jobs_limit(limit);

        tracing::event!(
            name: "memoir.service.admin.list_failed_jobs.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            limit.requested = limit,
            limit.resolved = resolved_limit,
            "AdminService.ListFailedJobs invoked",
        );

        let jobs = self
            .ctx
            .memoir
            .failed_jobs(resolved_limit)
            .await
            .map_err(WireError::into_status)?;

        let proto_jobs = jobs.into_iter().map(|j| WireFailedJob::from(j).0).collect();
        Ok(Response::new(ListFailedJobsResponse { jobs: proto_jobs }))
    }

    /// Returns the number of jobs currently in `pending` state.
    async fn pending_jobs_count(
        &self,
        request: Request<PendingJobsCountRequest>,
    ) -> Result<Response<PendingJobsCountResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();

        tracing::event!(
            name: "memoir.service.admin.pending_jobs_count.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            "AdminService.PendingJobsCount invoked",
        );

        let count = self
            .ctx
            .memoir
            .pending_jobs_count()
            .await
            .map_err(WireError::into_status)?;
        let wire_count = u64_count_to_proto(count, "pending_jobs_count")?;
        Ok(Response::new(PendingJobsCountResponse { count: wire_count }))
    }

    /// Retries one failed job by id, clearing the attempt counter.
    async fn retry_job(&self, request: Request<RetryJobRequest>) -> Result<Response<RetryJobResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let RetryJobRequest { id } = request.into_inner();

        tracing::event!(
            name: "memoir.service.admin.retry_job.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            job.id = id,
            "AdminService.RetryJob invoked",
        );

        self.ctx.memoir.retry_job(id).await.map_err(WireError::into_status)?;
        Ok(Response::new(RetryJobResponse {}))
    }

    /// Permanently deletes one failed job by id. Destructive.
    ///
    /// The referenced memory row is untouched. Once a failed job is
    /// deleted, the memory's processing state is unrecoverable except by
    /// re-running `Remember`. Operators should triage the failure first
    /// (read `failure_reason`, optionally inspect the source memory via
    /// `MemoryService.Recall`) before calling this.
    async fn delete_failed_job(
        &self,
        request: Request<DeleteFailedJobRequest>,
    ) -> Result<Response<DeleteFailedJobResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let DeleteFailedJobRequest { id } = request.into_inner();

        tracing::event!(
            name: "memoir.service.admin.delete_failed_job.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            job.id = id,
            "AdminService.DeleteFailedJob invoked",
        );

        self.ctx
            .memoir
            .delete_failed_job(id)
            .await
            .map_err(WireError::into_status)?;
        Ok(Response::new(DeleteFailedJobResponse {}))
    }

    /// Bulk-retries failed jobs, optionally filtered by kind.
    ///
    /// `dry_run = true` returns the affected count without modifying any
    /// rows — a wide retry against many failed extract jobs can DoS the
    /// configured LLM provider, so the preview lets operators size the
    /// impact before firing.
    async fn retry_failed_jobs(
        &self,
        request: Request<RetryFailedJobsRequest>,
    ) -> Result<Response<RetryFailedJobsResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let WireRetryArgs { of_kind, dry_run } = WireRetryArgs::try_from(request.into_inner())?;

        tracing::event!(
            name: "memoir.service.admin.retry_failed_jobs.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            dry_run = dry_run,
            kind_filter = ?of_kind,
            "AdminService.RetryFailedJobs invoked",
        );

        let mut builder = self.ctx.memoir.retry_failed_jobs();
        if let Some(kind) = of_kind {
            builder = builder.of_kind(kind);
        }
        if dry_run {
            builder = builder.dry_run();
        }
        let affected = builder.await.map_err(WireError::into_status)?;
        let wire_affected = u64_count_to_proto(affected, "retry_failed_jobs.affected")?;

        Ok(Response::new(RetryFailedJobsResponse {
            affected: wire_affected,
            dry_run,
        }))
    }

    /// Clears the supersession marker on a memory, restoring it to active state.
    ///
    /// Operator discretion required — supersession is normally set by the
    /// contradiction-detection pass against verified contradicting facts.
    /// Reversing it should only happen when an operator has confirmed the
    /// supersession was a false positive.
    async fn unsupersede(&self, request: Request<UnsupersedeRequest>) -> Result<Response<UnsupersedeResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let UnsupersedeRequest { pid } = request.into_inner();

        if pid.is_empty() {
            return Err(Status::invalid_argument("pid: required"));
        }

        tracing::event!(
            name: "memoir.service.admin.unsupersede.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            memory.pid = %pid,
            "AdminService.Unsupersede invoked",
        );

        self.ctx
            .memoir
            .unsupersede(&pid)
            .await
            .map_err(WireError::into_status)?;
        Ok(Response::new(UnsupersedeResponse {}))
    }

    /// Runs reconciliation: retries failed rows + cleans orphan vectors.
    ///
    /// Heavyweight — may take many seconds on large stores. **Do not poll
    /// from a UI button.** Operators trigger this manually for queue
    /// maintenance; memoir-ui should not bind a periodic refresh to it.
    ///
    /// Both proto flags `false` (proto3 default) runs both passes
    /// (matching the library's `ReconcileBuilder` default).
    async fn reconcile(&self, request: Request<ReconcileRequest>) -> Result<Response<ReconcileResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let ReconcileRequest {
            only_retry_failed,
            only_clean_orphans,
        } = request.into_inner();

        if only_retry_failed && only_clean_orphans {
            return Err(Status::invalid_argument(
                "only_retry_failed and only_clean_orphans are mutually exclusive; \
                 omit both (or set both false) to run both passes",
            ));
        }

        tracing::event!(
            name: "memoir.service.admin.reconcile.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            only_retry_failed = only_retry_failed,
            only_clean_orphans = only_clean_orphans,
            "AdminService.Reconcile invoked",
        );

        let mut builder = self.ctx.memoir.reconcile();
        if only_retry_failed {
            builder = builder.only_retry_failed();
        }
        if only_clean_orphans {
            builder = builder.only_clean_orphans();
        }
        let summary = builder.await.map_err(WireError::into_status)?;

        Ok(Response::new(WireReconcileResponse::from(summary).0))
    }

    /// Returns extraction accuracy per (provider, model) over a scope slice.
    ///
    /// Read-only aggregate, no LLM call. The request's optional scope fields
    /// narrow the slice (AND-combined); all-unset aggregates the whole store.
    async fn extraction_stats(
        &self,
        request: Request<ExtractionStatsRequest>,
    ) -> Result<Response<ExtractionStatsResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let filter = WireStatsFilter::from(request.into_inner()).0;

        tracing::event!(
            name: "memoir.service.admin.extraction_stats.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            filter.agent_id = filter.agent_id.is_some(),
            filter.org_id = filter.org_id.is_some(),
            filter.user_id = filter.user_id.is_some(),
            "AdminService.ExtractionStats invoked",
        );

        let mut builder = self.ctx.memoir.extraction_stats();
        if let Some(agent_id) = filter.agent_id {
            builder = builder.agent(agent_id);
        }
        if let Some(org_id) = filter.org_id {
            builder = builder.org(org_id);
        }
        if let Some(user_id) = filter.user_id {
            builder = builder.user(user_id);
        }
        let stats = builder.await.map_err(WireError::into_status)?;

        let proto_stats = stats
            .into_iter()
            .map(|s| WireExtractionStat::try_from(s).map(|w| w.0))
            .collect::<Result<Vec<_>, Status>>()?;
        Ok(Response::new(ExtractionStatsResponse { stats: proto_stats }))
    }

    /// Returns a whole-scope snapshot of the knowledge graph.
    ///
    /// Read-only graph traversal, no LLM call. The request's optional scope
    /// fields narrow the view (AND-combined); an unset field widens across that
    /// dimension, so all-unset inspects every tenant — the cross-scope admin
    /// view, gated here by `require_admin`. A `limit` of `0` uses the library
    /// default; the library clamps to its maximum. An unconfigured graph yields
    /// an empty snapshot.
    async fn inspect_graph(
        &self,
        request: Request<InspectGraphRequest>,
    ) -> Result<Response<InspectGraphResponse>, Status> {
        let caller = self.auth().authenticate(&request).await?;
        caller.require_admin()?;
        let admin_pid = principal_pid(&caller.principal).to_owned();
        let InspectGraphRequest {
            agent_id,
            org_id,
            user_id,
            limit,
        } = request.into_inner();

        tracing::event!(
            name: "memoir.service.admin.inspect_graph.invoked",
            tracing::Level::INFO,
            admin.pid = %admin_pid,
            scope.agent_id = agent_id.is_some(),
            scope.org_id = org_id.is_some(),
            scope.user_id = user_id.is_some(),
            limit = limit,
            "AdminService.InspectGraph invoked",
        );

        let mut builder = self.ctx.memoir.inspect_graph();
        if let Some(agent_id) = agent_id {
            builder = builder.agent(agent_id);
        }

        if let Some(org_id) = org_id {
            builder = builder.org(org_id);
        }

        if let Some(user_id) = user_id {
            builder = builder.user(user_id);
        }

        if limit > 0 {
            builder = builder.limit(limit as usize);
        }

        let snapshot = builder.await.map_err(WireError::into_status)?;

        Ok(Response::new(WireInspectGraphResponse::from(snapshot).into_inner()))
    }
}

/// Resolves the wire `limit` to the value passed to `Client::failed_jobs`.
///
/// - `limit == 0` (proto3 unset default) → [`DEFAULT_FAILED_JOBS_LIMIT`].
/// - `limit < 0` → treated as the default (signed proto wire types allow
///   negative values; reject gracefully).
/// - `limit > MAX_FAILED_JOBS_LIMIT` → clamped to [`MAX_FAILED_JOBS_LIMIT`].
fn resolve_failed_jobs_limit(requested: i32) -> usize {
    if requested <= 0 {
        return DEFAULT_FAILED_JOBS_LIMIT;
    }
    (requested as usize).min(MAX_FAILED_JOBS_LIMIT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_resolve_zero_limit_to_default() {
        assert_eq!(resolve_failed_jobs_limit(0), DEFAULT_FAILED_JOBS_LIMIT);
    }

    #[test]
    fn should_resolve_negative_limit_to_default() {
        assert_eq!(resolve_failed_jobs_limit(-5), DEFAULT_FAILED_JOBS_LIMIT);
    }

    #[test]
    fn should_clamp_huge_limit_to_max() {
        assert_eq!(resolve_failed_jobs_limit(10_000), MAX_FAILED_JOBS_LIMIT);
    }

    #[test]
    fn should_preserve_reasonable_limit() {
        assert_eq!(resolve_failed_jobs_limit(25), 25);
    }
}
