//! Integration tests for the feedback correction surface (epic 0011 ticket 0011).
//!
//! `Client::feedback(wrong_semantic_pid).correction(text)` drives the reprocess
//! engine with reason=Rejected. These run against live Postgres + Qdrant with a
//! draining worker but no extraction LLM, so they assert the deterministic
//! effects (the wrong row is retired as Rejected; the feedback lands as an
//! origin:feedback episodic row) — the corrected re-derivation is LLM-bound and
//! belongs in an Ollama-gated test.

#![cfg(feature = "integration")]

use std::time::{Duration, Instant};

use memoir_core::client::ClientError;
use memoir_core::memory::{Confidence, MemoryKind, RetirementReason};
use memoir_core::store::{MemoryStore, NewMemory};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_retire_wrong_semantic_as_rejected_via_feedback() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let store = client.store();

    // A real episodic source, plus a wrong semantic fact derived from it.
    let episodic = client.remember("my favorite color is green", scope.clone()).await?;
    let wrong = store
        .remember(NewMemory {
            scope: scope.clone(),
            content: "the user hates the color green".to_string(),
            metadata: serde_json::json!({ "origin": "test" }),
            kind: MemoryKind::Semantic,
            source_pid: Some(episodic.pid.clone()),
            event_at: None,
            confidence: Confidence::new(80),
        })
        .await?;

    client
        .feedback(&wrong.pid)
        .correction("green is actually my favorite color")
        .await?;

    // The worker drains the Reprocess job: the wrong row is retired as Rejected
    // (kept + reason set) and its vector evicted. Poll recall until it flips.
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut delay = Duration::from_millis(50);
    let mut retired = false;
    while Instant::now() < deadline {
        let m = client.recall(&wrong.pid).await?;
        if m.retirement == Some(RetirementReason::Rejected) {
            retired = true;
            break;
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_millis(500));
    }

    assert!(retired, "feedback must retire the wrong semantic row as Rejected");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_persist_feedback_as_origin_feedback_episodic_row() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let store = client.store();

    let episodic = client.remember("I dislike cilantro strongly", scope.clone()).await?;
    let wrong = store
        .remember(NewMemory {
            scope: scope.clone(),
            content: "the user loves cilantro".to_string(),
            metadata: serde_json::json!({ "origin": "test" }),
            kind: MemoryKind::Semantic,
            source_pid: Some(episodic.pid.clone()),
            event_at: None,
            confidence: Confidence::new(70),
        })
        .await?;

    let correction = "the user dislikes cilantro";
    client.feedback(&wrong.pid).correction(correction).await?;

    // The engine persists the correction as an origin:feedback episodic row.
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut delay = Duration::from_millis(50);
    let mut found = false;
    while Instant::now() < deadline {
        let timeline = client.timeline(scope.clone()).episodic().limit(50).await?;
        if timeline.iter().any(|m| {
            m.content == correction && m.metadata.get("origin").and_then(|v| v.as_str()) == Some("feedback")
        }) {
            found = true;
            break;
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_millis(500));
    }

    assert!(
        found,
        "feedback must persist the correction as an origin:feedback episodic row"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_feedback_on_episodic_target() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // An episodic memory is not a correctable extraction — edit() is its path.
    let episodic = client.remember("a plain utterance", scope.clone()).await?;

    let result = client.feedback(&episodic.pid).correction("nope").await;
    assert!(
        matches!(&result, Err(ClientError::NotCorrectable { .. })),
        "feedback on an episodic target must be NotCorrectable; got {result:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_not_found_when_feedback_targets_unknown_pid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let result = client.feedback("does-not-exist").correction("x").await;
    assert!(result.is_err(), "feedback on an unknown pid must error; got {result:?}");

    Ok(())
}
