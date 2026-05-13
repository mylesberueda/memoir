mod clean;
mod init;
mod projects;
mod types;

pub use types::*;

use crate::Result;

pub async fn run(args: &Arguments) -> Result<()> {
    match &args.command {
        Commands::Init(init_args) => init::run(init_args).await,
        Commands::Clean(clean_args) => clean::run(clean_args).await,
    }
}
