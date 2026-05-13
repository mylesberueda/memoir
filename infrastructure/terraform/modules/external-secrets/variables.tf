variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

variable "gcp_project_id" {
  description = "GCP project ID for Secret Manager"
  type        = string
}

variable "gcp_service_account_email" {
  description = "GCP service account email for Workload Identity"
  type        = string
}

variable "chart_version" {
  description = "External Secrets Operator Helm chart version"
  type        = string
  default     = "0.9.13"
}
