use crate::api::message::{self, Message, MessageRole, MessageStatus};
use proto_rs::rig::v1;
use sea_orm::{ActiveModelBehavior, ConnectionTrait as _, DatabaseConnection, FromQueryResult, Statement};

pub(crate) use super::_entity::messages::*;

/// Helper struct for mapping similarity search query results.
#[derive(Debug, FromQueryResult)]
pub(crate) struct SimilarMessageRow {
    pub(crate) pid: String,
    pub(crate) role: String,
    pub(crate) content: String,
    pub(crate) similarity: f64,
    pub(crate) created_at: chrono::NaiveDateTime,
}

impl Entity {
    /// Update the embedding vector for a message by its pid.
    /// Formats `Vec<f32>` as a pgvector literal (safe — numeric values only).
    pub(crate) async fn update_embedding(
        db: &DatabaseConnection,
        pid: &str,
        embedding: Vec<f32>,
    ) -> Result<(), sea_orm::DbErr> {
        let vec_literal = format!(
            "[{}]",
            embedding.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
        );

        let sql = format!("UPDATE messages SET embedding = '{}' WHERE pid = $1", vec_literal);
        db.execute_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &sql,
            vec![pid.into()],
        ))
        .await?;

        Ok(())
    }

    /// Retrieve messages with embeddings similar to the query vector, scoped
    /// by user/agent, excluding the current conversation.
    pub(crate) async fn retrieve_similar(
        db: &DatabaseConnection,
        query_embedding: Vec<f32>,
        user_id: &str,
        agent_id: i64,
        exclude_conversation_id: i64,
        limit: u32,
        min_similarity: f32,
    ) -> Result<Vec<SimilarMessageRow>, sea_orm::DbErr> {
        let vec_literal = format!(
            "[{}]",
            query_embedding
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let sql = format!(
            r#"SELECT m.pid, m.role, m.content, m.created_at,
                      1.0 - (m.embedding <=> '{vec}') as similarity
               FROM messages m
               JOIN conversations c ON c.id = m.conversation_id
               WHERE c.user_id = $1
                 AND c.agent_id = $2
                 AND m.conversation_id != $3
                 AND m.embedding IS NOT NULL
                 AND m.is_deleted = false
                 AND m.status = 'complete'
                 AND m.content IS NOT NULL
                 AND 1.0 - (m.embedding <=> '{vec}') >= $4
               ORDER BY m.embedding <=> '{vec}'
               LIMIT $5"#,
            vec = vec_literal,
        );

        SimilarMessageRow::find_by_statement(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &sql,
            vec![
                user_id.into(),
                agent_id.into(),
                exclude_conversation_id.into(),
                (min_similarity as f64).into(),
                (limit as i64).into(),
            ],
        ))
        .all(db)
        .await
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Message {
    fn from(model: Model) -> Self {
        let role = match model.role.as_str() {
            "assistant" => MessageRole::Assistant,
            _ => MessageRole::User,
        };

        let status = model.status.parse::<MessageStatus>().unwrap_or(MessageStatus::Complete);
        let created_at = model.created_at.and_utc();

        // DB stores parts as JSON-serialized proto MessagePart structs.
        // Deserialize them, then convert via the proto helper.
        let proto_parts: Vec<v1::MessagePart> = serde_json::from_value(model.parts).unwrap_or_default();

        let parts = proto_parts
            .into_iter()
            .filter_map(message::proto::proto_part_to_message_part)
            .collect();

        Message::full(model.pid, role, parts, status, created_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proto_rs::rig::v1::{MessagePartKind, MessagePartStatus};

    mod from_model {
        use super::*;

        fn make_model(pid: &str, role: &str, status: &str, parts_json: serde_json::Value) -> Model {
            Model {
                id: 1,
                pid: pid.into(),
                conversation_id: 100,
                role: role.into(),
                content: Some("hello".into()),
                parts: parts_json,
                status: status.into(),
                is_deleted: false,
                #[expect(deprecated, reason = "sea_orm generates this type.")]
                created_at: chrono::NaiveDateTime::from_timestamp_millis(1704067200000).unwrap(),
                embedding: None,
            }
        }

        #[test]
        fn should_convert_user_text_message() {
            let parts = serde_json::json!([{
                "id": "p1",
                "kind": MessagePartKind::Text as i32,
                "status": MessagePartStatus::Complete as i32,
                "content": "hello"
            }]);
            let model = make_model("msg_1", "user", "complete", parts);

            let msg: Message = model.into();

            assert_eq!(msg.pid(), "msg_1");
            assert_eq!(msg.role(), MessageRole::User);
            assert_eq!(msg.status(), MessageStatus::Complete);
            assert_eq!(msg.text_content(), "hello");
            assert_eq!(msg.created_at().timestamp_millis(), 1704067200000);
        }

        #[test]
        fn should_convert_assistant_message() {
            let parts = serde_json::json!([{
                "id": "p1",
                "kind": MessagePartKind::Text as i32,
                "status": MessagePartStatus::Complete as i32,
                "content": "hi there"
            }]);
            let model = make_model("msg_2", "assistant", "complete", parts);

            let msg: Message = model.into();

            assert_eq!(msg.role(), MessageRole::Assistant);
            assert_eq!(msg.text_content(), "hi there");
        }

        #[test]
        fn should_convert_cancelled_status() {
            let model = make_model("msg_3", "user", "cancelled", serde_json::json!([]));

            let msg: Message = model.into();

            assert_eq!(msg.status(), MessageStatus::Cancelled);
        }

        #[test]
        fn should_default_to_complete_on_invalid_status() {
            let model = make_model("msg_4", "user", "bogus", serde_json::json!([]));

            let msg: Message = model.into();

            assert_eq!(msg.status(), MessageStatus::Complete);
        }

        #[test]
        fn should_handle_empty_parts_json() {
            let model = make_model("msg_5", "user", "complete", serde_json::json!([]));

            let msg: Message = model.into();

            assert!(msg.parts().is_empty());
        }

        #[test]
        fn should_convert_created_at_to_utc() {
            let model = make_model("msg_6", "user", "complete", serde_json::json!([]));

            let msg: Message = model.into();

            assert_eq!(msg.created_at().timestamp_millis(), 1704067200000);
        }

        #[test]
        fn should_preserve_stored_part_order_when_loading_message_from_db_json() {
            let parts = serde_json::json!([
                {
                    "id": "meta_1",
                    "kind": MessagePartKind::Metadata as i32,
                    "status": MessagePartStatus::Complete as i32,
                    "content": "{\"model_id\":\"gpt-4.1\",\"agent_name\":\"Startup Agent\"}"
                },
                {
                    "id": "text_1",
                    "kind": MessagePartKind::Text as i32,
                    "status": MessagePartStatus::Complete as i32,
                    "content": "hello"
                },
                {
                    "id": "tool_1",
                    "kind": MessagePartKind::ToolResult as i32,
                    "status": MessagePartStatus::Complete as i32,
                    "tool_result": {
                        "tool_call_id": "call_1",
                        "result": "done",
                        "status": 2
                    }
                }
            ]);
            let model = make_model("msg_7", "assistant", "complete", parts);

            let msg: Message = model.into();

            assert_eq!(msg.parts().len(), 3);
            assert!(matches!(
                msg.parts()[0],
                crate::api::message::MessagePart::Metadata { .. }
            ));
            assert!(matches!(msg.parts()[1], crate::api::message::MessagePart::Text { .. }));
            assert!(matches!(
                msg.parts()[2],
                crate::api::message::MessagePart::ToolResult { .. }
            ));
        }
    }
}
