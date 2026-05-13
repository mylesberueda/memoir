# ============================================================================
# Kubernetes Configuration
# ============================================================================

variable "enable_kubernetes" {
  description = "Enable Kubernetes-dependent resources (ArgoCD, docker-services bridge, GHCR pull secret). Set to false to run Zitadel/Stripe/Postgres setup without a Kind cluster."
  type        = bool
  default     = false
}

variable "kubeconfig_path" {
  description = "Path to kubeconfig file"
  type        = string
  default     = "~/.kube/config"
}

variable "kubeconfig_context" {
  description = "Kubeconfig context for Kind cluster"
  type        = string
  default     = "kind-memoir"
}

# ============================================================================
# ArgoCD Configuration
# ============================================================================

variable "github_repo_url" {
  description = "GitHub repository URL for ArgoCD"
  type        = string
  default     = "https://github.com/mylesberueda/memoir"
}

variable "github_argocd_token" {
  description = "GitHub PAT for private repo access"
  type        = string
  default     = ""
  sensitive   = true
}

variable "argocd_target_revision" {
  description = "Git revision for ArgoCD to track (branch, tag, or HEAD)"
  type        = string
  default     = "HEAD"
}

variable "argocd_services" {
  description = "Services to deploy via ArgoCD"
  type = list(object({
    name    = string
    enabled = bool
  }))
  default = [
    { name = "api-service", enabled = true },
    { name = "rig-service", enabled = true },
    { name = "chat-service", enabled = true },
    { name = "notification-service", enabled = true },
    { name = "web", enabled = true }
  ]
}

# ============================================================================
# Cloudflare Tunnel (Optional - for instant ArgoCD sync)
# ============================================================================

variable "cloudflare_tunnel_token" {
  description = "Cloudflare Tunnel token (from `cloudflared tunnel token <TUNNEL_NAME>`). If set, deploys cloudflared to the cluster."
  type        = string
  default     = ""
  sensitive   = true
}

# ============================================================================
# GitHub Container Registry (for pulling images from ghcr.io)
# ============================================================================
# Fine-grained tokens don't support GHCR - requires a classic PAT with read:packages
# See: https://github.com/orgs/community/discussions/38467

variable "github_username" {
  description = "GitHub username for ghcr.io registry access"
  type        = string
  default     = "mylesberueda"
}

variable "argocd_ghcr_token" {
  description = "Classic GitHub PAT with read:packages scope for GHCR pulls (fine-grained tokens not supported)"
  type        = string
  default     = ""
  sensitive   = true
}

# ============================================================================
# Zitadel Configuration
# ============================================================================

variable "zitadel_domain" {
  description = "Zitadel domain"
  type        = string
  default     = "localhost"
}

variable "zitadel_port" {
  description = "Zitadel port"
  type        = string
  default     = "5150"
}

variable "zitadel_insecure" {
  description = "Allow insecure Zitadel connections"
  type        = bool
  default     = true
}

variable "zitadel_jwt_profile_file" {
  description = "Path to Zitadel JWT profile"
  type        = string
  default     = "../../../../.data/zitadel/zitadel-admin-sa.json"
}

variable "zitadel_org_name" {
  description = "Zitadel organization name"
  type        = string
  default     = "memoir"
}

variable "zitadel_redirect_uri" {
  description = "OIDC redirect URI"
  type        = string
  default     = "http://localhost:3000/api/auth/callback"
}

variable "app_login_url" {
  description = "App login URL"
  type        = string
  default     = "http://localhost:3000"
}

variable "app_internal_url" {
  description = "Internal URL for webhooks"
  type        = string
  default     = "http://host.docker.internal:3000"
}

# Optional identity providers
variable "discord_client_id" {
  type      = string
  default   = ""
  sensitive = true
}

variable "discord_client_secret" {
  type      = string
  default   = ""
  sensitive = true
}

variable "github_client_id" {
  type      = string
  default   = ""
  sensitive = true
}

variable "github_client_secret" {
  type      = string
  default   = ""
  sensitive = true
}

# ============================================================================
# Stripe Configuration
# ============================================================================

variable "stripe_price_plus_cents" {
  type    = number
  default = 2000
}

variable "stripe_price_pro_cents" {
  type    = number
  default = 10000
}

variable "stripe_price_enterprise_cents" {
  type    = number
  default = 50000
}

variable "stripe_webhook_url" {
  type    = string
  default = "http://localhost:5154"
}

# ============================================================================
# PostgreSQL Configuration (Docker Compose)
# ============================================================================

variable "postgres_host" {
  description = "PostgreSQL host (Docker Compose container reachable from host)"
  type        = string
  default     = "localhost"
}

variable "postgres_port" {
  description = "PostgreSQL port"
  type        = number
  default     = 54321
}

variable "postgres_user" {
  description = "PostgreSQL admin user"
  type        = string
  default     = "postgres"
}

variable "postgres_password" {
  description = "PostgreSQL admin password"
  type        = string
  default     = "postgres"
  sensitive   = true
}

variable "postgres_databases" {
  description = "List of databases to create for services"
  type        = list(string)
  default = [
    "api_service", "api_service_test",
    "chat_service", "chat_service_test",
    "notification_service", "notification_service_test",
    "rig_service", "rig_service_test",
    "startup", "startup_test"
  ]
}

# ============================================================================
# Redis Configuration (Docker Compose)
# ============================================================================

variable "redis_port" {
  description = "Redis port (mapped from Docker Compose)"
  type        = number
  default     = 63791
}

