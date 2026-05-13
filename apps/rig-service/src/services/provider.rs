use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    models::{
        provider_secrets, providers,
        secrets::{self, SecretKind},
    },
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::rig::v1::*;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait, Condition, EntityTrait as _, PaginatorTrait as _,
    QueryFilter as _, QueryOrder as _, QuerySelect as _,
};
use std::sync::Arc;
use tracing::instrument;

const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;

#[derive(Debug, Clone)]
pub(crate) struct ProviderService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    context: Arc<AppContext<EM>>,
}

impl<EM> ProviderService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(context: Arc<AppContext<EM>>) -> Self {
        Self { context }
    }
}

#[tonic::async_trait]
impl<EM> provider_service_server::ProviderService for ProviderService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn list_providers(
        &self,
        request: tonic::Request<ListProvidersRequest>,
    ) -> std::result::Result<tonic::Response<ListProvidersResponse>, tonic::Status> {
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

        let mut query = providers::Entity::find();

        // Access control: system providers + user's own or org's providers
        match &organization_pid {
            Some(org_pid) => {
                query = query.filter(
                    Condition::any()
                        .add(providers::Column::CreatedBy.is_null()) // System providers
                        .add(providers::Column::OrganizationPid.eq(org_pid)),
                );
            }
            None => {
                query = query.filter(
                    Condition::any()
                        .add(providers::Column::CreatedBy.is_null()) // System providers
                        .add(
                            Condition::all()
                                .add(providers::Column::CreatedBy.eq(&user_id))
                                .add(providers::Column::OrganizationPid.is_null()),
                        ),
                );
            }
        };

        // Optional filters
        if let Some(is_active) = req.is_active {
            query = query.filter(providers::Column::IsActive.eq(is_active));
        }

        if let Some(provider_type) = &req.provider_type {
            query = query.filter(providers::Column::ProviderType.eq(provider_type));
        }

        if let Some(source) = req.source.and_then(|s| ProviderSource::try_from(s).ok()) {
            match source {
                ProviderSource::System => {
                    query = query.filter(providers::Column::CreatedBy.is_null());
                }
                ProviderSource::User => {
                    query = query.filter(providers::Column::CreatedBy.is_not_null());
                }
                ProviderSource::Unspecified => {}
            }
        }

        let total = query.clone().count(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to count providers");
            tonic::Status::internal("Failed to count providers")
        })? as i32;

        let results = query
            .order_by_desc(providers::Column::CreatedAt)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch providers");
                tonic::Status::internal("Failed to fetch providers")
            })?;

        let providers = results
            .into_iter()
            .map(|provider| Provider::from(providers::Entity::assemble_provider(provider, None)))
            .collect();

        Ok(tonic::Response::new(ListProvidersResponse {
            providers,
            total,
            page: page as i32,
            page_size: page_size as i32,
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, pid))]
    async fn get_provider(
        &self,
        request: tonic::Request<GetProviderRequest>,
    ) -> std::result::Result<tonic::Response<GetProviderResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("pid", &req.pid);

        // Find provider with linked secret via junction table
        let provider = providers::Entity::find_provider_by_pid(&self.context.db, &req.pid)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch provider");
                match e {
                    providers::ProviderError::NotFound => tonic::Status::not_found("Provider not found"),
                    providers::ProviderError::Query(_) => tonic::Status::internal("Failed to fetch provider"),
                    providers::ProviderError::Secret(_) => tonic::Status::internal("Failed to fetch provider"),
                }
            })?;

        // Access control
        let accessible = match &organization_pid {
            Some(org_pid) => provider.model.is_accessible_in_org_context(org_pid),
            None => provider.model.is_accessible_in_user_context(&user_id),
        };

        if !accessible {
            return Err(tonic::Status::not_found("Provider not found"));
        }

        Ok(tonic::Response::new(GetProviderResponse {
            provider: Some(Provider::from(provider)),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn create_provider(
        &self,
        request: tonic::Request<CreateProviderRequest>,
    ) -> std::result::Result<tonic::Response<CreateProviderResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);

        // Validate required fields
        if req.name.is_empty() {
            return Err(tonic::Status::invalid_argument("name is required"));
        }
        if req.provider_type.is_empty() {
            return Err(tonic::Status::invalid_argument("provider_type is required"));
        }

        // Create provider
        let provider = providers::ActiveModel {
            pid: Set(nanoid::nanoid!()),
            organization_pid: Set(organization_pid),
            created_by: Set(Some(user_id)),
            name: Set(req.name),
            provider_type: Set(req.provider_type),
            base_url: Set(if req.endpoint_url.is_empty() {
                None
            } else {
                Some(req.endpoint_url)
            }),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(&self.context.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to create provider");
            tonic::Status::internal("Failed to create provider")
        })?;

        // Create secret if credentials provided
        let decrypted_secret = if !req.credentials.is_empty() {
            let secret = secrets::ActiveModel {
                pid: Set(nanoid::nanoid!()),
                secret_type: Set(SecretKind::ApiKey.to_string()),
                encrypted_value: Set(req.credentials.as_bytes().to_vec()), // Auto-encrypted by before_save
                ..Default::default()
            }
            .insert(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to create secret");
                tonic::Status::internal("Failed to create provider secret")
            })?;

            // Link provider to secret via junction table
            provider_secrets::ActiveModel {
                provider_id: Set(provider.id),
                secret_id: Set(secret.id),
            }
            .insert(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to link provider to secret");
                tonic::Status::internal("Failed to link provider to secret")
            })?;

            secret.decrypt().ok()
        } else {
            None
        };

        Ok(tonic::Response::new(CreateProviderResponse {
            provider: Some(Provider::from(providers::Entity::assemble_provider(
                provider,
                decrypted_secret,
            ))),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, pid))]
    async fn update_provider(
        &self,
        request: tonic::Request<UpdateProviderRequest>,
    ) -> std::result::Result<tonic::Response<UpdateProviderResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("pid", &req.pid);

        // Find existing provider with secret
        let (existing, old_secret): (providers::Model, Option<secrets::Model>) = providers::Entity::find()
            .filter(providers::Column::Pid.eq(&req.pid))
            .find_also_related(secrets::Entity)
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch provider");
                tonic::Status::internal("Failed to fetch provider")
            })?
            .ok_or(tonic::Status::not_found("Provider not found"))?;

        // Access control - cannot update system providers
        if existing.is_system() {
            return Err(tonic::Status::permission_denied("Cannot update system provider"));
        }

        let accessible = match &organization_pid {
            Some(org_pid) => existing.is_accessible_in_org_context(org_pid),
            None => existing.is_accessible_in_user_context(&user_id),
        };

        if !accessible {
            return Err(tonic::Status::not_found("Provider not found"));
        }

        let provider_id = existing.id;
        let mut active: providers::ActiveModel = existing.into();

        // Apply partial updates
        if let Some(name) = req.name {
            active.name = Set(name);
        }

        if let Some(provider_type) = req.provider_type {
            active.provider_type = Set(provider_type);
        }

        if let Some(endpoint_url) = req.endpoint_url {
            active.base_url = Set(if endpoint_url.is_empty() {
                None
            } else {
                Some(endpoint_url)
            });
        }

        if let Some(is_active) = req.is_active {
            active.is_active = Set(is_active);
        }

        let updated = active.update(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to update provider");
            tonic::Status::internal("Failed to update provider")
        })?;

        // Handle credentials update
        let decrypted_secret = if let Some(credentials) = req.credentials {
            if credentials.is_empty() {
                // Empty credentials = preserve existing
                old_secret.and_then(|s| s.decrypt().ok())
            } else {
                // New credentials: create new secret, update junction, delete old
                let new_secret = secrets::ActiveModel {
                    pid: Set(nanoid::nanoid!()),
                    secret_type: Set(SecretKind::ApiKey.to_string()),
                    encrypted_value: Set(credentials.as_bytes().to_vec()),
                    ..Default::default()
                }
                .insert(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to create new secret");
                    tonic::Status::internal("Failed to update credentials")
                })?;

                // Delete old junction and secret if exists
                if let Some(old) = old_secret {
                    provider_secrets::Entity::delete_many()
                        .filter(provider_secrets::Column::ProviderId.eq(provider_id))
                        .filter(provider_secrets::Column::SecretId.eq(old.id))
                        .exec(&self.context.db)
                        .await
                        .ok();

                    secrets::Entity::delete_by_id(old.id).exec(&self.context.db).await.ok();
                }

                // Create new junction
                provider_secrets::ActiveModel {
                    provider_id: Set(provider_id),
                    secret_id: Set(new_secret.id),
                }
                .insert(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to link new secret");
                    tonic::Status::internal("Failed to update credentials")
                })?;

                new_secret.decrypt().ok()
            }
        } else {
            // No credentials field = preserve existing
            old_secret.and_then(|s| s.decrypt().ok())
        };

        Ok(tonic::Response::new(UpdateProviderResponse {
            provider: Some(Provider::from(providers::Entity::assemble_provider(
                updated,
                decrypted_secret,
            ))),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid, pid))]
    async fn delete_provider(
        &self,
        request: tonic::Request<DeleteProviderRequest>,
    ) -> std::result::Result<tonic::Response<DeleteProviderResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);
        tracing::Span::current().record("pid", &req.pid);

        let existing = providers::Entity::find()
            .filter(providers::Column::Pid.eq(&req.pid))
            .filter(providers::Column::IsActive.eq(true))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch provider");
                tonic::Status::internal("Failed to fetch provider")
            })?
            .ok_or(tonic::Status::not_found("Provider not found"))?;

        // Access control - cannot delete system providers
        if existing.is_system() {
            return Err(tonic::Status::permission_denied("Cannot delete system provider"));
        }

        let accessible = match &organization_pid {
            Some(org_pid) => existing.is_accessible_in_org_context(org_pid),
            None => existing.is_accessible_in_user_context(&user_id),
        };

        if !accessible {
            return Err(tonic::Status::not_found("Provider not found"));
        }

        // Soft delete
        let mut active: providers::ActiveModel = existing.into();
        active.is_active = Set(false);

        active.update(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to delete provider");
            tonic::Status::internal("Failed to delete provider")
        })?;

        Ok(tonic::Response::new(DeleteProviderResponse {}))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::{TestContext, init_test_crypto};
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::provider_service_server::ProviderService as _;
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

    fn create_service(ctx: &TestContext) -> ProviderService<crate::test_utils::MockEmbeddingModel> {
        ProviderService::new(ctx.app_ctx())
    }

    mod list_providers {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_system_providers_for_any_user(ctx: &mut TestContext) {
            init_test_crypto();
            let system = ctx.create_system_provider("list-system", "openai").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "random_user",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                response
                    .providers
                    .iter()
                    .any(|p| p.identifier == Some(provider::Identifier::Pid(system.pid.clone()))),
                "should include system provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_personal_providers_for_owner(ctx: &mut TestContext) {
            init_test_crypto();
            let personal = ctx
                .create_personal_provider("list-personal", "openai", "user_test")
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                response
                    .providers
                    .iter()
                    .any(|p| p.identifier == Some(provider::Identifier::Pid(personal.pid.clone()))),
                "should include personal provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_other_users_personal_providers(ctx: &mut TestContext) {
            init_test_crypto();
            let other_personal = ctx.create_personal_provider("list-other", "openai", "other_user").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                !response
                    .providers
                    .iter()
                    .any(|p| p.identifier == Some(provider::Identifier::Pid(other_personal.pid.clone()))),
                "should not include other user's provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_org_providers_in_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let org = ctx
                .create_org_provider("list-org", "openai", "org_123", "user_test")
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_123"),
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                response
                    .providers
                    .iter()
                    .any(|p| p.identifier == Some(provider::Identifier::Pid(org.pid.clone()))),
                "should include org provider"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_org_provider_from_different_org(ctx: &mut TestContext) {
            init_test_crypto();
            let org_provider = ctx
                .create_org_provider("list-cross-org", "openai", "org_a", "user_test")
                .await;
            let service = create_service(ctx);

            // Request from org_b — should NOT see org_a's provider
            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                Some("org_b"),
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                !response
                    .providers
                    .iter()
                    .any(|p| p.identifier == Some(provider::Identifier::Pid(org_provider.pid.clone()))),
                "should not include provider from different org"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_provider_type(ctx: &mut TestContext) {
            init_test_crypto();
            let _openai = ctx.create_personal_provider("list-openai", "openai", "user_test").await;
            let _ollama = ctx.create_personal_provider("list-ollama", "ollama", "user_test").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: Some("openai".to_string()),
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                response.providers.iter().all(|p| p.provider_type == "openai"),
                "all providers should be openai type"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_source_system(ctx: &mut TestContext) {
            init_test_crypto();
            ctx.create_system_provider("list-src-sys", "openai").await;
            ctx.create_personal_provider("list-src-user", "openai", "user_test")
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: Some(ProviderSource::System.into()),
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert!(
                response
                    .providers
                    .iter()
                    .all(|p| p.source == ProviderSource::System as i32),
                "all providers should be system source"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_respect_pagination(ctx: &mut TestContext) {
            init_test_crypto();
            for i in 0..5 {
                ctx.create_personal_provider(&format!("list-page-{i}"), "openai", "user_test")
                    .await;
            }
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: Some(ProviderSource::User.into()),
                    provider_type: None,
                    page: 1,
                    page_size: 2,
                },
                "user_test",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            assert_eq!(response.providers.len(), 2, "should return 2 providers");
            assert!(response.total >= 5, "total should be at least 5");
            assert_eq!(response.page, 1);
            assert_eq!(response.page_size, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_include_credentials_in_list(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("list-creds", "openai", "user_test").await;
            ctx.create_api_key_secret("list-creds", provider.id).await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListProvidersRequest {
                    is_active: None,
                    source: None,
                    provider_type: None,
                    page: 1,
                    page_size: 20,
                },
                "user_test",
                None,
            );

            let response = service.list_providers(request).await.unwrap().into_inner();
            let found = response
                .providers
                .iter()
                .find(|p| p.identifier == Some(provider::Identifier::Pid(provider.pid.clone())));
            assert!(found.is_some(), "should find provider");
            assert!(
                found.unwrap().credentials.is_empty(),
                "credentials should be empty in list"
            );
        }
    }

    mod get_provider {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_provider_with_redacted_credentials(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("get-creds", "openai", "user_test").await;
            ctx.create_api_key_secret("get-creds", provider.id).await;
            let service = create_service(ctx);

            let request = authenticated_request(
                GetProviderRequest {
                    pid: provider.pid.clone(),
                },
                "user_test",
                None,
            );

            let response = service.get_provider(request).await.unwrap().into_inner();
            let p = response.provider.unwrap();
            assert!(!p.credentials.is_empty(), "should have redacted credentials");
            assert!(p.credentials.starts_with("***"), "credentials should be redacted");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_system_provider_to_any_user(ctx: &mut TestContext) {
            init_test_crypto();
            let system = ctx.create_system_provider("get-system", "openai").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                GetProviderRequest {
                    pid: system.pid.clone(),
                },
                "random_user",
                None,
            );

            let response = service.get_provider(request).await;
            assert!(response.is_ok(), "should succeed");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                GetProviderRequest {
                    pid: "nonexistent".to_string(),
                },
                "user_test",
                None,
            );

            let response = service.get_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let other = ctx.create_personal_provider("get-other", "openai", "other_user").await;
            let service = create_service(ctx);

            let request = authenticated_request(GetProviderRequest { pid: other.pid.clone() }, "user_test", None);

            let response = service.get_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_provider_source_and_endpoint_url(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("get-endpoint", "ollama", "user_test")
                .await;
            let mut active: providers::ActiveModel = provider.clone().into();
            active.base_url = Set(Some("http://localhost:11434".to_string()));
            let provider = active.update(ctx.db.as_ref()).await.unwrap();
            let service = create_service(ctx);

            let request = authenticated_request(
                GetProviderRequest {
                    pid: provider.pid.clone(),
                },
                "user_test",
                None,
            );

            let response = service.get_provider(request).await.unwrap().into_inner();
            let provider = response.provider.unwrap();

            assert_eq!(provider.source, ProviderSource::User as i32);
            assert_eq!(provider.endpoint_url, "http://localhost:11434");
            assert_eq!(provider.provider_type, "ollama");
        }
    }

    mod create_provider {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_provider_with_credentials(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateProviderRequest {
                    name: "My OpenAI".to_string(),
                    provider_type: "openai".to_string(),
                    source: ProviderSource::User.into(),
                    credentials: "sk-test-key-1234567890".to_string(),
                    endpoint_url: String::new(),
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.create_provider(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let p = response.unwrap().into_inner().provider.unwrap();
            assert!(!p.credentials.is_empty(), "should have redacted credentials");
            assert!(p.credentials.starts_with("***"), "credentials should be redacted");
            assert!(p.credentials.contains("7890"), "should show last part of key");

            // Clean up
            let pid = match &p.identifier {
                Some(provider::Identifier::Pid(pid)) => pid.clone(),
                _ => panic!("expected pid identifier"),
            };

            ctx.created_providers.push(
                providers::Entity::find()
                    .filter(providers::Column::Pid.eq(&pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap()
                    .id,
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_provider_without_credentials(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateProviderRequest {
                    name: "My Ollama".to_string(),
                    provider_type: "ollama".to_string(),
                    source: ProviderSource::User.into(),
                    credentials: String::new(),
                    endpoint_url: "http://localhost:11434".to_string(),
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.create_provider(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let p = response.unwrap().into_inner().provider.unwrap();
            assert!(p.credentials.is_empty(), "should have no credentials");

            // Clean up
            let pid = match &p.identifier {
                Some(provider::Identifier::Pid(pid)) => pid.clone(),
                _ => panic!("expected pid identifier"),
            };
            ctx.created_providers.push(
                providers::Entity::find()
                    .filter(providers::Column::Pid.eq(&pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap()
                    .id,
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_org_provider_in_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateProviderRequest {
                    name: "Org Provider".to_string(),
                    provider_type: "openai".to_string(),
                    source: ProviderSource::User.into(),
                    credentials: "sk-org-key".to_string(),
                    endpoint_url: String::new(),
                    config: None,
                },
                "user_test",
                Some("org_123"),
            );

            let response = service.create_provider(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let pid = match response.unwrap().into_inner().provider.unwrap().identifier {
                Some(provider::Identifier::Pid(pid)) => pid,
                _ => unreachable!(),
            };

            let get_request = authenticated_request(
                GetProviderRequest { pid: pid.clone() },
                "other_org_user",
                Some("org_123"),
            );
            let get_response = service.get_provider(get_request).await;
            assert!(get_response.is_ok(), "other org user should access org provider");

            // Clean up
            ctx.created_providers.push(
                providers::Entity::find()
                    .filter(providers::Column::Pid.eq(&pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap()
                    .id,
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_name(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateProviderRequest {
                    name: String::new(),
                    provider_type: "openai".to_string(),
                    source: ProviderSource::User.into(),
                    credentials: String::new(),
                    endpoint_url: String::new(),
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.create_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_provider_type(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateProviderRequest {
                    name: "My Provider".to_string(),
                    provider_type: String::new(),
                    source: ProviderSource::User.into(),
                    credentials: String::new(),
                    endpoint_url: String::new(),
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.create_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }
    }

    mod update_provider {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_provider_name(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("update-name", "openai", "user_test").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateProviderRequest {
                    pid: provider.pid.clone(),
                    name: Some("Updated Name".to_string()),
                    provider_type: None,
                    endpoint_url: None,
                    credentials: None,
                    is_active: None,
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.update_provider(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            assert_eq!(response.unwrap().into_inner().provider.unwrap().name, "Updated Name");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_credentials(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("update-creds", "openai", "user_test")
                .await;
            ctx.create_api_key_secret("update-creds", provider.id).await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateProviderRequest {
                    pid: provider.pid.clone(),
                    name: None,
                    provider_type: None,
                    endpoint_url: None,
                    credentials: Some("sk-new-key-abcdefgh".to_string()),
                    is_active: None,
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.update_provider(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let p = response.unwrap().into_inner().provider.unwrap();
            assert!(p.credentials.contains("efgh"), "should show new key suffix");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_preserve_credentials_when_empty_string(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("update-preserve", "openai", "user_test")
                .await;
            ctx.create_api_key_secret("update-preserve", provider.id).await;
            let service = create_service(ctx);

            // Get original credentials
            let get_request = authenticated_request(
                GetProviderRequest {
                    pid: provider.pid.clone(),
                },
                "user_test",
                None,
            );
            let original_creds = service
                .get_provider(get_request)
                .await
                .unwrap()
                .into_inner()
                .provider
                .unwrap()
                .credentials;

            // Update with empty string credentials
            let request = authenticated_request(
                UpdateProviderRequest {
                    pid: provider.pid.clone(),
                    name: Some("New Name".to_string()),
                    provider_type: None,
                    endpoint_url: None,
                    credentials: Some(String::new()),
                    is_active: None,
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.update_provider(request).await.unwrap().into_inner();
            assert_eq!(
                response.provider.unwrap().credentials,
                original_creds,
                "credentials should be preserved"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_update_on_system_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let system = ctx.create_system_provider("update-system", "openai").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateProviderRequest {
                    pid: system.pid.clone(),
                    name: Some("Hacked Name".to_string()),
                    provider_type: None,
                    endpoint_url: None,
                    credentials: None,
                    is_active: None,
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.update_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_update_on_other_users_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let other = ctx
                .create_personal_provider("update-other", "openai", "other_user")
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateProviderRequest {
                    pid: other.pid.clone(),
                    name: Some("Hacked".to_string()),
                    provider_type: None,
                    endpoint_url: None,
                    credentials: None,
                    is_active: None,
                    config: None,
                },
                "user_test",
                None,
            );

            let response = service.update_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod delete_provider {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_soft_delete_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_personal_provider("delete-soft", "openai", "user_test").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteProviderRequest {
                    pid: provider.pid.clone(),
                },
                "user_test",
                None,
            );

            let response = service.delete_provider(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            // Verify it's soft-deleted (get should still work but is_active=false)
            let deleted = providers::Entity::find()
                .filter(providers::Column::Pid.eq(&provider.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            assert!(!deleted.is_active, "should be soft-deleted");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_delete_on_system_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let system = ctx.create_system_provider("delete-system", "openai").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteProviderRequest {
                    pid: system.pid.clone(),
                },
                "user_test",
                None,
            );

            let response = service.delete_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_delete_on_other_users_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let other = ctx
                .create_personal_provider("delete-other", "openai", "other_user")
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(DeleteProviderRequest { pid: other.pid.clone() }, "user_test", None);

            let response = service.delete_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_already_deleted_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("delete-twice", "openai", "user_test")
                .await;
            let service = create_service(ctx);

            // Delete once
            let request = authenticated_request(
                DeleteProviderRequest {
                    pid: provider.pid.clone(),
                },
                "user_test",
                None,
            );
            service.delete_provider(request).await.unwrap();

            // Try to delete again
            let request = authenticated_request(
                DeleteProviderRequest {
                    pid: provider.pid.clone(),
                },
                "user_test",
                None,
            );
            let response = service.delete_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteProviderRequest {
                    pid: "nonexistent".to_string(),
                },
                "user_test",
                None,
            );

            let response = service.delete_provider(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }
}
