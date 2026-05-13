variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region for the registry"
  type        = string
}

variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

variable "repository_name" {
  description = "Name of the Artifact Registry repository"
  type        = string
  default     = "memoir"
}

variable "gke_service_account_email" {
  description = "GKE node service account email (for pull access)"
  type        = string
  default     = ""
}

variable "cicd_service_account_email" {
  description = "CI/CD service account email (for push access)"
  type        = string
  default     = ""
}
