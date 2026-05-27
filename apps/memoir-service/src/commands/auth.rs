//! `memoir auth` CLI subcommand.
//!
//! Provides operator-facing management of admin users and (eventually) API
//! keys. Today exposes only `create`, the production path for bootstrapping
//! the first admin alongside the dev-mode and one-time-token paths the
//! server itself runs.
//!
//! ## Password input
//!
//! Two channels by design: `--password-stdin` reads from stdin, and
//! `--password-file <path>` reads from a file. A `--password <value>` flag
//! is intentionally NOT provided — passwords on argv leak to `ps`, shell
//! history, and audit logs. Operators piping from a secret manager use
//! one of these two channels.

use std::io::Read as _;
use std::path::PathBuf;

use color_eyre::eyre::{Context as _, bail};

use crate::AppContext;
use crate::services::auth::create_user;

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    /// Create a user.
    Create(CreateArgs),
}

#[derive(clap::Args)]
pub(crate) struct CreateArgs {
    /// Username for the new user. Must be unique.
    #[clap(long)]
    username: String,

    /// Mark the user as an admin.
    #[clap(long, default_value_t = false)]
    admin: bool,

    /// Read the password from stdin (terminated by EOF or newline).
    ///
    /// Mutually exclusive with --password-file.
    #[clap(long, conflicts_with = "password_file")]
    password_stdin: bool,

    /// Read the password from a file. Trailing whitespace is stripped.
    ///
    /// Mutually exclusive with --password-stdin.
    #[clap(long)]
    password_file: Option<PathBuf>,
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Commands::Create(create_args) => create(create_args).await,
    }
}

async fn create(args: &CreateArgs) -> crate::Result<()> {
    common_rs::logging::init_with_defaults()?;
    let password = read_password(args)?;

    let ctx = AppContext::new().await?;
    let user = create_user(ctx.db.as_ref(), args.username.clone(), &password, args.admin)
        .await
        .wrap_err("failed to create user")?;

    let role = if user.is_admin { "admin" } else { "user" };
    println!("Created {role} \"{}\" (pid={})", user.username, user.pid);

    Ok(())
}

fn read_password(args: &CreateArgs) -> crate::Result<String> {
    let raw = match (&args.password_stdin, &args.password_file) {
        (true, _) => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .wrap_err("failed to read password from stdin")?;
            buf
        }
        (false, Some(path)) => std::fs::read_to_string(path)
            .wrap_err_with(|| format!("failed to read password file at {}", path.display()))?,
        (false, None) => {
            bail!("must supply either --password-stdin or --password-file");
        }
    };

    let trimmed = raw.trim_end_matches(['\n', '\r']).to_string();
    if trimmed.is_empty() {
        bail!("password is empty");
    }
    Ok(trimmed)
}
