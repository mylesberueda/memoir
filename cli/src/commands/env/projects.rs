use super::types::{
    extract_vars, generate_env_content, parse_env_file, EnvSection, EnvVar, TerraformOutputs,
};
use crate::api::Terminal;
use crate::Result;
use indexmap::IndexMap;
use std::path::Path;

// =============================================================================
// ProjectEnvironment Trait
// =============================================================================

pub trait ProjectEnvironment {
    const NAME: &'static str;
    const PATH: &'static str;
    const TERRAFORM_KEY: &'static str;

    /// Backup .env -> .env.old
    fn backup(&self, term: &Terminal) -> Result<()> {
        let env_path = Path::new(Self::PATH).join(".env");
        let old_path = Path::new(Self::PATH).join(".env.old");

        if env_path.exists() {
            std::fs::copy(&env_path, &old_path)?;
            term.add_message("Backed up .env -> .env.old").ok();
        }

        Ok(())
    }

    /// Project-specific sections beyond Zitadel (override in projects that need Stripe, etc.)
    fn additional_sections(
        &self,
        _tf: &TerraformOutputs,
        _user_vars: &IndexMap<String, EnvVar>,
    ) -> Vec<EnvSection> {
        vec![]
    }

    /// Write the final .env file
    fn write(&self, tf: &TerraformOutputs, term: &Terminal) -> Result<()> {
        // Step 1: Backup
        self.backup(term)?;

        // Step 2-3: Generate base from .env.example + existing .env
        let example_path = Path::new(Self::PATH).join(".env.example");
        let env_path = Path::new(Self::PATH).join(".env");

        if !example_path.exists() {
            return Err(color_eyre::eyre::eyre!(
                ".env.example not found at {}",
                example_path.display()
            ));
        }

        let example_content = std::fs::read_to_string(&example_path)?;

        let user_vars = if env_path.exists() {
            let env_content = std::fs::read_to_string(&env_path)?;
            let lines = parse_env_file(&env_content);
            extract_vars(&lines)
        } else {
            indexmap::IndexMap::new()
        };

        // Step 4-5: Collect all generated sections
        // Order: project-specific sections first, then Zitadel (at the bottom for easy access)
        let mut sections = Vec::new();

        // Project-specific sections (e.g., Stripe for api-service)
        sections.extend(self.additional_sections(tf, &user_vars));

        // Zitadel section last (all projects need this, easiest to find at bottom)
        if let Some(zitadel) = tf.zitadel_section(Self::TERRAFORM_KEY) {
            sections.push(zitadel);
        } else {
            term.add_message(&format!(
                "⚠ Warning: No Zitadel config found for key '{}'",
                Self::TERRAFORM_KEY
            ))
            .ok();
        }

        // Generate final content
        let (content, warnings) = generate_env_content(&example_content, &user_vars, &sections);

        // Log warnings
        for warning in &warnings {
            term.add_message(&format!("⚠ {}", warning)).ok();
        }

        // Write .env
        std::fs::write(&env_path, content)?;

        Ok(())
    }
}

// =============================================================================
// Project Implementations
// =============================================================================

pub struct ApiService;

impl ProjectEnvironment for ApiService {
    const NAME: &'static str = "api-service";
    const PATH: &'static str = "apps/api-service";
    const TERRAFORM_KEY: &'static str = "api_service";

    fn additional_sections(
        &self,
        tf: &TerraformOutputs,
        user_vars: &IndexMap<String, EnvVar>,
    ) -> Vec<EnvSection> {
        let mut sections = Vec::new();
        if let Some(storage) = tf.storage_section(Self::NAME) {
            sections.push(storage);
        }
        sections.push(tf.stripe_section(user_vars));
        sections
    }
}

pub struct RigService;

impl ProjectEnvironment for RigService {
    const NAME: &'static str = "rig-service";
    const PATH: &'static str = "apps/rig-service";
    const TERRAFORM_KEY: &'static str = "rig_service";

    fn additional_sections(
        &self,
        tf: &TerraformOutputs,
        _user_vars: &IndexMap<String, EnvVar>,
    ) -> Vec<EnvSection> {
        let mut sections = Vec::new();
        if let Some(storage) = tf.storage_section(Self::NAME) {
            sections.push(storage);
        }
        sections
    }
}

pub struct ChatService;

impl ProjectEnvironment for ChatService {
    const NAME: &'static str = "chat-service";
    const PATH: &'static str = "apps/chat-service";
    const TERRAFORM_KEY: &'static str = "chat_service";

    fn additional_sections(
        &self,
        tf: &TerraformOutputs,
        _user_vars: &IndexMap<String, EnvVar>,
    ) -> Vec<EnvSection> {
        let mut sections = Vec::new();
        if let Some(storage) = tf.storage_section(Self::NAME) {
            sections.push(storage);
        }
        sections
    }
}

pub struct NotificationService;

impl ProjectEnvironment for NotificationService {
    const NAME: &'static str = "notification-service";
    const PATH: &'static str = "apps/notification-service";
    const TERRAFORM_KEY: &'static str = "notification_service";

    fn additional_sections(
        &self,
        tf: &TerraformOutputs,
        _user_vars: &IndexMap<String, EnvVar>,
    ) -> Vec<EnvSection> {
        let mut sections = Vec::new();
        if let Some(storage) = tf.storage_section(Self::NAME) {
            sections.push(storage);
        }
        sections
    }
}

pub struct Web;

impl ProjectEnvironment for Web {
    const NAME: &'static str = "web";
    const PATH: &'static str = "apps/web";
    const TERRAFORM_KEY: &'static str = "web";

    fn additional_sections(
        &self,
        tf: &TerraformOutputs,
        _user_vars: &IndexMap<String, EnvVar>,
    ) -> Vec<EnvSection> {
        let mut sections = Vec::new();
        if let Some(storage) = tf.storage_section(Self::NAME) {
            sections.push(storage);
        }
        sections
    }
}

// =============================================================================
// Registry of all projects
// =============================================================================

/// Get all registered projects
pub fn all_projects() -> Vec<Box<dyn ProjectEnvironmentDyn>> {
    vec![
        Box::new(ApiService),
        Box::new(RigService),
        Box::new(ChatService),
        Box::new(NotificationService),
        Box::new(Web),
    ]
}

/// Object-safe version of ProjectEnvironment for dynamic dispatch
pub trait ProjectEnvironmentDyn {
    fn name(&self) -> &'static str;
    fn path(&self) -> &'static str;
    fn write_env(&self, tf: &TerraformOutputs, term: &Terminal) -> Result<()>;
}

impl<T: ProjectEnvironment> ProjectEnvironmentDyn for T {
    fn name(&self) -> &'static str {
        T::NAME
    }

    fn path(&self) -> &'static str {
        T::PATH
    }

    fn write_env(&self, tf: &TerraformOutputs, term: &Terminal) -> Result<()> {
        self.write(tf, term)
    }
}
