//! Integration tests for `Client::search` and its caller-supplied filters.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::vector::{FilterCondition, MatchValue, MemoryFilter};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_exclude_memories_matching_must_not_metadata_filter() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Three memories, two from conversation 1 and one from conversation 2.
    // A search with `must_not conversation_id = 1` must surface only the
    // conversation-2 row.
    let keep = client
        .remember("the deploy process uses kubernetes manifests", scope.clone())
        .metadata(serde_json::json!({ "conversation_id": 2 }))
        .await?;
    let _ = client
        .remember("the deploy process uses kubernetes manifests too", scope.clone())
        .metadata(serde_json::json!({ "conversation_id": 1 }))
        .await?;
    let _ = client
        .remember("deploy process question from conv one again", scope.clone())
        .metadata(serde_json::json!({ "conversation_id": 1 }))
        .await?;

    common::wait_until_indexed(&client, &keep.pid, &scope, "deploy process", Duration::from_secs(15)).await?;

    let exclude_conversation_1 = MemoryFilter {
        must_not: vec![FilterCondition::Equals {
            field: "conversation_id".into(),
            value: MatchValue::Integer(1),
        }],
        ..MemoryFilter::default()
    };

    let filtered = client
        .search("deploy process", scope.clone())
        .limit(50)
        .metadata_filter(exclude_conversation_1)
        .await?;

    let pids: Vec<&str> = filtered.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(
        pids.contains(&keep.pid.as_str()),
        "conversation-2 row must be present; got {pids:?}"
    );
    assert_eq!(
        filtered.list().len(),
        1,
        "must_not filter must drop both conversation-1 rows; got {pids:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_drop_hits_below_min_similarity_threshold() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let related = client.remember("rust borrow checker prevents data races", scope.clone()).await?;
    let _unrelated = client.remember("recipe for chocolate chip cookies", scope.clone()).await?;

    common::wait_until_indexed(&client, &related.pid, &scope, "rust borrow checker", Duration::from_secs(15)).await?;

    // Use an absurdly high floor — even the on-topic row should be dropped.
    let none_qualify = client
        .search("rust borrow checker", scope.clone())
        .limit(50)
        .min_similarity(0.999_999)
        .await?;
    assert!(
        none_qualify.list().is_empty(),
        "min_similarity = 0.999999 should drop every hit; got {}",
        none_qualify.list().len()
    );

    // With a tiny floor every indexed hit qualifies.
    let some_qualify = client
        .search("rust borrow checker", scope.clone())
        .limit(50)
        .min_similarity(-1.0)
        .await?;
    assert!(
        some_qualify.list().iter().any(|m| m.pid == related.pid),
        "min_similarity = -1.0 should preserve hits; got {} results",
        some_qualify.list().len()
    );

    Ok(())
}
