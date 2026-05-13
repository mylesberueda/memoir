# Memorystore for Redis Module
#
# Creates:
# - Redis instance with private IP (VPC peered)
# - Connection string output for services

# ============================================================================
# Memorystore Redis Instance
# ============================================================================

resource "google_redis_instance" "redis" {
  name           = var.instance_name
  project        = var.project_id
  region         = var.region
  display_name   = "${var.environment} Redis"
  memory_size_gb = var.memory_size_gb
  tier           = var.environment == "production" ? "STANDARD_HA" : "BASIC"

  # Redis version
  redis_version = var.redis_version

  # Network configuration (private IP via VPC peering)
  authorized_network = var.vpc_network_id
  connect_mode       = "PRIVATE_SERVICE_ACCESS"

  # Maintenance window (Sunday 4am UTC)
  maintenance_policy {
    weekly_maintenance_window {
      day = "SUNDAY"
      start_time {
        hours   = 4
        minutes = 0
        seconds = 0
        nanos   = 0
      }
    }
  }

  labels = {
    environment = var.environment
    managed_by  = "terraform"
  }

  lifecycle {
    prevent_destroy = false # Set to true for production if needed
  }
}
