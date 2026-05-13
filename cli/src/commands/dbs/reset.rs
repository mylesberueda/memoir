use crate::api::Terminal;
use sqlx::{Connection, PgConnection, postgres::PgConnectOptions};
use std::io::{self, Write};

#[derive(clap::Args)]
pub(crate) struct ResetArgs {
    /// PostgreSQL host
    #[clap(long, env = "DB_HOST", default_value = "localhost")]
    host: String,
    /// PostgreSQL port
    #[clap(long, env = "DB_PORT", default_value = "54321")]
    port: u16,
    /// PostgreSQL username
    #[clap(long, env = "DB_USER", default_value = "postgres")]
    user: String,
    /// PostgreSQL password
    #[clap(long, env = "DB_PASS", default_value = "postgres")]
    password: String,
    /// Skip confirmation prompt
    #[clap(long, short = 'y')]
    yes: bool,
}

const DB_NAMES: [&str; 3] = ["agent_service", "startup", "dash"];

pub(crate) async fn run(args: &ResetArgs) -> crate::Result<()> {
    // Confirmation prompt before initializing Terminal (so it doesn't interfere)
    if !args.yes {
        print!("⚠️  This will DROP and recreate all databases. All data will be lost! Continue? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("❌ Reset cancelled.");
            return Ok(());
        }
    }

    // Each DB has main + test = 2 tasks per DB
    let task_count = DB_NAMES.len() * 2;
    let term = Terminal::new("Resetting databases", task_count, 5);

    term.add_message(&format!("Server: {}:{}", args.host, args.port))?;

    // Add all tasks upfront
    for db_name in DB_NAMES {
        term.add_task(db_name);
        term.add_task(format!("{}_test", db_name));
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for db_name in DB_NAMES {
        // Reset main database
        term.update_task(db_name);

        match reset_database(&args.host, args.port, db_name, &args.user, &args.password, &term).await {
            Ok(()) => {
                term.finish_task(db_name, Some("reset"), true);
                success_count += 1;
            }
            Err(e) => {
                term.finish_task(db_name, Some(&e.to_string()), false);
                error_count += 1;
            }
        }

        // Reset test database
        let test_db_name = format!("{}_test", db_name);
        term.update_task(&test_db_name);

        match reset_database(&args.host, args.port, &test_db_name, &args.user, &args.password, &term).await {
            Ok(()) => {
                term.finish_task(&test_db_name, Some("reset"), true);
                success_count += 1;
            }
            Err(e) => {
                term.finish_task(&test_db_name, Some(&e.to_string()), false);
                error_count += 1;
            }
        }
    }

    let status = if error_count > 0 {
        format!(
            "Completed with errors: {} reset, {} failed",
            success_count, error_count
        )
    } else {
        format!("🎉 {} databases reset (empty, no migrations)", success_count)
    };

    term.finish(Some(&status))?;

    if error_count > 0 {
        return Err(color_eyre::eyre::eyre!("Failed to reset {} database(s)", error_count));
    }

    Ok(())
}

async fn reset_database(
    host: &str,
    port: u16,
    database: &str,
    user: &str,
    password: &str,
    term: &Terminal,
) -> crate::Result<()> {
    term.add_message(&format!("Connecting to reset '{}'...", database))?;

    let postgres_options = PgConnectOptions::new()
        .host(host)
        .port(port)
        .username(user)
        .password(password)
        .database("postgres");

    let mut conn = PgConnection::connect_with(&postgres_options).await?;

    // Check if database exists
    let query = "SELECT 1 FROM pg_database WHERE datname = $1";
    let exists = sqlx::query(query)
        .bind(database)
        .fetch_optional(&mut conn)
        .await?
        .is_some();

    if exists {
        term.add_message(&format!("Dropping '{}'...", database))?;

        // Force disconnect all connections to the database
        let disconnect_query = format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid) \
             FROM pg_stat_activity \
             WHERE pg_stat_activity.datname = '{}' \
             AND pid <> pg_backend_pid()",
            database
        );
        sqlx::query(&disconnect_query).execute(&mut conn).await.ok();

        // Drop the database
        let drop_query = format!("DROP DATABASE IF EXISTS \"{}\"", database);
        sqlx::query(&drop_query).execute(&mut conn).await?;
    }

    term.add_message(&format!("Creating fresh '{}'...", database))?;

    // Create the database
    let create_query = format!("CREATE DATABASE \"{}\"", database);
    sqlx::query(&create_query).execute(&mut conn).await?;

    conn.close().await?;
    Ok(())
}
