variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

variable "service_accounts" {
  description = "Map of service accounts to create with their configurations"
  type = map(object({
    k8s_namespace         = string
    k8s_service_account   = string
    secret_manager_access = optional(bool, false)
    cloud_sql_client      = optional(bool, false)
    storage_viewer        = optional(bool, false)
    pubsub_publisher      = optional(bool, false)
    pubsub_subscriber     = optional(bool, false)
  }))
}
