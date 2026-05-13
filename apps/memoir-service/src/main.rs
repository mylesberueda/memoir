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
    Server(server::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
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
mod integration_tests {
    #[tokio::test]
    async fn placeholder() {
        assert_eq!(2 + 2, 4);
    }
}
