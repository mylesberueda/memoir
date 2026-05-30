variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
}

variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

variable "instance_name" {
  description = "Cloud SQL instance name"
  type        = string
}

variable "vpc_network_id" {
  description = "VPC network ID for private IP"
  type        = string
}

variable "database_version" {
  description = "PostgreSQL version"
  type        = string
  default     = "POSTGRES_15"
}

variable "tier" {
  description = "Cloud SQL machine tier"
  type        = string
  default     = "db-custom-2-4096" # 2 vCPU, 4GB RAM
}

variable "disk_size_gb" {
  description = "Initial disk size in GB"
  type        = number
  default     = 20
}

variable "databases" {
  description = "List of database names to create. Empty until memoir-service adds memoir_service."
  type        = list(string)
  default     = []
}
