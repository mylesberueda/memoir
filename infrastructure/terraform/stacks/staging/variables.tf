# ============================================================================
# GCP Configuration
# ============================================================================

variable "gcp_project_id" {
  description = "GCP project ID for staging environment (set via TF_VAR_gcp_project_id)"
  type        = string
}

variable "gcp_region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "domain" {
  description = "Base domain for the application (e.g., example.com -> staging.example.com)"
  type        = string
}

# ============================================================================
# GKE Configuration
# ============================================================================

variable "gke_node_machine_type" {
  description = "Machine type for GKE nodes"
  type        = string
  default     = "e2-standard-4"
}

variable "gke_min_nodes" {
  description = "Minimum number of nodes per zone"
  type        = number
  default     = 1
}

variable "gke_max_nodes" {
  description = "Maximum number of nodes per zone"
  type        = number
  default     = 5
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
  description = "GitHub Personal Access Token for private repo access"
  type        = string
  default     = ""
  sensitive   = true
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
# Zitadel Configuration
# ============================================================================

variable "zitadel_jwt_profile_file" {
  description = "Path to Zitadel JWT profile file for admin access"
  type        = string
  default     = ""
}

variable "zitadel_org_name" {
  description = "Zitadel organization name"
  type        = string
  default     = "memoir"
}

# ============================================================================
# Identity Providers (Optional)
# ============================================================================

variable "discord_client_id" {
  description = "Discord OAuth client ID"
  type        = string
  default     = ""
  sensitive   = true
}

variable "discord_client_secret" {
  description = "Discord OAuth client secret"
  type        = string
  default     = ""
  sensitive   = true
}

variable "github_client_id" {
  description = "GitHub OAuth client ID"
  type        = string
  default     = ""
  sensitive   = true
}

variable "github_client_secret" {
  description = "GitHub OAuth client secret"
  type        = string
  default     = ""
  sensitive   = true
}

# ============================================================================
# Stripe Configuration
# ============================================================================

variable "stripe_price_plus_cents" {
  description = "Plus tier monthly price in cents"
  type        = number
  default     = 2000
}

variable "stripe_price_pro_cents" {
  description = "Pro tier monthly price in cents"
  type        = number
  default     = 10000
}

variable "stripe_price_enterprise_cents" {
  description = "Enterprise tier monthly price in cents"
  type        = number
  default     = 50000
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
