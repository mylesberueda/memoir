mod init;
mod reset;

use clap::Subcommand;

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize all databases (main and test databases)
    Init(init::InitArgs),
    /// Drop and recreate databases (without running migrations)
    Reset(reset::ResetArgs),
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Commands::Init(init_args) => init::run(init_args).await,
        Commands::Reset(reset_args) => reset::run(reset_args).await,
    }
}
