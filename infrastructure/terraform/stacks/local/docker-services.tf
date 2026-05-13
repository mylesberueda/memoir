# Docker Compose Services → Kubernetes DNS Bridge
#
# Creates Kubernetes Services + Endpoints for Docker Compose containers,
# allowing pods to resolve container names (e.g., memoir-postgres:5432).
#
# This bridges the gap between Docker's DNS (which knows container names)
# and Kubernetes CoreDNS (which only knows K8s Services).
#
# On every `terraform apply`, container IPs are read from Docker and
# the Endpoints are updated if they've changed.

# ============================================================================
# Docker Container IP Lookup (via external data source)
# ============================================================================
# The kreuzwerker/docker provider doesn't support a docker_container data source.
# We use the external provider to query container IPs via docker inspect.
#
# Requirements:
# - Docker CLI must be available
# - Containers must be running on the "kind" network

data "external" "container_ip" {
  for_each = var.enable_kubernetes ? toset(["memoir-postgres", "memoir-redis", "memoir-zitadel"]) : toset([])

  program = ["bash", "-c", <<-EOF
    IP=$(docker inspect --format '{{range .NetworkSettings.Networks}}{{if eq .NetworkID "'$(docker network inspect kind --format '{{.Id}}')'"}}{{.IPAddress}}{{end}}{{end}}' ${each.value} 2>/dev/null)
    if [ -z "$IP" ]; then
      echo '{"ip": "", "error": "Container not found or not on kind network"}' >&2
      exit 1
    fi
    echo "{\"ip\": \"$IP\"}"
  EOF
  ]
}

# ============================================================================
# Local Values
# ============================================================================

locals {
  # Service definitions for Docker Compose containers
  # Each entry creates a K8s Service + Endpoints pair
  # Only populated when enable_kubernetes is true (requires Kind cluster)
  docker_services = var.enable_kubernetes ? {
    "memoir-postgres" = {
      ip   = data.external.container_ip["memoir-postgres"].result.ip
      port = 5432
    }
    "memoir-redis" = {
      ip   = data.external.container_ip["memoir-redis"].result.ip
      port = 6379
    }
    "memoir-zitadel" = {
      ip   = data.external.container_ip["memoir-zitadel"].result.ip
      port = 8080
    }
  } : {}
}

# ============================================================================
# Kubernetes Services (without selectors)
# ============================================================================
# Services without selectors don't auto-create Endpoints.
# We manually create Endpoints pointing to Docker container IPs.

resource "kubernetes_service_v1" "docker_service" {
  for_each = local.docker_services

  metadata {
    name      = each.key
    namespace = "default"
    labels = {
      "app.kubernetes.io/managed-by" = "terraform"
      "app.kubernetes.io/component"  = "docker-bridge"
    }
  }

  spec {
    # No selector - we'll manually manage Endpoints
    port {
      port        = each.value.port
      target_port = each.value.port
      protocol    = "TCP"
    }
  }
}

# ============================================================================
# Kubernetes Endpoints
# ============================================================================
# Endpoints map Service names to actual IP addresses.
# These point to Docker container IPs on the "kind" network.

resource "kubernetes_endpoints_v1" "docker_service" {
  for_each = local.docker_services

  metadata {
    name      = each.key # Must match Service name
    namespace = "default"
    labels = {
      "app.kubernetes.io/managed-by" = "terraform"
      "app.kubernetes.io/component"  = "docker-bridge"
    }
  }

  subset {
    address {
      ip = each.value.ip
    }
    port {
      port     = each.value.port
      protocol = "TCP"
    }
  }

  depends_on = [kubernetes_service_v1.docker_service]
}

# ============================================================================
# Outputs
# ============================================================================

output "docker_service_ips" {
  description = "Docker container IPs registered in Kubernetes"
  value = var.enable_kubernetes ? {
    for name, svc in local.docker_services :
    name => svc.ip
  } : {}
}
