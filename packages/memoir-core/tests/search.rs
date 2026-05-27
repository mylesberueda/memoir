//! Integration tests for `Client::search` and its caller-supplied filters.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::vector::{FilterCondition, MatchValue, MatchValues, MemoryFilter, NumericRange};

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

    let related = client
        .remember("rust borrow checker prevents data races", scope.clone())
        .await?;
    let _unrelated = client
        .remember("recipe for chocolate chip cookies", scope.clone())
        .await?;

    common::wait_until_indexed(
        &client,
        &related.pid,
        &scope,
        "rust borrow checker",
        Duration::from_secs(15),
    )
    .await?;

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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_narrow_results_to_must_matching_rows() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let user_a = client
        .remember("question about deployment", scope.clone())
        .metadata(serde_json::json!({ "role": "user" }))
        .await?;
    let _assistant = client
        .remember("answer about deployment", scope.clone())
        .metadata(serde_json::json!({ "role": "assistant" }))
        .await?;
    let user_b = client
        .remember("followup deployment question", scope.clone())
        .metadata(serde_json::json!({ "role": "user" }))
        .await?;

    common::wait_until_indexed(&client, &user_a.pid, &scope, "deployment", Duration::from_secs(15)).await?;
    common::wait_until_indexed(&client, &user_b.pid, &scope, "deployment", Duration::from_secs(15)).await?;

    let only_user_rows = MemoryFilter {
        must: vec![FilterCondition::Equals {
            field: "role".into(),
            value: MatchValue::Keyword("user".into()),
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("deployment", scope)
        .limit(50)
        .metadata_filter(only_user_rows)
        .await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(hits.list().len(), 2, "expected exactly 2 user rows; got {pids:?}");
    assert!(pids.contains(&user_a.pid.as_str()));
    assert!(pids.contains(&user_b.pid.as_str()));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_union_of_matches_for_should_conditions() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let pri_1 = client
        .remember("task priority one details", scope.clone())
        .metadata(serde_json::json!({ "priority": 1 }))
        .await?;
    let _pri_2 = client
        .remember("task priority two details", scope.clone())
        .metadata(serde_json::json!({ "priority": 2 }))
        .await?;
    let pri_3 = client
        .remember("task priority three details", scope.clone())
        .metadata(serde_json::json!({ "priority": 3 }))
        .await?;

    common::wait_until_indexed(&client, &pri_1.pid, &scope, "task priority", Duration::from_secs(15)).await?;
    common::wait_until_indexed(&client, &pri_3.pid, &scope, "task priority", Duration::from_secs(15)).await?;

    let priority_one_or_three = MemoryFilter {
        should: vec![
            FilterCondition::Equals {
                field: "priority".into(),
                value: MatchValue::Integer(1),
            },
            FilterCondition::Equals {
                field: "priority".into(),
                value: MatchValue::Integer(3),
            },
        ],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("task priority", scope)
        .limit(50)
        .metadata_filter(priority_one_or_three)
        .await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(hits.list().len(), 2, "should returns OR of conditions; got {pids:?}");
    assert!(pids.contains(&pri_1.pid.as_str()));
    assert!(pids.contains(&pri_3.pid.as_str()));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_match_any_value_in_in_list() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let alpha = client
        .remember("team alpha standup notes", scope.clone())
        .metadata(serde_json::json!({ "team": "alpha" }))
        .await?;
    let _beta = client
        .remember("team beta standup notes", scope.clone())
        .metadata(serde_json::json!({ "team": "beta" }))
        .await?;
    let gamma = client
        .remember("team gamma standup notes", scope.clone())
        .metadata(serde_json::json!({ "team": "gamma" }))
        .await?;

    common::wait_until_indexed(&client, &alpha.pid, &scope, "standup notes", Duration::from_secs(15)).await?;
    common::wait_until_indexed(&client, &gamma.pid, &scope, "standup notes", Duration::from_secs(15)).await?;

    let alpha_or_gamma = MemoryFilter {
        must: vec![FilterCondition::In {
            field: "team".into(),
            values: MatchValues::Keywords(vec!["alpha".into(), "gamma".into()]),
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("standup notes", scope)
        .limit(50)
        .metadata_filter(alpha_or_gamma)
        .await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(hits.list().len(), 2, "in-list matches alpha and gamma; got {pids:?}");
    assert!(pids.contains(&alpha.pid.as_str()));
    assert!(pids.contains(&gamma.pid.as_str()));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_include_only_rows_within_numeric_range() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let mut all_pids: Vec<(i64, String)> = Vec::new();
    for n in 1..=10 {
        let written = client
            .remember(format!("ranking entry number {n}"), scope.clone())
            .metadata(serde_json::json!({ "rank": n }))
            .await?;
        all_pids.push((n, written.pid));
    }

    let last_pid = &all_pids.last().expect("ten rows written").1;
    common::wait_until_indexed(&client, last_pid, &scope, "ranking entry", Duration::from_secs(15)).await?;

    // gte = 5 AND lt = 8 → rank values {5, 6, 7}.
    let mid_range = MemoryFilter {
        must: vec![FilterCondition::Range {
            field: "rank".into(),
            range: NumericRange {
                gte: Some(5.0),
                lt: Some(8.0),
                ..NumericRange::default()
            },
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("ranking entry", scope)
        .limit(50)
        .metadata_filter(mid_range)
        .await?;

    let returned: std::collections::HashSet<String> = hits.list().iter().map(|m| m.pid.clone()).collect();
    let expected: std::collections::HashSet<String> = all_pids
        .iter()
        .filter(|(n, _)| (5..8).contains(n))
        .map(|(_, p)| p.clone())
        .collect();
    assert_eq!(returned, expected, "range gte=5, lt=8 must select exactly ranks 5/6/7");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_treat_empty_filter_identically_to_no_filter() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    for content in [
        "first batch entry one",
        "first batch entry two",
        "first batch entry three",
    ] {
        let _ = client.remember(content, scope.clone()).await?;
    }
    let last = client.remember("first batch entry four", scope.clone()).await?;
    common::wait_until_indexed(&client, &last.pid, &scope, "first batch entry", Duration::from_secs(15)).await?;

    let unfiltered: std::collections::HashSet<String> = client
        .search("first batch entry", scope.clone())
        .limit(50)
        .await?
        .list()
        .iter()
        .map(|m| m.pid.clone())
        .collect();

    let empty: std::collections::HashSet<String> = client
        .search("first batch entry", scope)
        .limit(50)
        .metadata_filter(MemoryFilter::default())
        .await?
        .list()
        .iter()
        .map(|m| m.pid.clone())
        .collect();

    assert_eq!(unfiltered, empty, "empty filter must behave as no filter");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_combine_metadata_filter_and_min_similarity_as_and() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Two on-topic rows (high similarity to query), one off-topic (low).
    // Among the two on-topic rows, only one is tagged `role = "user"`.
    let on_topic_user = client
        .remember("the deploy pipeline runs nightly with kubernetes", scope.clone())
        .metadata(serde_json::json!({ "role": "user" }))
        .await?;
    let _on_topic_assistant = client
        .remember("the deploy pipeline yes uses kubernetes manifests", scope.clone())
        .metadata(serde_json::json!({ "role": "assistant" }))
        .await?;
    let _off_topic_user = client
        .remember("breakfast tacos with eggs and chorizo", scope.clone())
        .metadata(serde_json::json!({ "role": "user" }))
        .await?;

    common::wait_until_indexed(
        &client,
        &on_topic_user.pid,
        &scope,
        "deploy pipeline",
        Duration::from_secs(15),
    )
    .await?;

    let only_user = MemoryFilter {
        must: vec![FilterCondition::Equals {
            field: "role".into(),
            value: MatchValue::Keyword("user".into()),
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("deploy pipeline kubernetes", scope)
        .limit(50)
        .metadata_filter(only_user)
        .min_similarity(0.4)
        .await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(
        pids.contains(&on_topic_user.pid.as_str()),
        "the on-topic user row must survive both filters; got {pids:?}"
    );
    // The off-topic user row should fall under the similarity floor; the
    // on-topic assistant row should fail the role filter. Neither should
    // appear.
    assert_eq!(
        hits.list().len(),
        1,
        "AND-composition: filter + threshold should leave one row; got {pids:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_scope_widening_via_caller_supplied_must() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    // Write a memory in scope A. Then search as scope B with a `must` that
    // tries to pull in scope A's `agent_id`. Scope B's own scope condition
    // ANDs with the caller's `must`, producing an unsatisfiable filter
    // (the row would need both agent_ids simultaneously) — so no rows leak.
    let leak_target = client
        .remember("scope A confidential planning notes", scope_a.clone())
        .await?;
    common::wait_until_indexed(
        &client,
        &leak_target.pid,
        &scope_a,
        "planning notes",
        Duration::from_secs(15),
    )
    .await?;

    let widen_attempt = MemoryFilter {
        must: vec![FilterCondition::Equals {
            field: "agent_id".into(),
            value: MatchValue::Keyword(scope_a.agent_id.clone()),
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("planning notes", scope_b)
        .limit(50)
        .metadata_filter(widen_attempt)
        .await?;

    assert!(
        hits.list().is_empty(),
        "caller in scope B must not see scope A's rows via must agent_id=A; got {} hits",
        hits.list().len()
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_not_leak_other_scope_rows_when_caller_uses_must_not() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    // Write in scope A; as a caller in scope B, set `must_not` excluding
    // scope A's user_id. The scope filter still pins the search to scope B's
    // own `user_id`, so this `must_not` is operating on a key that doesn't
    // overlap. No scope A rows should appear.
    let row_a = client.remember("scope A private fact", scope_a.clone()).await?;
    common::wait_until_indexed(&client, &row_a.pid, &scope_a, "private fact", Duration::from_secs(15)).await?;

    let dodge_attempt = MemoryFilter {
        must_not: vec![FilterCondition::Equals {
            field: "user_id".into(),
            value: MatchValue::Keyword(scope_a.user_id.clone()),
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("private fact", scope_b)
        .limit(50)
        .metadata_filter(dodge_attempt)
        .await?;

    assert!(
        hits.list().is_empty(),
        "must_not on another scope's user_id must not leak its rows; got {} hits",
        hits.list().len()
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_no_hits_when_filter_targets_unknown_field() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let row = client
        .remember("subject matter expert document", scope.clone())
        .metadata(serde_json::json!({ "known_field": "value" }))
        .await?;
    common::wait_until_indexed(&client, &row.pid, &scope, "subject matter", Duration::from_secs(15)).await?;

    let target_missing_field = MemoryFilter {
        must: vec![FilterCondition::Equals {
            field: "does_not_exist".into(),
            value: MatchValue::Keyword("anything".into()),
        }],
        ..MemoryFilter::default()
    };

    let hits = client
        .search("subject matter", scope)
        .limit(50)
        .metadata_filter(target_missing_field)
        .await?;

    assert!(
        hits.list().is_empty(),
        "Qdrant treats a `must` on an absent payload field as no match; got {} hits",
        hits.list().len()
    );

    Ok(())
}

// ---------- Time-range builder methods (ticket 0006) ----------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_filter_by_event_at_window_when_set() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let january = chrono::DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")?;
    let march = chrono::DateTime::parse_from_rfc3339("2026-03-15T12:00:00Z")?;
    let in_window = client
        .remember("the deployment happened in January", scope.clone())
        .event_at(january)
        .await?;
    let out_of_window = client
        .remember("the deployment happened in March", scope.clone())
        .event_at(march)
        .await?;
    common::wait_until_indexed(
        &client,
        &in_window.pid,
        &scope,
        "deployment January",
        Duration::from_secs(15),
    )
    .await?;
    common::wait_until_indexed(
        &client,
        &out_of_window.pid,
        &scope,
        "deployment March",
        Duration::from_secs(15),
    )
    .await?;

    let feb_1 = chrono::DateTime::parse_from_rfc3339("2026-02-01T00:00:00Z")?;
    let jan_1 = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")?;
    let hits = client
        .search("deployment", scope)
        .event_at_after(jan_1)
        .event_at_before(feb_1)
        .await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(
        pids.contains(&in_window.pid.as_str()),
        "January memory must be in window"
    );
    assert!(
        !pids.contains(&out_of_window.pid.as_str()),
        "March memory must be filtered out"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_exclude_memories_with_null_event_at_when_filtering_by_event_at() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Two memories with the same content, one carrying event_at and the
    // other not. event_at-range filtering must drop the latter — Qdrant
    // treats missing payload keys on a range target as non-matches, which
    // is the omit-when-None semantic from ticket 0004.
    let with_event_at = client
        .remember("deployment status", scope.clone())
        .event_at(chrono::DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")?)
        .await?;
    let without_event_at = client.remember("deployment status update", scope.clone()).await?;
    common::wait_until_indexed(
        &client,
        &with_event_at.pid,
        &scope,
        "deployment",
        Duration::from_secs(15),
    )
    .await?;
    common::wait_until_indexed(
        &client,
        &without_event_at.pid,
        &scope,
        "deployment",
        Duration::from_secs(15),
    )
    .await?;

    let jan_1 = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")?;
    let hits = client.search("deployment", scope).event_at_after(jan_1).await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&with_event_at.pid.as_str()));
    assert!(
        !pids.contains(&without_event_at.pid.as_str()),
        "memory without event_at must not satisfy event-time range filter; got {pids:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_filter_by_created_at_window() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Both memories get created_at = now. Capture a boundary after the
    // first and before the second to exercise the .created_after filter.
    let early = client.remember("early note about kubernetes", scope.clone()).await?;
    common::wait_until_indexed(&client, &early.pid, &scope, "kubernetes", Duration::from_secs(15)).await?;
    let boundary = chrono::Utc::now();
    // Sleep a beat to make sure the second memory's created_at is strictly
    // after `boundary` even at millisecond resolution.
    tokio::time::sleep(Duration::from_millis(20)).await;
    let late = client.remember("late note about kubernetes too", scope.clone()).await?;
    common::wait_until_indexed(&client, &late.pid, &scope, "kubernetes", Duration::from_secs(15)).await?;

    let hits = client.search("kubernetes", scope).created_after(boundary).await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&late.pid.as_str()), "late memory must be in window");
    assert!(
        !pids.contains(&early.pid.as_str()),
        "early memory (before boundary) must be filtered out; got {pids:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_compose_time_range_with_metadata_filter() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Two memories with different roles. Time-range plus role filter
    // must AND-join: only the matching role *and* the matching window
    // returns.
    let target = client
        .remember("deployment status from the user", scope.clone())
        .metadata(serde_json::json!({ "role": "user" }))
        .event_at(chrono::DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")?)
        .await?;
    let wrong_role = client
        .remember("deployment status from the assistant", scope.clone())
        .metadata(serde_json::json!({ "role": "assistant" }))
        .event_at(chrono::DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")?)
        .await?;
    common::wait_until_indexed(&client, &target.pid, &scope, "deployment", Duration::from_secs(15)).await?;
    common::wait_until_indexed(&client, &wrong_role.pid, &scope, "deployment", Duration::from_secs(15)).await?;

    let only_user = MemoryFilter {
        must: vec![FilterCondition::Equals {
            field: "role".into(),
            value: MatchValue::Keyword("user".into()),
        }],
        ..MemoryFilter::default()
    };
    let jan_1 = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")?;
    let feb_1 = chrono::DateTime::parse_from_rfc3339("2026-02-01T00:00:00Z")?;

    let hits = client
        .search("deployment", scope)
        .metadata_filter(only_user)
        .event_at_after(jan_1)
        .event_at_before(feb_1)
        .await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(
        pids,
        vec![target.pid.as_str()],
        "AND-join must keep only role=user in window"
    );

    Ok(())
}
