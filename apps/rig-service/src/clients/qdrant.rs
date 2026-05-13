use crate::context::{DOCUMENT_CHUNKS_COLLECTION, DOCUMENT_METADATA_COLLECTION};
use qdrant_client::{
    Qdrant,
    qdrant::{
        Condition, DeletePointsBuilder, Filter, QueryPointsBuilder, ScoredPoint, SetPayloadPointsBuilder,
        UpsertPointsBuilder, points_selector::PointsSelectorOneOf,
    },
};
use std::{collections::HashMap, ops::Deref};

type InnerClient = Qdrant;

#[derive(Clone)]
pub(crate) struct QdrantClient {
    client: InnerClient,
}

impl Deref for QdrantClient {
    type Target = InnerClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum QdrantError {
    #[error("qdrant error: {0}")]
    Client(String),
}

impl QdrantClient {
    pub(crate) fn new(client: InnerClient) -> Self {
        Self { client }
    }

    /// Upsert points into a collection.
    pub(crate) async fn upsert_points(&self, request: UpsertPointsBuilder) -> Result<(), QdrantError> {
        self.client
            .upsert_points(request)
            .await
            .map_err(|e| QdrantError::Client(format!("upsert: {e}")))?;
        Ok(())
    }

    /// Delete all points for a document from both metadata and chunks collections.
    pub(crate) async fn delete_document_points(&self, doc_pid: &str) -> Result<(), QdrantError> {
        let filter = Self::document_filter(doc_pid);

        for collection in [DOCUMENT_METADATA_COLLECTION, DOCUMENT_CHUNKS_COLLECTION] {
            self.client
                .delete_points(DeletePointsBuilder::new(collection).points(filter.clone()))
                .await
                .map_err(|e| QdrantError::Client(format!("delete from {collection}: {e}")))?;
        }

        Ok(())
    }

    /// Search the document metadata collection for the most relevant documents.
    ///
    /// Filters optionally by `organization_pid` and/or a set of `document_id` values.
    /// When `organization_pid` is `None`, the org condition is omitted (e.g. conversation-scoped search
    /// where document PIDs are already resolved from the join table).
    pub(crate) async fn search_document_metadata(
        &self,
        query_embedding: Vec<f32>,
        organization_pid: Option<&str>,
        document_pids: Option<&[String]>,
        limit: u64,
    ) -> Result<Vec<ScoredPoint>, QdrantError> {
        let filter = Self::build_search_filter(organization_pid, document_pids);
        self.search_collection(DOCUMENT_METADATA_COLLECTION, query_embedding, filter, limit)
            .await
    }

    /// Search the document chunks collection for the most relevant text chunks.
    ///
    /// Filters optionally by `organization_pid` and/or a set of `document_id` values.
    /// When searching conversation-scoped documents, `organization_pid` can be `None` since the
    /// document PIDs already provide sufficient scoping.
    pub(crate) async fn search_document_chunks(
        &self,
        query_embedding: Vec<f32>,
        organization_pid: Option<&str>,
        document_pids: Option<&[String]>,
        limit: u64,
    ) -> Result<Vec<ScoredPoint>, QdrantError> {
        let filter = Self::build_search_filter(organization_pid, document_pids);
        self.search_collection(DOCUMENT_CHUNKS_COLLECTION, query_embedding, filter, limit)
            .await
    }

    /// Update `conversation_pids` on all Qdrant points for a given document.
    /// Uses `set_payload` which merges — it won't touch other payload fields.
    /// Caller provides the full, current list of conversation PIDs (from the
    /// DB source of truth).
    pub(crate) async fn set_conversation_pids(
        &self,
        doc_pid: &str,
        conversation_pids: Vec<String>,
    ) -> Result<(), QdrantError> {
        let filter = Self::document_filter(doc_pid);
        let selector = PointsSelectorOneOf::Filter(filter);

        let payload: HashMap<String, qdrant_client::qdrant::Value> =
            HashMap::from([("conversation_pids".into(), conversation_pids.into())]);

        for collection in [DOCUMENT_METADATA_COLLECTION, DOCUMENT_CHUNKS_COLLECTION] {
            self.client
                .set_payload(
                    SetPayloadPointsBuilder::new(collection, payload.clone()).points_selector(selector.clone()),
                )
                .await
                .map_err(|e| QdrantError::Client(format!("set_payload on {collection}: {e}")))?;
        }

        Ok(())
    }

    /// Search the document chunks collection filtered by a single conversation PID.
    /// Returns chunks whose `conversation_pids` array contains the given value.
    pub(crate) async fn search_chunks_by_conversation(
        &self,
        query_embedding: Vec<f32>,
        conversation_pid: &str,
        limit: u64,
    ) -> Result<Vec<ScoredPoint>, QdrantError> {
        let filter = Filter::must([Condition::matches("conversation_pids", conversation_pid.to_string())]);
        self.search_collection(DOCUMENT_CHUNKS_COLLECTION, query_embedding, filter, limit)
            .await
    }

    /// Perform a vector similarity search against a single collection.
    async fn search_collection(
        &self,
        collection: &str,
        query_embedding: Vec<f32>,
        filter: Filter,
        limit: u64,
    ) -> Result<Vec<ScoredPoint>, QdrantError> {
        let response = self
            .client
            .query(
                QueryPointsBuilder::new(collection)
                    .query(query_embedding)
                    .filter(filter)
                    .limit(limit)
                    .with_payload(true),
            )
            .await
            .map_err(|e| QdrantError::Client(format!("search {collection}: {e}")))?;

        Ok(response.result)
    }

    /// Build a Qdrant filter with optional `organization_pid` and optional `document_id` restriction.
    /// When `organization_pid` is `None`, the org condition is omitted entirely.
    fn build_search_filter(organization_pid: Option<&str>, document_pids: Option<&[String]>) -> Filter {
        let mut conditions = Vec::new();

        if let Some(org_pid) = organization_pid {
            conditions.push(Condition::matches("organization_pid", org_pid.to_string()));
        }

        if let Some(pids) = document_pids {
            conditions.push(Condition::matches("document_pid", pids.to_vec()));
        }

        Filter::must(conditions)
    }

    fn document_filter(doc_pid: &str) -> Filter {
        Filter::must([Condition::matches("document_pid", doc_pid.to_string())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::qdrant::condition::ConditionOneOf;

    /// Extract the field key from the first condition in a filter's `must` list.
    fn filter_field_keys(filter: &Filter) -> Vec<String> {
        filter
            .must
            .iter()
            .filter_map(|cond| match &cond.condition_one_of {
                Some(ConditionOneOf::Field(fc)) => Some(fc.key.clone()),
                _ => None,
            })
            .collect()
    }

    // These tests verify that the Qdrant filter keys match the payload field
    // names written during ingestion (ingestion/mod.rs). A mismatch means
    // searches silently return zero results.

    mod build_search_filter {
        use super::*;

        #[test]
        fn should_filter_organization_by_organization_pid_key() {
            let filter = QdrantClient::build_search_filter(Some("org_abc"), None);
            let keys = filter_field_keys(&filter);

            assert_eq!(
                keys,
                vec!["organization_pid"],
                "filter must use 'organization_pid' to match ingestion payload key"
            );
        }

        #[test]
        fn should_filter_documents_by_document_pid_key() {
            let pids = vec!["doc_1".to_string()];
            let filter = QdrantClient::build_search_filter(None, Some(&pids));
            let keys = filter_field_keys(&filter);

            assert_eq!(
                keys,
                vec!["document_pid"],
                "filter must use 'document_pid' to match ingestion payload key"
            );
        }

        #[test]
        fn should_include_both_keys_when_both_provided() {
            let pids = vec!["doc_1".to_string()];
            let filter = QdrantClient::build_search_filter(Some("org_abc"), Some(&pids));
            let keys = filter_field_keys(&filter);

            assert_eq!(keys, vec!["organization_pid", "document_pid"]);
        }

        #[test]
        fn should_produce_empty_filter_when_neither_provided() {
            let filter = QdrantClient::build_search_filter(None, None);
            assert!(filter.must.is_empty());
        }
    }

    mod document_filter {
        use super::*;

        #[test]
        fn should_filter_by_document_pid_key() {
            let filter = QdrantClient::document_filter("doc_abc");
            let keys = filter_field_keys(&filter);

            assert_eq!(
                keys,
                vec!["document_pid"],
                "document_filter must use 'document_pid' to match ingestion payload key"
            );
        }
    }
}
