//! [`VectorIndex`] implementation backed by Qdrant.
//!
//! Qdrant only accepts `u64` or UUID values for point IDs (`PointId`), but
//! memoir-core pids are nanoid strings — incompatible. Each upsert generates
//! a fresh UUIDv4 as the point ID and stores the memoir pid in the point's
//! payload under the `pid` key. Search, scroll, and delete paths all
//! resolve the memoir pid via the payload; the UUID point ID is an
//! implementation detail nobody outside this module sees.

use std::collections::HashMap;

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, DeletePointsBuilder, Distance, Filter, PointStruct, QueryPointsBuilder,
    ScrollPointsBuilder, UpsertPointsBuilder, Value, VectorParamsBuilder,
};
use uuid::Uuid;

use super::{MemoryFilter, VectorError, VectorIndex};
use crate::memory::{KindSelector, Memory, Scope};

const DEFAULT_COLLECTION: &str = "memoir_memories";

/// Payload key under which each point stores its memoir pid.
const PID_PAYLOAD_KEY: &str = "pid";

/// Payload key for wall-clock write time, encoded as i64 epoch milliseconds.
///
/// Filterable via [`super::FilterCondition::Range`] in millisecond units.
/// Matches the encoding used elsewhere in the polypixel template (verified
/// against rig-service's `models/messages.rs:139` use of `timestamp_millis`).
const CREATED_AT_PAYLOAD_KEY: &str = "created_at";

/// Payload key for event time, encoded as i64 epoch milliseconds.
///
/// Omitted entirely (not written as null) when the memory has no event-time
/// known. Range filters against this key implicitly exclude memories whose
/// event-time is unknown — the desired semantics for "find memories from
/// last week" (memories without event-time can't satisfy the constraint).
const EVENT_AT_PAYLOAD_KEY: &str = "event_at";

/// Payload keys owned by memoir-core; consumer metadata cannot use these.
///
/// The memory's `metadata` JSON is flattened to top-level payload keys so
/// caller-supplied [`super::FilterCondition`] entries can match against
/// metadata fields directly (e.g. `field: "role"` matches `metadata.role`).
/// Reserved keys are protected from clobbering by validation at write time
/// — see [`crate::store::MemoryStore::remember`] / the remember client
/// path — so callers can't smuggle a `pid` or scope value in via metadata.
pub(crate) const RESERVED_PAYLOAD_KEYS: &[&str] = &[
    PID_PAYLOAD_KEY,
    "agent_id",
    "org_id",
    "user_id",
    "kind",
    CREATED_AT_PAYLOAD_KEY,
    EVENT_AT_PAYLOAD_KEY,
];

/// Default [`VectorIndex`] backed by Qdrant.
///
/// Constructed via [`Self::new`]. Collection name defaults to
/// `memoir_memories`; override with [`Self::with_collection`].
#[derive(Clone)]
pub struct QdrantIndex {
    qdrant: Qdrant,
    collection: String,
}

impl std::fmt::Debug for QdrantIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QdrantIndex")
            .field("collection", &self.collection)
            .finish_non_exhaustive()
    }
}

impl QdrantIndex {
    /// Builds an index from an existing Qdrant client.
    pub fn new(qdrant: Qdrant) -> Self {
        Self {
            qdrant,
            collection: DEFAULT_COLLECTION.to_string(),
        }
    }

    /// Sets the Qdrant collection name used for vector storage.
    pub fn with_collection(mut self, collection: impl Into<String>) -> Self {
        self.collection = collection.into();
        self
    }

    /// Returns the Qdrant collection name configured for this index.
    pub fn collection_name(&self) -> &str {
        &self.collection
    }
}

impl VectorIndex for QdrantIndex {
    async fn ensure_collection(&self, vector_dim: usize) -> Result<(), VectorError> {
        let exists = self
            .qdrant
            .collection_exists(&self.collection)
            .await
            .map_err(connection)?;
        if exists {
            return Ok(());
        }

        self.qdrant
            .create_collection(
                CreateCollectionBuilder::new(&self.collection)
                    .vectors_config(VectorParamsBuilder::new(vector_dim as u64, Distance::Cosine)),
            )
            .await
            .map_err(connection)?;
        Ok(())
    }

    async fn upsert(&self, memory: &Memory, vector: Vec<f32>) -> Result<(), VectorError> {
        // First delete any prior points carrying this pid in their payload,
        // since the Qdrant point ID is a fresh UUID per upsert and won't
        // collide with a previous write's ID.
        self.delete_by_pids(&[&memory.pid]).await?;

        // First-class payload keys. Owned by memoir-core and protected
        // against consumer-metadata clobbering by `RESERVED_PAYLOAD_KEYS`.
        // Timestamps are i64 epoch milliseconds, matching the polypixel
        // template convention (rig-service `models/messages.rs:139`).
        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert(PID_PAYLOAD_KEY.to_string(), Value::from(memory.pid.clone()));
        payload.insert("agent_id".to_string(), Value::from(memory.scope.agent_id.clone()));
        payload.insert("org_id".to_string(), Value::from(memory.scope.org_id.clone()));
        payload.insert("user_id".to_string(), Value::from(memory.scope.user_id.clone()));
        payload.insert("kind".to_string(), Value::from(memory.kind.to_string()));
        payload.insert(
            CREATED_AT_PAYLOAD_KEY.to_string(),
            Value::from(memory.created_at.timestamp_millis()),
        );
        if let Some(event_at) = memory.event_at {
            // Omit (not write null): Qdrant range filters treat missing
            // payload keys as "fail to match", which is the right semantic
            // for "memories with known event-time in this window."
            payload.insert(
                EVENT_AT_PAYLOAD_KEY.to_string(),
                Value::from(event_at.timestamp_millis()),
            );
        }

        // Flatten metadata's top-level object into the payload alongside
        // the first-class keys. Reserved-key collisions are prevented by
        // validation at the write boundary (Client::remember /
        // RememberBuilder); reaching this code with a colliding key would
        // mean a bug upstream, so we drop the colliding entries
        // defensively rather than panicking. The `From<serde_json::Value>`
        // impl on qdrant_client `Value` handles every JSON variant.
        if let Some(obj) = memory.metadata.as_object() {
            for (k, v) in obj {
                if RESERVED_PAYLOAD_KEYS.iter().any(|reserved| reserved == k) {
                    continue;
                }
                payload.insert(k.clone(), Value::from(v.clone()));
            }
        }

        let point = PointStruct::new(Uuid::new_v4().to_string(), vector, payload);

        self.qdrant
            .upsert_points(UpsertPointsBuilder::new(&self.collection, vec![point]))
            .await
            .map_err(connection)?;

        Ok(())
    }

    async fn search(
        &self,
        scope: Scope,
        query_embedding: Vec<f32>,
        limit: usize,
        kinds: KindSelector,
        extra_filter: Option<MemoryFilter>,
        min_similarity: Option<f32>,
    ) -> Result<Vec<(String, f32)>, VectorError> {
        if kinds.is_empty() {
            return Ok(Vec::new());
        }

        // Scope conditions go in `must` first so an `extra_filter.must` cannot
        // accidentally widen scope: a caller-supplied `must` adds to AND, not
        // replaces. A caller-supplied `must_not` on `agent_id` (or any scope
        // field) would only narrow further, not widen — Qdrant evaluates
        // `must AND NOT must_not`.
        let mut must = vec![
            Condition::matches("agent_id", scope.agent_id),
            Condition::matches("org_id", scope.org_id),
            Condition::matches("user_id", scope.user_id),
        ];
        if !kinds.includes_all() {
            let names: Vec<String> = kinds.included_kinds().into_iter().map(|k| k.to_string()).collect();
            must.push(Condition::matches("kind", names));
        }

        let mut must_not = Vec::new();
        let mut should = Vec::new();
        if let Some(extra) = extra_filter {
            let translated: Filter = extra.into();
            must.extend(translated.must);
            must_not.extend(translated.must_not);
            should.extend(translated.should);
        }

        let filter = Filter {
            must,
            must_not,
            should,
            min_should: None,
        };

        let mut request = QueryPointsBuilder::new(&self.collection)
            .query(query_embedding)
            .limit(limit as u64)
            .filter(filter)
            .with_payload(true);
        if let Some(threshold) = min_similarity {
            request = request.score_threshold(threshold);
        }

        let response = self.qdrant.query(request).await.map_err(connection)?;

        let mut hits = Vec::with_capacity(response.result.len());
        for scored in response.result {
            if let Some(pid) = pid_from_payload(&scored.payload) {
                hits.push((pid, scored.score));
            }
        }
        Ok(hits)
    }

    async fn delete_by_pids(&self, pids: &[&str]) -> Result<(), VectorError> {
        if pids.is_empty() {
            return Ok(());
        }

        // Pids live in payload, not in the point ID, so delete by payload
        // filter. Each pid translates to a `match` condition; the wrapper
        // `Filter::should` (logical OR) covers a batch of pids in one call.
        let conditions: Vec<Condition> = pids
            .iter()
            .map(|p| Condition::matches(PID_PAYLOAD_KEY, (*p).to_string()))
            .collect();
        let filter = Filter::should(conditions);

        self.qdrant
            .delete_points(DeletePointsBuilder::new(&self.collection).points(filter))
            .await
            .map_err(connection)?;
        Ok(())
    }

    async fn list_pids_in_scope(&self, scope: Scope, page_size: usize) -> Result<Vec<String>, VectorError> {
        let filter = Filter::must(vec![
            Condition::matches("agent_id", scope.agent_id),
            Condition::matches("org_id", scope.org_id),
            Condition::matches("user_id", scope.user_id),
        ]);

        let mut pids = Vec::new();
        let mut offset: Option<qdrant_client::qdrant::PointId> = None;

        loop {
            let mut request = ScrollPointsBuilder::new(&self.collection)
                .filter(filter.clone())
                .limit(page_size as u32)
                .with_payload(true)
                .with_vectors(false);
            if let Some(o) = offset.take() {
                request = request.offset(o);
            }

            let response = self.qdrant.scroll(request).await.map_err(connection)?;

            for point in response.result {
                if let Some(pid) = pid_from_payload(&point.payload) {
                    pids.push(pid);
                }
            }

            match response.next_page_offset {
                Some(next) => offset = Some(next),
                None => break,
            }
        }

        Ok(pids)
    }
}

fn connection<E: std::fmt::Display>(err: E) -> VectorError {
    VectorError::Connection(err.to_string())
}

/// Extracts the memoir pid from a Qdrant point's payload, if present.
///
/// Returns `None` when the payload lacks a `pid` key or carries a non-string
/// value — both should be impossible for points written via [`QdrantIndex::upsert`],
/// but defending against malformed remote state keeps the search side robust.
fn pid_from_payload(payload: &HashMap<String, Value>) -> Option<String> {
    payload
        .get(PID_PAYLOAD_KEY)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_pid_from_payload_when_present() {
        let payload = HashMap::from([(PID_PAYLOAD_KEY.to_string(), Value::from("my-pid".to_string()))]);
        assert_eq!(pid_from_payload(&payload), Some("my-pid".to_string()));
    }

    #[test]
    fn should_return_none_when_pid_absent_from_payload() {
        let payload = HashMap::from([("other".to_string(), Value::from("x".to_string()))]);
        assert_eq!(pid_from_payload(&payload), None);
    }

    #[test]
    fn should_return_none_when_pid_value_is_not_a_string() {
        let payload = HashMap::from([(PID_PAYLOAD_KEY.to_string(), Value::from(42i64))]);
        assert_eq!(pid_from_payload(&payload), None);
    }
}
