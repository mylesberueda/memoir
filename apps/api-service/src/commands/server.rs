use axum::Router;
use common_rs::ratelimit::RedisRateLimitStore;
use migration::MigratorTrait as _;
use platform_rs::cache::UserCache;
use platform_rs::middleware::{
    auth::{AuthConfig, AuthLayer, ZitadelUserExtractor},
    organization::OrganizationLayer,
    ratelimit::RateLimitLayer,
    user_cache::UserCacheLayer,
};
use proto_rs::api::v1::{
    admin_service_server::AdminServiceServer, billing_service_server::BillingServiceServer,
    organization_service_server::OrganizationServiceServer, user_service_server::UserServiceServer,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tonic::service::Routes;

use crate::AppContext;

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

    let ctx = AppContext::new().await?;

    migration::Migrator::up(ctx.db.as_ref(), None)
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

    // Add services here
    let admin_service = crate::AdminService::new(ctx.clone());
    let organization_service = crate::OrganizationService::new(ctx.clone());
    let user_service = crate::UserService::new(ctx.clone());
    let billing_service = crate::BillingService::new(ctx.clone());

    tracing::info!("Services initialized");

    let user_cache = UserCache::new(ctx.redis.clone(), crate::REDIS_SERVICE_KEY);
    let user_cache_layer = UserCacheLayer::new(user_cache);
    let org_layer = OrganizationLayer::new();
    let org_context_layer = crate::middleware::OrganizationContextLayer::new(ctx.clone());
    let user_context_layer = crate::middleware::UserContextLayer::new(ctx.clone());
    let ratelimit_store = Arc::new(RedisRateLimitStore::new(ctx.redis.clone(), crate::REDIS_SERVICE_KEY));
    let tier_limits: &'static platform_rs::ratelimit::TierLimits =
        Box::leak(Box::new(crate::TIER_LIMITS.with_env_multiplier()));
    let ratelimit_layer = RateLimitLayer::new(ratelimit_store, tier_limits);

    let middleware_stack = tower::ServiceBuilder::new()
        .layer(auth_layer)
        .layer(user_cache_layer)
        .layer(org_layer)
        .layer(user_context_layer)
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
        .set_serving::<AdminServiceServer<crate::AdminService>>()
        .await;
    health_reporter
        .set_serving::<OrganizationServiceServer<crate::OrganizationService>>()
        .await;
    health_reporter
        .set_serving::<UserServiceServer<crate::UserService>>()
        .await;
    health_reporter
        .set_serving::<BillingServiceServer<crate::BillingService>>()
        .await;

    let grpc_router = Routes::builder()
        .add_service(health_service)
        .add_service(AdminServiceServer::new(admin_service))
        .add_service(OrganizationServiceServer::new(organization_service))
        .add_service(UserServiceServer::new(user_service))
        .add_service(BillingServiceServer::new(billing_service))
        .to_owned()
        .routes()
        .prepare()
        .into_axum_router()
        .layer(middleware_stack);

    let http_service = crate::HttpService::new(ctx.clone());
    let http_router = http_service.router().await;

    let app = Router::new().merge(http_router).merge(grpc_router);

    tracing::info!("Starting hybrid HTTP/gRPC server on {addr}");
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
