//! Integration tests for the Qdrant payload extension landed in ticket 0004.
//!
//! These tests scroll Qdrant directly rather than going through the
//! `Client::search` path because the search builder filters out payload
//! contents — it returns the hydrated `Memory` from Postgres, not the
//! raw payload. Verifying the upsert wrote the expected keys requires
//! peeking at the Qdrant point itself.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::client::ClientError;
use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder, Value};

mod common;

/// Scrolls the test's Qdrant collection for a single point by pid.
async fn point_payload_for_pid(
    qdrant: &qdrant_client::Qdrant,
    collection: &str,
    pid: &str,
) -> anyhow::Result<std::collections::HashMap<String, Value>> {
    let response = qdrant
        .scroll(
            ScrollPointsBuilder::new(collection)
                .filter(Filter {
                    must: vec![Condition::matches("pid", pid.to_string())],
                    ..Default::default()
                })
                .with_payload(true)
                .limit(1u32),
        )
        .await?;
    let point = response
        .result
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("no Qdrant point found for pid {pid}"))?;
    Ok(point.payload)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_write_created_at_as_integer_milliseconds_to_payload() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    let written = client.remember("memoir payload test", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "memoir payload", Duration::from_secs(15)).await?;

    let payload = point_payload_for_pid(&qdrant, &client.collection, &written.pid).await?;
    let created_at = payload.get("created_at").expect("created_at key present");
    let ms = created_at.as_integer().expect("created_at is integer");
    let expected = written.created_at.timestamp_millis();
    assert_eq!(
        ms, expected,
        "payload created_at must match Memory.created_at (ms since epoch)"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_omit_event_at_from_payload_when_none() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    // Default writes leave event_at as None. Payload key must be absent,
    // not stored as null, so range filters against event_at exclude these
    // rows.
    let written = client.remember("no event time", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "no event", Duration::from_secs(15)).await?;

    let payload = point_payload_for_pid(&qdrant, &client.collection, &written.pid).await?;
    assert!(
        !payload.contains_key("event_at"),
        "payload must omit event_at when Memory.event_at is None; got {payload:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_write_event_at_to_payload_as_integer_milliseconds_when_set() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    let deploy_time = chrono::DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")?;
    let written = client
        .remember("the deployment happened mid-January", scope.clone())
        .event_at(deploy_time)
        .await?;
    common::wait_until_indexed(
        &client,
        &written.pid,
        &scope,
        "deployment mid-January",
        Duration::from_secs(15),
    )
    .await?;

    assert_eq!(
        written.event_at,
        Some(deploy_time),
        "returned Memory.event_at must round-trip through the store",
    );

    let payload = point_payload_for_pid(&qdrant, &client.collection, &written.pid).await?;
    let event_at = payload.get("event_at").expect("event_at key present when set");
    assert_eq!(
        event_at.as_integer(),
        Some(deploy_time.timestamp_millis()),
        "payload event_at must match the builder-supplied timestamp (ms since epoch)",
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_flatten_metadata_top_level_keys_into_payload() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    let written = client
        .remember("metadata flatten test", scope.clone())
        .metadata(serde_json::json!({
            "role": "user",
            "priority": 7,
            "tags": ["alpha", "beta"],
        }))
        .await?;
    common::wait_until_indexed(
        &client,
        &written.pid,
        &scope,
        "metadata flatten",
        Duration::from_secs(15),
    )
    .await?;

    let payload = point_payload_for_pid(&qdrant, &client.collection, &written.pid).await?;

    let role = payload.get("role").expect("role payload key present");
    assert_eq!(role.as_str().map(String::as_str), Some("user"));

    let priority = payload.get("priority").expect("priority payload key present");
    assert_eq!(priority.as_integer(), Some(7));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_write_confidence_to_payload_as_integer() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    // Episodic writes pin confidence to 100 (the user said it). Confidence is
    // always present in the payload so the selection layer (ticket 0008) can
    // range-filter on it.
    let written = client.remember("confidence payload test", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "confidence payload", Duration::from_secs(15)).await?;

    let payload = point_payload_for_pid(&qdrant, &client.collection, &written.pid).await?;
    let confidence = payload.get("confidence").expect("confidence key present");
    assert_eq!(
        confidence.as_integer(),
        Some(100),
        "episodic confidence must be 100 in the payload"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_omit_category_from_payload_when_none() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    // An episodic write is never categorized, so the category key must be
    // absent (not null) — an equality filter on category should exclude
    // uncategorized rows, matching the event_at omission semantics.
    let written = client.remember("no category yet", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "no category", Duration::from_secs(15)).await?;

    let payload = point_payload_for_pid(&qdrant, &client.collection, &written.pid).await?;
    assert!(
        !payload.contains_key("category"),
        "payload must omit category when Memory.category is None; got {payload:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_metadata_using_reserved_payload_key() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // `pid` is a reserved first-class payload key. Setting it via metadata
    // would clobber the canonical pid at upsert time; the write boundary
    // must reject it before the row is even persisted.
    let result = client
        .remember("attempt to clobber pid", scope)
        .metadata(serde_json::json!({ "pid": "smuggled" }))
        .await;

    let err = result.expect_err("write must fail when metadata uses a reserved key");
    assert!(
        matches!(&err, ClientError::ReservedMetadataKey { key } if key == "pid"),
        "expected ReservedMetadataKey error for 'pid'; got {err:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_metadata_using_each_reserved_scope_key() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    for reserved_key in ["agent_id", "org_id", "user_id", "kind", "created_at", "event_at"] {
        let mut metadata = serde_json::Map::new();
        metadata.insert(reserved_key.to_string(), serde_json::json!("anything"));
        let result = client
            .remember(format!("attempt to clobber {reserved_key}"), scope.clone())
            .metadata(serde_json::Value::Object(metadata))
            .await;
        let err = result.expect_err(&format!("write must fail when metadata uses '{reserved_key}'"));
        assert!(
            matches!(&err, ClientError::ReservedMetadataKey { key } if key == reserved_key),
            "expected ReservedMetadataKey for '{reserved_key}'; got {err:?}"
        );
    }

    Ok(())
}
