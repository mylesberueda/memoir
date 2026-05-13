# Startup CLI

A Rust CLI tool for startup.ai maintenance and setup tasks, replacing Python scripts with faster, more robust implementations.

## Features

- **Professional Logging**: Uses `env_logger` with `RUST_LOG` environment variable support
- **Progress Indicators**: Beautiful progress bars and spinners via `indicatif`
- **Database Management**: PostgreSQL schema and test database creation
- **API Key Management**: Secure service-to-service API key generation and validation
- **Performance**: Fast startup and execution compared to Python equivalents

## Installation

```bash
# Build the CLI
cargo build --release

# The binary will be available at target/release/startup-cli
```

## Usage

### Database Commands

```bash
# Create main database schema
startup-cli database create-schema --host localhost --port 5432 --database mydb --user postgres --password secret

# Create test database with verification
startup-cli db create-test-db --host localhost --database test_db --verify

# Use environment variables (POSTGRES_HOST, POSTGRES_PORT, POSTGRES_DB, POSTGRES_USER, POSTGRES_PASSWORD)
POSTGRES_DB=myapp startup-cli db create-schema
```

### API Key Commands

```bash
# Generate new API key
startup-cli api-keys generate --agent-db-url "postgresql://postgres:postgres@localhost:5432/agent_service"

# Generate with custom name and save to file
startup-cli keys generate \
  --agent-db-url "postgresql://postgres:postgres@localhost:5432/agent_service" \
  --key-name "production-api-key" \
  --output-file .env.local

# List existing API keys
startup-cli api-keys list --agent-db-url "postgresql://postgres:postgres@localhost:5432/agent_service"

# Validate an API key
startup-cli keys validate \
  --agent-db-url "postgresql://postgres:postgres@localhost:5432/agent_service" \
  --api-key "svc_main_abc123..."

# Rotate existing key
startup-cli api-keys generate --rotate --key-name "main-api-service" \
  --agent-db-url "postgresql://postgres:postgres@localhost:5432/agent_service"
```

### System Provider Commands

Manage AI provider configurations (OpenAI, Anthropic, Azure) for your applications.

#### Prerequisites

Before using provider commands, ensure you have initialized Zitadel service accounts:

```bash
# Initialize Zitadel service accounts (required for API authentication)
startup-cli zitadel init --services api
```

This creates the necessary authentication credentials in `.data/zitadel/keys/` that the providers command uses to authenticate with the API.

#### Basic Usage

```bash
# Initialize all providers using environment variables
startup-cli providers init

# Initialize specific providers with CLI flags
startup-cli providers init --openai-key sk-... --anthropic-key sk-ant-...

# Preview what would be created (dry run)
startup-cli providers init --dry-run --openai-key sk-...

# Initialize specific provider types only
startup-cli providers init --providers openai,anthropic --openai-key sk-... --anthropic-key sk-ant-...
```

#### Environment Variable Configuration

Set provider API keys using environment variables with the `SYSTEM_PROVIDER_` prefix:

```bash
# OpenAI Configuration
export SYSTEM_PROVIDER_OPENAI_API_KEY="sk-..."
export SYSTEM_PROVIDER_OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional

# Anthropic Configuration  
export SYSTEM_PROVIDER_ANTHROPIC_API_KEY="sk-ant-..."
export SYSTEM_PROVIDER_ANTHROPIC_BASE_URL="https://api.anthropic.com"  # Optional

# Azure OpenAI Configuration
export SYSTEM_PROVIDER_AZURE_API_KEY="..."
export SYSTEM_PROVIDER_AZURE_ENDPOINT="https://your-resource.openai.azure.com"
export SYSTEM_PROVIDER_AZURE_DEPLOYMENT="gpt-4"

# Optional: Set rate limits
export SYSTEM_PROVIDER_OPENAI_MAX_DAILY_REQUESTS="1000"

# Initialize providers from environment variables
startup-cli providers init
```

#### CLI Flag Configuration

Override or provide credentials directly via command-line flags:

```bash
# OpenAI provider
startup-cli providers init \
  --openai-key "sk-..." \
  --openai-base-url "https://api.openai.com/v1"

# Multiple providers
startup-cli providers init \
  --openai-key "sk-..." \
  --anthropic-key "sk-ant-..." \
  --azure-key "..." \
  --azure-endpoint "https://your-resource.openai.azure.com" \
  --azure-deployment "gpt-4"

# Specific providers only
startup-cli providers init \
  --providers "openai,anthropic" \
  --openai-key "sk-..." \
  --anthropic-key "sk-ant-..."
```

#### Provider Types

Supported provider types:

- **`openai`**: OpenAI GPT models (GPT-4, GPT-3.5, etc.)
- **`anthropic`**: Anthropic Claude models  
- **`azure`**: Azure OpenAI Service
- **`custom`**: Custom provider implementations (LM Studio, Ollama, etc.)

#### Configuration Priority

Settings are applied in the following order (later values override earlier ones):

1. Environment variables (`SYSTEM_PROVIDER_*`)
2. CLI flags (`--openai-key`, `--anthropic-key`, etc.)

#### Examples

```bash
# Quick setup with just OpenAI
export SYSTEM_PROVIDER_OPENAI_API_KEY="sk-..."
startup-cli providers init

# Production setup with multiple providers
export SYSTEM_PROVIDER_OPENAI_API_KEY="sk-..."
export SYSTEM_PROVIDER_ANTHROPIC_API_KEY="sk-ant-..."
export SYSTEM_PROVIDER_AZURE_API_KEY="..."
export SYSTEM_PROVIDER_AZURE_ENDPOINT="https://company.openai.azure.com"
export SYSTEM_PROVIDER_AZURE_DEPLOYMENT="gpt-4"
startup-cli providers init

# One-time setup with CLI flags
startup-cli providers init \
  --openai-key "sk-..." \
  --anthropic-key "sk-ant-..." \
  --dry-run  # Preview first

# After reviewing, run for real
startup-cli providers init \
  --openai-key "sk-..." \
  --anthropic-key "sk-ant-..."

# Test connectivity before creating
startup-cli providers init --dry-run --openai-key "sk-..."
```

#### Custom Providers (LM Studio, Ollama, etc.)

For local or custom AI providers, use environment variables with custom names:

```bash
# LM Studio (local OpenAI-compatible API)
export SYSTEM_PROVIDER_LMSTUDIO_API_KEY="not-needed"  # LM Studio typically doesn't require auth
export SYSTEM_PROVIDER_LMSTUDIO_BASE_URL="http://localhost:1234/v1"
export SYSTEM_PROVIDER_LMSTUDIO_PROVIDER_TYPE="openai"  # Use OpenAI-compatible format

# Ollama (local API)
export SYSTEM_PROVIDER_OLLAMA_API_KEY="not-needed"
export SYSTEM_PROVIDER_OLLAMA_BASE_URL="http://localhost:11434/v1"
export SYSTEM_PROVIDER_OLLAMA_PROVIDER_TYPE="openai"

# Custom provider with authentication
export SYSTEM_PROVIDER_MYAPI_API_KEY="your-custom-key"
export SYSTEM_PROVIDER_MYAPI_BASE_URL="https://my-ai-api.com/v1"
export SYSTEM_PROVIDER_MYAPI_PROVIDER_TYPE="openai"
export SYSTEM_PROVIDER_MYAPI_MAX_DAILY_REQUESTS="500"

# Initialize custom providers
startup-cli providers init
```

**Environment Variables:**

**Service Configuration:**
- `ZITADEL_URL`: Zitadel authentication server URL (default: `http://localhost:5150`)
- `API_BASE_URL`: Main API service URL (default: `http://localhost:5151`)

**Custom Provider Naming Convention:**
- Environment variables follow the pattern: `SYSTEM_PROVIDER_{NAME}_{SETTING}`
- `{NAME}` can be any alphanumeric identifier (e.g., `LMSTUDIO`, `OLLAMA`, `MYAPI`)
- Available settings:
  - `API_KEY`: Authentication key (use "not-needed" for local APIs without auth)
  - `BASE_URL`: API endpoint URL
  - `PROVIDER_TYPE`: Backend type (`openai`, `anthropic`, `azure`, or `custom`)
  - `MAX_DAILY_REQUESTS`: Optional rate limit

**Example: Complete LM Studio Setup**

1. Start LM Studio with server mode:
   ```bash
   # In LM Studio, go to "Local Server" tab and start server
   # Default endpoint: http://localhost:1234
   ```

2. Configure and initialize provider:
   ```bash
   export SYSTEM_PROVIDER_LMSTUDIO_API_KEY="not-needed"
   export SYSTEM_PROVIDER_LMSTUDIO_BASE_URL="http://localhost:1234/v1"
   export SYSTEM_PROVIDER_LMSTUDIO_PROVIDER_TYPE="openai"
   
   # Test the configuration first
   startup-cli providers init --dry-run
   
   # Create the provider
   startup-cli providers init
   ```

**Example: Ollama Setup**

1. Start Ollama service:
   ```bash
   ollama serve
   # Default endpoint: http://localhost:11434
   ```

2. Configure provider:
   ```bash
   export SYSTEM_PROVIDER_OLLAMA_API_KEY="not-needed"
   export SYSTEM_PROVIDER_OLLAMA_BASE_URL="http://localhost:11434/v1"
   export SYSTEM_PROVIDER_OLLAMA_PROVIDER_TYPE="openai"
   
   startup-cli providers init
   ```

#### Troubleshooting

**Authentication Error**: "No Zitadel service account credentials found"
```bash
# Solution: Initialize Zitadel service accounts first
startup-cli zitadel init --services api
```

**No Providers Found**: "No provider configurations found"
```bash
# Solution: Set environment variables or use CLI flags
export SYSTEM_PROVIDER_OPENAI_API_KEY="sk-..."
# OR
startup-cli providers init --openai-key "sk-..."
```

**Connectivity Test Failed**: Check your API keys and network connectivity
```bash
# Test with dry-run first to validate configuration
startup-cli providers init --dry-run --openai-key "sk-..."
```

#### Getting Help

```bash
# See all available provider commands
startup-cli providers --help

# Get detailed help for the init command
startup-cli providers init --help
```

## Logging

Control log verbosity with the `RUST_LOG` environment variable:

```bash
# Show all logs
RUST_LOG=debug startup-cli database create-schema

# Show only info and above
RUST_LOG=info startup-cli api-keys generate ...

# Show only warnings and errors
RUST_LOG=warn startup-cli db create-test-db
```

## Command Aliases

- `db` = `database`
- `keys` = `api-keys`

## Available Commands

- `database` (`db`) - Database management commands
- `github` (`gh`) - GitHub Actions and cluster management  
- `init` - Initialize databases and service accounts
- `providers` - System provider management (OpenAI, Anthropic, Azure)
- `zitadel` - Zitadel service account management
- `api-keys` (`keys`) - API key generation and management

## Replaced Python Scripts

This CLI replaces the following Python maintenance scripts:

1. **`apps/agent-api/scripts/create-schema.py`** → `startup-cli db create-schema`
2. **`apps/agent-api/scripts/create-test-db.py`** → `startup-cli db create-test-db`
3. **`infrastructure/api-keys/scripts/setup_api_keys.py`** → `startup-cli keys generate`

## Benefits over Python Scripts

- **Faster startup**: No Python interpreter overhead
- **Better UX**: Progress indicators and professional logging
- **Self-contained**: Single binary with no dependency management
- **Type safety**: Rust's compile-time guarantees
- **Cross-platform**: Works on all platforms without Python environment setup
- **Consistent interface**: Unified CLI with help documentation

## Development

To add new commands:

1. Create a new module in `src/commands/`
2. Follow the pattern in `src/commands/example.rs`
3. Add the module to `src/commands.rs`
4. Add the command to the `Commands` enum in `src/main.rs`
5. Handle the command in the match statement

The CLI uses:
- `clap` with derive features for argument parsing
- `tokio` for async database operations
- `sqlx` for PostgreSQL connectivity
- `indicatif` for progress indicators
- `env_logger` + `log` for structured logging
- `xshell` for any shell command execution needs