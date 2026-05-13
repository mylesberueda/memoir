variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

variable "gcp_service_account_email" {
  description = "GCP service account email for Workload Identity"
  type        = string
}

variable "database_host" {
  description = "PostgreSQL host (Cloud SQL private IP)"
  type        = string
}

variable "database_name" {
  description = "PostgreSQL database name"
  type        = string
  default     = "zitadel"
}

variable "database_user" {
  description = "PostgreSQL username"
  type        = string
  default     = "zitadel"
}

variable "database_password" {
  description = "PostgreSQL password"
  type        = string
  sensitive   = true
}

variable "external_domain" {
  description = "External domain for Zitadel (e.g., auth.staging.example.com)"
  type        = string
}

variable "static_ip_name" {
  description = "Name of the static IP for ingress"
  type        = string
}

variable "master_key" {
  description = "Zitadel master key for encryption (32 bytes, base64 encoded)"
  type        = string
  sensitive   = true
}

variable "chart_version" {
  description = "Zitadel Helm chart version"
  type        = string
  default     = "8.5.0"
}
