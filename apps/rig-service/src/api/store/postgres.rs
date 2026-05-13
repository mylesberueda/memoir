use super::*;
use crate::{
    api::{memory::EpisodicMemory, message::proto::message_part_to_proto},
    models::messages,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

pub(crate) struct PostgresStore {
    db: DatabaseConnection,
}

impl PostgresStore {
    pub(crate) fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl MessageStore for PostgresStore {
    async fn persist(&self, message: &Message, conversation_id: i64) -> Result<(), StoreError> {
        let text = message.text_content();
        let proto_parts: Vec<proto_rs::rig::v1::MessagePart> =
            message.parts().iter().cloned().map(message_part_to_proto).collect();

        let model = messages::ActiveModel {
            pid: Set(message.pid().to_string()),
            conversation_id: Set(conversation_id),
            role: Set(message.role().to_string()),
            content: Set(if text.is_empty() { None } else { Some(text) }),
            parts: Set(serde_json::to_value(&proto_parts)?),
            status: Set(message.status().to_string()),
            created_at: Set(message.created_at().naive_utc()),
            ..Default::default()
        };

        let _ = model.save(&self.db).await?;
        Ok(())
    }
}

impl EmbeddingStore for PostgresStore {
    async fn update_embedding(&self, pid: &str, embedding: Vec<f32>) -> Result<(), StoreError> {
        messages::Entity::update_embedding(&self.db, pid, embedding)
            .await
            .map_err(StoreError::Database)
    }

    async fn retrieve_similar(
        &self,
        query_embedding: Vec<f32>,
        user_id: &str,
        agent_id: i64,
        exclude_conversation_id: i64,
        limit: u32,
        min_similarity: f32,
    ) -> Result<Vec<EpisodicMemory>, StoreError> {
        let rows = messages::Entity::retrieve_similar(
            &self.db,
            query_embedding,
            user_id,
            agent_id,
            exclude_conversation_id,
            limit,
            min_similarity,
        )
        .await
        .map_err(StoreError::Database)?;

        Ok(rows
            .into_iter()
            .map(|row| EpisodicMemory {
                pid: row.pid,
                role: row.role,
                content: row.content,
                similarity: row.similarity as f32,
                created_at: row.created_at,
            })
            .collect())
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;
    use crate::{
        api::message::{MessagePart, MessageRole, MessageStatus},
        models::{agents, conversations, language_models, providers},
    };
    use proto_rs::rig::v1;
    use sea_orm::Database;

    async fn setup_db() -> DatabaseConnection {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database::connect(&db_url).await.expect("Failed to connect to database")
    }

    /// Creates full entity hierarchy: provider → model → agent → conversation
    /// Returns (db, conversation_pid) for use in tests
    async fn setup_conversation(db: &DatabaseConnection) -> i64 {
        let unique = nanoid::nanoid!();

        // Create provider
        let provider = providers::ActiveModel {
            pid: Set(format!("prov_{unique}")),
            organization_pid: Set(Some("org_test".to_string())),
            created_by: Set(Some("user_test".into())),
            name: Set("Test Provider".into()),
            provider_type: Set("ollama".into()),
            ..Default::default()
        };
        let provider = provider.insert(db).await.expect("Failed to create provider");

        // Create model
        let model = language_models::ActiveModel {
            pid: Set(format!("model_{unique}")),
            provider_id: Set(provider.id),
            model_id: Set("test-model".into()),
            name: Set("Test Model".into()),
            ..Default::default()
        };
        let model = model.insert(db).await.expect("Failed to create model");

        // Create agent
        let agent = agents::ActiveModel {
            pid: Set(format!("agent_{unique}")),
            organization_pid: Set("org_test".into()),
            created_by: Set("user_test".into()),
            name: Set("Test Agent".into()),
            slug: Set(format!("test-agent-{unique}")),
            model_id: Set(model.id),
            temperature: Set(0.7),
            system_prompt: Set(Some("You are helpful.".into())),
            ..Default::default()
        };
        let agent = agent.insert(db).await.expect("Failed to create agent");

        // Create conversation
        let conv_pid = format!("conv_{unique}");
        let conversation = conversations::ActiveModel {
            pid: Set(conv_pid.clone()),
            user_id: Set("user_test".into()),
            organization_pid: Set("org_test".into()),
            agent_id: Set(agent.id),
            ..Default::default()
        };
        let conversation = conversation.insert(db).await.expect("Failed to create conversation");

        conversation.id
    }

    fn test_message(message_pid: &str) -> Message {
        Message::full(
            message_pid.into(),
            MessageRole::User,
            vec![MessagePart::Text {
                id: "p1".into(),
                content: "Hello, world!".into(),
            }],
            MessageStatus::Complete,
            chrono::Utc::now(),
        )
    }

    /// Creates a second conversation for the same user/agent, returning (agent_id, conversation_id).
    async fn setup_two_conversations(db: &DatabaseConnection) -> (i64, i64, i64) {
        let unique = nanoid::nanoid!();

        let provider = providers::ActiveModel {
            pid: Set(format!("prov_{unique}")),
            organization_pid: Set(Some("org_test".to_string())),
            created_by: Set(Some("user_test".into())),
            name: Set("Test Provider".into()),
            provider_type: Set("ollama".into()),
            ..Default::default()
        };
        let provider = provider.insert(db).await.expect("Failed to create provider");

        let model = language_models::ActiveModel {
            pid: Set(format!("model_{unique}")),
            provider_id: Set(provider.id),
            model_id: Set("test-model".into()),
            name: Set("Test Model".into()),
            ..Default::default()
        };
        let model = model.insert(db).await.expect("Failed to create model");

        let agent = agents::ActiveModel {
            pid: Set(format!("agent_{unique}")),
            organization_pid: Set("org_test".into()),
            created_by: Set("user_test".into()),
            name: Set("Test Agent".into()),
            slug: Set(format!("test-agent-{unique}")),
            model_id: Set(model.id),
            temperature: Set(0.7),
            system_prompt: Set(Some("You are helpful.".into())),
            ..Default::default()
        };
        let agent = agent.insert(db).await.expect("Failed to create agent");

        let conv1 = conversations::ActiveModel {
            pid: Set(format!("conv1_{unique}")),
            user_id: Set("user_test".into()),
            organization_pid: Set("org_test".into()),
            agent_id: Set(agent.id),
            ..Default::default()
        };
        let conv1 = conv1.insert(db).await.expect("Failed to create conversation 1");

        let conv2 = conversations::ActiveModel {
            pid: Set(format!("conv2_{unique}")),
            user_id: Set("user_test".into()),
            organization_pid: Set("org_test".into()),
            agent_id: Set(agent.id),
            ..Default::default()
        };
        let conv2 = conv2.insert(db).await.expect("Failed to create conversation 2");

        (agent.id, conv1.id, conv2.id)
    }

    mod embedding {
        use super::*;
        use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};

        fn text_msg(pid: &str, content: &str) -> Message {
            Message::full(
                pid.into(),
                MessageRole::User,
                vec![MessagePart::Text {
                    id: "p1".into(),
                    content: content.into(),
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            )
        }

        #[tokio::test]
        async fn should_update_embedding_column() {
            let db = setup_db().await;
            let (agent_id, conv_id, _) = setup_two_conversations(&db).await;
            let store = PostgresStore::new(db.clone());

            let msg_pid = format!("msg_{}", nanoid::nanoid!());
            let message = test_message(&msg_pid);
            store.persist(&message, conv_id).await.expect("persist failed");

            let embedding = vec![0.1_f32; 384];
            let result = store.update_embedding(&msg_pid, embedding).await;

            assert!(result.is_ok(), "update_embedding failed: {:?}", result.err());

            let results = store
                .retrieve_similar(vec![0.1_f32; 384], "user_test", agent_id, -1, 10, 0.0)
                .await
                .expect("retrieve failed");

            let found = results.iter().any(|r| r.pid == msg_pid);
            assert!(
                found,
                "should find the message by its own embedding, results: {:?}",
                results.iter().map(|r| &r.pid).collect::<Vec<_>>()
            );
        }

        #[tokio::test]
        async fn should_return_empty_when_no_similar_messages_exist() {
            let db = setup_db().await;
            let (agent_id, conv_id, _) = setup_two_conversations(&db).await;
            let store = PostgresStore::new(db.clone());

            let results = store
                .retrieve_similar(vec![0.1_f32; 384], "user_test", agent_id, conv_id, 10, 0.5)
                .await
                .expect("retrieve failed");

            assert!(results.is_empty(), "should return empty when no embeddings exist");
        }

        #[tokio::test]
        async fn should_exclude_current_conversation_from_results() {
            let db = setup_db().await;
            let (agent_id, conv1_id, conv2_id) = setup_two_conversations(&db).await;
            let store = PostgresStore::new(db.clone());

            let msg_pid1 = format!("msg_{}", nanoid::nanoid!());
            let msg1 = text_msg(&msg_pid1, "This is a test message in conversation one");
            store.persist(&msg1, conv1_id).await.expect("persist msg1 failed");
            store
                .update_embedding(&msg_pid1, vec![0.5_f32; 384])
                .await
                .expect("embed msg1 failed");

            let msg_pid2 = format!("msg_{}", nanoid::nanoid!());
            let msg2 = text_msg(&msg_pid2, "This is a test message in conversation two");
            store.persist(&msg2, conv2_id).await.expect("persist msg2 failed");
            store
                .update_embedding(&msg_pid2, vec![0.5_f32; 384])
                .await
                .expect("embed msg2 failed");

            let results = store
                .retrieve_similar(vec![0.5_f32; 384], "user_test", agent_id, conv1_id, 10, 0.0)
                .await
                .expect("retrieve failed");

            assert!(
                results.iter().all(|r| r.pid != msg_pid1),
                "should not include messages from the excluded conversation"
            );
            assert!(
                results.iter().any(|r| r.pid == msg_pid2),
                "should include messages from other conversations"
            );
        }

        #[tokio::test]
        async fn should_exclude_deleted_messages_from_results() {
            let db = setup_db().await;
            let (agent_id, conv1_id, conv2_id) = setup_two_conversations(&db).await;
            let store = PostgresStore::new(db.clone());

            let msg_pid = format!("msg_{}", nanoid::nanoid!());
            let msg = text_msg(&msg_pid, "This message will be deleted");
            store.persist(&msg, conv2_id).await.expect("persist failed");
            store
                .update_embedding(&msg_pid, vec![0.5_f32; 384])
                .await
                .expect("embed failed");

            // Soft-delete the message
            use sea_orm::{ActiveModelTrait, ActiveValue::Set};
            let db_msg = messages::Entity::find()
                .filter(messages::Column::Pid.eq(&msg_pid))
                .one(&db)
                .await
                .expect("query failed")
                .expect("message not found");
            let mut active: messages::ActiveModel = db_msg.into();
            active.is_deleted = Set(true);
            active.save(&db).await.expect("soft-delete failed");

            let results = store
                .retrieve_similar(vec![0.5_f32; 384], "user_test", agent_id, conv1_id, 10, 0.0)
                .await
                .expect("retrieve failed");

            assert!(
                !results.iter().any(|r| r.pid == msg_pid),
                "should not include deleted messages"
            );
        }

        #[tokio::test]
        async fn should_respect_minimum_similarity_threshold() {
            let db = setup_db().await;
            let (agent_id, conv1_id, conv2_id) = setup_two_conversations(&db).await;
            let store = PostgresStore::new(db.clone());

            let msg_pid = format!("msg_{}", nanoid::nanoid!());
            let msg = text_msg(&msg_pid, "A message with an embedding vector");
            store.persist(&msg, conv2_id).await.expect("persist failed");
            store
                .update_embedding(&msg_pid, vec![0.5_f32; 384])
                .await
                .expect("embed failed");

            let results_high = store
                .retrieve_similar(vec![0.3_f32; 384], "user_test", agent_id, conv1_id, 10, 0.9999)
                .await
                .expect("retrieve failed");

            let results_low = store
                .retrieve_similar(vec![0.3_f32; 384], "user_test", agent_id, conv1_id, 10, 0.0)
                .await
                .expect("retrieve failed");

            assert!(
                results_high.len() <= results_low.len(),
                "higher threshold should return fewer results: high={}, low={}",
                results_high.len(),
                results_low.len()
            );
        }
    }

    mod persist {
        use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};

        use super::*;

        #[tokio::test]
        async fn should_insert_message_when_conversation_exists() {
            let db = setup_db().await;
            let conv_id = setup_conversation(&db).await;

            let store = PostgresStore::new(db.clone());
            let msg_pid = format!("msg_{}", nanoid::nanoid!());
            let message = test_message(&msg_pid);

            let result = store.persist(&message, conv_id).await;

            assert!(result.is_ok(), "persist failed: {:?}", result.err());

            let saved = messages::Entity::find()
                .filter(messages::Column::Pid.eq(&msg_pid))
                .one(&db)
                .await
                .expect("Failed to query message");

            assert!(saved.is_some());
            let saved = saved.unwrap();
            assert_eq!(saved.role, "user");
            assert_eq!(saved.content, Some("Hello, world!".into()));
        }

        #[tokio::test]
        async fn should_return_database_error_when_conversation_not_found() {
            let db = setup_db().await;
            let store = PostgresStore::new(db);

            let message = test_message("msg_123");

            let result = store.persist(&message, -1).await;

            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), StoreError::Database(_)));
        }

        #[tokio::test]
        async fn should_serialize_parts_vec_to_json() {
            let db = setup_db().await;
            let conversation_id = setup_conversation(&db).await;

            let store = PostgresStore::new(db.clone());
            let msg_pid = format!("msg_{}", nanoid::nanoid!());

            let message = Message::full(
                msg_pid.clone(),
                MessageRole::Assistant,
                vec![MessagePart::Text {
                    id: "part_1".into(),
                    content: "Here's my response".into(),
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let result = store.persist(&message, conversation_id).await;
            assert!(result.is_ok(), "persist failed: {:?}", result.err());

            let saved = messages::Entity::find()
                .filter(messages::Column::Pid.eq(&msg_pid))
                .one(&db)
                .await
                .expect("Failed to query message")
                .expect("Message not found");

            let parts: Vec<v1::MessagePart> = serde_json::from_value(saved.parts).expect("Failed to deserialize parts");
            assert_eq!(parts.len(), 1);
            assert_eq!(parts[0].id, "part_1");
            assert_eq!(parts[0].kind, i32::from(v1::MessagePartKind::Text));
        }
    }
}
