//! Document context component — conversation-scoped document retrieval via Qdrant.
//!
//! Registers a vector index on `BaseAgent`'s dynamic context at init time.
//! No per-message hooks — Rig automatically queries the index on each completion.

mod vector;

use crate::{agents::rig::DynamicContextStore, api::embedding::EmbeddingModel, clients::QdrantClient};
use std::{future::Future, pin::Pin, sync::Arc};
use vector::DocumentVectorIndex;

use super::AgentComponent;

/// The system prompt section describing how document context appears to the model.
const SYSTEM_PROMPT_SECTION: &str = "\
## Documents\n\
The user may attach files (documents) to this conversation. Document content is \
provided in two ways:\n\
1. **Inline attachments**: When a user sends a message with files, the file contents \
appear directly in the message as document parts. Use these for the immediate question.\n\
2. **Background context**: All documents attached to this conversation are automatically \
searched on each message. Relevant chunks appear as context documents with \"filename\", \
\"text\", and \"chunk_index\" fields. Use these to answer follow-up questions about \
previously attached files.\n\n\
When referencing document content:\n\
- Cite the filename when quoting or paraphrasing document content\n\
- If no relevant document context is found, say so — do not fabricate content\n\
- If the user asks about a file but no matching context appears, suggest they \
re-attach the file or check that it was uploaded successfully\n\
- Use the `document_search` tool for targeted deep content retrieval when \
background context is insufficient";

pub(crate) struct DocumentsComponent<EM> {
    qdrant: QdrantClient,
    embedding: Arc<EM>,
    conversation_pid: String,
    result_count: usize,
}

impl<EM> DocumentsComponent<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) const NAME: &str = "documents";

    pub(crate) fn new(qdrant: QdrantClient, embedding: Arc<EM>, conversation_pid: String, result_count: usize) -> Self {
        Self {
            qdrant,
            embedding,
            conversation_pid,
            result_count,
        }
    }
}

impl<EM> AgentComponent for DocumentsComponent<EM>
where
    EM: EmbeddingModel,
{
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn init<'a>(
        &'a mut self,
        dynamic_context: &'a DynamicContextStore,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let doc_index = DocumentVectorIndex::new(
                self.qdrant.clone(),
                self.embedding.clone(),
                self.conversation_pid.clone(),
                self.result_count,
            );

            let mut guard = dynamic_context.write().await;
            guard.push((self.result_count, Box::new(doc_index)));
        })
    }

    fn system_prompt_section(&self) -> Option<&str> {
        Some(SYSTEM_PROMPT_SECTION)
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

    fn make_component() -> DocumentsComponent<StubEmbedding> {
        let qdrant_inner = qdrant_client::Qdrant::from_url("http://localhost:6334")
            .build()
            .expect("qdrant client");

        DocumentsComponent::new(
            QdrantClient::new(qdrant_inner),
            Arc::new(StubEmbedding),
            "conv_test_123".into(),
            5,
        )
    }

    #[test]
    fn should_return_documents_for_name() {
        let component = make_component();
        assert_eq!(component.name(), "documents");
    }

    #[test]
    fn should_return_system_prompt_section() {
        let component = make_component();
        let section = component.system_prompt_section();
        assert!(section.is_some());
        assert!(section.unwrap().contains("## Documents"));
    }

    #[test]
    fn should_be_boxable_as_dyn_agent_component() {
        let component = make_component();
        let _: Box<dyn AgentComponent> = Box::new(component);
    }
}
