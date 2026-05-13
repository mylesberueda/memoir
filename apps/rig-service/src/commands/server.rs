use crate::{
    api::hooks::{Auth, Crypto, seed, session_registry, tool_registry},
    consts::{REDIS_SERVICE_KEY, REDIS_USER_CACHE_KEY, TIER_LIMITS},
    context::AppContext,
};
use axum::{Router, routing::get};
use common_rs::ratelimit::RedisRateLimitStore;
use migration::MigratorTrait as _;
use platform_rs::{
    cache::UserCache,
    middleware::{
        organization::{OrgContextLayer, OrganizationLayer},
        ratelimit::RateLimitLayer,
        user_cache::UserCacheLayer,
    },
    ratelimit::TierLimits,
};
use proto_rs::rig::v1::{
    agent_service_server::AgentServiceServer, document_group_service_server::DocumentGroupServiceServer,
    document_search_service_server::DocumentSearchServiceServer, document_service_server::DocumentServiceServer,
    inference_service_server::InferenceServiceServer, model_service_server::ModelServiceServer,
    provider_service_server::ProviderServiceServer, tool_service_server::ToolServiceServer,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tonic::service::Routes;

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    Start { host: Option<String>, port: Option<String> },
    Noop,
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    if let Some(command) = &args.command {
        match command {
            Commands::Start { host, port } => start(host, port).await,
            Commands::Noop => todo!(),
        }
    } else {
        Ok(())
    }
}

async fn start(host: &Option<String>, port: &Option<String>) -> crate::Result<()> {
    common_rs::logging::init_with_defaults()?;

    Crypto::init().await?;

    let ctx = AppContext::new().await?;

    migration::Migrator::up(&ctx.db, None)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations ran!");

    let tool_registry = tool_registry::Registry::init(ctx.clone()).await?;
    let session_registry = session_registry::Registry::init(ctx.clone(), tool_registry).await?;
    seed::Seed::init(ctx.clone()).await?;

    tracing::info!("Initializing notification client");
    let notification_url = std::env::var("NOTIFICATION_SERVICE_URL").expect("NOTIFICATION_SERVICE_URL must be set");
    let notification_client =
        Arc::new(crate::clients::NotificationClient::new(&notification_url).expect("Invalid NOTIFICATION_SERVICE_URL"));
    tracing::info!("Notification client configured: {notification_url}");

    tracing::info!("Initializing services");
    let agent_service = crate::AgentService::new(ctx.clone());
    let provider_service = crate::ProviderService::new(ctx.clone());
    let model_service = crate::ModelService::new(ctx.clone(), notification_client);
    let inference_service = crate::InferenceService::new(ctx.clone(), session_registry.clone());
    let tool_service = crate::ToolService::new(ctx.clone());
    let document_service = crate::DocumentService::new(ctx.clone(), session_registry);
    let document_group_service = crate::DocumentGroupService::new(ctx.clone());
    let document_search_service = crate::DocumentSearchService::new(ctx.clone());
    tracing::info!("Services initialized");

    let user_cache = UserCache::new(ctx.redis.clone(), REDIS_USER_CACHE_KEY);
    let user_cache_layer = UserCacheLayer::new(user_cache);
    let org_layer = OrganizationLayer::new();
    let ratelimit_store = Arc::new(RedisRateLimitStore::new(ctx.redis.clone(), REDIS_SERVICE_KEY));
    let tier_limits: &'static TierLimits = Box::leak(Box::new(TIER_LIMITS.with_env_multiplier()));
    let ratelimit_layer = RateLimitLayer::new(ratelimit_store, tier_limits);
    let auth_layer = Auth::init().await?;

    let org_context_layer = OrgContextLayer::new();

    let middleware_stack = tower::ServiceBuilder::new()
        .layer(auth_layer)
        .layer(user_cache_layer)
        .layer(org_layer)
        .layer(org_context_layer)
        .layer(ratelimit_layer)
        .into_inner();

    let host_env = std::env::var("HOST").ok();
    let port_env = std::env::var("PORT").ok();
    let host = host.as_deref().or(host_env.as_deref()).unwrap_or("0.0.0.0");
    let port = port.as_deref().or(port_env.as_deref()).unwrap_or("5153");
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<AgentServiceServer<crate::AgentService>>()
        .await;

    health_reporter
        .set_serving::<ProviderServiceServer<crate::ProviderService>>()
        .await;

    health_reporter
        .set_serving::<InferenceServiceServer<crate::InferenceService>>()
        .await;

    health_reporter
        .set_serving::<ToolServiceServer<crate::ToolService>>()
        .await;

    health_reporter
        .set_serving::<ModelServiceServer<crate::ModelService>>()
        .await;

    health_reporter
        .set_serving::<DocumentServiceServer<crate::DocumentService>>()
        .await;

    health_reporter
        .set_serving::<DocumentGroupServiceServer<crate::DocumentGroupService>>()
        .await;

    health_reporter
        .set_serving::<DocumentSearchServiceServer<crate::DocumentSearchService>>()
        .await;

    // HTTP health endpoint (no auth)
    let health_router = Router::new().route("/health", get(|| async { "ok" }));

    // gRPC services with middleware
    let grpc_router = Routes::builder()
        .add_service(health_service)
        .add_service(AgentServiceServer::new(agent_service))
        .add_service(ProviderServiceServer::new(provider_service))
        .add_service(ModelServiceServer::new(model_service))
        .add_service(InferenceServiceServer::new(inference_service))
        .add_service(ToolServiceServer::new(tool_service))
        .add_service(DocumentServiceServer::new(document_service))
        .add_service(DocumentGroupServiceServer::new(document_group_service))
        .add_service(DocumentSearchServiceServer::new(document_search_service))
        .to_owned()
        .routes()
        .prepare()
        .into_axum_router()
        .layer(middleware_stack);

    let app = Router::new().merge(health_router).merge(grpc_router);

    tracing::info!("Starting gRPC server on {addr} with JWT authentication");
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
