use std::net::SocketAddr;

use memoir_sdk::memoir::v1::auth_service_server::AuthServiceServer;
use migration::MigratorTrait as _;
use tonic::transport::Server;

use crate::AppContext;
use crate::services::auth::Auth;

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    Start {
        #[clap(long)]
        host: Option<String>,
        #[clap(long)]
        port: Option<String>,
    },
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    if let Some(command) = &args.command {
        match command {
            Commands::Start { host, port } => start(host, port).await,
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
    tracing::info!("migrations applied");

    let auth_handler = Auth::new(ctx.clone());

    let host_env = std::env::var("HOST").ok();
    let port_env = std::env::var("PORT").ok();
    let host = host.as_deref().or(host_env.as_deref()).unwrap_or("0.0.0.0");
    let port = port.as_deref().or(port_env.as_deref()).unwrap_or("5153");
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AuthServiceServer<Auth>>()
        .await;

    tracing::info!(server.address = %addr, "starting gRPC server (auth unauthenticated until interceptor lands)");

    Server::builder()
        .add_service(health_service)
        .add_service(AuthServiceServer::new(auth_handler))
        .serve(addr)
        .await?;

    Ok(())
}
