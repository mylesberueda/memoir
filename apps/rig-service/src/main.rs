pub(crate) mod actors;
pub(crate) mod agents;
pub(crate) mod api;
mod clients;
mod commands;
mod consts;
mod context;
mod middleware;
mod models;
mod services;
pub(crate) mod test_utils;
mod tools;

use clap::Parser as _;
use commands::*;
pub(crate) use context::*;
pub(crate) use services::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "Agent (Rig) Service")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Basic command that does things and stuff
    Basic,
    Example(example::Arguments),
    Server(server::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Basic => basic_command(),
            Commands::Example(args) => example::run(args),
            Commands::Server(args) => server::run(args).await,
        }?;
    };

    Ok(())
}

fn basic_command() -> crate::Result<()> {
    println!("Running the basic command from the top level");
    Ok(())
}

#[cfg(all(test, feature = "integration"))]
mod cross_service_cache_tests {
    use crate::consts::REDIS_USER_CACHE_KEY;
    use crate::test_utils::TestContext;
    use platform_rs::cache::{CachedOrg, CachedUserData, OrgRole, PlanTier, UserCache};
    use serial_test::serial;
    use test_context::test_context;

    /// Verifies rig-service can read user cache entries written with the api-service key prefix.
    ///
    /// api-service is the single writer of user cache data (key: `api:user-cache:{user_id}`).
    /// rig-service reads from that same prefix via `REDIS_USER_CACHE_KEY`.
    /// If this test fails, the OrgContextLayer middleware in rig-service cannot validate
    /// org membership, breaking all org-scoped requests.
    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_read_user_cache_written_by_api_service(ctx: &mut TestContext) {
        let user_id = "cross-service-cache-test-user";

        // Simulate api-service writing with "api" prefix
        let api_writer = UserCache::new(ctx.redis.clone(), "api");
        let data = CachedUserData {
            email: "test@example.com".into(),
            organizations: vec![CachedOrg::new("org_cross_svc", PlanTier::Pro, OrgRole::Owner)],
        };
        api_writer.set(user_id, &data).await;

        // Rig-service reads with REDIS_USER_CACHE_KEY
        let rig_reader = UserCache::new(ctx.redis.clone(), REDIS_USER_CACHE_KEY);
        let result = rig_reader.get(user_id).await;

        assert_eq!(result, Some(data), "rig-service should read what api-service wrote");

        // Cleanup
        api_writer.delete(user_id).await;
    }
}
