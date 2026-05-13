use crate::{
    AppContext,
    actors::{CancelInference, GetOrCreateSession, GetSession, SessionRegistryActor, StreamMessage},
    api::{
        PostgresStore,
        embedding::{DefaultEmbedding, EmbeddingModel},
        message::Message,
    },
    consts::REDIS_USER_CACHE_KEY,
    models::{agents, conversation_documents, conversation_users, conversations, documents, messages},
};
use kameo::actor::ActorRef;
use platform_rs::{
    cache::{ResourceType, UserCache},
    ext::RequestAuthExt,
};
use proto_rs::rig::v1;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, Condition, DbBackend, EntityTrait as _,
    FromQueryResult as _, IntoActiveModel as _, JoinType, PaginatorTrait as _, QueryFilter as _, QueryOrder as _,
    QuerySelect as _, RelationTrait as _, Statement,
};
use std::{collections::HashMap, sync::Arc};
use tokio_stream::StreamExt as _;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;
const MESSAGES_PER_CONVERSATION: i64 = 5;
const MAX_CHANNEL_MESSAGES: usize = 32;

#[derive(Debug, Clone)]
pub(crate) struct InferenceService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
    registry: ActorRef<SessionRegistryActor<PostgresStore, EM>>,
}

impl<EM> InferenceService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(ctx: Arc<AppContext<EM>>, registry: ActorRef<SessionRegistryActor<PostgresStore, EM>>) -> Self {
        Self { ctx, registry }
    }
}

#[tonic::async_trait]
impl<EM> v1::inference_service_server::InferenceService for InferenceService<EM>
where
    EM: EmbeddingModel,
{
    #[doc = " Server streaming response type for the Infer method."]
    type InferStream = tokio_stream::wrappers::ReceiverStream<Result<v1::InferResponse, tonic::Status>>;

    #[instrument(skip(self, request), fields(user_id, organization_pid, request_id))]
    async fn infer(
        &self,
        request: tonic::Request<v1::InferRequest>,
    ) -> std::result::Result<tonic::Response<Self::InferStream>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;

        // Type-level check: can this role execute conversations (send messages)?
        // Instance-level share checks happen in the actor layer (load_conversation_context).
        if !request
            .org_permissions()
            .map(|p| p.can_execute(ResourceType::Conversations))
            .unwrap_or(true)
        {
            return Err(tonic::Status::permission_denied(
                "Insufficient permissions for inference",
            ));
        }

        let request = request.into_inner();
        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_id", &organization_pid);
        tracing::Span::current().record("request_id", &request.request_id);

        tracing::debug!(
            agent_pid = %request.agent_pid,
            conversation_pid = ?request.conversation_pid,
            message_len = request.message.len(),
            "infer: starting streaming inference"
        );

        tracing::trace!("creating conversation with SessionRegistryActor");
        let (actor, _conversation_pid) = self
            .registry
            .ask(GetOrCreateSession {
                request: request.clone(),
                organization_pid,
                user_id,
            })
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to send request to SessionRegistryActor");
                tonic::Status::internal("Endpoint error: infer")
            })?;

        let cancel_token = CancellationToken::new();
        let request_id = request.request_id.clone();

        let (tx, rx) = tokio::sync::mpsc::channel(MAX_CHANNEL_MESSAGES);

        tracing::debug!("infer: channel created, capacity={}", MAX_CHANNEL_MESSAGES);

        tracing::trace!("sending user message to ChatSessionActor");
        actor
            .tell(StreamMessage {
                request_id: request_id.clone(),
                content: request.message,
                response_tx: tx,
                cancel_token,
                document_pids: request.document_pids,
            })
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to send request to ChatSessionActor");
                tonic::Status::internal("Endpoint error: infer")
            })?;

        tracing::debug!(request_id = %request_id, "infer: returning stream to client");
        Ok(tonic::Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok()), request_id
    )]
    async fn infer_sync(
        &self,
        request: tonic::Request<v1::InferRequest>,
    ) -> std::result::Result<tonic::Response<v1::InferSyncResponse>, tonic::Status> {
        let request_id = request.get_ref().request_id.clone();
        tracing::Span::current().record("request_id", &request_id);

        let mut stream = self.infer(request).await?.into_inner();

        while let Some(result) = stream.next().await {
            let response = result?;
            if let Some(v1::infer_response::Event::Complete(complete)) = response.event {
                let message = complete
                    .message
                    .ok_or_else(|| tonic::Status::internal("InferenceComplete missing message"))?;
                return Ok(tonic::Response::new(v1::InferSyncResponse {
                    request_id,
                    conversation_pid: complete.conversation_pid,
                    message_pid: message.pid.clone(),
                    message: Some(message),
                    metadata: complete.metadata,
                }));
            }
        }

        Err(tonic::Status::internal("Stream ended without InferenceComplete"))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok(),
            request_id,
            conversation_pid
        ),
    )]
    async fn cancel_inference(
        &self,
        request: tonic::Request<v1::CancelInferenceRequest>,
    ) -> std::result::Result<tonic::Response<v1::CancelInferenceResponse>, tonic::Status> {
        let req = request.into_inner();
        tracing::Span::current().record("request_id", &req.request_id);
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        tracing::info!(
            request_id = %req.request_id,
            conversation_pid = %req.conversation_pid,
            "cancel_inference: received cancellation request"
        );

        let session = self
            .registry
            .ask(GetSession {
                conversation_pid: req.conversation_pid.clone(),
                request_id: Some(req.request_id.clone()),
            })
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "cancel_inference: failed to query session");
                tonic::Status::internal("Failed to query session registry")
            })?;

        let cancelled = match session {
            Some(session) => {
                tracing::info!(
                    request_id = %req.request_id,
                    "cancel_inference: found session, routing cancellation"
                );
                session
                    .ask(CancelInference {
                        request_id: req.request_id.clone(),
                    })
                    .await
                    .unwrap_or(false)
            }
            None => {
                tracing::warn!(
                    request_id = %req.request_id,
                    "cancel_inference: session not found for request"
                );
                false
            }
        };

        tracing::info!(
            request_id = %req.request_id,
            cancelled = cancelled,
            "cancel_inference: completed"
        );

        Ok(tonic::Response::new(v1::CancelInferenceResponse { cancelled }))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok()
        )
    )]
    async fn list_conversations(
        &self,
        request: tonic::Request<v1::ListConversationsRequest>,
    ) -> std::result::Result<tonic::Response<v1::ListConversationsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();

        let page = (req.page.max(1)) as u64;
        let page_size = if req.page_size == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            (req.page_size as u64).clamp(1, MAX_PAGE_SIZE)
        };

        let mut query = conversations::Entity::find()
            .filter(conversations::Column::IsDeleted.eq(false))
            .filter(conversations::Column::OrganizationPid.eq(&organization_pid))
            .join(JoinType::LeftJoin, conversations::Relation::ConversationUsers.def())
            .filter(
                Condition::any()
                    .add(conversations::Column::UserId.eq(&user_id))
                    .add(conversation_users::Column::UserId.eq(&user_id)),
            );

        if let Some(agent_pid) = &req.agent_pid {
            let agent = agents::Entity::find()
                .filter(agents::Column::Pid.eq(agent_pid))
                .one(&self.ctx.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to find agent");
                    tonic::Status::internal("Failed to filter by agent")
                })?
                .ok_or_else(|| tonic::Status::not_found("Agent not found"))?;

            query = query.filter(conversations::Column::AgentId.eq(agent.id));
        }

        let total = query.clone().count(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to count conversations");
            tonic::Status::internal("Failed to count conversations")
        })? as i32;

        let results: Vec<(conversations::Model, Option<agents::Model>)> = query
            .find_also_related(agents::Entity)
            .order_by_desc(conversations::Column::UpdatedAt)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch conversations");
                tonic::Status::internal("Failed to fetch conversations")
            })?;

        let conversation_ids: Vec<i64> = results.iter().map(|(c, _)| c.id).collect();
        let all_messages: Vec<messages::Model> = if conversation_ids.is_empty() {
            vec![]
        } else {
            messages::Model::find_by_statement(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                    SELECT id, pid, conversation_id, role, content, parts, status, is_deleted, created_at
                    FROM (
                        SELECT m.*, ROW_NUMBER() OVER (
                            PARTITION BY conversation_id
                            ORDER BY created_at DESC
                        ) as rn
                        FROM messages m
                        WHERE conversation_id = ANY($1) AND is_deleted = false
                    ) sub
                    WHERE rn <= $2
                "#,
                vec![conversation_ids.into(), MESSAGES_PER_CONVERSATION.into()],
            ))
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch messages");
                tonic::Status::internal("Failed to fetch messages")
            })?
        };

        let mut conversation_messages: HashMap<i64, Vec<v1::Message>> = HashMap::new();
        for msg in all_messages {
            let conv_id = msg.conversation_id;
            let msg: Message = msg.into();
            conversation_messages.entry(conv_id).or_default().push(msg.into());
        }

        let conversations = results
            .into_iter()
            .map(|(conv, agent)| {
                let agent = agent.ok_or_else(|| {
                    tracing::error!(conversation_pid = %conv.pid, "conversation missing agent");
                    tonic::Status::internal("Conversation missing agent")
                })?;
                let msgs = conversation_messages.remove(&conv.id).unwrap_or_default();
                Ok(v1::Conversation::from(conversations::Entity::assemble_conversation(
                    conv, agent.pid, msgs,
                )))
            })
            .collect::<Result<Vec<_>, tonic::Status>>()?;

        Ok(tonic::Response::new(v1::ListConversationsResponse {
            conversations,
            total,
            page: page as i32,
            page_size: page_size as i32,
        }))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok(),
            conversation_pid
        )
    )]
    async fn get_conversation(
        &self,
        request: tonic::Request<v1::GetConversationRequest>,
    ) -> std::result::Result<tonic::Response<v1::GetConversationResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("conversation_pid", &req.pid);

        let conversation = conversations::Entity::find_conversation_by_pid(&self.ctx.db, &req.pid, &user_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch conversation");
                match e {
                    conversations::ConversationError::NotFound => tonic::Status::not_found("Conversation not found"),
                    conversations::ConversationError::MissingAgent => {
                        tonic::Status::internal("Conversation missing agent")
                    }
                    conversations::ConversationError::Query(_) => {
                        tonic::Status::internal("Failed to fetch conversation")
                    }
                }
            })?;

        Ok(tonic::Response::new(v1::GetConversationResponse {
            conversation: Some(v1::Conversation::from(conversation)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok()
        )
    )]
    async fn create_conversation(
        &self,
        request: tonic::Request<v1::CreateConversationRequest>,
    ) -> std::result::Result<tonic::Response<v1::CreateConversationResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;

        if !request
            .org_permissions()
            .map(|p| p.can_write(ResourceType::Conversations))
            .unwrap_or(true)
        {
            return Err(tonic::Status::permission_denied(
                "Insufficient permissions to create conversations",
            ));
        }

        let req = request.into_inner();

        let agent = agents::Entity::find()
            .filter(agents::Column::Pid.eq(&req.agent_pid))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to find agent");
                tonic::Status::internal("Failed to find agent")
            })?
            .ok_or_else(|| tonic::Status::not_found("Agent not found"))?;

        let title = req.title.unwrap_or(format!("Chat with {}", agent.name));
        let model = conversations::ActiveModel {
            user_id: Set(user_id),
            organization_pid: Set(organization_pid),
            agent_id: Set(agent.id),
            title: Set(Some(title)),
            ..Default::default()
        };

        let conversation = model.insert(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to create conversation");
            tonic::Status::internal("Failed to create conversation")
        })?;

        Ok(tonic::Response::new(v1::CreateConversationResponse {
            conversation: Some(v1::Conversation::from(conversations::Entity::assemble_conversation(
                conversation,
                agent.pid,
                vec![],
            ))),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok()
        )
    )]
    async fn delete_conversation(
        &self,
        request: tonic::Request<v1::DeleteConversationRequest>,
    ) -> std::result::Result<tonic::Response<v1::DeleteConversationResponse>, tonic::Status> {
        let user_id = request.user_id()?;

        if !request
            .org_permissions()
            .map(|p| p.can_write(ResourceType::Conversations))
            .unwrap_or(true)
        {
            return Err(tonic::Status::permission_denied(
                "Insufficient permissions to delete conversations",
            ));
        }

        let req = request.into_inner();

        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .filter(conversations::Column::UserId.eq(&user_id))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to find conversation");
                tonic::Status::internal("Failed to find conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        let mut active: conversations::ActiveModel = conversation.into_active_model();
        active.is_deleted = Set(true); // Soft-delete
        active.update(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to delete conversation");
            tonic::Status::internal("Failed to delete conversation")
        })?;

        Ok(tonic::Response::new(v1::DeleteConversationResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, conversation_pid))]
    async fn attach_documents(
        &self,
        request: tonic::Request<v1::AttachDocumentsRequest>,
    ) -> std::result::Result<tonic::Response<v1::AttachDocumentsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.conversation_pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .filter(conversations::Column::UserId.eq(&user_id))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to find conversation");
                tonic::Status::internal("Failed to find conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        // Resolve document pids to models, verifying access
        let docs = documents::Entity::find()
            .filter(documents::Column::Pid.is_in(&req.document_pids))
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch documents");
                tonic::Status::internal("Failed to fetch documents")
            })?;

        let mut attached_count = 0i32;
        for doc in &docs {
            if !doc.is_accessible(&user_id, &organization_pid) {
                continue;
            }

            // ON CONFLICT DO NOTHING for idempotency
            let result = conversation_documents::Entity::insert(conversation_documents::ActiveModel {
                conversation_id: Set(conversation.id),
                document_id: Set(doc.id),
                ..Default::default()
            })
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([
                    conversation_documents::Column::ConversationId,
                    conversation_documents::Column::DocumentId,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec_without_returning(&self.ctx.db)
            .await;

            match result {
                Ok(_) => {
                    attached_count += 1;

                    // Best-effort: sync Qdrant conversation_pids from DB source of truth
                    match conversations::Entity::get_conversation_pids_for_document(&self.ctx.db, &doc.pid).await {
                        Ok(conv_pids) => {
                            if let Err(e) = self.ctx.qdrant.set_conversation_pids(&doc.pid, conv_pids).await {
                                tracing::warn!(error = %e, doc_pid = %doc.pid, "failed to sync conversation_pids to qdrant");
                            }
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, doc_pid = %doc.pid, "failed to query conversation_pids for qdrant sync");
                        }
                    }
                }
                Err(e) => tracing::warn!(error = %e, doc_pid = %doc.pid, "failed to attach document"),
            }
        }

        Ok(tonic::Response::new(v1::AttachDocumentsResponse { attached_count }))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok(),
            conversation_pid
        )
    )]
    async fn detach_documents(
        &self,
        request: tonic::Request<v1::DetachDocumentsRequest>,
    ) -> std::result::Result<tonic::Response<v1::DetachDocumentsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.conversation_pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .filter(conversations::Column::UserId.eq(&user_id))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to find conversation");
                tonic::Status::internal("Failed to find conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        // Resolve document pids to IDs
        let doc_ids: Vec<i64> = documents::Entity::find()
            .select_only()
            .column(documents::Column::Id)
            .filter(documents::Column::Pid.is_in(&req.document_pids))
            .into_tuple()
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to resolve document pids");
                tonic::Status::internal("Failed to resolve documents")
            })?;

        if doc_ids.is_empty() {
            return Ok(tonic::Response::new(v1::DetachDocumentsResponse { detached_count: 0 }));
        }

        let result = conversation_documents::Entity::delete_many()
            .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
            .filter(conversation_documents::Column::DocumentId.is_in(doc_ids))
            .exec(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to detach documents");
                tonic::Status::internal("Failed to detach documents")
            })?;

        // Best-effort: sync Qdrant conversation_pids for each detached document
        for doc_pid in &req.document_pids {
            match conversations::Entity::get_conversation_pids_for_document(&self.ctx.db, doc_pid).await {
                Ok(remaining_pids) => {
                    if let Err(e) = self.ctx.qdrant.set_conversation_pids(doc_pid, remaining_pids).await {
                        tracing::warn!(error = %e, doc_pid = %doc_pid, "failed to sync conversation_pids to qdrant after detach");
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, doc_pid = %doc_pid, "failed to query conversation_pids for qdrant sync after detach");
                }
            }
        }

        Ok(tonic::Response::new(v1::DetachDocumentsResponse {
            detached_count: result.rows_affected as i32,
        }))
    }

    #[instrument(
        skip(self, request),
        fields(
            user_id = request.user_id()?,
            organization_pid = ?request.organization_pid().ok(),
            conversation_pid
        )
    )]
    async fn list_conversation_documents(
        &self,
        request: tonic::Request<v1::ListConversationDocumentsRequest>,
    ) -> std::result::Result<tonic::Response<v1::ListConversationDocumentsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.conversation_pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .filter(conversations::Column::UserId.eq(&user_id))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to find conversation");
                tonic::Status::internal("Failed to find conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        let docs = documents::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                documents::Relation::ConversationDocuments.def(),
            )
            .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
            .order_by_desc(conversation_documents::Column::CreatedAt)
            .all(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to list conversation documents");
                tonic::Status::internal("Failed to list conversation documents")
            })?;

        Ok(tonic::Response::new(v1::ListConversationDocumentsResponse {
            documents: docs.into_iter().map(|d| d.into_proto()).collect(),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, conversation_pid))]
    async fn share_conversation(
        &self,
        request: tonic::Request<v1::ShareConversationRequest>,
    ) -> std::result::Result<tonic::Response<v1::ShareConversationResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        // Find conversation and verify ownership
        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.conversation_pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch conversation");
                tonic::Status::internal("Failed to fetch conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        // Only the creator can share
        if conversation.user_id != user_id {
            return Err(tonic::Status::permission_denied(
                "Only the conversation owner can share",
            ));
        }

        // Verify recipient is a member of the conversation's org
        let cache = UserCache::new(self.ctx.redis.clone(), REDIS_USER_CACHE_KEY);
        let recipient_data = cache
            .get(&req.user_id)
            .await
            .ok_or_else(|| tonic::Status::failed_precondition("Recipient user not found"))?;
        if recipient_data.org(&conversation.organization_pid).is_none() {
            return Err(tonic::Status::failed_precondition(
                "Recipient is not a member of the conversation's organization",
            ));
        }

        let permissions = conversation_users::Permissions::from(req.permissions as i16);

        // Upsert share record
        let existing = conversation_users::Entity::find()
            .filter(conversation_users::Column::ConversationId.eq(conversation.id))
            .filter(conversation_users::Column::UserId.eq(&req.user_id))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to check existing share");
                tonic::Status::internal("Failed to share conversation")
            })?;

        if let Some(existing) = existing {
            existing
                .into_active_model()
                .into_ex()
                .set_permissions(permissions.value())
                .update(&self.ctx.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to update share");
                    tonic::Status::internal("Failed to update share")
                })?;
        } else {
            conversation_users::ActiveModelEx::new()
                .set_conversation_id(conversation.id)
                .set_user_id(&req.user_id)
                .set_permissions(permissions.value())
                .set_shared_by(&user_id)
                .insert(&self.ctx.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to create share");
                    tonic::Status::internal("Failed to share conversation")
                })?;
        }

        tracing::info!(shared_with = %req.user_id, permissions = %permissions, "Conversation shared");
        Ok(tonic::Response::new(v1::ShareConversationResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id, conversation_pid))]
    async fn unshare_conversation(
        &self,
        request: tonic::Request<v1::UnshareConversationRequest>,
    ) -> std::result::Result<tonic::Response<v1::UnshareConversationResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.conversation_pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch conversation");
                tonic::Status::internal("Failed to fetch conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        if conversation.user_id != user_id {
            return Err(tonic::Status::permission_denied(
                "Only the conversation owner can unshare",
            ));
        }

        conversation_users::Entity::delete_many()
            .filter(conversation_users::Column::ConversationId.eq(conversation.id))
            .filter(conversation_users::Column::UserId.eq(&req.user_id))
            .exec(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to unshare conversation");
                tonic::Status::internal("Failed to unshare conversation")
            })?;

        tracing::info!(unshared_from = %req.user_id, "Conversation unshared");
        Ok(tonic::Response::new(v1::UnshareConversationResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id, conversation_pid))]
    async fn list_conversation_shares(
        &self,
        request: tonic::Request<v1::ListConversationSharesRequest>,
    ) -> std::result::Result<tonic::Response<v1::ListConversationSharesResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let auth_token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .unwrap_or_default()
            .to_string();
        let req = request.into_inner();
        tracing::Span::current().record("conversation_pid", &req.conversation_pid);

        let conversation = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(&req.conversation_pid))
            .filter(conversations::Column::IsDeleted.eq(false))
            .one(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch conversation");
                tonic::Status::internal("Failed to fetch conversation")
            })?
            .ok_or_else(|| tonic::Status::not_found("Conversation not found"))?;

        // Must be owner to list shares
        if conversation.user_id != user_id {
            return Err(tonic::Status::not_found("Conversation not found"));
        }

        let page_size = if req.page_size == 0 {
            20
        } else {
            (req.page_size as u64).clamp(1, 100)
        };

        // Total count
        let total = conversation_users::Entity::find()
            .filter(conversation_users::Column::ConversationId.eq(conversation.id))
            .count(&self.ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to count shares");
                tonic::Status::internal("Failed to count shares")
            })? as i32;

        // Cursor pagination: sort by (created_at, id)
        let mut cursor = conversation_users::Entity::find()
            .filter(conversation_users::Column::ConversationId.eq(conversation.id))
            .cursor_by((conversation_users::Column::CreatedAt, conversation_users::Column::Id));

        if let Some(ref cursor_str) = req.cursor {
            let c = conversation_users::ShareCursor::try_from(cursor_str.as_str())?;
            cursor.after(c.into_inner());
        }

        let share_records = cursor.first(page_size).all(&self.ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to list shares");
            tonic::Status::internal("Failed to list shares")
        })?;

        let next_cursor = if share_records.len() == page_size as usize {
            share_records
                .last()
                .map(|last| conversation_users::ShareCursor::new((last.created_at, last.id)).to_string())
        } else {
            None
        };

        // Enrich with user display data from api-service
        let share_user_ids: Vec<String> = share_records.iter().map(|s| s.user_id.clone()).collect();
        let user_map = if !auth_token.is_empty() && !share_records.is_empty() {
            self.ctx
                .api_service
                .get_users(&auth_token, &organization_pid, share_user_ids)
                .await
                .unwrap_or_default()
        } else {
            Default::default()
        };

        let shares = share_records
            .into_iter()
            .map(|s| {
                let user_info = user_map.get(&s.user_id);
                v1::ConversationShare {
                    display_name: user_info.and_then(|u| u.display_name.clone()),
                    email: user_info.map(|u| u.email.clone()).unwrap_or_default(),
                    user_id: s.user_id,
                    permissions: s.permissions as i32,
                    shared_by: s.shared_by,
                    created_at: s.created_at.and_utc().to_rfc3339(),
                }
            })
            .collect();

        Ok(tonic::Response::new(v1::ListConversationSharesResponse {
            shares,
            next_cursor,
            total,
        }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        actors::SessionRegistryActor,
        api::{
            PostgresStore,
            embedding::{BoxFuture, EmbeddingError},
        },
        clients::{QdrantClient, StorageClient},
        test_utils::{TestContext, init_test_crypto},
        tools::ToolRegistry,
    };
    use kameo::actor::Spawn as _;
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::inference_service_server::InferenceService as _;
    use serial_test::serial;
    use std::sync::Arc;
    use test_context::test_context;

    /// Creates an authenticated tonic::Request with user_id in User
    fn authenticated_request<T>(inner: T, user_id: &str, org_pid: Option<&str>) -> tonic::Request<T> {
        authenticated_request_with_token(inner, user_id, org_pid, None)
    }

    /// Creates an authenticated tonic::Request with real tokens for service-to-service calls.
    fn authenticated_request_with_token<T>(
        inner: T,
        user_id: &str,
        org_pid: Option<&str>,
        tokens: Option<&platform_rs::test_utils::TokenPair>,
    ) -> tonic::Request<T> {
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
        if let Some(tp) = tokens {
            request
                .metadata_mut()
                .insert("authorization", format!("Bearer {}", tp.access_token).parse().unwrap());
            if let Some(id_token) = &tp.id_token {
                request.metadata_mut().insert("x-id-token", id_token.parse().unwrap());
            }
        }
        request
    }

    struct MockEmbeddingModel;

    impl EmbeddingModel for MockEmbeddingModel {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.0; 384]) })
        }
        fn dimensions(&self) -> usize {
            384
        }
    }

    fn mock_app_ctx(ctx: &TestContext) -> Arc<AppContext<MockEmbeddingModel>> {
        let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".into());
        let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
            .build()
            .expect("test qdrant client");
        let qdrant = QdrantClient::new(qdrant_inner);

        let s3_config = aws_sdk_s3::Config::builder()
            .endpoint_url("http://localhost:9000")
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new("test", "test", None, None, "test"))
            .force_path_style(true)
            .behavior_version_latest()
            .build();
        let storage = StorageClient::new(aws_sdk_s3::Client::from_conf(s3_config), "test".into());

        Arc::new(AppContext {
            db: (*ctx.db).clone(),
            redis: ctx.redis.clone(),
            qdrant,
            embedding: Arc::new(MockEmbeddingModel),
            storage,
            api_service: crate::clients::ApiServiceClient::new(
                &std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set"),
            )
            .unwrap(),
        })
    }

    /// Creates InferenceService with real registry and store
    async fn create_service(ctx: &TestContext) -> InferenceService<MockEmbeddingModel> {
        let store = Arc::new(PostgresStore::new((*ctx.db).clone()));
        let app_ctx = mock_app_ctx(ctx);
        let registry = SessionRegistryActor::spawn((store, app_ctx.clone(), ToolRegistry::new(app_ctx.clone())));
        InferenceService::new(app_ctx, registry)
    }

    mod infer_sync {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_complete_response(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("infer-sync").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Say hello".to_string(),
                    request_id: "req-sync-1".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer_sync(request).await;
            assert!(response.is_ok(), "infer_sync should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert_eq!(response.request_id, "req-sync-1");
            assert!(!response.conversation_pid.is_empty(), "should have conversation_pid");
            assert!(!response.message_pid.is_empty(), "should have message_pid");
            assert!(response.message.is_some(), "should have message");

            let message = response.message.unwrap();
            assert_eq!(message.role, "assistant");
            assert!(!message.parts.is_empty(), "message should have parts");
            // Check that at least one TEXT part has content
            let has_text_content = message.parts.iter().any(|p| {
                p.kind == i32::from(v1::MessagePartKind::Text)
                    && p.content.as_ref().map(|c| !c.is_empty()).unwrap_or(false)
            });
            assert!(has_text_content, "message should have text content in parts");
        }
    }

    mod infer {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_stream_response_for_new_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("infer-new-conv").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Say hello".to_string(),
                    request_id: "req-1".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(request).await;
            assert!(response.is_ok(), "infer should succeed: {:?}", response.err());

            let mut stream = response.unwrap().into_inner();
            let mut events = Vec::new();
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => events.push(event),
                    Err(e) => panic!("stream error: {:?}", e),
                }
            }

            assert!(!events.is_empty(), "should receive streaming events");

            let last_event = events.last().expect("should have events");
            assert!(
                last_event
                    .event
                    .as_ref()
                    .map(|e| matches!(e, v1::infer_response::Event::Complete(_)))
                    .unwrap_or(false),
                "last event should be InferenceComplete"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_stream_response_for_existing_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("infer-existing-conv").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: Some(conversation.pid.clone()),
                    message: "Continue our chat".to_string(),
                    request_id: "req-2".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(request).await;
            assert!(response.is_ok(), "infer should succeed: {:?}", response.err());

            let mut stream = response.unwrap().into_inner();
            let mut events = Vec::new();
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => events.push(event),
                    Err(e) => panic!("stream error: {:?}", e),
                }
            }

            assert!(!events.is_empty(), "should receive streaming events");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_invalid_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: "nonexistent-agent".to_string(),
                    conversation_pid: None,
                    message: "Hello".to_string(),
                    request_id: "req-3".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(request).await;
            assert!(response.is_err(), "should fail for invalid agent");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_invalid_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("infer-invalid-conv").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: Some("nonexistent-conversation".to_string()),
                    message: "Hello".to_string(),
                    request_id: "req-4".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(request).await;
            assert!(response.is_err(), "should fail for invalid conversation");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_unauthorized_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            // Create conversation owned by other_user
            let provider = ctx
                .create_personal_provider("infer-unauth", "ollama", "other_user")
                .await;
            ctx.create_api_key_secret("infer-unauth", provider.id).await;
            let model = ctx.create_model("infer-unauth", provider.id).await;
            let agent = ctx.create_personal_agent("infer-unauth", model.id, "other_user").await;
            let conversation = ctx
                .create_conversation_for_user("infer-unauth", agent.id, "other_user")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: Some(conversation.pid.clone()),
                    message: "Hello".to_string(),
                    request_id: "req-5".to_string(),
                    document_pids: vec![],
                },
                "user_test", // Different user
                None,
            );

            let response = service.infer(request).await;
            assert!(response.is_err(), "should fail for unauthorized conversation");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_when_unauthenticated(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("infer-unauth-req").await;
            let service = create_service(ctx).await;

            // Request without auth extensions
            let request = tonic::Request::new(v1::InferRequest {
                agent_pid: agent.pid.clone(),
                conversation_pid: None,
                message: "Hello".to_string(),
                request_id: "req-6".to_string(),
                document_pids: vec![],
            });

            let response = service.infer(request).await;
            assert!(response.is_err(), "should fail without authentication");

            let status = response.unwrap_err();
            assert_eq!(
                status.code(),
                tonic::Code::Unauthenticated,
                "should return UNAUTHENTICATED"
            );
        }
    }

    mod cancel_inference {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_cancel_an_active_inference_request(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("cancel-active-inference").await;
            let service = create_service(ctx).await;
            let request_id = "req-cancel-1".to_string();

            let infer_request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Write a very long explanation of distributed systems, consensus, replication, fault tolerance, observability, and recovery patterns in several detailed sections.".to_string(),
                    request_id: request_id.clone(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(infer_request).await;
            assert!(response.is_ok(), "infer should succeed: {:?}", response.err());

            let mut stream = response.unwrap().into_inner();
            let first_event = stream
                .next()
                .await
                .expect("stream should emit at least one event")
                .expect("first stream event should be ok");

            assert!(
                first_event.event.is_some(),
                "first stream event should contain an event"
            );

            let cancel_request = authenticated_request(
                v1::CancelInferenceRequest {
                    request_id: request_id.clone(),
                    conversation_pid: String::new(),
                },
                "user_test",
                Some("org_test"),
            );

            let cancel_response = service.cancel_inference(cancel_request).await;
            assert!(
                cancel_response.is_ok(),
                "cancel_inference should succeed: {:?}",
                cancel_response.err()
            );
            assert!(
                cancel_response.unwrap().into_inner().cancelled,
                "cancel_inference should report the active request as cancelled"
            );

            let mut saw_cancelled = false;
            while let Some(result) = stream.next().await {
                let event = result.expect("stream event should be ok");
                if matches!(event.event, Some(v1::infer_response::Event::Cancelled(_))) {
                    saw_cancelled = true;
                }
            }

            assert!(saw_cancelled, "stream should emit an InferenceCancelled event");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_cancelled_false_after_the_request_has_already_completed(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("cancel-after-complete").await;
            let service = create_service(ctx).await;
            let request_id = "req-cancel-after-complete".to_string();

            let infer_request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Say hello in one short sentence.".to_string(),
                    request_id: request_id.clone(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(infer_request).await;
            assert!(response.is_ok(), "infer should succeed: {:?}", response.err());

            let mut stream = response.unwrap().into_inner();
            while let Some(result) = stream.next().await {
                result.expect("stream event should be ok");
            }

            let request_id = request_id.clone();
            let cancelled = tokio::time::timeout(std::time::Duration::from_secs(2), async {
                loop {
                    let cancel_response = service
                        .cancel_inference(authenticated_request(
                            v1::CancelInferenceRequest {
                                request_id: request_id.clone(),
                                conversation_pid: String::new(),
                            },
                            "user_test",
                            None,
                        ))
                        .await
                        .expect("cancel_inference should succeed")
                        .into_inner();

                    if !cancel_response.cancelled {
                        break false;
                    }

                    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
                }
            })
            .await
            .expect("completed request should eventually stop reporting as cancelled");

            assert!(
                !cancelled,
                "completed request should not report as cancelled once cleanup has completed"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_cancelled_false_for_an_unknown_request_id(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let cancel_response = service
                .cancel_inference(authenticated_request(
                    v1::CancelInferenceRequest {
                        request_id: "req-does-not-exist".to_string(),
                        conversation_pid: String::new(),
                    },
                    "user_test",
                    None,
                ))
                .await;

            assert!(
                cancel_response.is_ok(),
                "cancel_inference should succeed: {:?}",
                cancel_response.err()
            );
            assert!(
                !cancel_response.unwrap().into_inner().cancelled,
                "unknown request id should not report as cancelled"
            );
        }
    }

    mod timeout {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_cancel_stream_when_wall_clock_timeout_exceeded(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_provider("wall-timeout", "ollama").await;
            ctx.create_api_key_secret("wall-timeout", provider.id).await;
            let model = ctx.create_model("wall-timeout", provider.id).await;
            let agent = ctx
                .create_agent_with_config(
                    "wall-timeout",
                    model.id,
                    serde_json::json!({
                        "base": {
                            "timeout_seconds": 1,
                            "idle_timeout_seconds": 300
                        }
                    }),
                )
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    // Long prompt to ensure response generation takes >1s.
                    message: "Write a 2000 word essay about the complete history of computing from Charles Babbage to modern AI. Include detailed descriptions of every major milestone.".to_string(),
                    request_id: "req-wall-timeout".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.infer(request).await;
            assert!(response.is_ok(), "infer should start: {:?}", response.err());

            let mut stream = response.unwrap().into_inner();
            let mut saw_cancelled = false;
            let mut saw_complete = false;

            while let Some(result) = stream.next().await {
                match result {
                    Ok(resp) => match resp.event {
                        Some(v1::infer_response::Event::Cancelled(_)) => saw_cancelled = true,
                        Some(v1::infer_response::Event::Complete(_)) => saw_complete = true,
                        _ => {}
                    },
                    Err(_) => break,
                }
            }

            assert!(
                saw_cancelled && !saw_complete,
                "stream should be cancelled by wall-clock timeout (cancelled={saw_cancelled}, complete={saw_complete})"
            );
        }
    }

    mod list_conversations {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_list_when_no_conversations(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_with_no_convs",
                Some("org_empty"),
            );

            let response = service.list_conversations(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert!(response.conversations.is_empty(), "should have no conversations");
            assert_eq!(response.total, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_conversations_for_user(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _agent, conversation) = ctx.create_full_setup("list-convs").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversations(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert!(!response.conversations.is_empty(), "should have conversations");
            assert!(
                response.conversations.iter().any(|c| c.pid == conversation.pid),
                "should include created conversation"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_agent_pid(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent1, conv1) = ctx.create_full_setup("list-filter-1").await;
            // Create second agent with different conversation
            let provider = ctx
                .create_personal_provider("list-filter-2", "ollama", "user_test")
                .await;
            ctx.create_api_key_secret("list-filter-2", provider.id).await;
            let model = ctx.create_model("list-filter-2", provider.id).await;
            let agent2 = ctx.create_personal_agent("list-filter-2", model.id, "user_test").await;
            let conv2 = ctx.create_conversation("list-filter-2", agent2.id).await;

            let service = create_service(ctx).await;

            // Filter by agent1
            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: Some(agent1.pid.clone()),
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversations(request).await.unwrap().into_inner();
            assert!(
                response.conversations.iter().all(|c| c.agent_pid == agent1.pid),
                "all conversations should be from agent1"
            );
            assert!(
                response.conversations.iter().any(|c| c.pid == conv1.pid),
                "should include conv1"
            );
            assert!(
                !response.conversations.iter().any(|c| c.pid == conv2.pid),
                "should not include conv2"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_exclude_deleted_conversations(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _agent, conversation) = ctx.create_full_setup("list-deleted").await;
            let service = create_service(ctx).await;

            // Delete the conversation
            let delete_request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );
            service.delete_conversation(delete_request).await.unwrap();

            // List should not include deleted conversation
            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversations(request).await.unwrap().into_inner();
            assert!(
                !response.conversations.iter().any(|c| c.pid == conversation.pid),
                "should not include deleted conversation"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_messages_in_response(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _agent, conversation) = ctx.create_full_setup("list-msgs").await;
            // Add some messages
            ctx.create_message("list-msgs-1", conversation.id, "user", "Hello")
                .await;
            ctx.create_message("list-msgs-2", conversation.id, "assistant", "Hi there")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversations(request).await.unwrap().into_inner();
            let conv = response.conversations.iter().find(|c| c.pid == conversation.pid);
            assert!(conv.is_some(), "should find conversation");
            assert!(!conv.unwrap().messages.is_empty(), "should include messages");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_respect_pagination(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("list-page", "ollama", "user_test").await;
            ctx.create_api_key_secret("list-page", provider.id).await;
            let model = ctx.create_model("list-page", provider.id).await;
            let agent = ctx.create_personal_agent("list-page", model.id, "user_test").await;

            // Create 5 conversations
            for i in 0..5 {
                ctx.create_conversation(&format!("list-page-{i}"), agent.id).await;
            }

            let service = create_service(ctx).await;

            // Request page 1 with size 2
            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: Some(agent.pid.clone()),
                    is_active: None,
                    page: 1,
                    page_size: 2,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversations(request).await.unwrap().into_inner();
            assert_eq!(response.conversations.len(), 2, "should return 2 conversations");
            assert_eq!(response.total, 5, "total should be 5");
            assert_eq!(response.page, 1);
            assert_eq!(response.page_size, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_other_users_conversations(ctx: &mut TestContext) {
            init_test_crypto();
            // Create conversation for other_user
            let provider = ctx.create_personal_provider("list-iso", "ollama", "other_user").await;
            ctx.create_api_key_secret("list-iso", provider.id).await;
            let model = ctx.create_model("list-iso", provider.id).await;
            let agent = ctx.create_personal_agent("list-iso", model.id, "other_user").await;
            let other_conv = ctx
                .create_conversation_for_user("list-iso", agent.id, "other_user")
                .await;

            let service = create_service(ctx).await;

            // Request as user_test
            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversations(request).await.unwrap().into_inner();
            assert!(
                !response.conversations.iter().any(|c| c.pid == other_conv.pid),
                "should not include other user's conversation"
            );
        }
    }

    mod get_conversation {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_conversation_with_messages(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("get-conv").await;
            ctx.create_message("get-conv-1", conversation.id, "user", "Hello").await;
            ctx.create_message("get-conv-2", conversation.id, "assistant", "Hi")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_conversation(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let conv = response.unwrap().into_inner().conversation.unwrap();
            assert_eq!(conv.pid, conversation.pid);
            assert_eq!(conv.agent_pid, agent.pid);
            assert_eq!(conv.messages.len(), 2, "should have 2 messages");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: "nonexistent-conv".to_string(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_conversation(request).await;
            assert!(response.is_err(), "should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_deleted_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("get-deleted").await;
            let service = create_service(ctx).await;

            // Delete the conversation
            let delete_request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );
            service.delete_conversation(delete_request).await.unwrap();

            // Try to get it
            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_conversation(request).await;
            assert!(response.is_err(), "should fail for deleted conversation");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            // Create conversation for other_user
            let provider = ctx.create_personal_provider("get-iso", "ollama", "other_user").await;
            ctx.create_api_key_secret("get-iso", provider.id).await;
            let model = ctx.create_model("get-iso", provider.id).await;
            let agent = ctx.create_personal_agent("get-iso", model.id, "other_user").await;
            let other_conv = ctx
                .create_conversation_for_user("get-iso", agent.id, "other_user")
                .await;

            let service = create_service(ctx).await;

            // Try to get as user_test
            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: other_conv.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_conversation(request).await;
            assert!(response.is_err(), "should fail for other user's conversation");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_messages_in_chronological_order(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("get-order").await;

            // Create messages with slight delay to ensure ordering
            ctx.create_message("get-order-1", conversation.id, "user", "First")
                .await;
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            ctx.create_message("get-order-2", conversation.id, "assistant", "Second")
                .await;
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            ctx.create_message("get-order-3", conversation.id, "user", "Third")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_conversation(request).await.unwrap().into_inner();
            let messages = response.conversation.unwrap().messages;

            assert_eq!(messages.len(), 3);

            let text =
                |msg: &v1::Message| -> String { msg.parts.iter().filter_map(|p| p.content.as_deref()).collect() };
            assert!(text(&messages[0]).contains("First"));
            assert!(text(&messages[1]).contains("Second"));
            assert!(text(&messages[2]).contains("Third"));
        }
    }

    mod create_conversation {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_conversation_for_valid_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("create-conv").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::CreateConversationRequest {
                    agent_pid: agent.pid.clone(),
                    title: Some("Test Conversation".to_string()),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.create_conversation(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let conv = response.unwrap().into_inner().conversation.unwrap();
            assert!(!conv.pid.is_empty(), "should have pid");
            assert_eq!(conv.agent_pid, agent.pid);
            assert_eq!(conv.title, Some("Test Conversation".to_string()));
            assert!(conv.messages.is_empty(), "should have no messages");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_use_default_title_when_not_provided(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("create-default").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::CreateConversationRequest {
                    agent_pid: agent.pid.clone(),
                    title: None,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.create_conversation(request).await.unwrap().into_inner();
            let conv = response.conversation.unwrap();
            assert!(
                conv.title.as_ref().map(|t| t.contains("Chat with")).unwrap_or(false),
                "should have default title with agent name"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_invalid_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::CreateConversationRequest {
                    agent_pid: "nonexistent-agent".to_string(),
                    title: None,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.create_conversation(request).await;
            assert!(response.is_err(), "should fail for invalid agent");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_set_correct_user_id(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("create-user").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::CreateConversationRequest {
                    agent_pid: agent.pid.clone(),
                    title: None,
                },
                "specific_user_123",
                Some("org_test"),
            );

            let response = service.create_conversation(request).await.unwrap().into_inner();
            let conv = response.conversation.unwrap();
            assert_eq!(conv.user_id, Some("specific_user_123".to_string()));
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_create_conversation_without_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("create-no-org").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::CreateConversationRequest {
                    agent_pid: agent.pid.clone(),
                    title: None,
                },
                "user_test",
                None, // no org context
            );

            let result = service.create_conversation(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }
    }

    mod infer_org_context {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_infer_without_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("infer-no-org").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Hello".to_string(),
                    request_id: "req-no-org".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                None, // no org context
            );

            let result = service.infer(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }
    }

    mod conversation_org_isolation {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_list_conversations_without_org_context(ctx: &mut TestContext) {
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None, // no org context
            );

            let result = service.list_conversations(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_isolate_conversations_across_orgs(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("conv-iso").await;
            let service = create_service(ctx).await;

            // Create conversation in org_test
            let create_request = authenticated_request(
                v1::CreateConversationRequest {
                    agent_pid: agent.pid.clone(),
                    title: Some("Org Test Conv".to_string()),
                },
                "user_test",
                Some("org_test"),
            );
            let create_response = service.create_conversation(create_request).await.unwrap().into_inner();
            let conv_pid = create_response.conversation.unwrap().pid;

            // List from org_test — should see the conversation
            let list_request = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_test"),
            );
            let list_response = service.list_conversations(list_request).await.unwrap().into_inner();
            assert!(
                list_response.conversations.iter().any(|c| c.pid == conv_pid),
                "should see conversation in org_test"
            );

            // List from org_other — should NOT see the conversation
            let list_other = authenticated_request(
                v1::ListConversationsRequest {
                    agent_pid: None,
                    is_active: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_other"),
            );
            let other_response = service.list_conversations(list_other).await.unwrap().into_inner();
            assert!(
                !other_response.conversations.iter().any(|c| c.pid == conv_pid),
                "should NOT see conversation from org_test when listing from org_other"
            );
        }
    }

    mod delete_conversation {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_soft_delete_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("delete-conv").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.delete_conversation(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            // Verify it's soft-deleted (get should fail)
            let get_request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );
            let get_response = service.get_conversation(get_request).await;
            assert!(get_response.is_err(), "deleted conversation should not be gettable");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: "nonexistent-conv".to_string(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.delete_conversation(request).await;
            assert!(response.is_err(), "should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_already_deleted_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("delete-twice").await;
            let service = create_service(ctx).await;

            // Delete once
            let request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );
            service.delete_conversation(request).await.unwrap();

            // Try to delete again
            let request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );
            let response = service.delete_conversation(request).await;
            assert!(response.is_err(), "should fail for already deleted");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            // Create conversation for other_user
            let provider = ctx.create_personal_provider("delete-iso", "ollama", "other_user").await;
            ctx.create_api_key_secret("delete-iso", provider.id).await;
            let model = ctx.create_model("delete-iso", provider.id).await;
            let agent = ctx.create_personal_agent("delete-iso", model.id, "other_user").await;
            let other_conv = ctx
                .create_conversation_for_user("delete-iso", agent.id, "other_user")
                .await;

            let service = create_service(ctx).await;

            // Try to delete as user_test
            let request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: other_conv.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.delete_conversation(request).await;
            assert!(response.is_err(), "should fail for other user's conversation");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod attach_documents {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_attach_documents_to_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("attach-ok").await;
            let doc = ctx.create_ready_document("attach-ok", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.attach_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert_eq!(response.unwrap().into_inner().attached_count, 1);

            // Verify the join table row exists
            let link = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .filter(conversation_documents::Column::DocumentId.eq(doc.id))
                .one(ctx.db.as_ref())
                .await
                .unwrap();
            assert!(link.is_some(), "join table row should exist");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_be_idempotent_when_attaching_same_document_twice(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("attach-idem").await;
            let doc = ctx.create_ready_document("attach-idem", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            // Attach once
            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.attach_documents(request).await.unwrap();

            // Attach again — same document, same conversation
            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            let response = service.attach_documents(request).await;
            assert!(response.is_ok(), "second attach should not error: {:?}", response.err());

            // Only one row in the join table
            let count = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .filter(conversation_documents::Column::DocumentId.eq(doc.id))
                .count(ctx.db.as_ref())
                .await
                .unwrap();
            assert_eq!(
                count, 1,
                "should have exactly one join table row despite two attach calls"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_inaccessible_documents(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("attach-skip").await;
            // Document belongs to a different org and different user — not accessible
            let doc = ctx
                .create_ready_document("attach-skip", "other_user", "other_org")
                .await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.attach_documents(request).await;
            assert!(
                response.is_ok(),
                "should succeed (silently skips): {:?}",
                response.err()
            );
            assert_eq!(
                response.unwrap().into_inner().attached_count,
                0,
                "inaccessible doc should not be attached"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_when_conversation_not_owned_by_user(ctx: &mut TestContext) {
            init_test_crypto();
            // Create conversation owned by other_user
            let provider = ctx
                .create_personal_provider("attach-deny", "ollama", "other_user")
                .await;
            ctx.create_api_key_secret("attach-deny", provider.id).await;
            let model = ctx.create_model("attach-deny", provider.id).await;
            let agent = ctx.create_personal_agent("attach-deny", model.id, "other_user").await;
            let other_conv = ctx
                .create_conversation_for_user("attach-deny", agent.id, "other_user")
                .await;

            let doc = ctx.create_ready_document("attach-deny", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: other_conv.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.attach_documents(request).await;
            assert!(response.is_err(), "should fail for other user's conversation");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_attach_multiple_documents_at_once(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("attach-multi").await;
            let doc1 = ctx
                .create_ready_document("attach-multi-1", "user_test", "org_test")
                .await;
            let doc2 = ctx
                .create_ready_document("attach-multi-2", "user_test", "org_test")
                .await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc1.pid.clone(), doc2.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.attach_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert_eq!(response.unwrap().into_inner().attached_count, 2);
        }
    }

    mod detach_documents {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_detach_document_from_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("detach-ok").await;
            let doc = ctx.create_ready_document("detach-ok", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            // Attach first
            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.attach_documents(request).await.unwrap();

            // Now detach
            let request = authenticated_request(
                v1::DetachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.detach_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert_eq!(response.unwrap().into_inner().detached_count, 1);

            // Verify join table row is gone
            let link = conversation_documents::Entity::find()
                .filter(conversation_documents::Column::ConversationId.eq(conversation.id))
                .filter(conversation_documents::Column::DocumentId.eq(doc.id))
                .one(ctx.db.as_ref())
                .await
                .unwrap();
            assert!(link.is_none(), "join table row should be removed after detach");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_zero_when_document_was_not_attached(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("detach-noop").await;
            let doc = ctx.create_ready_document("detach-noop", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            // Detach without ever attaching
            let request = authenticated_request(
                v1::DetachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.detach_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert_eq!(response.unwrap().into_inner().detached_count, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_when_conversation_not_owned_by_user(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("detach-deny", "ollama", "other_user")
                .await;
            ctx.create_api_key_secret("detach-deny", provider.id).await;
            let model = ctx.create_model("detach-deny", provider.id).await;
            let agent = ctx.create_personal_agent("detach-deny", model.id, "other_user").await;
            let other_conv = ctx
                .create_conversation_for_user("detach-deny", agent.id, "other_user")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::DetachDocumentsRequest {
                    conversation_pid: other_conv.pid.clone(),
                    document_pids: vec!["any-pid".to_string()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.detach_documents(request).await;
            assert!(response.is_err(), "should fail for other user's conversation");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod list_conversation_documents {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_attached_documents(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("list-docs").await;
            let doc1 = ctx.create_ready_document("list-docs-1", "user_test", "org_test").await;
            let doc2 = ctx.create_ready_document("list-docs-2", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            // Attach both documents
            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc1.pid.clone(), doc2.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.attach_documents(request).await.unwrap();

            // List
            let request = authenticated_request(
                v1::ListConversationDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversation_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let docs = response.unwrap().into_inner().documents;
            assert_eq!(docs.len(), 2, "should return both attached documents");

            let pids: Vec<&str> = docs.iter().map(|d| d.pid.as_str()).collect();
            assert!(pids.contains(&doc1.pid.as_str()), "should contain doc1");
            assert!(pids.contains(&doc2.pid.as_str()), "should contain doc2");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_list_when_no_documents_attached(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("list-empty").await;
            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListConversationDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversation_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert!(
                response.unwrap().into_inner().documents.is_empty(),
                "should return empty list"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_detached_documents(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("list-detached").await;
            let doc = ctx
                .create_ready_document("list-detached", "user_test", "org_test")
                .await;
            let service = create_service(ctx).await;

            // Attach then detach
            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.attach_documents(request).await.unwrap();

            let request = authenticated_request(
                v1::DetachDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.detach_documents(request).await.unwrap();

            // List should be empty
            let request = authenticated_request(
                v1::ListConversationDocumentsRequest {
                    conversation_pid: conversation.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversation_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert!(
                response.unwrap().into_inner().documents.is_empty(),
                "should not return detached documents"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_when_conversation_not_owned_by_user(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("list-deny", "ollama", "other_user").await;
            ctx.create_api_key_secret("list-deny", provider.id).await;
            let model = ctx.create_model("list-deny", provider.id).await;
            let agent = ctx.create_personal_agent("list-deny", model.id, "other_user").await;
            let other_conv = ctx
                .create_conversation_for_user("list-deny", agent.id, "other_user")
                .await;

            let service = create_service(ctx).await;

            let request = authenticated_request(
                v1::ListConversationDocumentsRequest {
                    conversation_pid: other_conv.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversation_documents(request).await;
            assert!(response.is_err(), "should fail for other user's conversation");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_show_documents_from_another_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conv_a) = ctx.create_full_setup("list-iso-a").await;
            let conv_b = ctx.create_conversation("list-iso-b", agent.id).await;
            let doc = ctx.create_ready_document("list-iso", "user_test", "org_test").await;
            let service = create_service(ctx).await;

            // Attach doc to conversation A only
            let request = authenticated_request(
                v1::AttachDocumentsRequest {
                    conversation_pid: conv_a.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.attach_documents(request).await.unwrap();

            // List conversation B — should be empty
            let request = authenticated_request(
                v1::ListConversationDocumentsRequest {
                    conversation_pid: conv_b.pid.clone(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.list_conversation_documents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert!(
                response.unwrap().into_inner().documents.is_empty(),
                "conversation B should not see documents attached to conversation A"
            );
        }
    }

    mod conversation_sharing {
        use super::*;
        use crate::consts::REDIS_USER_CACHE_KEY;
        use platform_rs::cache::{CachedOrg, CachedUserData, OrgRole, PlanTier, UserCache};

        /// Seed the user cache so the org membership check passes at share time.
        async fn seed_recipient_cache(ctx: &TestContext, user_id: &str, org_pid: &str) {
            let cache = UserCache::new(ctx.redis.clone(), REDIS_USER_CACHE_KEY);
            cache
                .set(
                    user_id,
                    &CachedUserData {
                        email: format!("{user_id}@test.com"),
                        organizations: vec![CachedOrg::new(org_pid, PlanTier::Free, OrgRole::Member)],
                    },
                )
                .await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_share_conversation_with_another_user(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Owner shares with read permission
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1, // READ
                },
                "owner_user",
                Some("org_1"),
            );

            let result = service.share_conversation(request).await;
            assert!(result.is_ok(), "owner should share conversation: {:?}", result.err());

            // Verify share appears in list
            let request = authenticated_request_with_token(
                v1::ListConversationSharesRequest {
                    conversation_pid: conversation.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );

            let response = service.list_conversation_shares(request).await.unwrap().into_inner();
            assert_eq!(response.shares.len(), 1);
            assert_eq!(response.shares[0].user_id, "recipient_user");
            assert_eq!(response.shares[0].permissions, 1);
            assert_eq!(response.shares[0].shared_by, "owner_user");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_upsert_permissions_on_reshare(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-upsert").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-upsert-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share with READ
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            // Re-share with RWX
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 7, // RWX
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            let request = authenticated_request_with_token(
                v1::ListConversationSharesRequest {
                    conversation_pid: conversation.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );

            let response = service.list_conversation_shares(request).await.unwrap().into_inner();
            assert_eq!(response.shares.len(), 1, "should have one share, not two");
            assert_eq!(
                response.shares[0].permissions, 7,
                "permissions should be updated to RWX"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_share_by_non_owner(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-deny").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-deny-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;

            // Non-owner tries to share
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "third_user".to_string(),
                    permissions: 1,
                },
                "not_the_owner",
                Some("org_1"),
            );

            let result = service.share_conversation(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_share_with_non_org_member(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-non-member").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-non-member-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;

            // Seed recipient as member of org_2, NOT org_1
            seed_recipient_cache(ctx, "outsider_user", "org_2").await;

            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "outsider_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );

            let result = service.share_conversation(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_unshare_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let (_, _, agent, _) = ctx.create_full_setup("unshare-conv").await;
            let conversation = ctx
                .create_conversation_in_org("unshare-conv-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share first
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 7,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            // Unshare
            let request = authenticated_request(
                v1::UnshareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                },
                "owner_user",
                Some("org_1"),
            );

            let result = service.unshare_conversation(request).await;
            assert!(result.is_ok(), "unshare should succeed: {:?}", result.err());

            // Verify shares list is empty
            let request = authenticated_request_with_token(
                v1::ListConversationSharesRequest {
                    conversation_pid: conversation.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );

            let response = service.list_conversation_shares(request).await.unwrap().into_inner();
            assert!(response.shares.is_empty(), "shares should be empty after unshare");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_shared_user_to_get_conversation_with_read(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-read").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-read-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share with READ
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1, // READ
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            // Recipient gets conversation
            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "recipient_user",
                Some("org_2"),
            );

            let result = service.get_conversation(request).await;
            assert!(
                result.is_ok(),
                "shared user should get conversation: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_unshared_user_from_getting_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-deny-get").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-deny-get-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;

            // No share — random user tries to get
            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "random_user",
                Some("org_2"),
            );

            let result = service.get_conversation(request).await;
            assert!(result.is_err(), "unshared user should not access conversation");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_revoke_access_after_unshare(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-revoke").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-revoke-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            // Verify access works
            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "recipient_user",
                Some("org_2"),
            );
            assert!(
                service.get_conversation(request).await.is_ok(),
                "should have access before unshare"
            );

            // Unshare
            let request = authenticated_request(
                v1::UnshareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                },
                "owner_user",
                Some("org_1"),
            );
            service.unshare_conversation(request).await.unwrap();

            // Verify access revoked
            let request = authenticated_request(
                v1::GetConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "recipient_user",
                Some("org_2"),
            );
            let result = service.get_conversation(request).await;
            assert!(result.is_err(), "access should be revoked after unshare");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_non_owner_from_listing_shares(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-list-deny").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-list-deny-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;

            // Non-owner tries to list shares
            let request = authenticated_request(
                v1::ListConversationSharesRequest {
                    conversation_pid: conversation.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "not_the_owner",
                Some("org_1"),
            );

            let result = service.list_conversation_shares(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_shared_conversation_in_listing(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-list").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-list-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share with READ
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            // Recipient lists conversations in org_1 — shared conversation should appear
            let request = authenticated_request(
                v1::ListConversationsRequest {
                    page: 1,
                    page_size: 50,
                    agent_pid: None,
                    is_active: None,
                },
                "recipient_user",
                Some("org_1"),
            );

            let response = service.list_conversations(request).await.unwrap().into_inner();
            let conv_pids: Vec<_> = response.conversations.iter().map(|c| c.pid.as_str()).collect();
            assert!(
                conv_pids.contains(&conversation.pid.as_str()),
                "shared conversation should appear in recipient's listing, got: {:?}",
                conv_pids
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_preserve_shares_after_soft_delete(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("share-conv-softdel").await;
            let conversation = ctx
                .create_conversation_in_org("share-conv-softdel-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share
            let request = authenticated_request(
                v1::ShareConversationRequest {
                    conversation_pid: conversation.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 7,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_conversation(request).await.unwrap();

            // Soft-delete the conversation
            let request = authenticated_request(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "owner_user",
                Some("org_1"),
            );
            service.delete_conversation(request).await.unwrap();

            // Share records should still exist in the DB
            let shares = conversation_users::Entity::find()
                .filter(conversation_users::Column::ConversationId.eq(conversation.id))
                .all(ctx.db.as_ref())
                .await
                .unwrap();
            assert_eq!(shares.len(), 1, "share records should survive soft-delete");
            assert_eq!(shares[0].user_id, "recipient_user");
        }
    }

    mod permission_matrix {
        use super::*;
        use platform_rs::cache::{OrgRole, PlanTier};
        use platform_rs::cache::{ResolvedPermissions, ResourcePermission, ResourceType};
        use platform_rs::middleware::organization::OrgContext;
        use std::collections::HashMap;

        fn request_with_permissions<T>(
            inner: T,
            user_id: &str,
            org_pid: &str,
            permissions: ResolvedPermissions,
        ) -> tonic::Request<T> {
            let mut request = authenticated_request(inner, user_id, Some(org_pid));
            request.extensions_mut().insert(OrgContext {
                pid: org_pid.to_string(),
                role: OrgRole::Guest,
                tier: PlanTier::Free,
                permissions,
            });
            request
        }

        fn perms_with(resource: ResourceType, perm: ResourcePermission) -> ResolvedPermissions {
            let mut map = HashMap::new();
            map.insert(resource, perm);
            ResolvedPermissions::new(map)
        }

        fn no_perms() -> ResolvedPermissions {
            ResolvedPermissions::default()
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_create_conversation_when_role_denies_write(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx).await;

            let request = request_with_permissions(
                v1::CreateConversationRequest {
                    agent_pid: "agent_123".to_string(),
                    title: Some("Test".to_string()),
                },
                "restricted_user",
                "org_1",
                perms_with(ResourceType::Conversations, ResourcePermission::READ), // read only
            );

            let result = service.create_conversation(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_delete_conversation_when_role_denies_write(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("perm-conv-delete").await;
            let conversation = ctx
                .create_conversation_in_org("perm-conv-delete-2", agent.id, "owner_user", "org_1")
                .await;

            let service = create_service(ctx).await;

            let request = request_with_permissions(
                v1::DeleteConversationRequest {
                    pid: conversation.pid.clone(),
                },
                "owner_user",
                "org_1",
                perms_with(ResourceType::Conversations, ResourcePermission::READ),
            );

            let result = service.delete_conversation(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_infer_when_role_denies_execute(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("perm-infer-deny").await;

            let service = create_service(ctx).await;

            let request = request_with_permissions(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Hello".to_string(),
                    request_id: "req-perm-1".to_string(),
                    document_pids: vec![],
                },
                "restricted_user",
                "org_1",
                no_perms(), // no execute on conversations
            );

            let result = service.infer(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_infer_when_role_grants_execute(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("perm-infer-allow").await;

            let service = create_service(ctx).await;

            let request = request_with_permissions(
                v1::InferRequest {
                    agent_pid: agent.pid.clone(),
                    conversation_pid: None,
                    message: "Hello".to_string(),
                    request_id: "req-perm-2".to_string(),
                    document_pids: vec![],
                },
                "user_test",
                "org_test",
                perms_with(ResourceType::Conversations, ResourcePermission::RWX),
            );

            let result = service.infer(request).await;
            assert!(
                result.is_ok(),
                "should allow infer with execute permission: {:?}",
                result.err()
            );
        }

        /// Tests for cross-org assistant access. These test `load_conversation_context`
        /// directly to get precise error types — the actor pipeline wraps errors into
        /// generic Status::internal, hiding the actual access check result.
        type TestInferenceActor = crate::actors::inference::InferenceActor<PostgresStore, MockEmbeddingModel>;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_assistant_inference_from_different_org(ctx: &mut TestContext) {
            init_test_crypto();
            let agent = ctx.create_system_user_assistant("cross-org-assist", "user_test").await;
            let conversation = ctx
                .create_conversation_in_org("cross-org-assist", agent.id, "user_test", "team_org")
                .await;
            let app_ctx = mock_app_ctx(ctx);

            let loaded =
                TestInferenceActor::load_conversation_context(&app_ctx, &conversation.pid, "team_org", "user_test")
                    .await
                    .expect("assistant agent should be accessible from any org");

            assert_eq!(loaded.conversation.pid, conversation.pid);
            assert_eq!(loaded.agent.pid, agent.pid);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_non_assistant_agent_from_different_org(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, model) = ctx.get_system_provider_and_model().await;
            let agent = ctx
                .create_agent_in_org("cross-org-deny", model.id, "org_test", "user_test")
                .await;
            let conversation = ctx
                .create_conversation_in_org("cross-org-deny", agent.id, "user_test", "other_org")
                .await;
            let app_ctx = mock_app_ctx(ctx);

            let err =
                TestInferenceActor::load_conversation_context(&app_ctx, &conversation.pid, "other_org", "user_test")
                    .await
                    .expect_err("non-assistant agent should not be accessible from different org");

            assert!(
                matches!(&err, crate::actors::ActorError::Unauthorized(msg) if msg.contains("agent not accessible")),
                "expected 'agent not accessible' Unauthorized, got: {err:?}"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_other_users_assistant_from_different_org(ctx: &mut TestContext) {
            init_test_crypto();
            let agent = ctx.create_system_user_assistant("other-assist", "user_other").await;
            let conversation = ctx
                .create_conversation_in_org("other-assist", agent.id, "user_test", "team_org")
                .await;
            let app_ctx = mock_app_ctx(ctx);

            let err =
                TestInferenceActor::load_conversation_context(&app_ctx, &conversation.pid, "team_org", "user_test")
                    .await
                    .expect_err("should not allow using another user's assistant");

            assert!(
                matches!(&err, crate::actors::ActorError::Unauthorized(msg) if msg.contains("agent not accessible")),
                "expected 'agent not accessible' Unauthorized, got: {err:?}"
            );
        }
    }
}
