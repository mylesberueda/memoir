#![cfg(all(test, feature = "integration"))]

use memoir_sdk::memoir::v1::forget_request::Target as ForgetTargetProto;
use memoir_sdk::memoir::v1::{
    FilterCondition, ForgetRequest, MatchValue, MemoryFilter, MemoryStatus, RecallRequest, RememberRequest,
    Scope as ProtoScope, SearchRequest, filter_condition, match_value,
};
use pbjson_types::{Struct, Value, value::Kind};
use tonic::{Code, Request};

use super::common::TestHarness;

#[tokio::test(flavor = "multi_thread")]
async fn should_remember_and_return_pending_memory() {
    let harness = TestHarness::start().await.expect("harness");
    let mut memory = harness.memory.clone();
    let scope = harness.fresh_scope();

    let resp = memory
        .remember(harness.authed(RememberRequest {
            scope: Some(scope.clone()),
            content: "the user prefers dark roast coffee".to_owned(),
            metadata: None,
        }))
        .await
        .expect("remember rpc")
        .into_inner();

    let written = resp.memory.expect("memory present in response");
    assert!(!written.pid.is_empty(), "pid must be set");
    assert_eq!(written.content, "the user prefers dark roast coffee");
    assert_eq!(written.status, MemoryStatus::Pending as i32);
    assert_eq!(written.scope, Some(scope));
}

#[tokio::test(flavor = "multi_thread")]
async fn should_search_and_return_indexed_memories() {
    let mut harness = TestHarness::start().await.expect("harness");
    let scope = harness.fresh_scope();

    let written = harness
        .memory
        .remember(harness.authed(RememberRequest {
            scope: Some(scope.clone()),
            content: "the user enjoys hiking on weekends".to_owned(),
            metadata: None,
        }))
        .await
        .expect("remember rpc")
        .into_inner()
        .memory
        .expect("memory present");

    harness
        .wait_until_searchable(&written.pid, scope.clone(), "hiking")
        .await
        .expect("wait_until_searchable");

    let hits = harness
        .memory
        .search(harness.authed(SearchRequest {
            scope: Some(scope),
            query: "hiking".to_owned(),
            limit: 10,
            metadata_filter: None,
            min_similarity: None,
            kinds: None,
            with_graph_enrichment: false,
            graph_depth: 0,
        }))
        .await
        .expect("search rpc")
        .into_inner()
        .hits;

    assert!(!hits.is_empty(), "search must return at least one hit");
    let hit = hits
        .iter()
        .find(|h| h.memory.as_ref().is_some_and(|m| m.pid == written.pid))
        .expect("hit for the just-written memory");
    assert!(hit.score > 0.0, "score must be a positive cosine similarity");
}

#[tokio::test(flavor = "multi_thread")]
async fn should_recall_by_pid() {
    let harness = TestHarness::start().await.expect("harness");
    let mut memory = harness.memory.clone();
    let scope = harness.fresh_scope();

    let written = memory
        .remember(harness.authed(RememberRequest {
            scope: Some(scope),
            content: "groceries: milk, eggs, bread".to_owned(),
            metadata: None,
        }))
        .await
        .expect("remember rpc")
        .into_inner()
        .memory
        .expect("memory present");

    let recalled = memory
        .recall(harness.authed(RecallRequest {
            pid: written.pid.clone(),
        }))
        .await
        .expect("recall rpc")
        .into_inner()
        .memory
        .expect("recall memory present");

    assert_eq!(recalled.pid, written.pid);
    assert_eq!(recalled.content, written.content);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_forget_by_pid() {
    let harness = TestHarness::start().await.expect("harness");
    let mut memory = harness.memory.clone();
    let scope = harness.fresh_scope();

    let written = memory
        .remember(harness.authed(RememberRequest {
            scope: Some(scope),
            content: "ephemeral note".to_owned(),
            metadata: None,
        }))
        .await
        .expect("remember rpc")
        .into_inner()
        .memory
        .expect("memory present");

    let deleted = memory
        .forget(harness.authed(ForgetRequest {
            target: Some(ForgetTargetProto::Pid(written.pid.clone())),
            hard_delete: true,
        }))
        .await
        .expect("forget rpc")
        .into_inner()
        .deleted_pids;
    assert_eq!(deleted, vec![written.pid.clone()]);

    let recall_err = memory
        .recall(harness.authed(RecallRequest { pid: written.pid }))
        .await
        .expect_err("recall of forgotten pid must fail");
    assert_eq!(recall_err.code(), Code::NotFound);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_forget_by_scope() {
    let harness = TestHarness::start().await.expect("harness");
    let mut memory = harness.memory.clone();
    let scope = harness.fresh_scope();

    let mut pids = Vec::new();
    for content in ["fact one", "fact two", "fact three"] {
        let written = memory
            .remember(harness.authed(RememberRequest {
                scope: Some(scope.clone()),
                content: content.to_owned(),
                metadata: None,
            }))
            .await
            .expect("remember rpc")
            .into_inner()
            .memory
            .expect("memory present");
        pids.push(written.pid);
    }

    let deleted = memory
        .forget(harness.authed(ForgetRequest {
            target: Some(ForgetTargetProto::Scope(scope)),
            hard_delete: true,
        }))
        .await
        .expect("forget rpc")
        .into_inner()
        .deleted_pids;

    assert_eq!(
        deleted.len(),
        pids.len(),
        "forget-by-scope must delete every memory in the scope; got {deleted:?}, expected {pids:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn should_reject_unauthenticated_requests() {
    let harness = TestHarness::start().await.expect("harness");
    let mut memory = harness.memory.clone();
    let scope = harness.fresh_scope();

    let err = memory
        .remember(Request::new(RememberRequest {
            scope: Some(scope),
            content: "should never land".to_owned(),
            metadata: None,
        }))
        .await
        .expect_err("unauthenticated remember must fail");
    assert_eq!(err.code(), Code::Unauthenticated);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_reject_empty_scope_fields() {
    let harness = TestHarness::start().await.expect("harness");
    let mut memory = harness.memory.clone();

    let err = memory
        .remember(harness.authed(RememberRequest {
            scope: Some(ProtoScope {
                agent_id: String::new(),
                org_id: "org_ok".into(),
                user_id: "user_ok".into(),
            }),
            content: "rejected".to_owned(),
            metadata: None,
        }))
        .await
        .expect_err("empty scope field must be rejected");
    assert_eq!(err.code(), Code::InvalidArgument);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_handle_concurrent_remembers_in_same_scope() {
    let harness = TestHarness::start().await.expect("harness");
    let scope = harness.fresh_scope();

    let mut handles = Vec::new();
    for i in 0..5 {
        let mut memory = harness.memory.clone();
        let req = harness.authed(RememberRequest {
            scope: Some(scope.clone()),
            content: format!("concurrent fact #{i}"),
            metadata: None,
        });
        handles.push(tokio::spawn(async move { memory.remember(req).await }));
    }

    let mut pids = Vec::new();
    for handle in handles {
        let resp = handle.await.expect("join").expect("remember rpc").into_inner();
        pids.push(resp.memory.expect("memory present").pid);
    }

    let unique: std::collections::HashSet<_> = pids.iter().collect();
    assert_eq!(
        unique.len(),
        pids.len(),
        "every concurrent write must produce a unique pid; got {pids:?}"
    );
}

/// Writes three memories with conversation_id metadata 1/1/2, then searches
/// with a must_not filter on conversation_id=1 and asserts only the
/// conversation-2 row survives. Mirrors rig-service's "exclude current
/// conversation" requirement end-to-end through the gRPC surface.
#[tokio::test(flavor = "multi_thread")]
async fn should_exclude_memories_via_must_not_metadata_filter() {
    let mut harness = TestHarness::start().await.expect("harness");
    let scope = harness.fresh_scope();

    let write = |conversation_id: i64, content: &str| {
        let mut metadata_fields = std::collections::HashMap::new();
        metadata_fields.insert(
            "conversation_id".to_owned(),
            Value {
                kind: Some(Kind::NumberValue(conversation_id as f64)),
            },
        );
        let metadata = Struct {
            fields: metadata_fields,
        };
        RememberRequest {
            scope: Some(scope.clone()),
            content: content.to_owned(),
            metadata: Some(metadata),
        }
    };

    let mut memory = harness.memory.clone();
    let keep = memory
        .remember(harness.authed(write(2, "deployment runbook lives in confluence")))
        .await
        .expect("remember rpc")
        .into_inner()
        .memory
        .expect("memory present");
    let _ = memory
        .remember(harness.authed(write(1, "deployment runbook is being updated")))
        .await
        .expect("remember rpc");
    let _ = memory
        .remember(harness.authed(write(1, "deployment runbook draft in progress")))
        .await
        .expect("remember rpc");

    harness
        .wait_until_searchable(&keep.pid, scope.clone(), "deployment runbook")
        .await
        .expect("wait_until_searchable");

    let must_not_conversation_1 = MemoryFilter {
        must_not: vec![FilterCondition {
            field: "conversation_id".to_owned(),
            condition: Some(filter_condition::Condition::Equals(MatchValue {
                value: Some(match_value::Value::Integer(1)),
            })),
        }],
        ..Default::default()
    };

    let hits = harness
        .memory
        .search(harness.authed(SearchRequest {
            scope: Some(scope),
            query: "deployment runbook".to_owned(),
            limit: 50,
            metadata_filter: Some(must_not_conversation_1),
            min_similarity: None,
            kinds: None,
            with_graph_enrichment: false,
            graph_depth: 0,
        }))
        .await
        .expect("search rpc")
        .into_inner()
        .hits;

    let pids: Vec<String> = hits
        .iter()
        .filter_map(|h| h.memory.as_ref().map(|m| m.pid.clone()))
        .collect();
    assert_eq!(
        pids,
        vec![keep.pid],
        "must_not filter should leave only the conversation-2 row; got {pids:?}"
    );
}

/// Asserts min_similarity is applied — an absurdly high floor drops every hit
/// while a permissive floor preserves them. Confirms the wire field actually
/// reaches the vector backend's score_threshold knob.
#[tokio::test(flavor = "multi_thread")]
async fn should_apply_min_similarity_floor() {
    let mut harness = TestHarness::start().await.expect("harness");
    let scope = harness.fresh_scope();

    let written = harness
        .memory
        .remember(harness.authed(RememberRequest {
            scope: Some(scope.clone()),
            content: "the project uses postgres for source of truth".to_owned(),
            metadata: None,
        }))
        .await
        .expect("remember rpc")
        .into_inner()
        .memory
        .expect("memory present");

    harness
        .wait_until_searchable(&written.pid, scope.clone(), "postgres source of truth")
        .await
        .expect("wait_until_searchable");

    let strict = harness
        .memory
        .search(harness.authed(SearchRequest {
            scope: Some(scope.clone()),
            query: "postgres source of truth".to_owned(),
            limit: 50,
            metadata_filter: None,
            min_similarity: Some(0.999_999),
            kinds: None,
            with_graph_enrichment: false,
            graph_depth: 0,
        }))
        .await
        .expect("search rpc")
        .into_inner()
        .hits;
    assert!(
        strict.is_empty(),
        "min_similarity = 0.999999 should drop every hit; got {} results",
        strict.len()
    );

    let permissive = harness
        .memory
        .search(harness.authed(SearchRequest {
            scope: Some(scope),
            query: "postgres source of truth".to_owned(),
            limit: 50,
            metadata_filter: None,
            min_similarity: Some(-1.0),
            kinds: None,
            with_graph_enrichment: false,
            graph_depth: 0,
        }))
        .await
        .expect("search rpc")
        .into_inner()
        .hits;
    assert!(
        permissive
            .iter()
            .any(|h| h.memory.as_ref().is_some_and(|m| m.pid == written.pid)),
        "min_similarity = -1.0 should preserve the just-written hit"
    );
}
