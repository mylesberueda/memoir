# ============================================================================
# Kubernetes Configuration
# ============================================================================

variable "enable_kubernetes" {
  description = "Enable Kubernetes-dependent resources (ArgoCD, docker-services bridge, GHCR pull secret). Set to false to run Postgres setup without a Kind cluster."
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
  description = "Services to deploy via ArgoCD. Empty until the memoir-server epic populates this with memoir-server and memoir-ui."
  type = list(object({
    name    = string
    enabled = bool
  }))
  default = []
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
  description = "List of databases to create for services."
  type        = list(string)
  default     = ["memoir_service", "memoir_service_test"]
}

# ============================================================================
# Redis Configuration (Docker Compose)
# ============================================================================

variable "redis_port" {
  description = "Redis port (mapped from Docker Compose)"
  type        = number
  default     = 63791
}
