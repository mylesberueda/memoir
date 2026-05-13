#![allow(dead_code, unused_variables)] // DELETEME(_): Delete if not example service

use crate::AppContext;
use migration::MigratorTrait as _;
use platform_rs::middleware::{
    auth::{AuthConfig, AuthLayer, ZitadelUserExtractor},
    organization::OrganizationLayer,
};
use std::net::SocketAddr;
use tonic::transport::Server;

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
    // let admin_service = crate::AdminService::new(db.clone());
    tracing::info!("Services initialized");

    let org_layer = OrganizationLayer::new();

    let middleware_stack = tower::ServiceBuilder::new()
        .layer(auth_layer)
        .layer(org_layer)
        // .layer(org_context_layer)
        // .layer(user_context_layer)
        .into_inner();

    let host_env = std::env::var("HOST").ok();
    let port_env = std::env::var("PORT").ok();
    let host = host.as_deref().or(host_env.as_deref()).unwrap_or("0.0.0.0");
    let port = port.as_deref().or(port_env.as_deref()).unwrap_or("5153");
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    // health_reporter
    //     .set_serving::<AdminServiceServer<crate::AdminService>>()
    //     .await;

    tracing::info!("Starting gRPC server on {addr} with JWT authentication");
    Server::builder()
        .layer(middleware_stack)
        .add_service(health_service)
        // .add_service(AdminServiceServer::new(admin_service))
        .serve(addr)
        .await?;

    Ok(())
}
