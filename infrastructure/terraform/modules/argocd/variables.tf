variable "environment" {
  description = "Environment name (local, staging, production)"
  type        = string

  validation {
    condition     = contains(["local", "staging", "production"], var.environment)
    error_message = "Environment must be one of: local, staging, production"
  }
}

variable "github_repo_url" {
  description = "GitHub repository URL for ArgoCD to watch"
  type        = string
  default     = "https://github.com/mylesberueda/memoir"
}

variable "github_repo_owner" {
  description = "GitHub repository owner (org or user) for OIDC RBAC"
  type        = string
  default     = "mylesberueda"
}

variable "github_repo_name" {
  description = "GitHub repository name for OIDC RBAC"
  type        = string
  default     = "memoir"
}

variable "github_argocd_token" {
  description = "GitHub Personal Access Token for private repo access (optional if repo is public)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "argocd_version" {
  description = "ArgoCD Helm chart version"
  type        = string
  default     = "9.4.1"
}

variable "ingress_nginx_version" {
  description = "Ingress-nginx Helm chart version (only used for local environment)"
  type        = string
  default     = "4.11.3"
}

variable "argocd_apps_version" {
  description = "ArgoCD Apps Helm chart version (for AppProject and ApplicationSet)"
  type        = string
  default     = "2.0.2"
}

variable "argocd_namespace" {
  description = "Namespace for ArgoCD installation"
  type        = string
  default     = "argocd"
}

variable "target_revision" {
  description = "Git revision for ArgoCD to track (branch, tag, or commit SHA). Use HEAD for default branch."
  type        = string
  default     = "HEAD"
}

variable "services" {
  description = "List of services to deploy via ArgoCD. Empty until the memoir-server epic populates this."
  type = list(object({
    name    = string
    enabled = bool
  }))
  default = []
}

