use crate::consts::{REDIS_SERVICE_KEY, TIER_LIMITS};
use axum::{Router, routing::get};
use common_rs::ratelimit::RedisRateLimitStore;
use migration::MigratorTrait as _;
use platform_rs::middleware::{
    auth::{AuthConfig, AuthLayer, ZitadelUserExtractor},
    organization::OrganizationLayer,
    ratelimit::RateLimitLayer,
};
use proto_rs::chat::v1::{
    channel_service_server::ChannelServiceServer, chat_service_server::ChatServiceServer,
    moderation_service_server::ModerationServiceServer,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tonic::service::Routes;

const DEV_ZITADEL_URL: &str = "http://localhost:5150";
const JWKS_REFRESH_TIMEOUT: u64 = 86400;

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

    let context = crate::AppContext::new().await?;

    migration::Migrator::up(context.db.as_ref(), None)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations ran!");

    let jwks_url = std::env::var("ZITADEL_JWKS_URL").unwrap_or_else(|_| {
        format!(
            "{}/oauth/v2/keys",
            std::env::var("ZITADEL_URL").unwrap_or_else(|_| DEV_ZITADEL_URL.to_string())
        )
    });
    let zitadel_url = std::env::var("ZITADEL_ISSUER")
        .or_else(|_| std::env::var("ZITADEL_URL"))
        .unwrap_or_else(|_| DEV_ZITADEL_URL.to_string());
    let audience = std::env::var("ZITADEL_AUDIENCE").expect("ZITADEL_AUDIENCE must be set for JWT validation");

    let auth_config = AuthConfig {
        jwks_url,
        issuer: zitadel_url,
        audience,
    };

    tracing::info!("Initializing auth layer");
    let auth_layer = AuthLayer::<ZitadelUserExtractor>::new(&auth_config)
        .await
        .expect("Failed to initialize auth layer");

    auth_layer.start_key_refresh(JWKS_REFRESH_TIMEOUT);
    tracing::info!("Started JWT key refresh task");

    tracing::info!("Initializing services");
    let chat_service = crate::ChatService::new();
    let channel_service = crate::ChannelService::new();
    let moderation_service = crate::ModerationService::new();
    tracing::info!("Services initialized");

    let org_layer = OrganizationLayer::new();
    let ratelimit_store = Arc::new(RedisRateLimitStore::new(context.redis.clone(), REDIS_SERVICE_KEY));
    let ratelimit_layer = RateLimitLayer::new(ratelimit_store, &TIER_LIMITS);

    let middleware_stack = tower::ServiceBuilder::new()
        .layer(auth_layer)
        .layer(org_layer)
        .layer(ratelimit_layer)
        .into_inner();

    let host_env = std::env::var("HOST").ok();
    let port_env = std::env::var("PORT").ok();
    let host = host.as_deref().or(host_env.as_deref()).unwrap_or("0.0.0.0");
    let port = port.as_deref().or(port_env.as_deref()).unwrap_or("5153");
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<ChatServiceServer<crate::ChatService>>()
        .await;

    health_reporter
        .set_serving::<ChannelServiceServer<crate::ChannelService>>()
        .await;

    health_reporter
        .set_serving::<ModerationServiceServer<crate::ModerationService>>()
        .await;

    // HTTP health endpoint (no auth)
    let health_router = Router::new().route("/health", get(|| async { "ok" }));

    // gRPC services with middleware
    let grpc_router = Routes::builder()
        .add_service(health_service)
        .add_service(ChatServiceServer::new(chat_service))
        .add_service(ChannelServiceServer::new(channel_service))
        .add_service(ModerationServiceServer::new(moderation_service))
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
