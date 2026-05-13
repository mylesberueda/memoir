variable "environment" {
  description = "Environment name (local, staging, production)"
  type        = string
}

variable "zitadel_domain" {
  description = "Zitadel domain (without protocol)"
  type        = string
  default     = "localhost"
}

variable "zitadel_port" {
  description = "Zitadel port"
  type        = string
  default     = "5150"
}

variable "zitadel_insecure" {
  description = "Allow insecure connections (for local development with http)"
  type        = bool
  default     = true
}

variable "zitadel_jwt_profile_file" {
  description = "Path to Zitadel JWT profile JSON file"
  type        = string
}

variable "zitadel_org_name" {
  description = "Zitadel organization name"
  type        = string
  default     = "memoir"
}

variable "project_name" {
  description = "Name of the Zitadel project"
  type        = string
  default     = "memoir"
}

variable "zitadel_redirect_uri" {
  description = "Redirect URI for OIDC authentication"
  type        = string
  default     = "http://localhost:3000/api/auth/callback"
}

variable "app_login_url" {
  description = "Base URL for custom login UI"
  type        = string
  default     = "http://localhost:3000"
}

variable "app_internal_url" {
  description = "Internal URL for server-to-server calls (webhooks)"
  type        = string
  default     = "http://host.docker.internal:3000"
}

# SMTP Configuration
variable "zitadel_smtp_sender_address" {
  description = "Email address used as sender"
  type        = string
  default     = "noreply@memoir.local"
}

variable "zitadel_smtp_sender_name" {
  description = "Display name used as sender"
  type        = string
  default     = "Memoir"
}

variable "zitadel_smtp_host" {
  description = "SMTP server host:port"
  type        = string
  default     = "maildev:1025"
}

variable "zitadel_smtp_tls" {
  description = "Use TLS for SMTP connection"
  type        = bool
  default     = false
}

variable "zitadel_smtp_user" {
  description = "SMTP authentication username"
  type        = string
  default     = ""
}

variable "zitadel_smtp_password" {
  description = "SMTP authentication password"
  type        = string
  default     = ""
  sensitive   = true
}

# Optional Identity Providers
variable "discord_client_id" {
  description = "Discord OAuth client ID (optional)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "discord_client_secret" {
  description = "Discord OAuth client secret (optional)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "github_client_id" {
  description = "GitHub OAuth client ID (optional)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "github_client_secret" {
  description = "GitHub OAuth client secret (optional)"
  type        = string
  default     = ""
  sensitive   = true
}

