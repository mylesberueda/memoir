use base64::Engine;
use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

// =============================================================================
// CLI Arguments
// =============================================================================

#[derive(clap::Args)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Initialize environment variables for each project
    Init(InitArgs),
    /// Clean up generated environment files
    Clean(CleanArgs),
}

#[derive(clap::Args)]
pub struct InitArgs {
    #[clap(long, default_value = "./.data/terraform/development.json")]
    pub terraform_outputs: PathBuf,
}

#[derive(clap::Args)]
pub struct CleanArgs {
    #[clap(short, long, help = "Skip confirmation prompt")]
    pub yes: bool,
}

// =============================================================================
// Terraform Outputs
// =============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct TerraformOutputs {
    pub zitadel_url: String,
    pub project_id: String,
    pub services: HashMap<String, ServiceCredentials>,
    pub stripe: StripeOutputs,
    #[serde(default)]
    pub postgres: Option<PostgresOutputs>,
    #[serde(default)]
    pub redis: Option<RedisOutputs>,
    #[allow(unused)]
    #[serde(default)]
    pub api_audiences: HashMap<String, String>,
    #[allow(unused)]
    #[serde(default)]
    pub cli: Option<CliCredentials>,
    #[allow(unused)]
    #[serde(default)]
    pub roles: HashMap<String, String>,
    #[serde(default)]
    pub generated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ServiceCredentials {
    ApiService {
        user_id: String,
        key_id: String,
        private_key_base64: String,
        client_id: String,
    },
    OidcApp {
        client_id: String,
        key_details: String,
        #[serde(default)]
        service_user_id: Option<String>,
        #[serde(default)]
        service_key_id: Option<String>,
        #[serde(default)]
        service_key_details: Option<String>,
        #[serde(default)]
        webhook_signing_key: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeOutputs {
    pub prices: StripePrices,
    #[serde(default)]
    pub webhook_secret: String,
    pub portal_config_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripePrices {
    pub plus_monthly: String,
    pub plus_annual: String,
    pub pro_monthly: String,
    pub pro_annual: String,
    pub enterprise_monthly: String,
    pub enterprise_annual: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CliCredentials {
    #[allow(unused)]
    pub user_id: String,
    #[allow(unused)]
    pub pat: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresOutputs {
    #[allow(unused)]
    pub databases: Vec<String>,
    pub urls: Vec<PostgresUrl>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresUrl {
    /// Service name (e.g., "api-service")
    pub service: String,
    /// Database name (e.g., "api_service")
    #[allow(unused)]
    pub database: String,
    /// URL for Kind cluster pods (via Docker network)
    #[allow(unused)]
    pub cluster: String,
    /// URL for local development (via port mapping)
    pub local: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisOutputs {
    pub urls: Vec<RedisUrl>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisUrl {
    /// Service name (e.g., "api-service", "rig-service", "web")
    pub service: String,
    /// URL for Kind cluster pods (via Docker network)
    #[allow(unused)]
    pub cluster: String,
    /// URL for local development (via port mapping)
    pub local: String,
}

impl TerraformOutputs {
    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        if !path.exists() {
            return Err(color_eyre::eyre::eyre!(
                "Terraform outputs not found at {}. Run 'terraform apply' first.",
                path.display()
            ));
        }

        let content = std::fs::read_to_string(path)?;
        let outputs: TerraformOutputs = serde_json::from_str(&content)?;
        Ok(outputs)
    }

    /// Get Zitadel EnvSection for a specific service
    pub fn zitadel_section(&self, service_key: &str) -> Option<EnvSection> {
        let creds = self.services.get(service_key)?;

        let mut section = EnvSection::new("Zitadel")
            .var("ZITADEL_URL", &self.zitadel_url, false)
            .var("ZITADEL_JWKS_URL", format!("{}/oauth/v2/keys", self.zitadel_url), false)
            .var("ZITADEL_ISSUER", &self.zitadel_url, false)
            .var("ZITADEL_PROJECT_ID", &self.project_id, false)
            .var("ZITADEL_AUDIENCE", &self.project_id, false);

        match creds {
            ServiceCredentials::ApiService {
                user_id,
                key_id,
                private_key_base64,
                client_id,
            } => {
                // Base64 encode the JSON to avoid shell quoting issues (contains " and \n)
                let encoded = base64::engine::general_purpose::STANDARD.encode(private_key_base64);
                section = section
                    .var("ZITADEL_KEY_ID", key_id, false)
                    .var("ZITADEL_PRIVATE_KEY", encoded, false)
                    .var("ZITADEL_USER_ID", user_id, false)
                    .var("ZITADEL_CLIENT_ID", client_id, false);
            }
            ServiceCredentials::OidcApp {
                client_id,
                key_details,
                service_user_id,
                service_key_id,
                service_key_details,
                webhook_signing_key,
            } => {
                // Base64 encode JSON keys to avoid shell quoting issues
                let encoded_app_key = base64::engine::general_purpose::STANDARD.encode(key_details);
                section = section.var("ZITADEL_CLIENT_ID", client_id, false).var(
                    "ZITADEL_APPLICATION_KEY",
                    encoded_app_key,
                    false,
                );

                if let Some(user_id) = service_user_id {
                    section = section.var("ZITADEL_SERVICE_USER_ID", user_id, false);
                }
                if let Some(key_id) = service_key_id {
                    section = section.var("ZITADEL_SERVICE_KEY_ID", key_id, false);
                }
                if let Some(key) = service_key_details {
                    let encoded_service_key = base64::engine::general_purpose::STANDARD.encode(key);
                    section = section.var("ZITADEL_SERVICE_KEY", encoded_service_key, false);
                }
                if let Some(signing_key) = webhook_signing_key {
                    section = section.var("ZITADEL_WEBHOOK_SECRET", signing_key, false);
                }
            }
        }

        Some(section)
    }

    /// Get Stripe EnvSection, merging with existing user values
    ///
    /// Logic:
    /// - STRIPE_SECRET_KEY: use user's value if present, else commented placeholder
    /// - STRIPE_WEBHOOK_SECRET: use TF output if non-empty, else user's value, else commented placeholder
    /// - Price IDs: always from TF output (they change with terraform apply)
    pub fn stripe_section(&self, user_vars: &IndexMap<String, EnvVar>) -> EnvSection {
        let mut section = EnvSection::new("Stripe");

        // STRIPE_SECRET_KEY - from user's .env or commented placeholder
        if let Some(user_key) = user_vars.get("STRIPE_SECRET_KEY") {
            section = section.var("STRIPE_SECRET_KEY", &user_key.value, user_key.commented);
        } else {
            section = section.var("STRIPE_SECRET_KEY", "sk_your_stripe_secret_key", true);
        }

        // STRIPE_WEBHOOK_SECRET - TF output > user value > commented placeholder
        if !self.stripe.webhook_secret.is_empty() {
            section = section.var("STRIPE_WEBHOOK_SECRET", &self.stripe.webhook_secret, false);
        } else if let Some(user_secret) = user_vars.get("STRIPE_WEBHOOK_SECRET") {
            section = section.var("STRIPE_WEBHOOK_SECRET", &user_secret.value, user_secret.commented);
        } else {
            section = section.var("STRIPE_WEBHOOK_SECRET", "whsec_your_stripe_webhook_secret", true);
        }

        // Price IDs - always from TF output (monthly and annual)
        section = section
            .var("STRIPE_PRICE_ID_PLUS", &self.stripe.prices.plus_monthly, false)
            .var("STRIPE_PRICE_ID_PLUS_ANNUAL", &self.stripe.prices.plus_annual, false)
            .var("STRIPE_PRICE_ID_PRO", &self.stripe.prices.pro_monthly, false)
            .var("STRIPE_PRICE_ID_PRO_ANNUAL", &self.stripe.prices.pro_annual, false)
            .var(
                "STRIPE_PRICE_ID_ENTERPRISE",
                &self.stripe.prices.enterprise_monthly,
                false,
            )
            .var(
                "STRIPE_PRICE_ID_ENTERPRISE_ANNUAL",
                &self.stripe.prices.enterprise_annual,
                false,
            )
            .var(
                "STRIPE_PORTAL_CONFIGURATION_ID",
                &self.stripe.portal_config_id,
                false,
            );

        section
    }

    /// Get Storage EnvSection for a specific service
    ///
    /// Includes DATABASE_URL (if service has postgres) and REDIS_URL (if service has redis).
    /// Returns None if the service has neither.
    pub fn storage_section(&self, service_name: &str) -> Option<EnvSection> {
        let db_url = self
            .postgres
            .as_ref()
            .and_then(|p| p.urls.iter().find(|u| u.service == service_name));

        let redis_url = self
            .redis
            .as_ref()
            .and_then(|r| r.urls.iter().find(|u| u.service == service_name));

        // Return None if service has neither database nor redis
        if db_url.is_none() && redis_url.is_none() {
            return None;
        }

        let mut section = EnvSection::new("Storage");

        if let Some(url) = db_url {
            section = section.var("DATABASE_URL", &url.local, false);
        }

        if let Some(url) = redis_url {
            section = section.var("REDIS_URL", &url.local, false);
        }

        Some(section)
    }
}

// =============================================================================
// Env Var
// =============================================================================

/// A single environment variable that may be commented out
#[derive(Debug, Clone)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
    pub commented: bool,
}

impl EnvVar {
    pub fn to_env_string(&self) -> String {
        if self.commented {
            format!("# {}={}", self.key, self.value)
        } else {
            format!("{}={}", self.key, self.value)
        }
    }
}

// Regex for matching env vars: optional # prefix, CAPS_KEY="value"
// Matches: KEY="value", # KEY="value", #KEY="value"
static ENV_VAR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^(\s*#\s*)?([A-Z][A-Z0-9_]*)=(.*)$"#).unwrap());

/// Parse a line that might be an env var
pub fn parse_env_var(line: &str) -> Option<EnvVar> {
    let captures = ENV_VAR_REGEX.captures(line)?;

    let commented = captures.get(1).is_some();
    let key = captures.get(2)?.as_str().to_string();
    let value = captures.get(3)?.as_str().to_string();

    Some(EnvVar { key, value, commented })
}

// =============================================================================
// Env Section
// =============================================================================

/// A section in an .env file with a header and key-value pairs
#[derive(Debug, Clone)]
pub struct EnvSection {
    pub name: String,
    pub vars: IndexMap<String, EnvVar>,
}

impl EnvSection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            vars: IndexMap::new(),
        }
    }

    pub fn var(mut self, key: impl Into<String>, value: impl Into<String>, commented: bool) -> Self {
        let key = key.into();
        self.vars.insert(
            key.clone(),
            EnvVar {
                key,
                value: value.into(),
                commented,
            },
        );
        self
    }

    /// Serialize to .env format with section header
    pub fn to_env_string(&self) -> String {
        let header = format!(
            "# =============================================================================\n# {}\n# =============================================================================",
            self.name
        );

        let vars: String = self
            .vars
            .values()
            .map(|v| v.to_env_string())
            .collect::<Vec<_>>()
            .join("\n");

        format!("{header}\n{vars}")
    }
}

// =============================================================================
// Env File Parsing & Generation
// =============================================================================

/// Represents a parsed line from an env file
#[derive(Debug, Clone)]
pub enum EnvLine {
    SectionHeader(String), // "# Section Name" followed by ===
    Separator,             // # ===...
    Var(EnvVar),
    Comment(String), // Regular comment
    Blank,
    Other(String), // vim modeline, etc.
}

/// Parse an entire .env file preserving structure
pub fn parse_env_file(content: &str) -> Vec<EnvLine> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Check for section header pattern:
        // # Section Name
        // # =============
        if trimmed.starts_with("# ")
            && !trimmed.starts_with("# =")
            && i + 1 < lines.len()
            && lines[i + 1].trim().starts_with("# =")
        {
            let name = trimmed.trim_start_matches("# ").trim().to_string();
            result.push(EnvLine::SectionHeader(name));
            result.push(EnvLine::Separator);
            i += 2;
            continue;
        }

        // Separator line
        if trimmed.starts_with("# =") {
            result.push(EnvLine::Separator);
            i += 1;
            continue;
        }

        // Blank line
        if trimmed.is_empty() {
            result.push(EnvLine::Blank);
            i += 1;
            continue;
        }

        // vim modeline
        if trimmed.starts_with("# vim:") {
            result.push(EnvLine::Other(line.to_string()));
            i += 1;
            continue;
        }

        // Try to parse as env var
        if let Some(var) = parse_env_var(trimmed) {
            result.push(EnvLine::Var(var));
            i += 1;
            continue;
        }

        // Regular comment
        if trimmed.starts_with('#') {
            result.push(EnvLine::Comment(line.to_string()));
            i += 1;
            continue;
        }

        // Anything else
        result.push(EnvLine::Other(line.to_string()));
        i += 1;
    }

    result
}

/// Extract all vars from parsed env file into a map
pub fn extract_vars(lines: &[EnvLine]) -> IndexMap<String, EnvVar> {
    let mut vars = IndexMap::new();
    for line in lines {
        if let EnvLine::Var(var) = line {
            vars.insert(var.key.clone(), var.clone());
        }
    }
    vars
}

/// Generate final .env content by:
/// 1. Starting with .env.example structure (skipping sections we'll generate)
/// 2. Overlaying user values from .env
/// 3. Appending generated sections
/// 4. Adding vim modeline at the end
pub fn generate_env_content(
    example_content: &str,
    user_vars: &IndexMap<String, EnvVar>,
    generated_sections: &[EnvSection],
) -> (String, Vec<String>) {
    let example_lines = parse_env_file(example_content);
    let mut warnings = Vec::new();
    let mut output_lines = Vec::new();
    let mut seen_keys = std::collections::HashSet::new();

    // Collect section names we're generating (to skip them from example)
    let generated_section_names: std::collections::HashSet<_> =
        generated_sections.iter().map(|s| s.name.to_lowercase()).collect();

    // Track if we're currently inside a section we should skip
    let mut skip_current_section = false;

    // Process example file, overlaying user values
    for line in &example_lines {
        match line {
            EnvLine::SectionHeader(name) => {
                // Check if this section should be skipped (we're generating it)
                skip_current_section = generated_section_names.contains(&name.to_lowercase());
                if !skip_current_section {
                    output_lines.push(format!("# {}", name));
                }
            }
            EnvLine::Separator => {
                if !skip_current_section {
                    output_lines.push(
                        "# =============================================================================".to_string(),
                    );
                }
            }
            EnvLine::Var(example_var) => {
                if skip_current_section {
                    // Still track the key so we don't warn about it
                    seen_keys.insert(example_var.key.clone());
                    continue;
                }

                seen_keys.insert(example_var.key.clone());

                if let Some(user_var) = user_vars.get(&example_var.key) {
                    // User has this key - use their value and comment state
                    output_lines.push(user_var.to_env_string());
                } else {
                    // Use example's value and comment state
                    output_lines.push(example_var.to_env_string());
                }
            }
            EnvLine::Comment(c) => {
                // Skip comments inside generated sections, and skip vim modelines entirely
                if !skip_current_section && !c.contains("# vim:") {
                    output_lines.push(c.clone());
                }
            }
            EnvLine::Blank => {
                if !skip_current_section {
                    output_lines.push(String::new());
                }
            }
            EnvLine::Other(o) => {
                // Skip vim modelines - we'll add one at the end
                if !o.contains("# vim:") && !skip_current_section {
                    output_lines.push(o.clone());
                }
            }
        }
    }

    // Mark keys from generated sections as "seen" so we don't warn about them
    for section in generated_sections {
        for key in section.vars.keys() {
            seen_keys.insert(key.clone());
        }
    }

    // Warn about user vars not in example and not in generated sections
    for (key, _var) in user_vars {
        if !seen_keys.contains(key) {
            warnings.push(format!(
                "Key '{}' exists in .env but not in .env.example - it will be lost. Preserve from .env.old if needed.",
                key
            ));
        }
    }

    // Append generated sections
    for section in generated_sections {
        output_lines.push(String::new());
        output_lines.push(section.to_env_string());
    }

    // Add vim modeline at the very end
    output_lines.push(String::new());
    output_lines.push("# vim: ft=sh".to_string());

    (output_lines.join("\n"), warnings)
}
