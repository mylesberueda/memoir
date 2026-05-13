use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    models::{document_group_memberships, document_groups, documents},
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::rig::v1;
use sea_orm::{ColumnTrait as _, EntityTrait as _, JoinType, QueryFilter as _, QuerySelect as _, RelationTrait as _};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

const DEFAULT_TOP_N: u64 = 10;
const DEFAULT_TOP_K: u64 = 20;

#[derive(Clone)]
pub(crate) struct DocumentSearchService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
}

impl<EM> std::fmt::Debug for DocumentSearchService<EM>
where
    EM: EmbeddingModel,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocumentSearchService").finish()
    }
}

impl<EM> DocumentSearchService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(ctx: Arc<AppContext<EM>>) -> Self {
        Self { ctx }
    }

    /// Embed a query string using the shared embedding model.
    async fn embed_query(&self, query: &str) -> Result<Vec<f32>, tonic::Status> {
        self.ctx.embedding.embed(query).await.map_err(|e| {
            tracing::error!(error = %e, "failed to embed query");
            tonic::Status::internal("Failed to process search query")
        })
    }

    /// Resolve group pids to the set of document pids belonging to those groups,
    /// scoped by organization_pid for tenant isolation.
    async fn resolve_group_document_pids(
        &self,
        group_pids: &[String],
        organization_pid: &str,
    ) -> Result<Vec<String>, tonic::Status> {
        // Verify all groups exist and are accessible (same org)
        let groups = document_groups::Entity::find()
            .filter(document_groups::Column::Pid.is_in(group_pids))
            .filter(document_groups::Column::OrganizationPid.eq(organization_pid))
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch groups");
                tonic::Status::internal("Failed to resolve document groups")
            })?;

        if groups.is_empty() {
            return Ok(vec![]);
        }

        let group_ids: Vec<i64> = groups.iter().map(|g| g.id).collect();

        // Join memberships → documents to get document pids
        let doc_pids: Vec<String> = documents::Entity::find()
            .select_only()
            .column(documents::Column::Pid)
            .join(JoinType::InnerJoin, documents::Relation::DocumentGroupMemberships.def())
            .filter(document_group_memberships::Column::GroupId.is_in(group_ids))
            .filter(documents::Column::OrganizationPid.eq(organization_pid))
            .into_tuple()
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to resolve group document pids");
                tonic::Status::internal("Failed to resolve document groups")
            })?;

        Ok(doc_pids)
    }

    /// Extract a string field from a Qdrant point payload.
    fn payload_str<'a>(payload: &'a HashMap<String, qdrant_client::qdrant::Value>, key: &str) -> &'a str {
        payload
            .get(key)
            .and_then(|v| v.as_str())
            .map(String::as_str)
            .unwrap_or("")
    }

    /// Build a `v1::DocumentChunk` proto from a Qdrant scored point (from the chunks collection).
    fn scored_point_to_chunk(point: &qdrant_client::qdrant::ScoredPoint) -> v1::DocumentChunk {
        let payload = &point.payload;
        v1::DocumentChunk {
            document_pid: Self::payload_str(payload, "document_pid").to_string(),
            filename: Self::payload_str(payload, "filename").to_string(),
            chunk_index: payload.get("chunk_index").and_then(|v| v.as_integer()).unwrap_or(0) as i32,
            text: Self::payload_str(payload, "text").to_string(),
            score: point.score,
        }
    }
}

#[tonic::async_trait]
impl<EM> v1::document_search_service_server::DocumentSearchService for DocumentSearchService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok())
    )]
    async fn search_documents(
        &self,
        request: tonic::Request<v1::SearchDocumentsRequest>,
    ) -> Result<tonic::Response<v1::SearchDocumentsResponse>, tonic::Status> {
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();

        if req.query.is_empty() {
            return Err(tonic::Status::invalid_argument("query is required"));
        }

        let top_n = req.top_n.map(|n| n.max(1) as u64).unwrap_or(DEFAULT_TOP_N);
        let top_k = req.top_k.map(|k| k.max(1) as u64).unwrap_or(DEFAULT_TOP_K);

        let query_embedding = self.embed_query(&req.query).await?;

        // Step 0: Resolve group scope to document pids (if groups specified)
        let scoped_doc_pids = if !req.group_pids.is_empty() {
            let pids = self
                .resolve_group_document_pids(&req.group_pids, &organization_pid)
                .await?;
            if pids.is_empty() {
                return Ok(tonic::Response::new(v1::SearchDocumentsResponse { results: vec![] }));
            }
            Some(pids)
        } else {
            None
        };

        // Step 1: Find relevant documents via metadata collection
        let metadata_results = self
            .ctx
            .qdrant
            .search_document_metadata(
                query_embedding.clone(),
                Some(&organization_pid),
                scoped_doc_pids.as_deref(),
                top_n,
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "metadata search failed");
                tonic::Status::internal("Search failed")
            })?;

        if metadata_results.is_empty() {
            return Ok(tonic::Response::new(v1::SearchDocumentsResponse { results: vec![] }));
        }

        // Collect the document pids from Step 1
        let relevant_doc_pids: Vec<String> = metadata_results
            .iter()
            .map(|point| Self::payload_str(&point.payload, "document_pid").to_string())
            .filter(|pid| !pid.is_empty())
            .collect();

        if relevant_doc_pids.is_empty() {
            return Ok(tonic::Response::new(v1::SearchDocumentsResponse { results: vec![] }));
        }

        // Step 2: Find relevant chunks scoped to the documents from Step 1
        let chunk_results = self
            .ctx
            .qdrant
            .search_document_chunks(
                query_embedding,
                Some(&organization_pid),
                Some(&relevant_doc_pids),
                top_k,
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "chunk search failed");
                tonic::Status::internal("Search failed")
            })?;

        // Fetch document models from DB for the proto response
        let doc_models: Vec<documents::Model> = documents::Entity::find()
            .filter(documents::Column::Pid.is_in(&relevant_doc_pids))
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch documents");
                tonic::Status::internal("Search failed")
            })?;

        let doc_by_pid: HashMap<String, documents::Model> =
            doc_models.into_iter().map(|d| (d.pid.clone(), d)).collect();

        // Group chunks by document, preserving relevance order within each document
        let mut chunks_by_doc: HashMap<String, Vec<v1::DocumentChunk>> = HashMap::new();
        for point in &chunk_results {
            let chunk = Self::scored_point_to_chunk(point);
            chunks_by_doc.entry(chunk.document_pid.clone()).or_default().push(chunk);
        }

        // Build results in the order of metadata relevance (Step 1 ordering)
        let results: Vec<v1::SearchResult> = relevant_doc_pids
            .iter()
            .filter_map(|pid| {
                let doc_model = doc_by_pid.get(pid)?;
                Some(v1::SearchResult {
                    document: Some(doc_model.clone().into_proto()),
                    chunks: chunks_by_doc.remove(pid).unwrap_or_default(),
                })
            })
            .collect();

        Ok(tonic::Response::new(v1::SearchDocumentsResponse { results }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok())
    )]
    async fn search_chunks(
        &self,
        request: tonic::Request<v1::SearchChunksRequest>,
    ) -> Result<tonic::Response<v1::SearchChunksResponse>, tonic::Status> {
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();

        if req.query.is_empty() {
            return Err(tonic::Status::invalid_argument("query is required"));
        }

        let top_k = req.top_k.map(|k| k.max(1) as u64).unwrap_or(DEFAULT_TOP_K);

        let query_embedding = self.embed_query(&req.query).await?;

        let chunk_results = self
            .ctx
            .qdrant
            .search_document_chunks(query_embedding, Some(&organization_pid), None, top_k)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "chunk search failed");
                tonic::Status::internal("Search failed")
            })?;

        // The chunks collection doesn't store `filename` in the payload,
        // so we need to look up filenames from the metadata or DB.
        let doc_pids: Vec<String> = chunk_results
            .iter()
            .map(|point| Self::payload_str(&point.payload, "document_pid").to_string())
            .filter(|pid| !pid.is_empty())
            .collect();

        let filename_map: HashMap<String, String> = if !doc_pids.is_empty() {
            documents::Entity::find()
                .filter(documents::Column::Pid.is_in(&doc_pids))
                .all(&self.ctx.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to fetch document filenames");
                    tonic::Status::internal("Search failed")
                })?
                .into_iter()
                .map(|d| (d.pid.clone(), d.filename))
                .collect()
        } else {
            HashMap::new()
        };

        let chunks: Vec<v1::DocumentChunk> = chunk_results
            .iter()
            .map(|point| {
                let mut chunk = Self::scored_point_to_chunk(point);
                // Enrich with filename from DB since chunk payload doesn't have it
                if let Some(filename) = filename_map.get(&chunk.document_pid) {
                    chunk.filename = filename.clone();
                }
                chunk
            })
            .collect();

        Ok(tonic::Response::new(v1::SearchChunksResponse { chunks }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::embedding::onnx::OnnxEmbedding;
    use qdrant_client::qdrant::{ScoredPoint, Value as QdrantValue};

    type TestService = DocumentSearchService<OnnxEmbedding>;

    /// Build a ScoredPoint with the exact payload keys that ingestion writes.
    /// If scored_point_to_chunk reads different keys, the test will fail.
    fn ingestion_chunk_point(doc_pid: &str, text: &str, chunk_index: i64) -> ScoredPoint {
        let payload = HashMap::from([
            ("document_pid".to_string(), QdrantValue::from(doc_pid.to_string())),
            ("filename".to_string(), QdrantValue::from("notes.md".to_string())),
            ("text".to_string(), QdrantValue::from(text.to_string())),
            ("chunk_index".to_string(), QdrantValue::from(chunk_index)),
        ]);

        ScoredPoint {
            id: None,
            payload,
            score: 0.85,
            version: 0,
            vectors: None,
            shard_key: None,
            order_value: None,
        }
    }

    mod scored_point_to_chunk {
        use super::*;

        #[test]
        fn should_extract_document_pid_from_ingestion_payload() {
            let point = ingestion_chunk_point("doc_xyz", "hello world", 0);

            let chunk = TestService::scored_point_to_chunk(&point);

            assert_eq!(
                chunk.document_pid, "doc_xyz",
                "scored_point_to_chunk must read 'document_pid' key to match ingestion payload"
            );
        }

        #[test]
        fn should_extract_text_from_ingestion_payload() {
            let point = ingestion_chunk_point("doc_1", "important content", 2);

            let chunk = TestService::scored_point_to_chunk(&point);

            assert_eq!(chunk.text, "important content");
            assert_eq!(chunk.chunk_index, 2);
        }

        #[test]
        fn should_extract_filename_from_ingestion_payload() {
            let point = ingestion_chunk_point("doc_1", "text", 0);

            let chunk = TestService::scored_point_to_chunk(&point);

            assert_eq!(chunk.filename, "notes.md");
        }
    }
}
