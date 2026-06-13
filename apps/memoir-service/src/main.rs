#![recursion_limit = "256"]

mod api;
mod commands;
mod context;
mod middleware;
mod models;
mod services;

use clap::Parser as _;
use commands::*;
pub(crate) use context::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "memoir")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Manage admin users and API keys.
    Auth(auth::Arguments),
    /// Run the gRPC server.
    Server(server::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    // Load .env at the very top so every downstream `std::env::var` lookup
    // (DATABASE_URL, JWT_SECRET, HOST, PORT, etc.) resolves to the
    // file's values when the shell hasn't exported them. Missing .env is
    // not an error — production deployments inject env vars directly.
    let _ = dotenvy::dotenv();

    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Auth(args) => auth::run(args).await,
            Commands::Server(args) => server::run(args).await,
        }?;
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests;
