use std::net::SocketAddr;
use std::sync::Arc;

use color_eyre::eyre::{Context as _, bail};
use common_rs::crypto::hashing::{generate_bootstrap_token, hash_password};
use memoir_sdk::memoir::v1::admin_service_server::AdminServiceServer;
use memoir_sdk::memoir::v1::auth_service_server::AuthServiceServer;
use memoir_sdk::memoir::v1::memory_service_server::MemoryServiceServer;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use tonic::transport::Server;

use crate::AppContext;
use crate::models::_entity::{bootstrap_tokens, users};
use crate::models::{BootstrapTokens, Users};
use crate::services::admin::Admin;
use crate::services::auth::{Auth, create_user};
use crate::services::memory::Memory;

/// Env var that opts the server into dev-mode bootstrap.
///
/// When set to `"true"`, the server reads admin credentials from
/// [`MEMOIR_DEV_ADMIN_USERNAME`] and [`MEMOIR_DEV_ADMIN_PASSWORD`] on first
/// start and creates the admin in-process. Loudly off by default; env vars
/// are visible to `docker inspect` and may leak to logs.
const ENV_DEV_MODE: &str = "MEMOIR_DEV_MODE";

/// Env var for the dev-mode bootstrap username.
const ENV_DEV_USERNAME: &str = "MEMOIR_DEV_ADMIN_USERNAME";

/// Env var for the dev-mode bootstrap password.
const ENV_DEV_PASSWORD: &str = "MEMOIR_DEV_ADMIN_PASSWORD";

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

    bootstrap_admin(ctx.clone()).await?;

    let admin_handler = Admin::new(ctx.clone());
    let auth_handler = Auth::new(ctx.clone());
    let memory_handler = Memory::new(ctx.clone());

    let host_env = std::env::var("HOST").ok();
    let port_env = std::env::var("PORT").ok();
    let host = host.as_deref().or(host_env.as_deref()).unwrap_or("0.0.0.0");
    let port = port.as_deref().or(port_env.as_deref()).unwrap_or("5153");
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<AdminServiceServer<Admin>>().await;
    health_reporter.set_serving::<AuthServiceServer<Auth>>().await;
    health_reporter.set_serving::<MemoryServiceServer<Memory>>().await;

    tracing::info!(server.address = %addr, "starting gRPC server");

    Server::builder()
        .add_service(health_service)
        .add_service(AdminServiceServer::new(admin_handler))
        .add_service(AuthServiceServer::new(auth_handler))
        .add_service(MemoryServiceServer::new(memory_handler))
        .serve(addr)
        .await?;

    Ok(())
}

/// Runs the first-admin bootstrap paths, in priority order.
///
/// 1. If any admin already exists, no-op.
/// 2. If `MEMOIR_DEV_MODE=true`, create an admin from `MEMOIR_DEV_ADMIN_*`
///    env vars and emit a loud warning.
/// 3. Otherwise, generate a one-time bootstrap token, hash it, persist with
///    a 24h TTL, and log the plaintext to stdout for the operator to consume
///    via the `ConsumeBootstrapToken` RPC.
///
/// The partial unique index on `bootstrap_tokens.status = 'pending'`
/// serializes the race between two memoir-service processes starting
/// concurrently: only the first INSERT succeeds.
async fn bootstrap_admin(ctx: Arc<AppContext>) -> crate::Result<()> {
    let db = ctx.db.as_ref();

    if admin_exists(db).await? {
        tracing::info!("admin user exists; skipping bootstrap");
        return Ok(());
    }

    if dev_mode_enabled() {
        bootstrap_dev_mode(db).await?;
        return Ok(());
    }

    bootstrap_one_time_token(db).await
}

async fn admin_exists(db: &DatabaseConnection) -> crate::Result<bool> {
    let count = Users::find()
        .filter(users::Column::IsAdmin.eq(true))
        .count(db)
        .await
        .wrap_err("failed to check for existing admin")?;
    Ok(count > 0)
}

fn dev_mode_enabled() -> bool {
    std::env::var(ENV_DEV_MODE)
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

async fn bootstrap_dev_mode(db: &DatabaseConnection) -> crate::Result<()> {
    let username = std::env::var(ENV_DEV_USERNAME)
        .wrap_err_with(|| format!("{ENV_DEV_MODE} is true but {ENV_DEV_USERNAME} is not set"))?;
    let password = std::env::var(ENV_DEV_PASSWORD)
        .wrap_err_with(|| format!("{ENV_DEV_MODE} is true but {ENV_DEV_PASSWORD} is not set"))?;
    if username.is_empty() || password.is_empty() {
        bail!("{ENV_DEV_USERNAME} and {ENV_DEV_PASSWORD} must be non-empty when {ENV_DEV_MODE} is true");
    }

    tracing::warn!(
        "MEMOIR_DEV_MODE is active. Creating bootstrap admin from env vars. \
         This path is for local development only — do not use in production."
    );

    let user = create_user(db, username, &password, true)
        .await
        .wrap_err("dev-mode admin creation failed")?;
    tracing::warn!(user.pid = %user.pid, "dev-mode admin created");

    Ok(())
}

async fn bootstrap_one_time_token(db: &DatabaseConnection) -> crate::Result<()> {
    let token = generate_bootstrap_token().wrap_err("failed to generate bootstrap token")?;
    let token_hash = hash_password(&token).wrap_err("failed to hash bootstrap token")?;

    let new_row = bootstrap_tokens::ActiveModel {
        token_hash: Set(token_hash),
        ..Default::default()
    };
    match BootstrapTokens::insert(new_row).exec(db).await {
        Ok(_) => {
            tracing::warn!(
                "FIRST-RUN BOOTSTRAP: no admin user exists. \
                 Use this single-use token to create one via the \
                 memoir.v1.AuthService.ConsumeBootstrapToken RPC.\n\n  {token}\n\n\
                 This token will not be shown again. \
                 If your logs are forwarded to a log aggregator, the token may be visible there \
                 until it is consumed."
            );
            Ok(())
        }
        Err(DbErr::Query(_) | DbErr::Exec(_)) => {
            // Partial-unique constraint violation: another process won the race.
            tracing::info!("bootstrap token already pending (another process won the first-start race); skipping");
            Ok(())
        }
        Err(other) => Err(other).wrap_err("failed to insert bootstrap token"),
    }
}
