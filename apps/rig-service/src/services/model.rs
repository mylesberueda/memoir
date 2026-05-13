use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    clients::{FetchedModel, NotificationClient, ProviderError, fetch_models},
    models::{language_models, providers, secrets},
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::rig::v1;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait as _, JoinType, PaginatorTrait as _,
    QueryFilter as _, QueryOrder as _, QuerySelect as _, QueryTrait as _, RelationTrait as _,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;

const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;
const MAX_RETRIES: u32 = 10;
const INITIAL_BACKOFF_MS: u64 = 1000;

#[derive(Clone)]
pub(crate) struct ModelService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    context: Arc<AppContext<EM>>,
    notifications: Arc<NotificationClient>,
}

impl<EM> ModelService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(context: Arc<AppContext<EM>>, notifications: Arc<NotificationClient>) -> Self {
        Self { context, notifications }
    }

    /// Spawn background task to refresh models for a provider.
    fn spawn_refresh_task(
        &self,
        provider: providers::Model,
        user_id: String,
        org_pid: String,
        api_key: Option<String>,
        auth_token: String,
    ) {
        let db = self.context.db.clone();
        let notifications = self.notifications.clone();

        tokio::spawn(async move {
            let provider_pid = provider.pid.clone();
            let provider_name = provider.name.clone();
            let kind = provider.kind();
            let base_url = provider.base_url.as_deref();

            match fetch_with_retry(kind, base_url, api_key.as_deref()).await {
                Ok(fetched) => {
                    let count = fetched.len();
                    if let Err(e) = sync_models_to_db(&db, provider.id, &fetched).await {
                        tracing::error!(provider_pid = %provider_pid, error = %e, "failed to sync models to db");
                        if let Err(e) = notifications
                            .send_model_sync_failure(
                                &auth_token,
                                &user_id,
                                &org_pid,
                                &provider_pid,
                                &provider_name,
                                MAX_RETRIES,
                            )
                            .await
                        {
                            tracing::warn!(error = %e, provider_pid = %provider_pid, "failed to send model sync failure notification");
                        }
                        return;
                    }

                    tracing::info!(provider_pid = %provider_pid, models_synced = count, "model refresh completed");
                    if let Err(e) = notifications
                        .send_model_sync_success(&auth_token, &user_id, &org_pid, &provider_pid, &provider_name, count)
                        .await
                    {
                        tracing::warn!(error = %e, provider_pid = %provider_pid, "failed to send model sync success notification");
                    }
                }
                Err(e) => {
                    tracing::error!(provider_pid = %provider_pid, error = %e, "model refresh failed after max retries");
                    if let Err(e) = notifications
                        .send_model_sync_failure(
                            &auth_token,
                            &user_id,
                            &org_pid,
                            &provider_pid,
                            &provider_name,
                            MAX_RETRIES,
                        )
                        .await
                    {
                        tracing::warn!(error = %e, provider_pid = %provider_pid, "failed to send model sync failure notification");
                    }
                }
            }
        });
    }
}

#[tonic::async_trait]
impl<EM> v1::model_service_server::ModelService for ModelService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn list_models(
        &self,
        request: tonic::Request<v1::ListModelsRequest>,
    ) -> Result<tonic::Response<v1::ListModelsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);

        let req = request.into_inner();

        let page = Ord::max(req.page, 1) as u64;
        let page_size = if req.page_size == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            (req.page_size as u64).clamp(1, MAX_PAGE_SIZE)
        };

        // Build query joining models with providers
        let mut query =
            language_models::Entity::find().join(JoinType::InnerJoin, language_models::Relation::Providers.def());

        // Access control: only models from accessible providers
        match &organization_pid {
            Some(org_pid) => {
                query = query.filter(
                    Condition::any()
                        .add(providers::Column::CreatedBy.is_null())
                        .add(providers::Column::OrganizationPid.eq(org_pid)),
                );
            }
            None => {
                query = query.filter(
                    Condition::any().add(providers::Column::CreatedBy.is_null()).add(
                        Condition::all()
                            .add(providers::Column::CreatedBy.eq(&user_id))
                            .add(providers::Column::OrganizationPid.is_null()),
                    ),
                );
            }
        };

        // Filter by provider pid
        if let Some(provider_pid) = &req.provider_pid {
            query = query.filter(providers::Column::Pid.eq(provider_pid));
        }

        // Filter by provider type
        if let Some(provider_type) = &req.provider_type {
            query = query.filter(providers::Column::ProviderType.eq(provider_type));
        }

        // Filter by active status
        if let Some(is_active) = req.is_active {
            query = query.filter(language_models::Column::IsActive.eq(is_active));
        }

        // Exclude deprecated unless requested
        if !req.include_deprecated.unwrap_or(false) {
            query = query.filter(language_models::Column::DeprecationMessage.is_null());
        }

        let total = query.clone().count(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to count models");
            tonic::Status::internal("Failed to count models")
        })? as i32;

        // Fetch models with their providers
        let results = language_models::Entity::find()
            .find_also_related(providers::Entity)
            .filter(
                language_models::Column::Id.in_subquery(
                    query
                        .select_only()
                        .column(language_models::Column::Id)
                        .order_by_asc(language_models::Column::Name)
                        .offset((page - 1) * page_size)
                        .limit(page_size)
                        .into_query(),
                ),
            )
            .order_by_asc(language_models::Column::Name)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch models");
                tonic::Status::internal("Failed to fetch models")
            })?;

        let models = results
            .into_iter()
            .filter_map(|(model, provider)| {
                let provider = provider?;
                Some(language_models::Entity::assemble_language_model(model, provider).into())
            })
            .collect();

        Ok(tonic::Response::new(v1::ListModelsResponse {
            models,
            total,
            page: page as i32,
            page_size: page_size as i32,
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, pid))]
    async fn get_model(
        &self,
        request: tonic::Request<v1::GetModelRequest>,
    ) -> Result<tonic::Response<v1::GetModelResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("pid", &req.pid);

        let model = language_models::Entity::find_language_model_by_pid(&self.context.db, &req.pid)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch model");
                match e {
                    language_models::ModelError::NotFound => tonic::Status::not_found("Model not found"),
                    language_models::ModelError::MissingProvider => tonic::Status::not_found("Model not found"),
                    language_models::ModelError::Query(_) => tonic::Status::internal("Failed to fetch model"),
                }
            })?;

        // Access control
        let accessible = match &organization_pid {
            Some(org_pid) => model.is_accessible_in_org_context(org_pid),
            None => model.is_accessible_in_user_context(&user_id),
        };

        if !accessible {
            return Err(tonic::Status::not_found("Model not found"));
        }

        Ok(tonic::Response::new(v1::GetModelResponse {
            model: Some(v1::Model::from(model)),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn refresh_provider_models(
        &self,
        request: tonic::Request<v1::RefreshProviderModelsRequest>,
    ) -> Result<tonic::Response<v1::RefreshProviderModelsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        // Capture the user's JWT so background tasks can forward it to notification-service
        let auth_token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .unwrap_or_default()
            .to_string();
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);

        // Build query for providers to refresh — includes system providers (no org) and org providers
        let mut query = providers::Entity::find().filter(providers::Column::IsActive.eq(true));
        query = query.filter(
            Condition::any()
                .add(providers::Column::CreatedBy.is_null())
                .add(providers::Column::OrganizationPid.eq(&organization_pid)),
        );

        // If specific provider requested, filter to that one
        if let Some(ref provider_pid) = req.provider_pid {
            query = query.filter(providers::Column::Pid.eq(provider_pid));
        }

        // Fetch providers with their secrets
        let providers_to_refresh: Vec<(providers::Model, Option<secrets::Model>)> = query
            .find_also_related(secrets::Entity)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch providers");
                tonic::Status::internal("Failed to fetch providers")
            })?;

        let provider_pids: Vec<String> = providers_to_refresh.iter().map(|(p, _)| p.pid.clone()).collect();
        let providers_queued = providers_to_refresh.len() as i32;

        // Spawn background refresh tasks
        for (provider, secret) in providers_to_refresh {
            let api_key = secret.and_then(|s| s.decrypt().ok()).map(|s| s.expose().to_string());

            self.spawn_refresh_task(
                provider,
                user_id.clone(),
                organization_pid.clone(),
                api_key,
                auth_token.clone(),
            );
        }

        Ok(tonic::Response::new(v1::RefreshProviderModelsResponse {
            providers_queued,
            provider_pids,
        }))
    }
}

/// Fetch models with exponential backoff retry.
async fn fetch_with_retry(
    kind: providers::ProviderKind,
    base_url: Option<&str>,
    api_key: Option<&str>,
) -> Result<Vec<FetchedModel>, ProviderError> {
    let mut attempt = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;

    loop {
        attempt += 1;
        tracing::debug!(kind = ?kind, attempt = attempt, "attempting to fetch models");

        match fetch_models(kind, base_url, api_key).await {
            Ok(models) => return Ok(models),
            Err(e) if attempt >= MAX_RETRIES => {
                tracing::error!(kind = ?kind, attempt = attempt, error = %e, "max retries reached");
                return Err(e);
            }
            Err(e) => {
                tracing::warn!(kind = ?kind, attempt = attempt, error = %e, backoff_ms = backoff_ms, "fetch failed, retrying");
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                backoff_ms *= 2;
            }
        }
    }
}

/// Sync fetched models to database using batch operations.
async fn sync_models_to_db(
    db: &DatabaseConnection,
    provider_id: i64,
    fetched: &[FetchedModel],
) -> Result<(), ProviderError> {
    use sea_orm::sea_query::OnConflict;

    if fetched.is_empty() {
        return Ok(());
    }

    let now = chrono::Utc::now().naive_utc();

    // Fetch existing model_ids for this provider to determine which to mark inactive
    let existing_model_ids: Vec<String> = language_models::Entity::find()
        .filter(language_models::Column::ProviderId.eq(provider_id))
        .filter(language_models::Column::IsActive.eq(true))
        .select_only()
        .column(language_models::Column::ModelId)
        .into_tuple()
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to fetch existing model ids");
            ProviderError::InvalidResponse(format!("DB error: {}", e))
        })?;

    let fetched_ids: std::collections::HashSet<&str> = fetched.iter().map(|m| m.model_id.as_str()).collect();

    // Build batch of models to upsert
    let models_to_upsert: Vec<language_models::ActiveModel> = fetched
        .iter()
        .map(|model| {
            let capabilities_json = serde_json::to_value(model.capabilities).unwrap_or_else(|_| serde_json::json!({}));
            let metadata_json = serde_json::to_value(&model.metadata).unwrap_or_else(|_| serde_json::json!({}));

            language_models::ActiveModel {
                pid: Set(nanoid::nanoid!()),
                provider_id: Set(provider_id),
                model_id: Set(model.model_id.clone()),
                name: Set(model.name.clone()),
                context_window: Set(model.context_window),
                capabilities: Set(capabilities_json),
                metadata: Set(metadata_json),
                is_active: Set(true),
                last_fetched_at: Set(Some(now)),
                ..Default::default()
            }
        })
        .collect();

    // Batch upsert: insert new models or update existing ones on model_id conflict
    language_models::Entity::insert_many(models_to_upsert)
        .on_conflict(
            OnConflict::columns([language_models::Column::ProviderId, language_models::Column::ModelId])
                .update_columns([
                    language_models::Column::Name,
                    language_models::Column::ContextWindow,
                    language_models::Column::Capabilities,
                    language_models::Column::Metadata,
                    language_models::Column::IsActive,
                    language_models::Column::LastFetchedAt,
                ])
                .to_owned(),
        )
        .exec(db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to upsert models");
            ProviderError::InvalidResponse(format!("DB upsert error: {}", e))
        })?;

    // Collect model_ids that were removed (exist in DB but not in fetched)
    let removed_ids: Vec<String> = existing_model_ids
        .into_iter()
        .filter(|id| !fetched_ids.contains(id.as_str()))
        .collect();

    // Bulk update: mark removed models as inactive
    if !removed_ids.is_empty() {
        language_models::Entity::update_many()
            .col_expr(
                language_models::Column::IsActive,
                sea_orm::sea_query::Expr::value(false),
            )
            .col_expr(
                language_models::Column::LastFetchedAt,
                sea_orm::sea_query::Expr::value(now),
            )
            .filter(language_models::Column::ProviderId.eq(provider_id))
            .filter(language_models::Column::ModelId.is_in(removed_ids))
            .exec(db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to mark removed models inactive");
                ProviderError::InvalidResponse(format!("DB update error: {}", e))
            })?;
    }

    Ok(())
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        clients::NotificationClient,
        models::language_models,
        test_utils::{TestContext, init_test_crypto},
    };
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::model_service_server::ModelService as _;
    use sea_orm::{ActiveModelTrait as _, ActiveValue::Set};
    use serial_test::serial;
    use test_context::test_context;

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

    fn create_service(ctx: &TestContext) -> ModelService<crate::test_utils::MockEmbeddingModel> {
        let notification_url = std::env::var("NOTIFICATION_SERVICE_URL").expect("NOTIFICATION_SERVICE_URL must be set");
        let notifications = Arc::new(NotificationClient::new(&notification_url).expect("valid notification URL"));
        ModelService::new(ctx.app_ctx(), notifications)
    }

    mod list_models {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_list_when_provider_has_no_models(ctx: &mut TestContext) {
            init_test_crypto();
            // Create a provider with no models
            let empty_provider = ctx
                .create_personal_provider("empty-provider", "ollama", "user_empty")
                .await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(empty_provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "user_empty",
                None,
            );

            let response = service.list_models(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert!(response.models.is_empty(), "should have no models for this provider");
            assert_eq!(response.total, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_models_from_system_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let system_provider = ctx.create_system_provider("list-system", "ollama").await;
            let model = ctx.create_model("list-system", system_provider.id).await;

            let service = create_service(ctx);

            // Filter by the specific provider to avoid pollution from other tests
            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(system_provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "random_user",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().any(|m| {
                    matches!(&m.identifier, Some(proto_rs::rig::v1::model::Identifier::Pid(p)) if p == &model.pid)
                }),
                "should include model from system provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_models_from_personal_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("list-personal", "ollama", "user_test")
                .await;
            ctx.create_api_key_secret("list-personal", provider.id).await;
            let model = ctx.create_model("list-personal", provider.id).await;

            let service = create_service(ctx);

            // Filter by the specific provider to avoid pollution from other tests
            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().any(|m| {
                    matches!(&m.identifier, Some(proto_rs::rig::v1::model::Identifier::Pid(p)) if p == &model.pid)
                }),
                "should include model from personal provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_models_from_other_users_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let other_provider = ctx.create_personal_provider("list-other", "ollama", "other_user").await;
            let other_model = ctx.create_model("list-other", other_provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: None,
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            // Check by pid (unique) not model_id (shared from env)
            assert!(
                !response.models.iter().any(|m| {
                    matches!(&m.identifier, Some(proto_rs::rig::v1::model::Identifier::Pid(p)) if p == &other_model.pid)
                }),
                "should not include model from other user's provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_models_from_org_provider_in_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let org_provider = ctx
                .create_org_provider("list-org", "ollama", "org_123", "user_test")
                .await;
            let model = ctx.create_model("list-org", org_provider.id).await;

            let service = create_service(ctx);

            // Filter by the specific provider to avoid pollution from other tests
            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(org_provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "other_org_user",
                Some("org_123"),
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().any(|m| {
                    matches!(&m.identifier, Some(proto_rs::rig::v1::model::Identifier::Pid(p)) if p == &model.pid)
                }),
                "should include model from org provider in org context"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_provider_pid(ctx: &mut TestContext) {
            init_test_crypto();
            let provider1 = ctx
                .create_personal_provider("list-filter-1", "ollama", "user_test")
                .await;
            let model1 = ctx.create_model("list-filter-1", provider1.id).await;

            let provider2 = ctx
                .create_personal_provider("list-filter-2", "ollama", "user_test")
                .await;
            let model2 = ctx.create_model("list-filter-2", provider2.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(provider1.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            // Check by pid (unique) not model_id (shared from env)
            assert!(
                response.models.iter().any(|m| {
                    matches!(&m.identifier, Some(proto_rs::rig::v1::model::Identifier::Pid(p)) if p == &model1.pid)
                }),
                "should include model from filtered provider"
            );
            assert!(
                !response.models.iter().any(|m| {
                    matches!(&m.identifier, Some(proto_rs::rig::v1::model::Identifier::Pid(p)) if p == &model2.pid)
                }),
                "should not include model from other provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_provider_type(ctx: &mut TestContext) {
            init_test_crypto();
            let ollama_provider = ctx
                .create_personal_provider("list-type-ollama", "ollama", "user_test")
                .await;
            let _ollama_model = ctx.create_model("list-type-ollama", ollama_provider.id).await;

            let openai_provider = ctx
                .create_personal_provider("list-type-openai", "openai", "user_test")
                .await;
            let _openai_model = ctx.create_model("list-type-openai", openai_provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: None,
                    provider_type: Some("ollama".to_string()),
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().all(|m| m.provider_type == "ollama"),
                "all models should be from ollama providers"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_active_status(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("list-active", "ollama", "user_test").await;
            let active_model = ctx.create_model("list-active", provider.id).await;

            // Create an inactive model
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let inactive_model = language_models::ActiveModel {
                pid: Set(format!("model-inactive-{timestamp}")),
                provider_id: Set(provider.id),
                model_id: Set("inactive-model".to_string()),
                name: Set("Inactive Model".to_string()),
                is_active: Set(false),
                ..Default::default()
            }
            .insert(ctx.db.as_ref())
            .await
            .unwrap();
            ctx.created_models.push(inactive_model.id);

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(provider.pid.clone()),
                    provider_type: None,
                    is_active: Some(true),
                    include_deprecated: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().any(|m| m.model_id == active_model.model_id),
                "should include active model"
            );
            assert!(
                !response.models.iter().any(|m| m.model_id == "inactive-model"),
                "should not include inactive model"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_exclude_deprecated_models_by_default(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("list-deprecated", "ollama", "user_test")
                .await;
            let normal_model = ctx.create_model("list-deprecated", provider.id).await;

            // Create a deprecated model
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let deprecated_model = language_models::ActiveModel {
                pid: Set(format!("model-deprecated-{timestamp}")),
                provider_id: Set(provider.id),
                model_id: Set("deprecated-model".to_string()),
                name: Set("Deprecated Model".to_string()),
                is_active: Set(true),
                deprecation_message: Set(Some("This model is deprecated".to_string())),
                ..Default::default()
            }
            .insert(ctx.db.as_ref())
            .await
            .unwrap();
            ctx.created_models.push(deprecated_model.id);

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None, // Default: exclude deprecated
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().any(|m| m.model_id == normal_model.model_id),
                "should include normal model"
            );
            assert!(
                !response.models.iter().any(|m| m.model_id == "deprecated-model"),
                "should exclude deprecated model by default"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_deprecated_models_when_requested(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("list-include-deprecated", "ollama", "user_test")
                .await;

            // Create a deprecated model
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let deprecated_model = language_models::ActiveModel {
                pid: Set(format!("model-inc-deprecated-{timestamp}")),
                provider_id: Set(provider.id),
                model_id: Set(format!("deprecated-model-inc-{timestamp}")),
                name: Set("Deprecated Model".to_string()),
                is_active: Set(true),
                deprecation_message: Set(Some("This model is deprecated".to_string())),
                ..Default::default()
            }
            .insert(ctx.db.as_ref())
            .await
            .unwrap();
            ctx.created_models.push(deprecated_model.id);

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: Some(true),
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert!(
                response.models.iter().any(|m| m.model_id == deprecated_model.model_id),
                "should include deprecated model when requested"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_respect_pagination(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("list-page", "ollama", "user_test").await;

            // Create 5 models
            for i in 0..5 {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let model = language_models::ActiveModel {
                    pid: Set(format!("model-page-{i}-{timestamp}")),
                    provider_id: Set(provider.id),
                    model_id: Set(format!("page-model-{i}-{timestamp}")),
                    name: Set(format!("Page Model {i}")),
                    is_active: Set(true),
                    ..Default::default()
                }
                .insert(ctx.db.as_ref())
                .await
                .unwrap();
                ctx.created_models.push(model.id);
            }

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListModelsRequest {
                    provider_pid: Some(provider.pid.clone()),
                    provider_type: None,
                    is_active: None,
                    include_deprecated: None,
                    page: 1,
                    page_size: 2,
                },
                "user_test",
                None,
            );

            let response = service.list_models(request).await.unwrap().into_inner();
            assert_eq!(response.models.len(), 2, "should return 2 models");
            assert_eq!(response.total, 5, "total should be 5");
            assert_eq!(response.page, 1);
            assert_eq!(response.page_size, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_when_unauthenticated(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = tonic::Request::new(v1::ListModelsRequest {
                provider_pid: None,
                provider_type: None,
                is_active: None,
                include_deprecated: None,
                page: 1,
                page_size: 20,
            });

            let response = service.list_models(request).await;
            assert!(response.is_err(), "should fail without authentication");
            assert_eq!(
                response.unwrap_err().code(),
                tonic::Code::Unauthenticated,
                "should return UNAUTHENTICATED"
            );
        }
    }

    mod get_model {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_model_by_pid(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("get-model", "ollama", "user_test").await;
            ctx.create_api_key_secret("get-model", provider.id).await;
            let model = ctx.create_model("get-model", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(v1::GetModelRequest { pid: model.pid.clone() }, "user_test", None);

            let response = service.get_model(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response_model = response.unwrap().into_inner().model.unwrap();
            assert!(matches!(
                response_model.identifier,
                Some(proto_rs::rig::v1::model::Identifier::Pid(ref p)) if p == &model.pid
            ));
            assert_eq!(response_model.model_id, model.model_id);
            assert_eq!(response_model.provider_pid, provider.pid);
            assert_eq!(response_model.provider_type, provider.provider_type);
            assert_eq!(response_model.provider_name, provider.name);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_model_metadata_and_capabilities(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("get-model-meta", "ollama", "user_test")
                .await;
            ctx.create_api_key_secret("get-model-meta", provider.id).await;

            let model = language_models::ActiveModel {
                pid: Set(format!("model-meta-{}", nanoid::nanoid!())),
                provider_id: Set(provider.id),
                model_id: Set("ollama-meta".to_string()),
                name: Set("Ollama Meta".to_string()),
                context_window: Set(Some(8192)),
                capabilities: Set(serde_json::json!({
                    "vision": false,
                    "function_calling": true,
                    "json_mode": true,
                    "streaming": true,
                    "system_prompt": true,
                    "multi_turn": true,
                    "thinking": false
                })),
                metadata: Set(serde_json::json!({
                    "family": "llama",
                    "quantization": "Q4_0"
                })),
                is_active: Set(true),
                ..Default::default()
            }
            .insert(ctx.db.as_ref())
            .await
            .unwrap();
            ctx.created_models.push(model.id);

            let service = create_service(ctx);

            let request = authenticated_request(v1::GetModelRequest { pid: model.pid.clone() }, "user_test", None);

            let response = service.get_model(request).await.unwrap().into_inner();
            let response_model = response.model.unwrap();

            assert_eq!(response_model.provider_name, provider.name);
            assert_eq!(response_model.context_length, Some(8192));
            assert!(response_model.capabilities.is_some(), "should include capabilities");
            assert!(response_model.metadata.is_some(), "should include metadata");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_model_from_system_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let system_provider = ctx.create_system_provider("get-system", "ollama").await;
            let model = ctx.create_model("get-system", system_provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(v1::GetModelRequest { pid: model.pid.clone() }, "random_user", None);

            let response = service.get_model(request).await;
            assert!(response.is_ok(), "any user should access system provider models");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_model(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetModelRequest {
                    pid: "nonexistent-model".to_string(),
                },
                "user_test",
                None,
            );

            let response = service.get_model(request).await;
            assert!(response.is_err(), "should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_model(ctx: &mut TestContext) {
            init_test_crypto();
            let other_provider = ctx.create_personal_provider("get-other", "ollama", "other_user").await;
            let other_model = ctx.create_model("get-other", other_provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetModelRequest {
                    pid: other_model.pid.clone(),
                },
                "user_test",
                None,
            );

            let response = service.get_model(request).await;
            assert!(response.is_err(), "should fail for other user's model");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_org_model_in_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let org_provider = ctx
                .create_org_provider("get-org", "ollama", "org_123", "user_test")
                .await;
            let model = ctx.create_model("get-org", org_provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetModelRequest { pid: model.pid.clone() },
                "other_org_user",
                Some("org_123"),
            );

            let response = service.get_model(request).await;
            assert!(response.is_ok(), "org user should access org provider models");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_org_model_without_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let org_provider = ctx
                .create_org_provider("get-org-no-ctx", "ollama", "org_123", "creator_user")
                .await;
            let model = ctx.create_model("get-org-no-ctx", org_provider.id).await;

            let service = create_service(ctx);

            // Request without org context
            let request = authenticated_request(
                v1::GetModelRequest { pid: model.pid.clone() },
                "other_user",
                None, // No org context
            );

            let response = service.get_model(request).await;
            assert!(response.is_err(), "should fail without org context");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod refresh_provider_models {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_queue_refresh_for_org_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_org_provider("refresh-org-own", "ollama", "org_test", "user_test")
                .await;
            ctx.create_api_key_secret("refresh-org-own", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::RefreshProviderModelsRequest {
                    provider_pid: Some(provider.pid.clone()),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert_eq!(response.providers_queued, 1, "should queue 1 provider");
            assert!(
                response.provider_pids.contains(&provider.pid),
                "should include the provider pid"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_queue_refresh_for_system_providers(ctx: &mut TestContext) {
            init_test_crypto();
            let system_provider = ctx.create_system_provider("refresh-system", "ollama").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::RefreshProviderModelsRequest {
                    provider_pid: Some(system_provider.pid.clone()),
                },
                "random_user",
                Some("org_test"),
            );

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_ok(), "any user can refresh system providers");

            let response = response.unwrap().into_inner();
            assert!(
                response.provider_pids.contains(&system_provider.pid),
                "should include system provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_queue_other_orgs_providers(ctx: &mut TestContext) {
            init_test_crypto();
            let other_provider = ctx
                .create_org_provider("refresh-other-org", "ollama", "org_other", "other_user")
                .await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::RefreshProviderModelsRequest {
                    provider_pid: Some(other_provider.pid.clone()),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_ok(), "should succeed but queue nothing");

            let response = response.unwrap().into_inner();
            assert_eq!(response.providers_queued, 0, "should not queue other org's provider");
            assert!(
                !response.provider_pids.contains(&other_provider.pid),
                "should not include other org's provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_queue_org_providers_in_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let org_provider = ctx
                .create_org_provider("refresh-org", "ollama", "org_123", "user_test")
                .await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::RefreshProviderModelsRequest {
                    provider_pid: Some(org_provider.pid.clone()),
                },
                "other_org_user",
                Some("org_123"),
            );

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_ok(), "org user can refresh org providers");

            let response = response.unwrap().into_inner();
            assert!(
                response.provider_pids.contains(&org_provider.pid),
                "should include org provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_queue_all_accessible_providers_when_no_pid_specified(ctx: &mut TestContext) {
            init_test_crypto();
            let org_provider = ctx
                .create_org_provider("refresh-all-org", "ollama", "org_test", "user_test")
                .await;
            let system = ctx.create_system_provider("refresh-all-system", "ollama").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::RefreshProviderModelsRequest { provider_pid: None },
                "user_test",
                Some("org_test"),
            );

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert!(response.providers_queued >= 2, "should queue at least 2 providers");
            assert!(
                response.provider_pids.contains(&org_provider.pid),
                "should include org provider"
            );
            assert!(
                response.provider_pids.contains(&system.pid),
                "should include system provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_queue_inactive_providers(ctx: &mut TestContext) {
            init_test_crypto();
            // Create an inactive provider
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let base_url = std::env::var("OLLAMA_BASE_URL").expect("OLLAMA_BASE_URL should exist");
            let inactive_provider = crate::models::providers::ActiveModel {
                pid: Set(format!("provider-inactive-{timestamp}")),
                organization_pid: Set(None),
                created_by: Set(Some("user_test".to_string())),
                name: Set("Inactive Provider".to_string()),
                provider_type: Set("ollama".to_string()),
                base_url: Set(Some(base_url)),
                is_active: Set(false),
                is_deprecated: Set(false),
                ..Default::default()
            }
            .insert(ctx.db.as_ref())
            .await
            .unwrap();
            ctx.created_providers.push(inactive_provider.id);

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::RefreshProviderModelsRequest {
                    provider_pid: Some(inactive_provider.pid.clone()),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_ok(), "should succeed but queue nothing");

            let response = response.unwrap().into_inner();
            assert_eq!(response.providers_queued, 0, "should not queue inactive provider");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_when_unauthenticated(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = tonic::Request::new(v1::RefreshProviderModelsRequest { provider_pid: None });

            let response = service.refresh_provider_models(request).await;
            assert!(response.is_err(), "should fail without authentication");
            assert_eq!(
                response.unwrap_err().code(),
                tonic::Code::Unauthenticated,
                "should return UNAUTHENTICATED"
            );
        }
    }
}
