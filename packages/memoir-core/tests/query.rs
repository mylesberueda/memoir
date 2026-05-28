//! Integration tests for `Client::query`.

#![cfg(feature = "integration")]

use std::time::Duration;

use chrono::Utc;
use memoir_core::client::{DecayFn, RankingStrategy};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_memory_context_with_query_results() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("the user likes Rust", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "Rust", Duration::from_secs(15)).await?;

    let context = client.query("what does the user like?", scope).await?;
    assert!(!context.memories().is_empty(), "query should return at least one memory");
    assert!(
        context
            .memories()
            .iter()
            .any(|m| m.content == "the user likes Rust"),
        "expected the written memory in results"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_default_to_hybrid_strategy() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("hybrid default check", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "hybrid default", Duration::from_secs(15)).await?;

    let context = client.query("hybrid default check", scope).await?;
    matches!(context.strategy_used(), RankingStrategy::Hybrid { .. });
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_blend_cosine_and_recency_via_hybrid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Two topically-relevant memories. The older one is written first; the
    // newer one second. Both should match the query roughly equally. Under
    // a recency-aware hybrid, the newer should rank higher.
    let old = client.remember("user uses Rust for systems work", scope.clone()).await?;
    common::wait_until_indexed(&client, &old.pid, &scope, "Rust", Duration::from_secs(15)).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    let new = client.remember("user uses Rust for web servers", scope.clone()).await?;
    common::wait_until_indexed(&client, &new.pid, &scope, "web servers", Duration::from_secs(15)).await?;

    // Force a recency-heavy hybrid so the newer memory dominates.
    let context = client
        .query("what does the user use Rust for?", scope)
        .ranking(RankingStrategy::Hybrid {
            alpha: 0.1,
            decay: DecayFn::Exponential {
                half_life: chrono::Duration::seconds(1),
            },
        })
        .await?;

    let top = context.memories().first().expect("at least one result");
    assert_eq!(top.pid, new.pid, "newer memory must rank first under recency-heavy hybrid");
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_populate_score_on_returned_memories() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("score population check", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "score population", Duration::from_secs(15)).await?;

    let context = client.query("score population check", scope).await?;
    let top = context.memories().first().expect("at least one result");
    assert!(top.score.is_some(), "score must be populated on query results");
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_render_display_with_date_and_relative_prefix() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("display rendering check", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "display rendering", Duration::from_secs(15)).await?;

    let context = client.query("display rendering check", scope).await?;
    let rendered = context.to_string();
    let today = Utc::now().format("%Y-%m-%d").to_string();
    assert!(
        rendered.contains(&today),
        "rendered context must contain today's date; got: {rendered}"
    );
    assert!(
        rendered.contains("ago") || rendered.contains("just now"),
        "rendered context must contain a relative-time label; got: {rendered}"
    );
    assert!(
        rendered.contains("display rendering check"),
        "rendered context must contain the memory content; got: {rendered}"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_respect_limit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    for i in 0..5 {
        let m = client.remember(format!("memory {i} about topic X"), scope.clone()).await?;
        common::wait_until_indexed(&client, &m.pid, &scope, "topic X", Duration::from_secs(15)).await?;
    }

    let context = client.query("topic X", scope).limit(2).await?;
    assert_eq!(context.memories().len(), 2);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_deref_to_memory_slice() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("deref test", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "deref test", Duration::from_secs(15)).await?;

    let context = client.query("deref test", scope).await?;
    assert!(!context.is_empty(), "Deref makes len/is_empty work");
    let _first = &context[0];
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_empty_context_when_no_matches() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    let written = client.remember("only in scope a", scope_a.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope_a, "only in scope a", Duration::from_secs(15)).await?;

    let context = client.query("only in scope a", scope_b).await?;
    assert!(context.memories().is_empty());
    let rendered = context.to_string();
    assert!(
        !rendered.contains("only in scope a"),
        "rendered empty context must not leak other scopes' content"
    );
    Ok(())
}
