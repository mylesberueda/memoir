use super::projects::all_projects;
use super::types::{InitArgs, TerraformOutputs};
use crate::Result;
use crate::api::Terminal;
use std::path::Path;

pub async fn run(args: &InitArgs) -> Result<()> {
    let projects = all_projects();
    let term = Terminal::new("Initializing environment files", projects.len(), 5);

    // Load terraform outputs
    let tf = TerraformOutputs::from_file(&args.terraform_outputs)?;
    term.add_message(&format!(
        "Loaded Terraform outputs (generated at: {})",
        tf.generated_at.as_deref().unwrap_or("unknown")
    ))?;

    // Add all tasks upfront
    for project in &projects {
        term.add_task(project.name());
    }

    let mut success_count = 0;
    let mut skip_count = 0;
    let mut error_count = 0;

    for project in &projects {
        let name = project.name();
        term.update_task(name);

        // Skip if project directory doesn't exist
        let project_path = Path::new(project.path());
        if !project_path.exists() {
            term.finish_task(name, Some("directory not found"), false);
            skip_count += 1;
            continue;
        }

        // Skip if no .env.example
        let example_path = project_path.join(".env.example");
        if !example_path.exists() {
            term.finish_task(name, Some("no .env.example"), false);
            skip_count += 1;
            continue;
        }

        match project.write_env(&tf, &term) {
            Ok(()) => {
                term.finish_task(name, Some("done"), true);
                success_count += 1;
            }
            Err(e) => {
                term.finish_task(name, Some(&e.to_string()), false);
                error_count += 1;
            }
        }
    }

    // Build summary message
    let summary = if error_count > 0 {
        format!(
            "Completed with errors: {} success, {} skipped, {} failed",
            success_count, skip_count, error_count
        )
    } else {
        format!(
            "🎉 Environment initialization complete! ({} success, {} skipped)",
            success_count, skip_count
        )
    };

    term.finish(Some(&summary))?;

    if error_count > 0 {
        return Err(color_eyre::eyre::eyre!("Failed to process {} project(s)", error_count));
    }

    Ok(())
}
