mod api;
mod commands;

use clap::Parser as _;
use commands::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "memoir-cli")]
#[clap(author = "Memoir Team")]
#[clap(version = "0.1.0")]
#[clap(about = "A CLI tool for memoir maintenance and setup tasks")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Database management commands (init and reset databases)
    Dbs(dbs::Arguments),
    /// Demo command to showcase the Terminal UI
    Demo(demo::DemoArgs),
    /// Environment file management (generate .env from terraform outputs)
    Env(env::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Dbs(args) => dbs::run(args).await,
            Commands::Demo(args) => demo::run(args).await,
            Commands::Env(args) => env::run(args).await,
        }?;
    };

    Ok(())
}
