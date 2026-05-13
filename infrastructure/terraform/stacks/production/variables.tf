# ============================================================================
# GCP Configuration
# ============================================================================

variable "gcp_project_id" {
  description = "GCP project ID for production environment (set via TF_VAR_gcp_project_id)"
  type        = string
}

variable "gcp_region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "domain" {
  description = "Base domain for the application"
  type        = string
}

# ============================================================================
# GKE Configuration
# ============================================================================

variable "gke_node_machine_type" {
  description = "Machine type for GKE nodes"
  type        = string
  default     = "e2-standard-8"
}

variable "gke_min_nodes" {
  description = "Minimum number of nodes per zone"
  type        = number
  default     = 2
}

variable "gke_max_nodes" {
  description = "Maximum number of nodes per zone"
  type        = number
  default     = 10
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
