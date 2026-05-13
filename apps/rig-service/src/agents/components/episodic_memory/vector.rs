use crate::api::{embedding::EmbeddingModel, memory::EpisodicMemory, store::EmbeddingStore};
use rig::vector_store::{VectorSearchRequest, VectorStoreError, VectorStoreIndex, request::Filter};
use std::sync::Arc;

/// A [`VectorStoreIndex`] implementation backed by our existing pgvector
/// embedding store. Registered on `rig::agent::Agent::dynamic_context` so
/// that memory retrieval happens automatically on every completion request
pub(crate) struct EpisodicMemoryVectorIndex<S, EM> {
    store: Arc<S>,
    embedding: Arc<EM>,
    user_id: String,
    agent_id: i64,
    exclude_conversation_id: i64,
    result_count: u32,
    min_similarity: f32,
}

impl<S, EM> EpisodicMemoryVectorIndex<S, EM> {
    pub(crate) fn new(
        store: Arc<S>,
        embedding: Arc<EM>,
        user_id: String,
        agent_id: i64,
        exclude_conversation_id: i64,
        result_count: u32,
        min_similarity: f32,
    ) -> Self {
        Self {
            store,
            embedding,
            user_id,
            agent_id,
            exclude_conversation_id,
            result_count,
            min_similarity,
        }
    }
}

/// Minimum query length to trigger memory retrieval. Short messages like
/// "hi" or "ok" are unlikely to produce meaningful semantic matches.
const MIN_QUERY_LEN: usize = 20;

/// Maximum characters of memory content to include per result, preventing
/// a single long message from dominating the context window.
const MAX_CONTENT_CHARS: usize = 500;

impl<S, EM> VectorStoreIndex for EpisodicMemoryVectorIndex<S, EM>
where
    S: EmbeddingStore,
    EM: EmbeddingModel,
{
    type Filter = Filter<serde_json::Value>;

    async fn top_n<T: for<'a> serde::Deserialize<'a> + Send>(
        &self,
        req: VectorSearchRequest<Self::Filter>,
    ) -> Result<Vec<(f64, String, T)>, VectorStoreError> {
        let query = req.query();

        if query.len() < MIN_QUERY_LEN {
            tracing::debug!(query_len = query.len(), "skipping memory retrieval: query too short");
            return Ok(vec![]);
        }

        // Memory retrieval is best-effort: failures should never prevent the
        // agent from responding. Log warnings and return empty results.
        let query_vec = match self.embedding.embed(query).await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(error = %e, "memory retrieval skipped: embedding failed");
                return Ok(vec![]);
            }
        };

        let limit = (req.samples() as u32).min(self.result_count);

        let memories: Vec<EpisodicMemory> = match self
            .store
            .retrieve_similar(
                query_vec,
                &self.user_id,
                self.agent_id,
                self.exclude_conversation_id,
                limit,
                self.min_similarity,
            )
            .await
        {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!(error = %e, "memory retrieval skipped: store query failed");
                return Ok(vec![]);
            }
        };

        tracing::debug!(count = memories.len(), "memory dynamic_context retrieved");

        let results: Vec<(f64, String, T)> = memories
            .into_iter()
            .filter_map(|m| {
                let content = if m.content.len() > MAX_CONTENT_CHARS {
                    format!("{}…", &m.content[..MAX_CONTENT_CHARS])
                } else {
                    m.content.clone()
                };

                // Rig pretty-prints this JSON as document content in the
                // completion request, so the model sees structured memory.
                let doc = serde_json::json!({
                    "role": m.role,
                    "content": content,
                    "timestamp": m.created_at.format("%Y-%m-%d %H:%M").to_string(),
                });

                let score = m.similarity as f64;
                let id = m.pid.clone();

                match serde_json::from_value::<T>(doc) {
                    Ok(parsed) => Some((score, id, parsed)),
                    Err(e) => {
                        tracing::warn!(pid = %m.pid, error = %e, "failed to deserialize memory document");
                        None
                    }
                }
            })
            .collect();

        Ok(results)
    }

    async fn top_n_ids(&self, req: VectorSearchRequest<Self::Filter>) -> Result<Vec<(f64, String)>, VectorStoreError> {
        // Reuse top_n, discarding the document body
        let results: Vec<(f64, String, serde_json::Value)> = self.top_n(req).await?;
        Ok(results.into_iter().map(|(score, id, _)| (score, id)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        embedding::{BoxFuture, EmbeddingError},
        store::StoreError,
    };

    struct StubEmbedding;

    impl EmbeddingModel for StubEmbedding {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.1, 0.2, 0.3]) })
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    struct StubStore {
        memories: Vec<EpisodicMemory>,
    }

    impl EmbeddingStore for StubStore {
        async fn update_embedding(&self, _pid: &str, _embedding: Vec<f32>) -> Result<(), StoreError> {
            Ok(())
        }

        async fn retrieve_similar(
            &self,
            _query_embedding: Vec<f32>,
            _user_id: &str,
            _agent_id: i64,
            _exclude_conversation_id: i64,
            limit: u32,
            _min_similarity: f32,
        ) -> Result<Vec<EpisodicMemory>, StoreError> {
            let results: Vec<EpisodicMemory> = self.memories.iter().take(limit as usize).cloned().collect();
            Ok(results)
        }
    }

    fn make_memory(pid: &str, role: &str, content: &str, similarity: f32) -> EpisodicMemory {
        EpisodicMemory {
            pid: pid.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            similarity,
            created_at: chrono::NaiveDateTime::parse_from_str("2024-06-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        }
    }

    fn make_index(memories: Vec<EpisodicMemory>) -> EpisodicMemoryVectorIndex<StubStore, StubEmbedding> {
        EpisodicMemoryVectorIndex::new(
            Arc::new(StubStore { memories }),
            Arc::new(StubEmbedding),
            "user_123".into(),
            1,
            100,
            10,
            0.7,
        )
    }

    fn search_req(query: &str, samples: u64) -> VectorSearchRequest<Filter<serde_json::Value>> {
        VectorSearchRequest::builder()
            .query(query)
            .samples(samples)
            .build()
            .expect("valid search request")
    }

    #[tokio::test]
    async fn should_return_empty_when_query_too_short() {
        let index = make_index(vec![make_memory("m1", "user", "hello", 0.9)]);
        let req = search_req("hi", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(results.is_empty(), "short queries should return no memories");
    }

    #[tokio::test]
    async fn should_return_memories_for_long_enough_query() {
        let index = make_index(vec![
            make_memory("m1", "user", "What is Rust programming?", 0.92),
            make_memory("m2", "assistant", "Rust is a systems language.", 0.88),
        ]);
        let req = search_req("Tell me about Rust programming language", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, "m1");
        assert_eq!(results[1].1, "m2");
    }

    #[tokio::test]
    async fn should_include_role_content_and_timestamp_in_document() {
        let index = make_index(vec![make_memory("m1", "user", "I like Rust a lot", 0.9)]);
        let req = search_req("What programming languages do I like?", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        let doc = &results[0].2;
        assert_eq!(doc["role"], "user");
        assert_eq!(doc["content"], "I like Rust a lot");
        assert_eq!(doc["timestamp"], "2024-06-15 10:30");
    }

    #[tokio::test]
    async fn should_use_similarity_as_score() {
        let index = make_index(vec![make_memory("m1", "user", "Rust is great for systems", 0.95)]);
        let req = search_req("What language is good for systems?", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(
            (results[0].0 - 0.95).abs() < 1e-6,
            "score should be ~0.95, got {}",
            results[0].0
        );
    }

    #[tokio::test]
    async fn should_limit_results_to_min_of_samples_and_result_count() {
        let memories = (0..20)
            .map(|i| {
                make_memory(
                    &format!("m{i}"),
                    "user",
                    &format!("Memory content number {i} is here"),
                    0.9,
                )
            })
            .collect();
        let index = make_index(memories); // result_count = 10

        // Request only 3 samples — should cap at 3
        let req = search_req("Tell me about all the memory content", 3);
        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn should_truncate_long_content() {
        let long_content = "x".repeat(600);
        let index = make_index(vec![make_memory("m1", "user", &long_content, 0.9)]);
        let req = search_req("Tell me about the really long message", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        let content = results[0].2["content"].as_str().unwrap();
        assert!(content.len() <= MAX_CONTENT_CHARS + 3, "content should be truncated"); // +3 for "…" (3 bytes)
    }

    #[tokio::test]
    async fn should_not_truncate_content_at_exactly_max_chars() {
        let exact_content = "y".repeat(MAX_CONTENT_CHARS);
        let index = make_index(vec![make_memory("m1", "user", &exact_content, 0.9)]);
        let req = search_req("Tell me about the exact length message", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        let content = results[0].2["content"].as_str().unwrap();
        assert_eq!(
            content, exact_content,
            "content at exactly MAX_CONTENT_CHARS should not be truncated"
        );
    }

    #[tokio::test]
    async fn should_return_empty_when_store_returns_no_matches() {
        let index = make_index(vec![]); // empty store — no memories exist
        let req = search_req("Tell me about something interesting", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(results.is_empty(), "should return empty when store has no matches");
    }

    // --- Error propagation tests require custom failing stubs ---

    struct FailingEmbedding;

    impl EmbeddingModel for FailingEmbedding {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Err(EmbeddingError::Embed("embedding service unavailable".into())) })
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    struct FailingStore;

    impl EmbeddingStore for FailingStore {
        async fn update_embedding(&self, _pid: &str, _embedding: Vec<f32>) -> Result<(), StoreError> {
            Ok(())
        }

        async fn retrieve_similar(
            &self,
            _query_embedding: Vec<f32>,
            _user_id: &str,
            _agent_id: i64,
            _exclude_conversation_id: i64,
            _limit: u32,
            _min_similarity: f32,
        ) -> Result<Vec<EpisodicMemory>, StoreError> {
            Err(StoreError::Internal("database connection lost".into()))
        }
    }

    #[tokio::test]
    async fn should_return_empty_when_embedding_fails() {
        let index = EpisodicMemoryVectorIndex::new(
            Arc::new(StubStore { memories: vec![] }),
            Arc::new(FailingEmbedding),
            "user_123".into(),
            1,
            100,
            10,
            0.7,
        );
        let req = search_req("This query is long enough to trigger embedding", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(results.is_empty(), "should return empty when embedding fails");
    }

    #[tokio::test]
    async fn should_return_empty_when_store_query_fails() {
        let index = EpisodicMemoryVectorIndex::new(
            Arc::new(FailingStore),
            Arc::new(StubEmbedding),
            "user_123".into(),
            1,
            100,
            10,
            0.7,
        );
        let req = search_req("This query is long enough to trigger retrieval", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(results.is_empty(), "should return empty when store query fails");
    }
}
