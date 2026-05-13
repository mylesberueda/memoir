variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

variable "secrets" {
  description = "Map of secrets to create. Key is the secret path (e.g., staging/api-service)"
  type = map(object({
    initial_value = optional(string)
  }))
}

variable "eso_service_account_email" {
  description = "External Secrets Operator service account email"
  type        = string
}
