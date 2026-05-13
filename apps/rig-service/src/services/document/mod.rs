use crate::{
    AppContext,
    actors::SessionRegistryActor,
    api::{
        PostgresStore,
        embedding::{DefaultEmbedding, EmbeddingModel},
        ingestion::IngestionPipeline,
    },
    models::{conversation_documents, conversations, document_group_memberships, document_groups, documents},
};
use kameo::actor::ActorRef;
use platform_rs::ext::RequestAuthExt;
use proto_rs::rig::v1;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, EntityTrait as _, JoinType, PaginatorTrait as _,
    QueryFilter as _, QueryOrder as _, QuerySelect as _, RelationTrait as _,
};
use std::sync::Arc;
use tracing::instrument;

const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;
const PRESIGN_EXPIRY_SECS: u64 = 900; // 15 minutes

/// Allowed MIME types for document upload.
/// Covers common document, text, and data formats that kreuzberg can extract.
const ALLOWED_CONTENT_TYPES: &[&str] = &[
    // PDF
    "application/pdf",
    // Plain text
    "text/plain",
    "text/csv",
    "text/html",
    "text/xml",
    "text/markdown",
    "text/rtf",
    // Microsoft Office
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.ms-powerpoint",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    // OpenDocument
    "application/vnd.oasis.opendocument.text",
    "application/vnd.oasis.opendocument.spreadsheet",
    "application/vnd.oasis.opendocument.presentation",
    // Other
    "application/json",
    "application/xml",
    "application/rtf",
    "application/epub+zip",
];

#[derive(Clone)]
pub(crate) struct DocumentService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
}

impl<EM> std::fmt::Debug for DocumentService<EM>
where
    EM: EmbeddingModel,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocumentService").finish()
    }
}

impl<EM> DocumentService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(ctx: Arc<AppContext<EM>>, _registry: ActorRef<SessionRegistryActor<PostgresStore, EM>>) -> Self {
        Self { ctx }
    }

    fn storage_path(user_id: &str, organization_pid: &str, conversation_pid: Option<&str>, filename: &str) -> String {
        let mut path = vec![organization_pid, user_id];

        if let Some(conversation_pid) = conversation_pid {
            path.push(conversation_pid);
        }

        path.push(filename);

        path.join("/")
    }
}

#[tonic::async_trait]
impl<EM> v1::document_service_server::DocumentService for DocumentService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn upload_document(
        &self,
        request: tonic::Request<v1::UploadDocumentRequest>,
    ) -> std::result::Result<tonic::Response<v1::UploadDocumentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &org_pid);

        if req.filename.is_empty() {
            return Err(tonic::Status::invalid_argument("filename is required"));
        }

        if req.content_type.is_empty() {
            return Err(tonic::Status::invalid_argument("content_type is required"));
        }

        if !ALLOWED_CONTENT_TYPES.contains(&req.content_type.as_str()) {
            return Err(tonic::Status::invalid_argument(format!(
                "unsupported content_type: {}",
                req.content_type
            )));
        }

        let conversation = if let Some(conversation_pid) = &req.conversation_pid {
            conversations::Entity::find()
                .filter(conversations::Column::Pid.eq(conversation_pid))
                .filter(conversations::Column::IsDeleted.eq(false))
                .filter(conversations::Column::UserId.eq(&user_id))
                .one(&self.ctx.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to find conversation for auto-attach");
                    tonic::Status::internal("Failed to attach document to conversation")
                })?
        } else {
            None
        };

        let storage_path = Self::storage_path(
            &user_id,
            &org_pid,
            conversation.as_ref().map(|c| c.pid.as_str()),
            &req.filename,
        );

        self.ctx
            .storage
            .upload_object(&storage_path, req.content.to_vec(), &req.content_type)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to upload to storage");
                tonic::Status::internal("Failed to upload document")
            })?;

        let active = documents::ActiveModelEx::new()
            .set_user_id(&user_id)
            .set_organization_pid(org_pid)
            .set_filename(&req.filename)
            .set_content_type(&req.content_type)
            .set_size_bytes(req.content.len() as i64)
            .set_storage_path(&storage_path)
            .set_status(documents::DocStatus::Processing)
            .set_summary(None)
            .set_error_message(None);

        let doc = active.insert(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to create document");
            tonic::Status::internal("Failed to create document")
        })?;

        let conversation_pid = conversation.as_ref().map(|c| c.pid.clone());

        if let Some(conversation) = conversation
            && let Err(e) = conversation_documents::ActiveModelEx::new()
                .set_conversation_id(conversation.id)
                .set_document_id(doc.id)
                .insert(&self.ctx.db)
                .await
        {
            tracing::warn!(error = %e, "failed to auto-attach document to conversation");
        }

        IngestionPipeline::spawn(self.ctx.clone(), doc.clone(), &req.content, conversation_pid);

        Ok(tonic::Response::new(v1::UploadDocumentResponse {
            document: Some(doc.into_proto()),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, pid))]
    async fn get_document(
        &self,
        request: tonic::Request<v1::GetDocumentRequest>,
    ) -> Result<tonic::Response<v1::GetDocumentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &org_pid);
        tracing::Span::current().record("pid", &req.pid);

        let doc = documents::Entity::find()
            .filter(documents::Column::Pid.eq(&req.pid))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch document");
                tonic::Status::internal("Failed to fetch document")
            })?
            .ok_or_else(|| tonic::Status::not_found("Document not found"))?;

        if !doc.is_accessible(&user_id, &org_pid) {
            return Err(tonic::Status::not_found("Document not found"));
        }

        Ok(tonic::Response::new(v1::GetDocumentResponse {
            document: Some(doc.into_proto()),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id, organization_pid, page, page_size, group_pid, status)
    )]
    async fn list_documents(
        &self,
        request: tonic::Request<v1::ListDocumentsRequest>,
    ) -> Result<tonic::Response<v1::ListDocumentsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();

        let page = req.page.max(1) as u64;
        let page_size = if req.page_size == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            (req.page_size as u64).clamp(1, MAX_PAGE_SIZE)
        };

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("page", page);
        tracing::Span::current().record("page_size", page_size);
        tracing::Span::current().record("group_pid", req.group_pid.as_deref().unwrap_or(""));
        tracing::Span::current().record("status", req.status.unwrap_or(0));

        let mut query = documents::Entity::find();

        query = query.filter(
            sea_orm::Condition::any()
                .add(documents::Column::UserId.eq(&user_id))
                .add(documents::Column::OrganizationPid.eq(&organization_pid)),
        );

        if let Some(group_pid) = &req.group_pid {
            let group = document_groups::Entity::find()
                .filter(document_groups::Column::Pid.eq(group_pid))
                .one(&self.ctx.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to fetch group");
                    tonic::Status::internal("Failed to fetch group")
                })?
                .ok_or_else(|| tonic::Status::not_found("Group not found"))?;

            if !group.is_accessible(&user_id, &organization_pid) {
                return Err(tonic::Status::not_found("Group not found"));
            }

            query = query
                .join(JoinType::InnerJoin, documents::Relation::DocumentGroupMemberships.def())
                .filter(document_group_memberships::Column::GroupId.eq(group.id));
        }

        if let Some(status) = req.status
            && let Some(doc_status) = v1::DocumentStatus::try_from(status).ok().and_then(|s| match s {
                v1::DocumentStatus::Pending => Some(documents::DocStatus::Pending),
                v1::DocumentStatus::Processing => Some(documents::DocStatus::Processing),
                v1::DocumentStatus::Ready => Some(documents::DocStatus::Ready),
                v1::DocumentStatus::Failed => Some(documents::DocStatus::Failed),
                v1::DocumentStatus::Unspecified => None,
            })
        {
            query = query.filter(documents::Column::Status.eq(doc_status.to_string()));
        }

        let total = query.clone().count(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to count documents");
            tonic::Status::internal("Failed to count documents")
        })? as i32;

        let docs = query
            .order_by_desc(documents::Column::CreatedAt)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch documents");
                tonic::Status::internal("Failed to fetch documents")
            })?;

        Ok(tonic::Response::new(v1::ListDocumentsResponse {
            documents: docs.into_iter().map(|d| d.into_proto()).collect(),
            total,
            page: page as i32,
            page_size: page_size as i32,
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, pid))]
    async fn delete_document(
        &self,
        request: tonic::Request<v1::DeleteDocumentRequest>,
    ) -> Result<tonic::Response<v1::DeleteDocumentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("pid", &req.pid);

        let doc = documents::Entity::find()
            .filter(documents::Column::Pid.eq(&req.pid))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch document");
                tonic::Status::internal("Failed to fetch document")
            })?
            .ok_or_else(|| tonic::Status::not_found("Document not found"))?;

        if !doc.is_accessible(&user_id, &organization_pid) {
            return Err(tonic::Status::not_found("Document not found"));
        }

        if let Err(e) = self.ctx.storage.delete_object(&doc.storage_path).await {
            tracing::warn!(error = %e, "failed to delete object from storage, continuing with DB delete");
        }

        if let Err(e) = self.ctx.qdrant.delete_document_points(&doc.pid).await {
            tracing::warn!(error = %e, "failed to delete Qdrant points, continuing with DB delete");
        }

        documents::Entity::delete_by_id(doc.id)
            .exec(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to delete document");
                tonic::Status::internal("Failed to delete document")
            })?;

        Ok(tonic::Response::new(v1::DeleteDocumentResponse {}))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn update_document(
        &self,
        request: tonic::Request<v1::UpdateDocumentRequest>,
    ) -> Result<tonic::Response<v1::UpdateDocumentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let doc = documents::Entity::find()
            .filter(documents::Column::Pid.eq(&req.pid))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch document");
                tonic::Status::internal("Failed to fetch document")
            })?
            .ok_or_else(|| tonic::Status::not_found("Document not found"))?;

        if !doc.is_accessible(&user_id, &organization_pid) {
            return Err(tonic::Status::not_found("Document not found"));
        }

        let mut active: documents::ActiveModel = doc.into();

        if let Some(filename) = &req.filename {
            if filename.is_empty() {
                return Err(tonic::Status::invalid_argument("filename cannot be empty"));
            }
            active.filename = Set(filename.clone());
        }

        let doc = active.update(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to update document");
            tonic::Status::internal("Failed to update document")
        })?;

        Ok(tonic::Response::new(v1::UpdateDocumentResponse {
            document: Some(doc.into_proto()),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn get_download_url(
        &self,
        request: tonic::Request<v1::GetDownloadUrlRequest>,
    ) -> Result<tonic::Response<v1::GetDownloadUrlResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let doc = documents::Entity::find()
            .filter(documents::Column::Pid.eq(&req.pid))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch document");
                tonic::Status::internal("Failed to fetch document")
            })?
            .ok_or_else(|| tonic::Status::not_found("Document not found"))?;

        if !doc.is_accessible(&user_id, &organization_pid) {
            return Err(tonic::Status::not_found("Document not found"));
        }

        let download_url = self
            .ctx
            .storage
            .presign_get(&doc.storage_path, PRESIGN_EXPIRY_SECS)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to generate presigned download URL");
                tonic::Status::internal("Failed to generate download URL")
            })?;

        Ok(tonic::Response::new(v1::GetDownloadUrlResponse { download_url }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        AppContext,
        api::embedding::{DefaultEmbedding, onnx::OnnxEmbedding},
        clients::{QdrantClient, StorageClient},
        test_utils::TestContext,
        tools::ToolRegistry,
    };
    use kameo::actor::Spawn as _;
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::document_service_server::DocumentService as _;
    use serial_test::serial;
    use std::sync::Arc;
    use test_context::test_context;
    use tokio::sync::OnceCell;

    static EMBEDDING_MODEL: OnceCell<Arc<DefaultEmbedding>> = OnceCell::const_new();

    async fn get_embedding_model() -> Arc<DefaultEmbedding> {
        EMBEDDING_MODEL
            .get_or_init(|| async {
                Arc::new(OnnxEmbedding::new().expect("Failed to initialize test embedding model"))
            })
            .await
            .clone()
    }

    fn authenticated_request<T>(inner: T, user_id: &str, org_pid: Option<&str>) -> tonic::Request<T> {
        let mut request = tonic::Request::new(inner);
        request.extensions_mut().insert(User {
            id: user_id.to_string(),
            email: Some(format!("{user_id}@test.com")),
            name: Some(format!("Test User {user_id}")),
            org_roles: std::collections::HashMap::new(),
            email_verified: Some(true),
            metadata: std::collections::HashMap::new(),
        });
        if let Some(org) = org_pid {
            request.extensions_mut().insert(OrganizationPid(org.to_string()));
        }
        request
    }

    async fn create_service(ctx: &TestContext) -> DocumentService {
        let endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set");
        let access_key = std::env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set");
        let secret_key = std::env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set");
        let bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET must be set");
        let region = std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        let credentials = aws_sdk_s3::config::Credentials::new(&access_key, &secret_key, None, None, "env");
        let s3_config = aws_sdk_s3::Config::builder()
            .endpoint_url(&endpoint)
            .region(aws_sdk_s3::config::Region::new(region))
            .credentials_provider(credentials)
            .force_path_style(true)
            .behavior_version_latest()
            .build();

        let s3_client = aws_sdk_s3::Client::from_conf(s3_config);

        // Ensure bucket exists (mirrors Storage::ensure_bucket lifecycle hook)
        if s3_client.head_bucket().bucket(&bucket).send().await.is_err() {
            s3_client
                .create_bucket()
                .bucket(&bucket)
                .send()
                .await
                .expect("Failed to create test S3 bucket");
        }

        let storage = StorageClient::new(s3_client, bucket);

        let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
        let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
            .build()
            .expect("Failed to create test Qdrant client");
        let qdrant = QdrantClient::new(qdrant_inner);

        let embedding_model = get_embedding_model().await;

        let app_ctx = Arc::new(AppContext {
            db: (*ctx.db).clone(),
            redis: ctx.redis.clone(),
            storage,
            qdrant,
            embedding: embedding_model,
            api_service: crate::clients::ApiServiceClient::new(
                &std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set"),
            )
            .unwrap(),
        });

        let store = Arc::new(PostgresStore::new((*ctx.db).clone()));
        let registry = SessionRegistryActor::spawn((store, app_ctx.clone(), ToolRegistry::new(app_ctx.clone())));

        DocumentService::new(app_ctx, registry)
    }

    mod upload_document {
        use super::*;

        /// Small test content used by most tests — enough to be non-empty.
        const TEST_CONTENT: &[u8] = b"test file content";

        fn upload_request(
            filename: &str,
            content_type: &str,
            content: &'static [u8],
            conversation_pid: Option<String>,
        ) -> v1::UploadDocumentRequest {
            v1::UploadDocumentRequest {
                filename: filename.to_string(),
                content_type: content_type.to_string(),
                content: content.into(),
                conversation_pid,
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_set_storage_path_with_org_and_user(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("notes.txt", "text/plain", TEST_CONTENT, None),
                "user_test",
                Some("org_path_test"),
            );

            let response = service.upload_document(request).await.unwrap().into_inner();
            let doc = response.document.expect("should have document");

            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();

            assert!(
                db_doc.storage_path.starts_with("org_path_test/"),
                "storage_path should start with organization_pid, got: {}",
                db_doc.storage_path
            );
            assert!(
                db_doc.storage_path.contains("user_test"),
                "storage_path should contain user_id, got: {}",
                db_doc.storage_path
            );
            assert!(
                db_doc.storage_path.ends_with("/notes.txt"),
                "storage_path should end with filename, got: {}",
                db_doc.storage_path
            );

            ctx.created_documents.push(db_doc.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_upload_without_org_context(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("personal.txt", "text/plain", TEST_CONTENT, None),
                "user_no_org",
                None,
            );

            let result = service.upload_document(request).await;
            assert!(result.is_err(), "should reject upload without org context");
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_filename(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("", "text/plain", TEST_CONTENT, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_content_type(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("test.txt", "", TEST_CONTENT, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_unsupported_content_type(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("photo.png", "image/png", TEST_CONTENT, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await;
            assert!(response.is_err());
            let err = response.unwrap_err();
            assert_eq!(err.code(), tonic::Code::InvalidArgument);
            assert!(
                err.message().contains("unsupported"),
                "error should mention unsupported: {}",
                err.message()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_accept_all_allowed_content_types(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let allowed = [
                "application/pdf",
                "text/plain",
                "text/csv",
                "text/html",
                "text/xml",
                "text/markdown",
                "text/rtf",
                "application/msword",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "application/vnd.ms-excel",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                "application/vnd.ms-powerpoint",
                "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                "application/vnd.oasis.opendocument.text",
                "application/vnd.oasis.opendocument.spreadsheet",
                "application/vnd.oasis.opendocument.presentation",
                "application/json",
                "application/xml",
                "application/rtf",
                "application/epub+zip",
            ];

            for content_type in allowed {
                let request = authenticated_request(
                    upload_request(
                        &format!("test.{}", content_type.split('/').next_back().unwrap_or("bin")),
                        content_type,
                        TEST_CONTENT,
                        None,
                    ),
                    "user_test",
                    Some("org_allowed_ct"),
                );

                let response = service.upload_document(request).await;
                assert!(
                    response.is_ok(),
                    "content_type {content_type} should be allowed: {:?}",
                    response.err()
                );

                let doc = response.unwrap().into_inner().document.unwrap();
                let db_doc = documents::Entity::find()
                    .filter(documents::Column::Pid.eq(&doc.pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();
                ctx.created_documents.push(db_doc.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_auto_attach_document_to_conversation(ctx: &mut TestContext) {
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("auto-attach").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("linked.txt", "text/plain", TEST_CONTENT, Some(conversation.pid.clone())),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let doc = response.unwrap().into_inner().document.expect("should have document");

            let link = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .filter(conversation_documents::Column::DocumentId.eq({
                    let db_doc = documents::Entity::find()
                        .filter(documents::Column::Pid.eq(&doc.pid))
                        .one(ctx.db.as_ref())
                        .await
                        .unwrap()
                        .unwrap();
                    ctx.created_documents.push(db_doc.id);
                    db_doc.id
                }))
                .one(ctx.db.as_ref())
                .await
                .unwrap();

            assert!(
                link.is_some(),
                "document should be linked to conversation via conversation_documents"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_upload_document_without_conversation_link(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("unlinked.txt", "text/plain", TEST_CONTENT, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let doc = response.unwrap().into_inner().document.expect("should have document");
            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_documents.push(db_doc.id);

            let links = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::DocumentId.eq(db_doc.id))
                .all(ctx.db.as_ref())
                .await
                .unwrap();

            assert!(
                links.is_empty(),
                "document with no conversation_pid should have no conversation links"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_duplicate_conversation_link(ctx: &mut TestContext) {
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("dup-link").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("first.txt", "text/plain", TEST_CONTENT, Some(conversation.pid.clone())),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await.unwrap().into_inner();
            let doc = response.document.expect("should have document");
            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_documents.push(db_doc.id);

            let links = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .all(ctx.db.as_ref())
                .await
                .unwrap();

            assert_eq!(links.len(), 1, "should have exactly one link per document");

            let request2 = authenticated_request(
                upload_request("second.txt", "text/plain", TEST_CONTENT, Some(conversation.pid.clone())),
                "user_test",
                Some("org_test"),
            );

            let response2 = service.upload_document(request2).await.unwrap().into_inner();
            let doc2 = response2.document.expect("should have document");
            let db_doc2 = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc2.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_documents.push(db_doc2.id);

            let links = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .all(ctx.db.as_ref())
                .await
                .unwrap();

            assert_eq!(links.len(), 2, "each document should have its own link, no duplicates");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_link_document_to_other_users_conversation(ctx: &mut TestContext) {
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("cross-user").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request(
                    "intruder.txt",
                    "text/plain",
                    TEST_CONTENT,
                    Some(conversation.pid.clone()),
                ),
                "user_attacker",
                Some("org_test"),
            );

            let response = service.upload_document(request).await;
            assert!(
                response.is_ok(),
                "document upload should succeed even if link fails: {:?}",
                response.err()
            );

            let doc = response.unwrap().into_inner().document.expect("should have document");
            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_documents.push(db_doc.id);

            let link = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .filter(conversation_documents::Column::DocumentId.eq(db_doc.id))
                .one(ctx.db.as_ref())
                .await
                .unwrap();

            assert!(
                link.is_none(),
                "document should NOT be linked to another user's conversation"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_status_processing(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("status.txt", "text/plain", TEST_CONTENT, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await.unwrap().into_inner();
            let doc = response.document.expect("should have document");

            assert_eq!(
                doc.status,
                v1::DocumentStatus::Processing as i32,
                "uploaded document should have processing status"
            );

            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_documents.push(db_doc.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_derive_size_bytes_from_content(ctx: &mut TestContext) {
            let service = create_service(ctx).await;
            let content = b"known length content for size test";

            let request = authenticated_request(
                upload_request("sized.txt", "text/plain", content, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await.unwrap().into_inner();
            let doc = response.document.expect("should have document");

            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();

            assert_eq!(
                db_doc.size_bytes,
                content.len() as i64,
                "size_bytes should match content length"
            );

            ctx.created_documents.push(db_doc.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_store_file_in_s3(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request("s3check.txt", "text/plain", TEST_CONTENT, None),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await.unwrap().into_inner();
            let doc = response.document.expect("should have document");

            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();

            let exists = service.ctx.storage.head_object(&db_doc.storage_path).await.unwrap();
            assert!(exists, "file should exist in S3 after upload");

            // Verify content round-trips
            let downloaded = service.ctx.storage.download_object(&db_doc.storage_path).await.unwrap();
            assert_eq!(
                downloaded, TEST_CONTENT,
                "downloaded content should match uploaded content"
            );

            ctx.created_documents.push(db_doc.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_conversation_pid_in_storage_path(ctx: &mut TestContext) {
            let (_provider, _model, _agent, conversation) = ctx.create_full_setup("path-conv").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                upload_request(
                    "conv-file.txt",
                    "text/plain",
                    TEST_CONTENT,
                    Some(conversation.pid.clone()),
                ),
                "user_test",
                Some("org_test"),
            );

            let response = service.upload_document(request).await.unwrap().into_inner();
            let doc = response.document.expect("should have document");

            let db_doc = documents::Entity::find()
                .filter(documents::Column::Pid.eq(&doc.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();

            assert!(
                db_doc.storage_path.contains(&conversation.pid),
                "storage_path should contain conversation pid, got: {}",
                db_doc.storage_path
            );
            assert!(
                db_doc.storage_path.ends_with("/conv-file.txt"),
                "storage_path should end with filename, got: {}",
                db_doc.storage_path
            );

            ctx.created_documents.push(db_doc.id);
        }
    }

    mod get_document {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_document_for_owner(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("get-owner", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDocumentRequest { pid: doc.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_document(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let returned = response.unwrap().into_inner().document.expect("should have document");
            assert_eq!(returned.pid, doc.pid);
            assert_eq!(returned.filename, doc.filename);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_document_for_org_member(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("get-org", "user_owner", "org_shared").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDocumentRequest { pid: doc.pid.clone() },
                "user_other",
                Some("org_shared"),
            );

            let response = service.get_document(request).await;
            assert!(response.is_ok(), "org member should access org document");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_other_user(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("get-deny", "user_owner", "org_a").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDocumentRequest { pid: doc.pid.clone() },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.get_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_different_org(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("get-difforg", "user_owner", "org_a").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDocumentRequest { pid: doc.pid.clone() },
                "user_stranger",
                Some("org_b"),
            );

            let response = service.get_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_pid(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDocumentRequest {
                    pid: "doc_nonexistent".to_string(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod list_documents {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_documents_for_owner(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("list-owner", "user_lister", "org_list").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 20,
                    group_pid: None,
                    status: None,
                },
                "user_lister",
                Some("org_list"),
            );

            let response = service.list_documents(request).await.unwrap().into_inner();
            assert!(
                response.documents.iter().any(|d| d.pid == doc.pid),
                "should include owned document"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_org_documents(ctx: &mut TestContext) {
            let doc = ctx
                .create_ready_document("list-org", "user_other", "org_shared_list")
                .await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 20,
                    group_pid: None,
                    status: None,
                },
                "user_viewer",
                Some("org_shared_list"),
            );

            let response = service.list_documents(request).await.unwrap().into_inner();
            assert!(
                response.documents.iter().any(|d| d.pid == doc.pid),
                "org member should see org documents"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_status(ctx: &mut TestContext) {
            ctx.create_ready_document("list-status-r", "user_filter", "org_filter")
                .await;
            ctx.create_document("list-status-p", "user_filter", "org_filter").await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 20,
                    group_pid: None,
                    status: Some(v1::DocumentStatus::Ready as i32),
                },
                "user_filter",
                Some("org_filter"),
            );

            let response = service.list_documents(request).await.unwrap().into_inner();
            assert!(
                response
                    .documents
                    .iter()
                    .all(|d| d.status == v1::DocumentStatus::Ready as i32),
                "all returned documents should have ready status"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_group(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("list-grp", "user_grp", "org_grp", false)
                .await;
            let doc_in = ctx.create_ready_document("list-grp-in", "user_grp", "org_grp").await;
            let doc_out = ctx.create_ready_document("list-grp-out", "user_grp", "org_grp").await;
            ctx.add_document_to_group(doc_in.id, group.id).await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 20,
                    group_pid: Some(group.pid.clone()),
                    status: None,
                },
                "user_grp",
                Some("org_grp"),
            );

            let response = service.list_documents(request).await.unwrap().into_inner();
            assert!(
                response.documents.iter().any(|d| d.pid == doc_in.pid),
                "should include document in group"
            );
            assert!(
                !response.documents.iter().any(|d| d.pid == doc_out.pid),
                "should not include document outside group"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_paginate_results(ctx: &mut TestContext) {
            ctx.create_ready_document("list-pg-1", "user_pg_doc", "org_pg_doc")
                .await;
            ctx.create_ready_document("list-pg-2", "user_pg_doc", "org_pg_doc")
                .await;
            ctx.create_ready_document("list-pg-3", "user_pg_doc", "org_pg_doc")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 2,
                    group_pid: None,
                    status: None,
                },
                "user_pg_doc",
                Some("org_pg_doc"),
            );

            let response = service.list_documents(request).await.unwrap().into_inner();
            assert_eq!(response.documents.len(), 2, "page 1 should have 2 items");
            assert!(response.total >= 3, "total should be at least 3");
            assert_eq!(response.page, 1);
            assert_eq!(response.page_size, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_for_no_documents(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 20,
                    group_pid: None,
                    status: None,
                },
                "user_with_no_docs",
                Some("org_empty"),
            );

            let response = service.list_documents(request).await.unwrap().into_inner();
            assert!(response.documents.is_empty(), "should have no documents");
            assert_eq!(response.total, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_filter_by_inaccessible_group(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("list-deny-grp", "user_owner", "org_a", false)
                .await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListDocumentsRequest {
                    page: 1,
                    page_size: 20,
                    group_pid: Some(group.pid.clone()),
                    status: None,
                },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.list_documents(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod update_document {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_filename(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("upd-name", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::UpdateDocumentRequest {
                    pid: doc.pid.clone(),
                    filename: Some("renamed.pdf".to_string()),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.update_document(request).await.unwrap().into_inner();
            let updated = response.document.expect("should have document");
            assert_eq!(updated.filename, "renamed.pdf");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_filename(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("upd-empty", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::UpdateDocumentRequest {
                    pid: doc.pid.clone(),
                    filename: Some("".to_string()),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.update_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_other_user(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("upd-deny", "user_owner", "org_a").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::UpdateDocumentRequest {
                    pid: doc.pid.clone(),
                    filename: Some("hijacked.txt".to_string()),
                },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.update_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod delete_document {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_delete_document(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("del-ok", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::DeleteDocumentRequest { pid: doc.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.delete_document(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            // Verify it's gone
            let get_request = authenticated_request(
                v1::GetDocumentRequest { pid: doc.pid.clone() },
                "user_test",
                Some("org_test"),
            );
            let get_response = service.get_document(get_request).await;
            assert!(get_response.is_err());
            assert_eq!(get_response.unwrap_err().code(), tonic::Code::NotFound);

            // Remove from cleanup tracking since already deleted
            ctx.created_documents.retain(|&id| id != doc.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_continue_if_storage_delete_fails(ctx: &mut TestContext) {
            // Document exists in DB but has no corresponding MinIO object — delete should still succeed
            let doc = ctx
                .create_ready_document("del-nostorage", "user_test", "org_test")
                .await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::DeleteDocumentRequest { pid: doc.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.delete_document(request).await;
            assert!(
                response.is_ok(),
                "should succeed even when storage object doesn't exist"
            );

            ctx.created_documents.retain(|&id| id != doc.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_other_user(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("del-deny", "user_owner", "org_a").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::DeleteDocumentRequest { pid: doc.pid.clone() },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.delete_document(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod get_download_url {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_download_url_for_owner(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("dl-owner", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDownloadUrlRequest { pid: doc.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_download_url(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let url = response.unwrap().into_inner().download_url;
            assert!(!url.is_empty(), "should return non-empty download URL");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_other_user(ctx: &mut TestContext) {
            let doc = ctx.create_ready_document("dl-deny", "user_owner", "org_a").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetDownloadUrlRequest { pid: doc.pid.clone() },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.get_download_url(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod ingestion_pipeline {
        use crate::context::{DOCUMENT_CHUNKS_COLLECTION, DOCUMENT_METADATA_COLLECTION};

        use super::*;
        use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};

        /// Poll document status until it leaves `processing`, with timeout.
        async fn poll_until_terminal(ctx: &TestContext, pid: &str) -> documents::Model {
            let timeout = std::time::Duration::from_secs(30);
            let start = std::time::Instant::now();

            loop {
                let doc = documents::Entity::find()
                    .filter(documents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .expect("DB query failed")
                    .expect("document should exist");

                let status = doc.status();
                if status == documents::DocStatus::Ready || status == documents::DocStatus::Failed {
                    return doc;
                }

                assert!(
                    start.elapsed() < timeout,
                    "ingestion timed out after {timeout:?} — status is still {status}"
                );

                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        }

        /// Build a raw Qdrant client for verifying points.
        fn qdrant_client() -> qdrant_client::Qdrant {
            let url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
            qdrant_client::Qdrant::from_url(&url)
                .build()
                .expect("Failed to create test Qdrant client")
        }

        /// Scroll all points matching a document_pid filter in a collection.
        async fn scroll_document_points(
            client: &qdrant_client::Qdrant,
            collection: &str,
            doc_pid: &str,
        ) -> Vec<qdrant_client::qdrant::RetrievedPoint> {
            let filter = Filter::must([Condition::matches("document_pid", doc_pid.to_string())]);

            let result = client
                .scroll(
                    ScrollPointsBuilder::new(collection)
                        .filter(filter)
                        .with_payload(true)
                        .limit(100),
                )
                .await
                .expect("Failed to scroll Qdrant points");

            result.result
        }

        /// Helper: upload a document via the service and wait for ingestion to reach terminal state.
        async fn ingest_document(
            ctx: &mut TestContext,
            suffix: &str,
            content: &'static [u8],
        ) -> (documents::Model, DocumentService) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::UploadDocumentRequest {
                    filename: format!("test-{suffix}.txt"),
                    content_type: "text/plain".to_string(),
                    content: content.into(),
                    conversation_pid: None,
                },
                "user_ingest",
                Some("org_ingest"),
            );

            let response = service.upload_document(request).await;
            assert!(response.is_ok(), "upload_document should succeed: {:?}", response.err());

            let doc = response.unwrap().into_inner().document.expect("should have document");
            let final_doc = poll_until_terminal(ctx, &doc.pid).await;

            (final_doc, service)
        }

        /// Cleanup helper: remove Qdrant points for a document.
        async fn cleanup_qdrant_points(doc_pid: &str) {
            let client = qdrant_client();
            let filter = Filter::must([Condition::matches("document_pid", doc_pid.to_string())]);

            for collection in [DOCUMENT_METADATA_COLLECTION, DOCUMENT_CHUNKS_COLLECTION] {
                let _ = client
                    .delete_points(qdrant_client::qdrant::DeletePointsBuilder::new(collection).points(filter.clone()))
                    .await;
            }
        }

        /// Cleanup helper: remove file from storage.
        async fn cleanup_storage(storage_path: &str) {
            let endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set");
            let access_key = std::env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set");
            let secret_key = std::env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set");
            let bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET must be set");
            let region = std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());

            let credentials = aws_sdk_s3::config::Credentials::new(&access_key, &secret_key, None, None, "env");
            let s3_config = aws_sdk_s3::Config::builder()
                .endpoint_url(&endpoint)
                .region(aws_sdk_s3::config::Region::new(region))
                .credentials_provider(credentials)
                .force_path_style(true)
                .behavior_version_latest()
                .build();

            let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
            let _ = s3_client.delete_object().bucket(&bucket).key(storage_path).send().await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_transition_to_ready_after_ingestion(ctx: &mut TestContext) {
            let content = b"This is a test document with enough text to be processed by the ingestion pipeline.";
            let (doc, _service) = ingest_document(ctx, "ingest-ready", content).await;

            assert_eq!(
                doc.status(),
                documents::DocStatus::Ready,
                "document should reach ready status"
            );

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_populate_summary_after_ingestion(ctx: &mut TestContext) {
            let content = b"This is a detailed test document about software architecture and design patterns. It contains enough text for the ingestion pipeline to generate a meaningful summary.";
            let (doc, _service) = ingest_document(ctx, "ingest-summary", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Ready);
            assert!(doc.summary.is_some(), "summary should be populated after ingestion");
            assert!(!doc.summary.as_ref().unwrap().is_empty(), "summary should not be empty");

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_qdrant_metadata_point(ctx: &mut TestContext) {
            let content = b"Metadata point test document with sufficient content for processing.";
            let (doc, _service) = ingest_document(ctx, "ingest-meta", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Ready);

            let client = qdrant_client();
            let points = scroll_document_points(&client, DOCUMENT_METADATA_COLLECTION, &doc.pid).await;

            assert_eq!(points.len(), 1, "should have exactly 1 metadata point");

            let payload = &points[0].payload;
            assert_eq!(
                payload.get("document_pid").and_then(|v| v.as_str()),
                Some(&doc.pid),
                "metadata point should have correct document_pid"
            );

            assert_eq!(
                payload.get("user_id").and_then(|v| v.as_str()).map(String::as_str),
                Some("user_ingest"),
                "metadata point should have correct user_id"
            );

            assert_eq!(
                payload
                    .get("organization_pid")
                    .and_then(|v| v.as_str())
                    .map(String::as_str),
                Some("org_ingest"),
                "metadata point should have correct organization_pid"
            );

            assert!(
                payload.get("filename").and_then(|v| v.as_str()).is_some(),
                "metadata point should have filename"
            );

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_qdrant_chunk_points(ctx: &mut TestContext) {
            let content = b"Chunk test document. This content will be processed by the ingestion pipeline and should produce at least one chunk point in the document_chunks Qdrant collection.";
            let (doc, _service) = ingest_document(ctx, "ingest-chunks", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Ready);

            let client = qdrant_client();
            let points = scroll_document_points(&client, DOCUMENT_CHUNKS_COLLECTION, &doc.pid).await;

            assert!(!points.is_empty(), "should have at least 1 chunk point");

            for point in &points {
                let payload = &point.payload;
                assert!(
                    payload
                        .get("text")
                        .and_then(|v| v.as_str())
                        .is_some_and(|t| !t.is_empty()),
                    "each chunk point should have non-empty text payload"
                );
                assert_eq!(
                    payload.get("document_pid").and_then(|v| v.as_str()),
                    Some(&doc.pid),
                    "chunk point should have correct document_pid"
                );
            }

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_on_empty_content(ctx: &mut TestContext) {
            let content = b"";
            let (doc, _service) = ingest_document(ctx, "ingest-empty", content).await;

            assert_eq!(
                doc.status(),
                documents::DocStatus::Failed,
                "empty content should result in failed status"
            );
            assert!(doc.error_message.is_some(), "failed doc should have error_message");
            assert!(
                doc.error_message.as_ref().unwrap().to_lowercase().contains("empty"),
                "error_message should mention empty: {:?}",
                doc.error_message
            );

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_cleanup_qdrant_on_failure(ctx: &mut TestContext) {
            let content = b"";
            let (doc, _service) = ingest_document(ctx, "ingest-cleanup", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Failed);

            let client = qdrant_client();
            let meta_points = scroll_document_points(&client, DOCUMENT_METADATA_COLLECTION, &doc.pid).await;
            let chunk_points = scroll_document_points(&client, DOCUMENT_CHUNKS_COLLECTION, &doc.pid).await;

            assert!(
                meta_points.is_empty(),
                "no orphaned metadata points should exist after failure"
            );
            assert!(
                chunk_points.is_empty(),
                "no orphaned chunk points should exist after failure"
            );

            cleanup_storage(&doc.storage_path).await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_set_error_message_on_failure(ctx: &mut TestContext) {
            let content = b"";
            let (doc, _service) = ingest_document(ctx, "ingest-errmsg", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Failed);
            assert!(doc.error_message.is_some(), "failed doc should have error_message");
            assert!(
                !doc.error_message.as_ref().unwrap().is_empty(),
                "error_message should not be empty"
            );
        }

        /// Full roundtrip test: ingest a document, then search for it using
        /// the QdrantClient wrapper's production search methods.
        ///
        /// This catches payload key mismatches between ingestion (write side)
        /// and build_search_filter (read side). If the filter keys don't match
        /// the payload keys, searches silently return zero results.
        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_find_ingested_document_via_search_metadata(ctx: &mut TestContext) {
            let content = b"Roundtrip integration test document about quantum computing and neural networks.";
            let (doc, _service) = ingest_document(ctx, "roundtrip-meta", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Ready);

            // Use the real embedding model to generate a query vector
            let embedding_model = get_embedding_model().await;
            let query_vec = embedding_model
                .embed("quantum computing neural networks")
                .await
                .expect("query embedding should succeed");

            // Search via the QdrantClient wrapper — uses build_search_filter
            let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
            let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
                .build()
                .expect("test qdrant client");
            let qdrant = QdrantClient::new(qdrant_inner);

            let results = qdrant
                .search_document_metadata(query_vec, Some("org_ingest"), Some(&[doc.pid.clone()]), 10)
                .await
                .expect("search_document_metadata should succeed");

            assert!(
                !results.is_empty(),
                "search_document_metadata should find the ingested document. \
                 Empty results likely means build_search_filter uses different payload keys \
                 than ingestion writes (e.g. 'org_id' vs 'organization_pid')."
            );

            // Verify the result is actually our document
            let found_pid = results[0]
                .payload
                .get("document_pid")
                .and_then(|v| v.as_str())
                .map(String::as_str);

            assert_eq!(
                found_pid,
                Some(doc.pid.as_str()),
                "returned document should match the ingested document's pid"
            );

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }

        /// Full roundtrip: ingest → search_document_chunks via QdrantClient wrapper.
        /// Verifies the chunk search filter keys match ingestion payload keys.
        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_find_ingested_chunks_via_search_chunks(ctx: &mut TestContext) {
            let content =
                b"Roundtrip chunk test document about distributed systems and consensus algorithms for testing search.";
            let (doc, _service) = ingest_document(ctx, "roundtrip-chunks", content).await;

            assert_eq!(doc.status(), documents::DocStatus::Ready);

            let embedding_model = get_embedding_model().await;
            let query_vec = embedding_model
                .embed("distributed systems consensus algorithms")
                .await
                .expect("query embedding should succeed");

            let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
            let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
                .build()
                .expect("test qdrant client");
            let qdrant = QdrantClient::new(qdrant_inner);

            let results = qdrant
                .search_document_chunks(query_vec, Some("org_ingest"), Some(&[doc.pid.clone()]), 20)
                .await
                .expect("search_document_chunks should succeed");

            assert!(
                !results.is_empty(),
                "search_document_chunks should find chunks from the ingested document. \
                 Empty results likely means build_search_filter uses different payload keys \
                 than ingestion writes (e.g. 'document_id' vs 'document_pid')."
            );

            // Verify chunks belong to our document
            for point in &results {
                let chunk_doc_pid = point
                    .payload
                    .get("document_pid")
                    .and_then(|v| v.as_str())
                    .map(String::as_str);

                assert_eq!(
                    chunk_doc_pid,
                    Some(doc.pid.as_str()),
                    "all returned chunks should belong to the ingested document"
                );

                assert!(
                    point
                        .payload
                        .get("text")
                        .and_then(|v| v.as_str())
                        .is_some_and(|t| !t.is_empty()),
                    "each chunk should have non-empty text"
                );
            }

            cleanup_qdrant_points(&doc.pid).await;
            cleanup_storage(&doc.storage_path).await;
        }
    }
}
