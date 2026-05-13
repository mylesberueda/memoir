# Kubernetes + Helm providers for Kind cluster
# When enable_kubernetes is false, these providers are configured with a dummy
# host so they initialize without needing a running cluster. No resources use
# them when disabled, so the dummy config is never actually connected to.
provider "helm" {
  kubernetes {
    config_path    = var.enable_kubernetes ? var.kubeconfig_path : null
    config_context = var.enable_kubernetes ? var.kubeconfig_context : null
    host           = var.enable_kubernetes ? null : "https://localhost:0"
  }
}

provider "kubernetes" {
  config_path    = var.enable_kubernetes ? var.kubeconfig_path : null
  config_context = var.enable_kubernetes ? var.kubeconfig_context : null
  host           = var.enable_kubernetes ? null : "https://localhost:0"
}

# Zitadel provider
provider "zitadel" {
  domain           = var.zitadel_domain
  port             = var.zitadel_port
  insecure         = var.zitadel_insecure
  jwt_profile_file = var.zitadel_jwt_profile_file
}

# Stripe provider (uses STRIPE_API_KEY env var)
provider "stripe" {}

# PostgreSQL provider for Docker Compose postgres
provider "postgresql" {
  host     = var.postgres_host
  port     = var.postgres_port
  username = var.postgres_user
  password = var.postgres_password
  sslmode  = "disable"
}

# Docker provider for reading container IPs
# Used to create K8s Services/Endpoints for Docker Compose containers
provider "docker" {
  # Uses default Docker socket: unix:///var/run/docker.sock
  # On macOS with Docker Desktop, this works automatically
  # On Linux, ensure the user has access to the Docker socket
}
