pub(crate) mod clients;
mod commands;
mod consts;
mod context;
mod middleware;
mod models;
mod services;
mod test_utils;
mod traits;

use clap::Parser as _;
use commands::*;
pub(crate) use consts::*;
pub(crate) use context::*;
pub(crate) use services::*;
#[expect(unused_imports)] // Auth provider metadata removed — tier delivery via Redis
pub(crate) use traits::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "API Service")]
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
