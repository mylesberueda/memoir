//! Logging and tracing utilities for memoir services
//!
//! This module provides centralized logging/tracing configuration.
//!
//! # Examples
//!
//! Quick setup with defaults:
//! ```no_run
//! use common_rs::logging;
//!
//! logging::init_with_defaults().expect("Failed to initialize logging");
//! ```
//!
//! Custom configuration:
//! ```no_run
//! use common_rs::logging::{init, LoggingConfig, LogFormat};
//!
//! let config = LoggingConfig {
//!     service_name: "my-service".to_string(),
//!     format: Some(LogFormat::Json),
//!     default_level: "info".to_string(),
//! };
//!
//! init(config).expect("Failed to initialize logging");
//! ```

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

/// Log output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable, pretty-printed logs
    Pretty,
    /// JSON-formatted logs for structured logging
    Json,
}

/// Configuration for logging initialization
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Service name for tracing context
    pub service_name: String,
    /// Log format (Pretty or Json)
    pub format: Option<LogFormat>,
    /// Default log level (e.g., "info", "debug", "warn")
    pub default_level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            service_name: "memoir-service".to_string(),
            format: None,
            default_level: "info".to_string(),
        }
    }
}

/// Initialize logging with default configuration
///
/// This is the recommended function for most services. It configures:
/// - Environment-based log filtering (RUST_LOG env var)
/// - Default fmt output
/// - Default log level of "info"
/// - Squelches noisy crates (sqlx, hyper, h2, tower) to warn level
///
/// # Errors
///
/// Returns an error if the subscriber fails to initialize
///
/// # Examples
///
/// ```no_run
/// common_rs::logging::init_with_defaults().expect("Failed to initialize logging");
/// tracing::info!("Service started");
/// ```
pub fn init_with_defaults() -> crate::Result<()> {
    let config = LoggingConfig {
        service_name: std::env::var("SERVICE_NAME").unwrap_or_else(|_| "memoir-service".to_string()),
        format: None,
        default_level: "info,sqlx=warn,sea_orm=warn,hyper=warn,h2=warn,tower=warn".to_string(),
    };

    init(config)
}

/// Initialize logging with custom configuration
///
/// # Arguments
///
/// * `config` - Configuration for logging behavior
///
/// # Errors
///
/// Returns an error if the tracing subscriber fails to initialize
///
/// # Examples
///
/// ```no_run
/// use common_rs::logging::{init, LoggingConfig, LogFormat};
///
/// let config = LoggingConfig {
///     service_name: "memoir-server".to_string(),
///     format: Some(LogFormat::Json),
///     default_level: "debug".to_string(),
/// };
///
/// init(config).expect("Failed to initialize logging");
/// ```
pub fn init(config: LoggingConfig) -> crate::Result<()> {
    // Create env filter from RUST_LOG or use default level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.default_level));

    // Create base registry with env filter
    let registry = tracing_subscriber::registry().with(env_filter);

    // Add format layer based on configuration
    if let Some(format) = config.format {
        match format {
            LogFormat::Pretty => {
                registry
                    .with(fmt::layer().pretty())
                    .try_init()
                    .map_err(|e| color_eyre::eyre::eyre!("Failed to initialize tracing subscriber: {}", e))?;
            }
            LogFormat::Json => {
                registry
                    .with(fmt::layer().json())
                    .try_init()
                    .map_err(|e| color_eyre::eyre::eyre!("Failed to initialize tracing subscriber: {}", e))?;
            }
        }
    } else {
        registry
            .with(fmt::layer())
            .try_init()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to initialize tracing subscriber: {}", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.service_name, "memoir-service");
        assert_eq!(config.format, None);
        assert_eq!(config.default_level, "info");
    }

    #[test]
    fn test_logging_config_custom() {
        let config = LoggingConfig {
            service_name: "test-service".to_string(),
            format: Some(LogFormat::Json),
            default_level: "debug".to_string(),
        };

        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.format, Some(LogFormat::Json));
        assert_eq!(config.default_level, "debug");
    }
}
