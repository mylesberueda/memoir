pub(crate) mod postgres;

mod error;

pub(crate) use error::StoreError;
pub(crate) use postgres::PostgresStore;

use crate::api::{memory::EpisodicMemory, message::Message};

pub(crate) trait EmbeddingStore: Send + Sync + 'static {
    /// Update the embedding vector for a message by its pid.
    #[allow(
        dead_code,
        reason = "Called via EpisodicMemoryComponent::embed_message, transitionally unused during component migration"
    )]
    fn update_embedding(
        &self,
        pid: &str,
        embedding: Vec<f32>,
    ) -> impl std::future::Future<Output = Result<(), StoreError>> + Send;

    /// Retrieve messages with embeddings similar to the query, scoped by user/agent,
    /// excluding the current conversation.
    fn retrieve_similar(
        &self,
        query_embedding: Vec<f32>,
        user_id: &str,
        agent_id: i64,
        exclude_conversation_id: i64,
        limit: u32,
        min_similarity: f32,
    ) -> impl std::future::Future<Output = Result<Vec<EpisodicMemory>, StoreError>> + Send;
}

pub(crate) trait MessageStore: Send + Sync + 'static {
    /// Persist a fully-formed message to storage.
    fn persist(
        &self,
        message: &Message,
        conversation_id: i64,
    ) -> impl std::future::Future<Output = Result<(), StoreError>> + Send;

    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Default batch persistence stays available while only single-message writes are used"
        )
    )]
    /// Save multiple messages (default: sequential [`MessageStore::persist`])
    async fn persist_many(&self, messages: &[(Message, i64)]) -> Result<usize, StoreError> {
        let mut count = 0;
        for (message, conversation_id) in messages {
            self.persist(message, *conversation_id).await?;
            count += 1;
        }
        Ok(count)
    }

    #[allow(unused, reason = "Needed when other stores implemented")]
    /// Flush buffered messages to persistent storage
    async fn flush(&self, _limit: usize) -> Result<usize, StoreError> {
        Err(StoreError::Internal(
            "Flush not implemented for this message store. Use persist to write directly".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::message::{MessagePart, MessageRole, MessageStatus};
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Mock store for testing default trait implementations
    struct MockStore {
        persist_count: AtomicUsize,
        fail_on_call: Option<usize>,
    }

    impl MockStore {
        fn new() -> Self {
            Self {
                persist_count: AtomicUsize::new(0),
                fail_on_call: None,
            }
        }

        fn fail_on_call(mut self, n: usize) -> Self {
            self.fail_on_call = Some(n);
            self
        }
    }

    impl MessageStore for MockStore {
        async fn persist(&self, _message: &Message, _conversation_id: i64) -> Result<(), StoreError> {
            let count = self.persist_count.fetch_add(1, Ordering::SeqCst);
            if self.fail_on_call == Some(count) {
                return Err(StoreError::Internal("mock failure".into()));
            }
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

    fn test_message(pid: &str) -> Message {
        Message::full(
            pid.into(),
            MessageRole::User,
            vec![MessagePart::Text {
                id: "p1".into(),
                content: "hello".into(),
            }],
            MessageStatus::Complete,
            chrono::Utc::now(),
        )
    }

    mod persist_many_default {
        use super::*;

        #[tokio::test]
        async fn should_return_count_of_persisted_messages_on_success() {
            let store = MockStore::new();
            let messages = vec![
                (test_message("msg_1"), 1),
                (test_message("msg_2"), 1),
                (test_message("msg_3"), 1),
            ];

            let result = store.persist_many(&messages).await;

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 3);
        }

        #[tokio::test]
        async fn should_stop_on_first_error_and_return_it() {
            let store = MockStore::new().fail_on_call(1);
            let messages = vec![
                (test_message("msg_1"), 1),
                (test_message("msg_2"), 1),
                (test_message("msg_3"), 1),
            ];

            let result = store.persist_many(&messages).await;

            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), StoreError::Internal(_)));
            assert_eq!(store.persist_count.load(Ordering::SeqCst), 2);
        }

        #[tokio::test]
        async fn should_return_zero_when_given_empty_slice() {
            let store = MockStore::new();
            let messages: Vec<(Message, i64)> = vec![];

            let result = store.persist_many(&messages).await;

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);
            assert_eq!(store.persist_count.load(Ordering::SeqCst), 0);
        }
    }

    mod flush_default {
        use super::*;

        #[tokio::test]
        async fn should_return_internal_error_indicating_flush_not_supported() {
            let store = MockStore::new();

            let result = store.flush(10).await;

            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(matches!(err, StoreError::Internal(_)));
            if let StoreError::Internal(msg) = err {
                assert!(msg.contains("not implemented"));
            }
        }
    }
}
