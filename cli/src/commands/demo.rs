use crate::Result;
use crate::api::Terminal;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(clap::Args)]
pub(crate) struct DemoArgs {
    /// Simulate a failure on the specified task (1-based index)
    #[clap(long)]
    fail_task: Option<usize>,

    /// Run tasks concurrently to demonstrate async support
    #[clap(long)]
    concurrent: bool,
}

const TASKS: &[(&str, u64, &[&str])] = &[
    (
        "Fetching configuration",
        800,
        &["Connecting to config server...", "Config loaded"],
    ),
    (
        "Validating schemas",
        600,
        &["Checking user schema", "Checking order schema", "All schemas valid"],
    ),
    (
        "Migrating database",
        1200,
        &[
            "Running migration 001_init",
            "Running migration 002_users",
            "Running migration 003_orders",
            "Migrations complete",
        ],
    ),
    (
        "Building assets",
        1000,
        &[
            "Compiling TypeScript",
            "Bundling CSS",
            "Optimizing images",
            "Assets built",
        ],
    ),
    (
        "Deploying services",
        1500,
        &[
            "Deploying api-service",
            "Deploying web-service",
            "Deploying worker-service",
            "All services deployed",
        ],
    ),
    (
        "Running health checks",
        700,
        &["Checking api health", "Checking database health", "All systems healthy"],
    ),
];

pub(crate) async fn run(args: &DemoArgs) -> Result<()> {
    if args.concurrent {
        run_concurrent(args).await
    } else {
        run_sequential(args).await
    }
}

async fn run_sequential(args: &DemoArgs) -> Result<()> {
    let term = Terminal::new("Demo: Sequential Deployment", TASKS.len(), 5);

    // Add all tasks upfront
    for (name, _, _) in TASKS {
        term.add_task(*name);
    }

    let mut success_count = 0;
    let mut fail_count = 0;

    for (idx, (name, duration_ms, messages)) in TASKS.iter().enumerate() {
        term.update_task(*name);

        // Simulate work with status messages
        let step_duration = duration_ms / messages.len() as u64;
        for (msg_idx, msg) in messages.iter().enumerate() {
            sleep(Duration::from_millis(step_duration)).await;
            term.add_message(msg)?;

            // Check if we should fail this task
            if args.fail_task == Some(idx + 1) && msg_idx == messages.len() - 1 {
                term.finish_task(*name, Some("simulated failure"), false);
                fail_count += 1;
                continue;
            }
        }

        if args.fail_task != Some(idx + 1) {
            term.finish_task(*name, Some("done"), true);
            success_count += 1;
        }
    }

    // Final status
    let status = if fail_count > 0 {
        format!(
            "Deployment completed with errors: {} succeeded, {} failed",
            success_count, fail_count
        )
    } else {
        format!("🚀 Deployment successful! All {} tasks completed", success_count)
    };

    term.finish(Some(&status))?;

    if fail_count > 0 {
        return Err(color_eyre::eyre::eyre!("{} task(s) failed", fail_count));
    }

    Ok(())
}

async fn run_concurrent(args: &DemoArgs) -> Result<()> {
    let term = Arc::new(Terminal::new("Demo: Concurrent Deployment", TASKS.len(), 5));

    // Add all tasks upfront
    for (name, _, _) in TASKS {
        term.add_task(*name);
    }

    // Spawn all tasks concurrently
    let mut handles = Vec::new();

    for (idx, (name, duration_ms, messages)) in TASKS.iter().enumerate() {
        let term = Arc::clone(&term);
        let name = *name;
        let duration_ms = *duration_ms;
        let messages: Vec<String> = messages.iter().map(|s| s.to_string()).collect();
        let should_fail = args.fail_task == Some(idx + 1);

        handles.push(tokio::spawn(async move {
            term.update_task(name);

            // Simulate work with status messages
            let step_duration = duration_ms / messages.len() as u64;
            for (msg_idx, msg) in messages.iter().enumerate() {
                sleep(Duration::from_millis(step_duration)).await;
                term.add_message(&format!("[{}] {}", name, msg)).ok();

                // Check if we should fail this task
                if should_fail && msg_idx == messages.len() - 1 {
                    term.finish_task(name, Some("simulated failure"), false);
                    return false;
                }
            }

            term.finish_task(name, Some("done"), true);
            true
        }));
    }

    // Wait for all tasks
    let mut success_count = 0;
    let mut fail_count = 0;

    for handle in handles {
        if handle.await.unwrap_or(false) {
            success_count += 1;
        } else {
            fail_count += 1;
        }
    }

    // Final status
    let status = if fail_count > 0 {
        format!(
            "Deployment completed with errors: {} succeeded, {} failed",
            success_count, fail_count
        )
    } else {
        format!("🚀 Deployment successful! All {} tasks completed", success_count)
    };

    // Extract from Arc - we need ownership to call finish()
    let term = Arc::try_unwrap(term).map_err(|_| color_eyre::eyre::eyre!("Terminal still in use"))?;
    term.finish(Some(&status))?;

    if fail_count > 0 {
        return Err(color_eyre::eyre::eyre!("{} task(s) failed", fail_count));
    }

    Ok(())
}
