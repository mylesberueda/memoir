# Cluster Configuration
variable "cluster_name" {
  description = "Name of the Kind cluster"
  type        = string
  default     = "memoir-github-runners"
}

variable "kubeconfig_path" {
  description = "Path to kubeconfig file"
  type        = string
  default     = "~/.kube/config"
}

# GitHub Configuration
variable "github_config_url" {
  description = "GitHub repository or organization URL"
  type        = string
  default     = "https://github.com/mylesberueda/memoir"
}

variable "github_runner_token" {
  description = "GitHub Personal Access Token for ARC runner registration"
  type        = string
  sensitive   = true
}

# ARC Versions
variable "arc_controller_version" {
  description = "Version of ARC controller Helm chart"
  type        = string
  default     = "0.12.1"
}

variable "arc_runner_scale_set_version" {
  description = "Version of ARC runner scale set Helm chart"
  type        = string
  default     = "0.12.1"
}

# Runner Configuration
variable "runner_min" {
  description = "Minimum number of runners"
  type        = number
  default     = 0
}

variable "runner_max" {
  description = "Maximum number of runners"
  type        = number
  default     = 5
}

variable "runner_group" {
  description = "GitHub runner group"
  type        = string
  default     = "default"
}

# MinIO Configuration
variable "minio_enabled" {
  description = "Enable MinIO for GitHub Actions cache"
  type        = bool
  default     = true
}

variable "minio_access_key" {
  description = "MinIO access key"
  type        = string
  sensitive   = true
}

variable "minio_secret_key" {
  description = "MinIO secret key"
  type        = string
  sensitive   = true
}

variable "buildkit_aws_ec2_metadata_disabled" {
  description = "Disable AWS EC2 IMDS for BuildKit (set to true for MinIO, false for real AWS with IRSA)"
  type        = bool
  default     = true
}

# Resource Limits
variable "controller_cpu_limit" {
  description = "CPU limit for ARC controller"
  type        = string
  default     = "500m"
}

variable "controller_memory_limit" {
  description = "Memory limit for ARC controller"
  type        = string
  default     = "512Mi"
}

variable "runner_cpu_limit" {
  description = "CPU limit for runners"
  type        = string
  default     = "4"
}

variable "runner_memory_limit" {
  description = "Memory limit for runners"
  type        = string
  default     = "12Gi"
}
