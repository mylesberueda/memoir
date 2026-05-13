#![allow(dead_code, unused_variables)] // DELETEME(_): Delete if not example service

use crate::AppContext;
use migration::MigratorTrait as _;
use std::net::SocketAddr;
use tonic::transport::Server;

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

    tracing::info!("Initializing services");
    // let admin_service = crate::AdminService::new(ctx.clone());
    tracing::info!("Services initialized");

    let host_env = std::env::var("HOST").ok();
    let port_env = std::env::var("PORT").ok();
    let host = host.as_deref().or(host_env.as_deref()).unwrap_or("0.0.0.0");
    let port = port.as_deref().or(port_env.as_deref()).unwrap_or("5153");
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    // health_reporter
    //     .set_serving::<AdminServiceServer<crate::AdminService>>()
    //     .await;

    // Auth + middleware wiring will be reintroduced by the local-auth epic.
    tracing::info!("Starting gRPC server on {addr} (no auth wired yet)");
    Server::builder()
        .add_service(health_service)
        // .add_service(AdminServiceServer::new(admin_service))
        .serve(addr)
        .await?;

    Ok(())
}
