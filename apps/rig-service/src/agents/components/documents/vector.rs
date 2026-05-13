use crate::{api::embedding::EmbeddingModel, clients::QdrantClient};
use rig::vector_store::{VectorSearchRequest, VectorStoreError, VectorStoreIndex, request::Filter};
use std::sync::Arc;

/// A [`VectorStoreIndex`] that searches document chunks scoped to a single
/// conversation. Registered on `rig::agent::Agent::dynamic_context` so that
/// document context is automatically injected on every completion request.
///
/// Mirrors `MemoryVectorIndex` — best-effort retrieval that never blocks
/// the agent from responding.
pub(crate) struct DocumentVectorIndex<EM> {
    qdrant: QdrantClient,
    embedding: Arc<EM>,
    conversation_pid: String,
    result_count: usize,
}

impl<EM> DocumentVectorIndex<EM> {
    pub(crate) fn new(qdrant: QdrantClient, embedding: Arc<EM>, conversation_pid: String, result_count: usize) -> Self {
        Self {
            qdrant,
            embedding,
            conversation_pid,
            result_count,
        }
    }
}

/// Minimum query length to trigger document retrieval. Short messages like
/// "hi" or "ok" are unlikely to produce meaningful semantic matches.
const MIN_QUERY_LEN: usize = 20;

/// Maximum characters of chunk text to include per result, preventing
/// a single large chunk from dominating the context window.
const MAX_TEXT_CHARS: usize = 2000;

/// Parse a Qdrant `ScoredPoint` from the chunks collection into a
/// `(score, id, doc_json)` tuple. Returns `None` if required payload
/// fields are missing. Extracted as a free function so unit tests can
/// verify payload key consistency without hitting Qdrant.
fn parse_chunk_point<T: for<'a> serde::Deserialize<'a>>(
    point: &qdrant_client::qdrant::ScoredPoint,
) -> Option<(f64, String, T)> {
    let payload = &point.payload;
    let filename = payload.get("filename")?.as_str()?.to_string();
    let text = payload.get("text")?.as_str()?.to_string();
    let chunk_index = payload.get("chunk_index").and_then(|v| v.as_integer()).unwrap_or(0);

    let truncated_text = if text.len() > MAX_TEXT_CHARS {
        format!("{}…", &text[..MAX_TEXT_CHARS])
    } else {
        text
    };

    let doc = serde_json::json!({
        "filename": filename,
        "text": truncated_text,
        "chunk_index": chunk_index,
    });

    let score = point.score as f64;
    let id = point.id.as_ref().map(|pid| format!("{pid:?}")).unwrap_or_default();

    match serde_json::from_value::<T>(doc) {
        Ok(parsed) => Some((score, id, parsed)),
        Err(e) => {
            tracing::warn!(error = %e, "failed to deserialize document chunk");
            None
        }
    }
}

impl<EM> VectorStoreIndex for DocumentVectorIndex<EM>
where
    EM: EmbeddingModel,
{
    type Filter = Filter<serde_json::Value>;

    async fn top_n<T: for<'a> serde::Deserialize<'a> + Send>(
        &self,
        req: VectorSearchRequest<Self::Filter>,
    ) -> Result<Vec<(f64, String, T)>, VectorStoreError> {
        let query = req.query();

        if query.len() < MIN_QUERY_LEN {
            tracing::debug!(query_len = query.len(), "skipping document retrieval: query too short");
            return Ok(vec![]);
        }

        let query_vec = match self.embedding.embed(query).await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(error = %e, "document retrieval skipped: embedding failed");
                return Ok(vec![]);
            }
        };

        let limit = req.samples().min(self.result_count as u64);

        let scored_points = match self
            .qdrant
            .search_chunks_by_conversation(query_vec, &self.conversation_pid, limit)
            .await
        {
            Ok(pts) => pts,
            Err(e) => {
                tracing::warn!(error = %e, "document retrieval skipped: qdrant search failed");
                return Ok(vec![]);
            }
        };

        tracing::debug!(count = scored_points.len(), "document dynamic_context retrieved");

        let results: Vec<(f64, String, T)> = scored_points.iter().filter_map(parse_chunk_point).collect();

        Ok(results)
    }

    async fn top_n_ids(&self, req: VectorSearchRequest<Self::Filter>) -> Result<Vec<(f64, String)>, VectorStoreError> {
        let results: Vec<(f64, String, serde_json::Value)> = self.top_n(req).await?;
        Ok(results.into_iter().map(|(score, id, _)| (score, id)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::embedding::{BoxFuture, EmbeddingError};

    struct StubEmbedding;

    impl EmbeddingModel for StubEmbedding {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.1, 0.2, 0.3]) })
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    struct FailingEmbedding;

    impl EmbeddingModel for FailingEmbedding {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Err(EmbeddingError::Embed("embedding service unavailable".into())) })
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    fn make_index(embedding: impl EmbeddingModel, result_count: usize) -> DocumentVectorIndex<impl EmbeddingModel> {
        // Uses a real QdrantClient pointing at a non-existent server — that's fine
        // because tests that don't hit Qdrant will short-circuit before the network call.
        // Tests that need Qdrant results belong in integration tests.
        let qdrant = qdrant_client::Qdrant::from_url("http://localhost:6334")
            .build()
            .expect("qdrant client");

        DocumentVectorIndex::new(
            QdrantClient::new(qdrant),
            Arc::new(embedding),
            "conv_test_123".into(),
            result_count,
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
        let index = make_index(StubEmbedding, 5);
        let req = search_req("hi", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(results.is_empty(), "short queries should return no documents");
    }

    #[tokio::test]
    async fn should_return_empty_when_embedding_fails() {
        let index = make_index(FailingEmbedding, 5);
        let req = search_req("This query is long enough to trigger embedding", 10);

        let results: Vec<(f64, String, serde_json::Value)> = index.top_n(req).await.unwrap();

        assert!(results.is_empty(), "should return empty when embedding fails");
    }

    /// Tests that verify parse_chunk_point reads the exact payload keys that
    /// ingestion writes. These will fail if the key names drift apart.
    mod parse_chunk_point_tests {
        use super::super::*;
        use qdrant_client::qdrant::{ScoredPoint, Value as QdrantValue};
        use std::collections::HashMap;

        /// Build a ScoredPoint with the exact payload keys ingestion writes.
        fn ingestion_chunk_point(filename: &str, text: &str, chunk_index: i64) -> ScoredPoint {
            let payload = HashMap::from([
                ("document_pid".to_string(), QdrantValue::from("doc_abc".to_string())),
                ("user_id".to_string(), QdrantValue::from("user_1".to_string())),
                ("filename".to_string(), QdrantValue::from(filename.to_string())),
                ("text".to_string(), QdrantValue::from(text.to_string())),
                ("chunk_index".to_string(), QdrantValue::from(chunk_index)),
                ("conversation_pids".to_string(), vec!["conv_1".to_string()].into()),
            ]);

            ScoredPoint {
                id: None,
                payload,
                score: 0.92,
                version: 0,
                vectors: None,
                shard_key: None,
                order_value: None,
            }
        }

        #[test]
        fn should_extract_filename_from_ingestion_payload() {
            let point = ingestion_chunk_point("report.pdf", "some content", 0);

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);

            let (_, _, doc) = result.expect("should parse ingestion payload successfully");
            assert_eq!(doc["filename"], "report.pdf");
        }

        #[test]
        fn should_extract_text_from_ingestion_payload() {
            let point = ingestion_chunk_point("notes.md", "important text here", 3);

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);

            let (_, _, doc) = result.expect("should parse ingestion payload");
            assert_eq!(doc["text"], "important text here");
            assert_eq!(doc["chunk_index"], 3);
        }

        #[test]
        fn should_return_none_when_filename_missing() {
            let payload = HashMap::from([
                ("text".to_string(), QdrantValue::from("content".to_string())),
                ("chunk_index".to_string(), QdrantValue::from(0_i64)),
            ]);

            let point = ScoredPoint {
                id: None,
                payload,
                score: 0.9,
                version: 0,
                vectors: None,
                shard_key: None,
                order_value: None,
            };

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);
            assert!(result.is_none(), "should return None when filename is missing");
        }

        #[test]
        fn should_return_none_when_text_missing() {
            let payload = HashMap::from([
                ("filename".to_string(), QdrantValue::from("file.pdf".to_string())),
                ("chunk_index".to_string(), QdrantValue::from(0_i64)),
            ]);

            let point = ScoredPoint {
                id: None,
                payload,
                score: 0.9,
                version: 0,
                vectors: None,
                shard_key: None,
                order_value: None,
            };

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);
            assert!(result.is_none(), "should return None when text is missing");
        }

        #[test]
        fn should_truncate_text_exceeding_max_chars() {
            let long_text = "x".repeat(MAX_TEXT_CHARS + 500);
            let point = ingestion_chunk_point("big.pdf", &long_text, 0);

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);

            let (_, _, doc) = result.expect("should parse");
            let text = doc["text"].as_str().unwrap();
            assert!(
                text.len() <= MAX_TEXT_CHARS + 3, // +3 for "…" (3 bytes)
                "text should be truncated to MAX_TEXT_CHARS, got {} chars",
                text.len()
            );
        }

        #[test]
        fn should_not_truncate_text_at_exactly_max_chars() {
            let exact_text = "y".repeat(MAX_TEXT_CHARS);
            let point = ingestion_chunk_point("exact.pdf", &exact_text, 0);

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);

            let (_, _, doc) = result.expect("should parse");
            assert_eq!(
                doc["text"].as_str().unwrap(),
                exact_text,
                "text at exactly MAX_TEXT_CHARS should not be truncated"
            );
        }

        #[test]
        fn should_use_score_from_scored_point() {
            let point = ingestion_chunk_point("file.pdf", "content", 0);

            let result: Option<(f64, String, serde_json::Value)> = parse_chunk_point(&point);

            let (score, _, _) = result.expect("should parse");
            assert!((score - 0.92).abs() < 1e-6, "score should be ~0.92, got {score}");
        }
    }
}
