mod extract;

use crate::{
    AppContext,
    api::embedding::EmbeddingModel,
    context::{DOCUMENT_CHUNKS_COLLECTION, DOCUMENT_METADATA_COLLECTION},
    models::documents::{self, DocStatus},
};
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Value as QdrantValue};
use sea_orm::IntoActiveModel as _;
use std::{collections::HashMap, sync::Arc};
use tracing::Instrument;
use uuid::Uuid;

/// Generate a random UUID for use as a Qdrant point ID.
/// Qdrant requires point IDs to be UUIDs or u64 integers.
fn point_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum IngestionError {
    #[error("storage: {0}")]
    Storage(#[from] crate::clients::storage::StorageError),
    #[error("extraction: {0}")]
    Extraction(String),
    #[error("embedding: {0}")]
    Embedding(String),
    #[error("qdrant: {0}")]
    Qdrant(#[from] crate::clients::qdrant::QdrantError),
    #[error("db: {0}")]
    Database(#[from] sea_orm::DbErr),
}

pub(crate) struct IngestionPipeline;

impl IngestionPipeline {
    /// Spawn an async ingestion task for a document.
    /// Updates status to `ready` on success, `failed` on error.
    /// When `conversation_pid` is provided, the Qdrant points are tagged with
    /// it so the document is immediately searchable from that conversation.
    pub(crate) fn spawn<EM>(
        ctx: Arc<AppContext<EM>>,
        doc: documents::ModelEx,
        bytes: &[u8],
        conversation_pid: Option<String>,
    ) where
        EM: EmbeddingModel,
    {
        let span = tracing::info_span!(
            "ingestion",
            document_pid = %doc.pid,
        );

        let bytes = bytes.to_vec();

        tokio::spawn(
            async move {
                let doc_id = doc.id;
                let doc_pid = doc.pid.clone();

                if let Err(e) = Self::run(&ctx, &doc, &bytes, conversation_pid.as_deref()).await {
                    tracing::error!(error = %e, "ingestion failed");
                    documents::Entity::set_status(&ctx.db, doc_id, DocStatus::Failed, Some(e.to_string())).await;

                    if let Err(cleanup_err) = ctx.qdrant.delete_document_points(&doc_pid).await {
                        tracing::warn!(error = %cleanup_err, "failed to cleanup qdrant on ingestion failure");
                    }
                }
            }
            .instrument(span),
        );
    }

    async fn run<EM>(
        ctx: &AppContext<EM>,
        doc: &documents::ModelEx,
        bytes: &[u8],
        conversation_pid: Option<&str>,
    ) -> Result<(), IngestionError>
    where
        EM: EmbeddingModel,
    {
        tracing::info!(document_pid = doc.pid, "extracting text and chunking");
        let result = extract::extract_and_chunk(bytes, &doc.content_type)
            .await
            .map_err(|e| IngestionError::Extraction(e.to_string()))?;

        let full_text = &result.content;
        if full_text.is_empty() {
            return Err(IngestionError::Extraction("extracted text is empty".into()));
        }

        // TODO(_): replace with LLM-generated summary
        let summary = match full_text.char_indices().nth(500) {
            Some((byte_pos, _)) => format!("{}...", &full_text[..byte_pos]),
            None => full_text.clone(),
        };

        doc.clone()
            .into_active_model()
            .set_summary(Some(summary.clone()))
            .update(&ctx.db)
            .await
            .map_err(IngestionError::Database)?;

        let pid: QdrantValue = doc.pid.clone().into();
        let user_id: QdrantValue = doc.user_id.clone().into();
        let filename: QdrantValue = doc.filename.clone().into();

        tracing::info!(document_pid = doc.pid, "generating metadata embedding");
        let metadata_text = format!("{}: {}", doc.filename, summary);
        let metadata_embedding = ctx
            .embedding
            .embed(&metadata_text)
            .await
            .map_err(|e| IngestionError::Embedding(e.to_string()))?;

        let conversation_pids: QdrantValue = match conversation_pid {
            Some(conversation_pid) => vec![conversation_pid.to_string()].into(),
            None => Vec::<String>::new().into(),
        };

        let mut metadata_payload: HashMap<&str, QdrantValue> = HashMap::from([
            ("document_pid", pid.clone()),
            ("user_id", user_id.clone()),
            ("filename", filename.clone()),
            ("summary", summary.into()),
            ("conversation_pids", conversation_pids.clone()),
        ]);

        metadata_payload.insert("organization_pid", doc.organization_pid.clone().into());

        let metadata_point = PointStruct::new(point_id(), metadata_embedding, metadata_payload);

        ctx.qdrant
            .upsert_points(UpsertPointsBuilder::new(
                DOCUMENT_METADATA_COLLECTION,
                vec![metadata_point],
            ))
            .await?;

        let chunks: Vec<(usize, String)> = match result.chunks {
            Some(chunks) => chunks
                .into_iter()
                .enumerate()
                .map(|(i, chunk)| (i, chunk.content))
                .collect(),
            None => vec![(0, full_text.clone())],
        };

        if !chunks.is_empty() {
            tracing::info!(
                document_pid = doc.pid,
                chunk_count = chunks.len(),
                "generating chunk embeddings"
            );

            let chunk_texts: Vec<&str> = chunks.iter().map(|(_, text)| text.as_str()).collect();
            let chunk_embeddings = ctx
                .embedding
                .embed_batch(&chunk_texts)
                .await
                .map_err(|e| IngestionError::Embedding(e.to_string()))?;

            let chunk_points: Vec<PointStruct> = chunks
                .iter()
                .zip(chunk_embeddings)
                .map(|((chunk_index, text), embedding)| {
                    let mut payload: HashMap<&str, QdrantValue> = HashMap::from([
                        ("document_pid", pid.clone()),
                        ("user_id", user_id.clone()),
                        ("filename", filename.clone()),
                        ("chunk_index", (*chunk_index as i64).into()),
                        ("text", text.clone().into()),
                        ("conversation_pids", conversation_pids.clone()),
                    ]);

                    payload.insert("organization_pid", doc.organization_pid.clone().into());

                    PointStruct::new(point_id(), embedding, payload)
                })
                .collect();

            ctx.qdrant
                .upsert_points(UpsertPointsBuilder::new(DOCUMENT_CHUNKS_COLLECTION, chunk_points))
                .await?;
        }

        documents::Entity::set_status(&ctx.db, doc.id, DocStatus::Ready, None).await;
        tracing::info!(document_pid = doc.pid, "ingestion complete");
        Ok(())
    }
}
