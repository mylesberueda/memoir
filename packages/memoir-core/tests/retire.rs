//! Integration tests for the correction-model retirement primitives
//! (`Client::reject` / `Client::mark_stale`, epic 0011 ticket 0009).
//!
//! Verifies the load-bearing behaviors against live Postgres + Qdrant: a
//! retired row disappears from search but stays recall-reachable (carrying
//! its reason), and its vector is evicted.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::memory::{Confidence, MemoryKind, RetirementReason};
use memoir_core::store::{MemoryStore, NewMemory};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_hide_from_search_but_keep_recall_reachable() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("the user dislikes cilantro", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "cilantro", Duration::from_secs(15)).await?;

    // Pre-rejection: the row is searchable.
    let before = client.search("cilantro", scope.clone()).limit(10).await?;
    assert!(
        before.list().iter().any(|m| m.pid == written.pid),
        "row must be searchable before rejection"
    );

    client.reject(&written.pid).await?;

    // Post-rejection: gone from search (vector evicted + read filter).
    let after = client.search("cilantro", scope.clone()).limit(10).await?;
    assert!(
        !after.list().iter().any(|m| m.pid == written.pid),
        "rejected row must not appear in search results"
    );

    // But still recall-reachable by pid, carrying its reason — the row is
    // kept for the reprocess guard + accuracy metric.
    let recalled = client.recall(&written.pid).await?;
    assert_eq!(
        recalled.retirement,
        Some(RetirementReason::Rejected),
        "recalled row must report Rejected; got {:?}",
        recalled.retirement
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_mark_stale_hide_from_search_with_stale_reason() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("deployment is on Friday", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "deployment", Duration::from_secs(15)).await?;

    client.mark_stale(&written.pid).await?;

    let after = client.search("deployment", scope.clone()).limit(10).await?;
    assert!(
        !after.list().iter().any(|m| m.pid == written.pid),
        "stale row must not appear in search results"
    );

    let recalled = client.recall(&written.pid).await?;
    assert_eq!(
        recalled.retirement,
        Some(RetirementReason::Stale),
        "recalled row must report Stale; got {:?}",
        recalled.retirement
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_not_found_when_retiring_unknown_pid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let result = client.reject("does-not-exist").await;
    assert!(result.is_err(), "retiring an unknown pid must error; got {result:?}");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_compute_extraction_accuracy_from_rejected_over_total() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let store = client.store();

    // An episodic source plus two semantic rows derived from the same model.
    let source = client.remember("my favorite color is green", scope.clone()).await?;
    let write_semantic = async |content: &str| -> anyhow::Result<String> {
        Ok(store
            .remember(NewMemory {
                scope: scope.clone(),
                content: content.to_string(),
                metadata: serde_json::json!({ "provider": "ollama", "model": "qwen3:14b" }),
                kind: MemoryKind::Semantic,
                source_pid: Some(source.pid.clone()),
                event_at: None,
                confidence: Confidence::new(80),
            })
            .await?
            .pid)
    };
    let wrong = write_semantic("the user hates green").await?;
    let _right = write_semantic("the user likes green").await?;

    client.reject(&wrong).await?;

    // The live aggregate reads provider/model out of the metadata JSON and
    // counts rejected vs total: 1 of 2 → 50% accuracy for this model.
    let stats = client
        .extraction_stats()
        .org(&scope.org_id)
        .user(&scope.user_id)
        .agent(&scope.agent_id)
        .await?;

    assert_eq!(stats.len(), 1, "one (provider, model) pair in this scope; got {stats:?}");
    let stat = &stats[0];
    assert_eq!((stat.provider.as_str(), stat.model.as_str()), ("ollama", "qwen3:14b"));
    assert_eq!((stat.total, stat.rejected), (2, 1));
    assert!((stat.accuracy() - 0.5).abs() < f64::EPSILON, "accuracy must be 1 − 1/2; got {}", stat.accuracy());

    Ok(())
}
