use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    models::{conversation_documents, document_group_memberships, document_groups, documents},
};
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::{JsonSchema, schema_for};
use sea_orm::{ColumnTrait as _, EntityTrait as _, JoinType, QueryFilter as _, QuerySelect as _, RelationTrait as _};
use std::{collections::HashMap, sync::Arc};

const DEFAULT_MAX_RESULTS: u64 = 10;
const DEFAULT_TOP_N_DOCS: u64 = 10;

/// Tool name constant — accessible without resolving the generic parameter.
pub(crate) const DOCUMENT_SEARCH_TOOL_NAME: &str = "document_search";

/// Maximum number of characters to include in the formatted output.
/// Roughly ~4000 tokens at ~4 chars/token.
const MAX_OUTPUT_CHARS: usize = 16_000;

#[derive(Debug)]
pub(crate) struct DocumentSearchTool<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
    /// The user_id for the current session — used to scope group resolution.
    user_id: String,
    /// The org_id for the current session — used to scope org-level Qdrant queries.
    organization_pid: Option<String>,
    /// If set, auto-scope search to documents attached to this conversation.
    conversation_id: Option<i64>,
}

impl<EM> DocumentSearchTool<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(
        ctx: Arc<AppContext<EM>>,
        user_id: String,
        organization_pid: Option<String>,
        conversation_id: Option<i64>,
    ) -> Self {
        Self {
            ctx,
            user_id,
            organization_pid,
            conversation_id,
        }
    }

    async fn embed_query(&self, query: &str) -> Result<Vec<f32>, DocumentSearchToolError> {
        self.ctx
            .embedding
            .embed(query)
            .await
            .map_err(|e| DocumentSearchToolError::Embedding(e.to_string()))
    }

    /// Resolve group names to document pids via PostgreSQL.
    ///
    /// Scopes by user_id (own groups) and optionally by org_id (org-shared groups).
    /// A user can always see their own groups. If they have an org context, they can
    /// also see groups shared within that org.
    async fn resolve_group_document_pids(
        &self,
        group_names: &[String],
    ) -> Result<Vec<String>, DocumentSearchToolError> {
        let mut group_filter = sea_orm::Condition::any().add(document_groups::Column::UserId.eq(&self.user_id));

        if let Some(ref organization_pid) = self.organization_pid {
            group_filter = group_filter.add(
                sea_orm::Condition::all()
                    .add(document_groups::Column::IsOrgShared.eq(true))
                    .add(document_groups::Column::OrganizationPid.eq(organization_pid)),
            );
        }

        let groups = document_groups::Entity::find()
            .filter(document_groups::Column::Name.is_in(group_names))
            .filter(group_filter)
            .all(&self.ctx.db)
            .await
            .map_err(|e| DocumentSearchToolError::Database(e.to_string()))?;

        if groups.is_empty() {
            return Ok(vec![]);
        }

        let group_ids: Vec<i64> = groups.iter().map(|g| g.id).collect();

        let doc_pids: Vec<String> = documents::Entity::find()
            .select_only()
            .column(documents::Column::Pid)
            .join(JoinType::InnerJoin, documents::Relation::DocumentGroupMemberships.def())
            .filter(document_group_memberships::Column::GroupId.is_in(group_ids))
            .into_tuple()
            .all(&self.ctx.db)
            .await
            .map_err(|e| DocumentSearchToolError::Database(e.to_string()))?;

        Ok(doc_pids)
    }

    /// Resolve document pids attached to a conversation via the join table.
    async fn resolve_conversation_document_pids(
        &self,
        conversation_id: i64,
    ) -> Result<Vec<String>, DocumentSearchToolError> {
        let doc_pids: Vec<String> = documents::Entity::find()
            .select_only()
            .column(documents::Column::Pid)
            .join(
                sea_orm::JoinType::InnerJoin,
                documents::Relation::ConversationDocuments.def(),
            )
            .filter(conversation_documents::Column::ConversationId.eq(conversation_id))
            .into_tuple()
            .all(&self.ctx.db)
            .await
            .map_err(|e| DocumentSearchToolError::Database(e.to_string()))?;

        Ok(doc_pids)
    }

    fn payload_str<'a>(payload: &'a HashMap<String, qdrant_client::qdrant::Value>, key: &str) -> &'a str {
        payload
            .get(key)
            .and_then(|v| v.as_str())
            .map(String::as_str)
            .unwrap_or("")
    }

    /// Intersect two optional scope filters into a single resolved scope.
    ///
    /// Returns `Ok(scope)` where scope is `None` (no filter) or `Some(pids)`.
    /// Returns `Err(message)` when the intersection is empty and the caller
    /// should return that message directly to the LLM.
    fn intersect_scopes(
        conversation_doc_pids: Option<Vec<String>>,
        group_doc_pids: Option<Vec<String>>,
    ) -> Result<Option<Vec<String>>, String> {
        match (conversation_doc_pids, group_doc_pids) {
            (Some(conv_pids), Some(group_pids)) => {
                let group_set: std::collections::HashSet<&str> = group_pids.iter().map(String::as_str).collect();
                let intersection: Vec<String> = conv_pids
                    .into_iter()
                    .filter(|pid| group_set.contains(pid.as_str()))
                    .collect();
                if intersection.is_empty() {
                    Err("No documents match both the conversation scope and the specified groups.".to_string())
                } else {
                    Ok(Some(intersection))
                }
            }
            (Some(pids), None) | (None, Some(pids)) => Ok(Some(pids)),
            (None, None) => Ok(None),
        }
    }

    /// Format search results as structured text for the LLM.
    /// Adds chunks in descending relevance order until the character budget is exhausted.
    fn format_results(chunks: &[qdrant_client::qdrant::ScoredPoint], filename_map: &HashMap<String, String>) -> String {
        if chunks.is_empty() {
            return "No relevant documents found.".to_string();
        }

        let mut output = String::new();
        let mut remaining = MAX_OUTPUT_CHARS;

        for point in chunks {
            let doc_pid = Self::payload_str(&point.payload, "document_pid");
            let text = Self::payload_str(&point.payload, "text");
            let filename = filename_map.get(doc_pid).map(String::as_str).unwrap_or("unknown");

            let entry = format!("[Document: {filename}]\n{text}\n---\n");

            if entry.len() > remaining {
                // Add a truncated version if we have room for at least a header
                if remaining > filename.len() + 30 {
                    let available = remaining.saturating_sub(filename.len() + 30);
                    let truncated_text = &text[..text.floor_char_boundary(available.min(text.len()))];
                    output.push_str(&format!("[Document: {filename}]\n{truncated_text}...\n---\n"));
                }
                break;
            }

            remaining -= entry.len();
            output.push_str(&entry);
        }

        output
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub(crate) struct DocumentSearchToolArgs {
    /// The search query to find relevant information in the user's documents.
    pub query: String,
    /// Optional group names to scope the search. If omitted, searches all documents.
    pub group_names: Option<Vec<String>>,
    /// Maximum number of text chunks to return (default: 10).
    pub max_results: Option<u32>,
}

impl<EM> Tool for DocumentSearchTool<EM>
where
    EM: EmbeddingModel,
{
    const NAME: &'static str = DOCUMENT_SEARCH_TOOL_NAME;
    type Error = DocumentSearchToolError;
    type Args = DocumentSearchToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search through documents attached to this conversation to find relevant information. \
                The user may have uploaded files or attached existing documents to this conversation. \
                Use this when the user asks questions that might be answered by their documents or files. \
                If no documents are attached, this tool will indicate that. \
                You can optionally scope the search to specific document groups by name."
                .to_string(),
            parameters: serde_json::to_value(schema_for!(DocumentSearchToolArgs))
                .expect("schema serialization should not fail"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let org_id = self.organization_pid.as_deref();
        let top_k = args.max_results.map(|k| k.max(1) as u64).unwrap_or(DEFAULT_MAX_RESULTS);

        let query_embedding = self.embed_query(&args.query).await?;

        // Resolve conversation scope if conversation_id is set
        let conversation_doc_pids = if let Some(conversation_id) = self.conversation_id {
            let pids = self.resolve_conversation_document_pids(conversation_id).await?;
            if pids.is_empty() {
                return Ok("No documents are attached to this conversation.".to_string());
            }
            Some(pids)
        } else {
            None
        };

        // Resolve group scope if group names are provided
        let group_doc_pids = if let Some(ref group_names) = args.group_names {
            if !group_names.is_empty() {
                let pids = self.resolve_group_document_pids(group_names).await?;
                if pids.is_empty() {
                    return Ok("No documents found in the specified groups.".to_string());
                }
                Some(pids)
            } else {
                None
            }
        } else {
            None
        };

        // Intersect conversation and group scopes if both are present
        let scoped_doc_pids = match Self::intersect_scopes(conversation_doc_pids, group_doc_pids) {
            Ok(pids) => pids,
            Err(message) => return Ok(message),
        };

        // When we have explicit document PIDs (from conversation or group scope),
        // skip the metadata narrowing step and search chunks directly — the PIDs
        // already provide sufficient scoping.
        let search_doc_pids = if let Some(ref pids) = scoped_doc_pids {
            pids.clone()
        } else {
            // No explicit scope — use metadata search to narrow from the full corpus.
            // This requires org_id to avoid searching across all tenants.
            let oid = org_id.ok_or(DocumentSearchToolError::MissingOrg)?;

            let metadata_results = self
                .ctx
                .qdrant
                .search_document_metadata(query_embedding.clone(), Some(oid), None, DEFAULT_TOP_N_DOCS)
                .await
                .map_err(|e| DocumentSearchToolError::Search(e.to_string()))?;

            if metadata_results.is_empty() {
                return Ok("No relevant documents found.".to_string());
            }

            metadata_results
                .iter()
                .map(|point| Self::payload_str(&point.payload, "document_pid").to_string())
                .filter(|pid| !pid.is_empty())
                .collect()
        };

        if search_doc_pids.is_empty() {
            return Ok("No relevant documents found.".to_string());
        }

        let chunk_results = self
            .ctx
            .qdrant
            .search_document_chunks(query_embedding, org_id, Some(&search_doc_pids), top_k)
            .await
            .map_err(|e| DocumentSearchToolError::Search(e.to_string()))?;

        // Build filename lookup for formatting
        let filename_map: HashMap<String, String> = documents::Entity::find()
            .filter(documents::Column::Pid.is_in(&search_doc_pids))
            .all(&self.ctx.db)
            .await
            .map_err(|e| DocumentSearchToolError::Database(e.to_string()))?
            .into_iter()
            .map(|d| (d.pid.clone(), d.filename))
            .collect();

        Ok(Self::format_results(&chunk_results, &filename_map))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum DocumentSearchToolError {
    #[error("no organization context available")]
    MissingOrg,
    #[error("embedding failed: {0}")]
    Embedding(String),
    #[error("search failed: {0}")]
    Search(String),
    #[error("database error: {0}")]
    Database(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::embedding::onnx::OnnxEmbedding;
    type TestDocSearchTool = DocumentSearchTool<OnnxEmbedding>;

    mod definition {
        use super::*;

        #[test]
        fn should_have_name_document_search() {
            assert_eq!(TestDocSearchTool::NAME, "document_search");
        }
    }

    mod intersect_scopes {
        use super::*;

        fn pids(values: &[&str]) -> Option<Vec<String>> {
            Some(values.iter().map(|s| s.to_string()).collect())
        }

        #[test]
        fn should_return_none_when_neither_scope_present() {
            let result = TestDocSearchTool::intersect_scopes(None, None);
            assert!(result.is_ok());
            assert!(result.unwrap().is_none());
        }

        #[test]
        fn should_return_conversation_pids_when_only_conversation_scope() {
            let result = TestDocSearchTool::intersect_scopes(pids(&["a", "b"]), None);
            assert_eq!(result.unwrap().unwrap(), vec!["a", "b"]);
        }

        #[test]
        fn should_return_group_pids_when_only_group_scope() {
            let result = TestDocSearchTool::intersect_scopes(None, pids(&["c", "d"]));
            assert_eq!(result.unwrap().unwrap(), vec!["c", "d"]);
        }

        #[test]
        fn should_return_intersection_when_both_scopes_overlap() {
            let result = TestDocSearchTool::intersect_scopes(pids(&["a", "b", "c"]), pids(&["b", "c", "d"]));
            let mut pids = result.unwrap().unwrap();
            pids.sort();
            assert_eq!(pids, vec!["b", "c"]);
        }

        #[test]
        fn should_return_error_message_when_scopes_have_no_overlap() {
            let result = TestDocSearchTool::intersect_scopes(pids(&["a"]), pids(&["b"]));
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                "No documents match both the conversation scope and the specified groups."
            );
        }
    }

    mod format_results {
        use super::*;
        use qdrant_client::qdrant::{ScoredPoint, Value as QdrantValue};

        fn make_scored_point(doc_pid: &str, text: &str, score: f32) -> ScoredPoint {
            let payload = HashMap::from([
                ("document_pid".to_string(), QdrantValue::from(doc_pid.to_string())),
                ("text".to_string(), QdrantValue::from(text.to_string())),
                ("chunk_index".to_string(), QdrantValue::from(0_i64)),
            ]);

            ScoredPoint {
                id: None,
                payload,
                score,
                version: 0,
                vectors: None,
                shard_key: None,
                order_value: None,
            }
        }

        #[test]
        fn should_return_no_documents_message_when_empty() {
            let chunks: Vec<ScoredPoint> = vec![];
            let filenames = HashMap::new();

            let result = TestDocSearchTool::format_results(&chunks, &filenames);
            assert_eq!(result, "No relevant documents found.");
        }

        #[test]
        fn should_include_filename_and_text() {
            let chunks = vec![make_scored_point("doc_1", "some important text", 0.9)];
            let filenames = HashMap::from([("doc_1".to_string(), "report.pdf".to_string())]);

            let result = TestDocSearchTool::format_results(&chunks, &filenames);
            assert!(result.contains("[Document: report.pdf]"));
            assert!(result.contains("some important text"));
        }

        #[test]
        fn should_use_unknown_for_missing_filename() {
            let chunks = vec![make_scored_point("doc_orphan", "orphan chunk", 0.5)];
            let filenames = HashMap::new();

            let result = TestDocSearchTool::format_results(&chunks, &filenames);
            assert!(result.contains("[Document: unknown]"));
        }

        #[test]
        fn should_respect_character_budget() {
            // Create chunks that exceed MAX_OUTPUT_CHARS
            let big_text = "x".repeat(MAX_OUTPUT_CHARS + 1000);
            let chunks = vec![
                make_scored_point("doc_1", &big_text, 0.9),
                make_scored_point("doc_2", "should not appear", 0.5),
            ];
            let filenames = HashMap::from([
                ("doc_1".to_string(), "big.pdf".to_string()),
                ("doc_2".to_string(), "small.pdf".to_string()),
            ]);

            let result = TestDocSearchTool::format_results(&chunks, &filenames);
            assert!(
                result.len() <= MAX_OUTPUT_CHARS + 200,
                "output should respect budget roughly"
            );
            assert!(!result.contains("should not appear"), "second chunk should be excluded");
        }
    }

    /// Tests that verify payload key names match what ingestion actually writes.
    /// Ingestion (ingestion/mod.rs) writes "document_pid"; these tests will fail
    /// if format_results or payload_str reads the wrong key.
    mod payload_key_consistency {
        use super::*;
        use qdrant_client::qdrant::{ScoredPoint, Value as QdrantValue};

        /// Build a ScoredPoint with the exact payload keys that ingestion writes.
        fn ingestion_scored_point(doc_pid: &str, text: &str, score: f32) -> ScoredPoint {
            let payload = HashMap::from([
                ("document_pid".to_string(), QdrantValue::from(doc_pid.to_string())),
                ("filename".to_string(), QdrantValue::from("report.pdf".to_string())),
                ("text".to_string(), QdrantValue::from(text.to_string())),
                ("chunk_index".to_string(), QdrantValue::from(0_i64)),
            ]);

            ScoredPoint {
                id: None,
                payload,
                score,
                version: 0,
                vectors: None,
                shard_key: None,
                order_value: None,
            }
        }

        #[test]
        fn should_read_document_pid_from_ingestion_payload() {
            let point = ingestion_scored_point("doc_abc", "some text", 0.9);
            let doc_pid = TestDocSearchTool::payload_str(&point.payload, "document_pid");

            assert_eq!(
                doc_pid, "doc_abc",
                "payload_str should find 'document_pid' key written by ingestion"
            );
        }

        #[test]
        fn should_resolve_filename_via_document_pid_key_in_format_results() {
            let chunks = vec![ingestion_scored_point("doc_1", "content here", 0.9)];
            let filenames = HashMap::from([("doc_1".to_string(), "report.pdf".to_string())]);

            let result = TestDocSearchTool::format_results(&chunks, &filenames);

            assert!(
                result.contains("[Document: report.pdf]"),
                "format_results must look up filename using 'document_pid' key. Got: {result}"
            );
            assert!(
                !result.contains("[Document: unknown]"),
                "filename should not fall back to 'unknown' when doc_pid key is correct. Got: {result}"
            );
        }
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;
    use crate::test_utils::{MockEmbeddingModel, TestContext};
    use serial_test::serial;
    use test_context::test_context;

    type TestDocSearchTool = DocumentSearchTool<MockEmbeddingModel>;

    /// Build a `DocumentSearchTool` from test context with the given scoping.
    fn build_tool(
        ctx: &TestContext,
        user_id: &str,
        org_pid: Option<&str>,
        conversation_id: Option<i64>,
    ) -> TestDocSearchTool {
        DocumentSearchTool::new(
            ctx.app_ctx(),
            user_id.to_string(),
            org_pid.map(String::from),
            conversation_id,
        )
    }

    mod resolve_conversation_document_pids {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_pids_for_linked_documents(ctx: &mut TestContext) {
            // Arrange: create a conversation and two documents, link them
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("conv-pids").await;
            let doc1 = ctx.create_ready_document("conv-pids-1", "user_test", "org_test").await;
            let doc2 = ctx.create_ready_document("conv-pids-2", "user_test", "org_test").await;
            ctx.link_document_to_conversation(doc1.id, conversation.id).await;
            ctx.link_document_to_conversation(doc2.id, conversation.id).await;

            let tool = build_tool(ctx, "user_test", Some("org_test"), Some(conversation.id));

            // Act
            let pids = tool.resolve_conversation_document_pids(conversation.id).await.unwrap();

            // Assert
            assert_eq!(pids.len(), 2);
            assert!(pids.contains(&doc1.pid));
            assert!(pids.contains(&doc2.pid));
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_when_no_documents_linked(ctx: &mut TestContext) {
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("conv-empty").await;
            let tool = build_tool(ctx, "user_test", Some("org_test"), Some(conversation.id));

            let pids = tool.resolve_conversation_document_pids(conversation.id).await.unwrap();
            assert!(pids.is_empty());
        }
    }

    mod resolve_group_document_pids {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_resolve_user_owned_groups_without_org(ctx: &mut TestContext) {
            // Arrange: personal group (no org) with a document
            let group = ctx
                .create_document_group_with_org("grp-no-org", "user_personal", "personal_org_user_personal", false)
                .await;
            let doc = ctx
                .create_ready_document_with_org("grp-no-org-doc", "user_personal", "personal_org_user_personal")
                .await;
            ctx.add_document_to_group(doc.id, group.id).await;

            // Tool scoped to user with no org
            let tool = build_tool(ctx, "user_personal", None, None);

            // Act
            let pids = tool.resolve_group_document_pids(&[group.name.clone()]).await.unwrap();

            // Assert
            assert_eq!(pids.len(), 1);
            assert_eq!(pids[0], doc.pid);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_resolve_org_shared_groups_for_org_member(ctx: &mut TestContext) {
            // Arrange: User A creates a shared group in org_shared
            let group = ctx
                .create_document_group("grp-shared", "user_owner_grp", "org_shared", true)
                .await;
            let doc = ctx
                .create_ready_document("grp-shared-doc", "user_owner_grp", "org_shared")
                .await;
            ctx.add_document_to_group(doc.id, group.id).await;

            // Tool scoped to User B in the same org
            let tool = build_tool(ctx, "user_member_grp", Some("org_shared"), None);

            // Act
            let pids = tool.resolve_group_document_pids(&[group.name.clone()]).await.unwrap();

            // Assert
            assert_eq!(pids.len(), 1);
            assert_eq!(pids[0], doc.pid);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_resolve_private_groups_from_other_users(ctx: &mut TestContext) {
            // Arrange: User A creates a private group
            let group = ctx
                .create_document_group("grp-priv", "user_owner_priv", "org_priv", false)
                .await;
            let doc = ctx
                .create_ready_document("grp-priv-doc", "user_owner_priv", "org_priv")
                .await;
            ctx.add_document_to_group(doc.id, group.id).await;

            // Tool scoped to User B in the same org — should not see private groups
            let tool = build_tool(ctx, "user_other_priv", Some("org_priv"), None);

            // Act
            let pids = tool.resolve_group_document_pids(&[group.name.clone()]).await.unwrap();

            // Assert
            assert!(pids.is_empty(), "should not access another user's private group");
        }
    }

    mod call {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_search_conversation_documents_without_org_context(ctx: &mut TestContext) {
            // Arrange: user with no org, document attached to conversation
            let (_provider, _model, agent, _conv) = ctx.create_full_setup("call-no-org").await;
            let conversation = ctx
                .create_conversation_for_user("call-no-org-conv", agent.id, "user_no_org_search")
                .await;
            let doc = ctx
                .create_ready_document_with_org(
                    "call-no-org-doc",
                    "user_no_org_search",
                    "personal_org_user_no_org_search",
                )
                .await;
            ctx.link_document_to_conversation(doc.id, conversation.id).await;

            // Tool with no org context but with conversation scope
            let tool = build_tool(ctx, "user_no_org_search", None, Some(conversation.id));

            // Act: the call() will embed the query (mock returns zeros), then search Qdrant.
            // Since we haven't indexed chunks in Qdrant, the Qdrant search returns empty.
            // But the key behavior is: it does NOT fail with MissingOrg.
            // The conversation PID resolution succeeds and the tool proceeds to chunk search.
            let result: Result<String, DocumentSearchToolError> = tool
                .call(DocumentSearchToolArgs {
                    query: "test query".to_string(),
                    group_names: None,
                    max_results: None,
                })
                .await;

            // Assert: should succeed (no MissingOrg error), even if no Qdrant results
            assert!(result.is_ok(), "should not fail with MissingOrg: {:?}", result.err());
            let output = result.unwrap();
            // The output will be either chunk results or "No relevant documents found."
            // since we have no indexed chunks — but the important thing is no error.
            assert!(!output.is_empty(), "should return a message (even if no chunks found)");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_no_documents_when_conversation_has_none(ctx: &mut TestContext) {
            // Arrange: conversation with no documents attached
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("call-no-docs").await;
            let tool = build_tool(ctx, "user_test", Some("org_test"), Some(conversation.id));

            // Act
            let result: Result<String, DocumentSearchToolError> = tool
                .call(DocumentSearchToolArgs {
                    query: "anything".to_string(),
                    group_names: None,
                    max_results: None,
                })
                .await;

            // Assert
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "No documents are attached to this conversation.");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_require_org_when_no_scope_provided(ctx: &mut TestContext) {
            // Arrange: no conversation, no groups, no org — should fail with MissingOrg
            let tool = build_tool(ctx, "user_unscoped", None, None);

            // Act
            let result: Result<String, DocumentSearchToolError> = tool
                .call(DocumentSearchToolArgs {
                    query: "test".to_string(),
                    group_names: None,
                    max_results: None,
                })
                .await;

            // Assert: the unscoped path requires org_id for metadata search
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(
                matches!(err, DocumentSearchToolError::MissingOrg),
                "should be MissingOrg, got: {err:?}"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_no_groups_message_when_group_is_empty(ctx: &mut TestContext) {
            // Arrange: user asks for a group that exists but has no documents
            let group = ctx
                .create_document_group_with_org(
                    "call-empty-grp",
                    "user_empty_grp",
                    "personal_org_user_empty_grp",
                    false,
                )
                .await;
            let tool = build_tool(ctx, "user_empty_grp", None, None);

            // Act
            let result: Result<String, DocumentSearchToolError> = tool
                .call(DocumentSearchToolArgs {
                    query: "test".to_string(),
                    group_names: Some(vec![group.name.clone()]),
                    max_results: None,
                })
                .await;

            // Assert
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "No documents found in the specified groups.");
        }
    }
}
