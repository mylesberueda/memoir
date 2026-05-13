use crate::api::Terminal;
use sqlx::{Connection, PgConnection, postgres::PgConnectOptions};

#[derive(clap::Args)]
pub(crate) struct InitArgs {
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
}

const DB_NAMES: [&str; 7] = [
    "agent_service",
    "startup",
    "api_service",
    "rig_service",
    "chat_service",
    "notification_service",
    "example_grpc_service",
];

pub(crate) async fn run(args: &InitArgs) -> crate::Result<()> {
    // Each DB has main + test = 2 tasks per DB
    let task_count = DB_NAMES.len() * 2;
    let term = Terminal::new("Initializing databases", task_count, 5);

    term.add_message(&format!("Server: {}:{}", args.host, args.port))?;

    // Add all tasks upfront
    for db_name in DB_NAMES {
        term.add_task(db_name);
        term.add_task(format!("{}_test", db_name));
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for db_name in DB_NAMES {
        // Create main database
        term.update_task(db_name);

        match create_database(&args.host, args.port, db_name, &args.user, &args.password, &term).await {
            Ok(created) => {
                let msg = if created { "created" } else { "exists" };
                term.finish_task(db_name, Some(msg), true);
                success_count += 1;
            }
            Err(e) => {
                term.finish_task(db_name, Some(&e.to_string()), false);
                error_count += 1;
            }
        }

        // Create test database
        let test_db_name = format!("{}_test", db_name);
        term.update_task(&test_db_name);

        match create_database(&args.host, args.port, &test_db_name, &args.user, &args.password, &term).await {
            Ok(created) => {
                let msg = if created { "created" } else { "exists" };
                term.finish_task(&test_db_name, Some(msg), true);
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
            "Completed with errors: {} created, {} failed",
            success_count, error_count
        )
    } else {
        format!("🎉 {} databases ready", success_count)
    };

    term.finish(Some(&status))?;

    if error_count > 0 {
        return Err(color_eyre::eyre::eyre!("Failed to create {} database(s)", error_count));
    }

    Ok(())
}

/// Creates a database if it doesn't exist. Returns true if created, false if already existed.
async fn create_database(
    host: &str,
    port: u16,
    database: &str,
    user: &str,
    password: &str,
    term: &Terminal,
) -> crate::Result<bool> {
    term.add_message(&format!("Connecting to create '{}'...", database))?;

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

    let created = if exists {
        term.add_message(&format!("Database '{}' already exists", database))?;
        false
    } else {
        term.add_message(&format!("Creating '{}'...", database))?;
        let create_query = format!("CREATE DATABASE \"{}\"", database);
        sqlx::query(&create_query).execute(&mut conn).await?;
        true
    };

    conn.close().await?;
    Ok(created)
}
