//! Lightweight test mocks available to all `#[cfg(test)]` code.
//!
//! No external services required — these are pure in-memory fakes.

use crate::api::{
    embedding::{BoxFuture, EmbeddingError, EmbeddingModel},
    memory::EpisodicMemory,
    message::Message,
    store::{EmbeddingStore, MessageStore, StoreError},
};

pub struct MockEmbeddingModel;

impl EmbeddingModel for MockEmbeddingModel {
    fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
        Box::pin(async { Ok(vec![0.0; 384]) })
    }
    fn dimensions(&self) -> usize {
        384
    }
}

#[derive(Debug)]
pub struct MockStore;

impl MessageStore for MockStore {
    async fn persist(&self, _message: &Message, _conversation_id: i64) -> Result<(), StoreError> {
        Ok(())
    }
}

impl EmbeddingStore for MockStore {
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
        Ok(vec![])
    }
}
