#![cfg(all(test, feature = "integration"))]

use memoir_core::store::MemoryStore as _;
use memoir_sdk::memoir::v1::{
    DeleteFailedJobRequest, JobKind, ListFailedJobsRequest, PendingJobsCountRequest, ReconcileRequest, RememberRequest,
    RetryJobRequest, UnsupersedeRequest,
};
use tonic::{Code, Request};

use super::common::TestHarness;

/// Writes a memory through the gRPC surface and returns its pid. Used to
/// produce a valid `source_pid` for seeded `memory_jobs` rows.
async fn write_memory_for_seeding(harness: &TestHarness) -> String {
    let mut memory = harness.memory.clone();
    let scope = harness.fresh_scope();
    let written = memory
        .remember(harness.authed(RememberRequest {
            scope: Some(scope),
            content: "seed-target memory".to_owned(),
            metadata: None,
        }))
        .await
        .expect("remember rpc")
        .into_inner()
        .memory
        .expect("memory present");
    written.pid
}

#[tokio::test(flavor = "multi_thread")]
async fn should_list_failed_jobs_when_admin() {
    let harness = TestHarness::start().await.expect("harness");
    let source_pid = write_memory_for_seeding(&harness).await;
    let job_id = harness
        .seed_failed_job(&source_pid, "embed", "test-injected failure")
        .await
        .expect("seed failed job");

    let library_view = harness
        .memoir
        .failed_jobs(50)
        .await
        .expect("library failed_jobs sanity");
    assert!(
        library_view.iter().any(|j| j.id == job_id),
        "library should see the seeded job"
    );

    let mut admin = harness.admin.clone();
    let resp = admin
        .list_failed_jobs(harness.authed(ListFailedJobsRequest { limit: 50 }))
        .await
        .expect("list_failed_jobs rpc")
        .into_inner();

    assert!(
        resp.jobs.iter().any(|j| j.id == job_id),
        "ListFailedJobs must include the seeded job; got {:?}",
        resp.jobs.iter().map(|j| j.id).collect::<Vec<_>>()
    );
    let seeded = resp.jobs.iter().find(|j| j.id == job_id).expect("seeded job present");
    assert_eq!(seeded.source_pid, source_pid);
    assert_eq!(seeded.kind, JobKind::Embed as i32);
    assert_eq!(seeded.attempts, 3);
    assert_eq!(seeded.failure_reason.as_deref(), Some("test-injected failure"));
}

#[tokio::test(flavor = "multi_thread")]
async fn should_return_pending_count() {
    let harness = TestHarness::start().await.expect("harness");

    let mut admin = harness.admin.clone();
    let resp = admin
        .pending_jobs_count(harness.authed(PendingJobsCountRequest {}))
        .await
        .expect("pending_jobs_count rpc")
        .into_inner();

    assert!(resp.count >= 0, "count must be non-negative; got {}", resp.count);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_retry_job_when_admin() {
    let harness = TestHarness::start().await.expect("harness");
    let source_pid = write_memory_for_seeding(&harness).await;
    let job_id = harness
        .seed_failed_job(&source_pid, "embed", "test-injected failure")
        .await
        .expect("seed");

    let failed_before = harness.count_jobs_with_state("failed").await.expect("count before");
    assert!(failed_before >= 1, "must have at least the seeded failed job");

    let mut admin = harness.admin.clone();
    admin
        .retry_job(harness.authed(RetryJobRequest { id: job_id }))
        .await
        .expect("retry_job rpc");

    let failed_after = harness.count_jobs_with_state("failed").await.expect("count after");
    assert_eq!(
        failed_after,
        failed_before - 1,
        "retry_job must move the row out of `failed` state"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn should_delete_failed_job_when_admin() {
    let harness = TestHarness::start().await.expect("harness");
    let source_pid = write_memory_for_seeding(&harness).await;
    let job_id = harness
        .seed_failed_job(&source_pid, "embed", "test-injected failure")
        .await
        .expect("seed");

    let mut admin = harness.admin.clone();
    admin
        .delete_failed_job(harness.authed(DeleteFailedJobRequest { id: job_id }))
        .await
        .expect("delete_failed_job rpc");

    let retry_err = admin
        .retry_job(harness.authed(RetryJobRequest { id: job_id }))
        .await
        .expect_err("retry_job after delete must fail");
    assert_eq!(retry_err.code(), Code::NotFound);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_unsupersede_memory_when_admin() {
    let harness = TestHarness::start().await.expect("harness");

    let pid_a = write_memory_for_seeding(&harness).await;
    let pid_b = write_memory_for_seeding(&harness).await;

    harness
        .memoir
        .store()
        .supersede(&pid_a, &pid_b)
        .await
        .expect("seed supersede");

    let mut admin = harness.admin.clone();
    admin
        .unsupersede(harness.authed(UnsupersedeRequest { pid: pid_a.clone() }))
        .await
        .expect("unsupersede rpc");

    let restored = harness.memoir.recall(&pid_a).await.expect("recall after unsupersede");
    assert_eq!(restored.supersession, None, "supersession marker must be cleared");
}

#[tokio::test(flavor = "multi_thread")]
async fn should_reconcile_when_admin() {
    let harness = TestHarness::start().await.expect("harness");

    // Reconcile's retry pass operates on memories.qdrant_status='failed',
    // not memory_jobs.state='failed' — those are distinct concepts. The
    // exact memoir-core reconcile semantics are covered by memoir-core's
    // own integration suite; this test asserts only that the RPC round-
    // trips successfully through the admin gate and returns a well-formed
    // ReconcileSummary.
    let mut admin = harness.admin.clone();
    let summary = admin
        .reconcile(harness.authed(ReconcileRequest {
            only_retry_failed: false,
            only_clean_orphans: false,
        }))
        .await
        .expect("reconcile rpc")
        .into_inner();

    assert!(summary.failed_retried >= 0);
    assert!(summary.failed_recovered >= 0);
    assert!(summary.orphans_deleted >= 0);
    assert!(summary.failed_recovered <= summary.failed_retried);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_reject_non_admin_caller() {
    let harness = TestHarness::start().await.expect("harness");
    let non_admin_token = harness.login_non_admin().await.expect("non-admin login");

    let mut admin = harness.admin.clone();
    let err = admin
        .pending_jobs_count(harness.authed_with(PendingJobsCountRequest {}, &non_admin_token))
        .await
        .expect_err("non-admin caller must be rejected");
    assert_eq!(err.code(), Code::PermissionDenied);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_reject_unauthenticated_caller() {
    let harness = TestHarness::start().await.expect("harness");

    let mut admin = harness.admin.clone();
    let err = admin
        .pending_jobs_count(Request::new(PendingJobsCountRequest {}))
        .await
        .expect_err("unauthenticated must be rejected");
    assert_eq!(err.code(), Code::Unauthenticated);
}

