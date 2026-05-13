# PostgreSQL Database Creation
#
# Creates databases in the Docker Compose PostgreSQL instance for each service.
# This runs during `terraform apply` and is idempotent - databases are only
# created if they don't exist.
#
# The PostgreSQL provider connects to the Docker Compose postgres container
# via localhost:54321 (the mapped port).

# ============================================================================
# Service Databases
# ============================================================================

resource "postgresql_database" "service_dbs" {
  for_each = toset(var.postgres_databases)

  name              = each.value
  owner             = var.postgres_user
  encoding          = "UTF8"
  lc_collate        = "en_US.utf8"
  lc_ctype          = "en_US.utf8"
  connection_limit  = -1
  allow_connections = true
}

# ============================================================================
# Outputs
# ============================================================================

locals {
  # Map database names to service names for clearer output
  # Database name -> Service name (as used in k8s/services)
  db_to_service = {
    "api_service"          = "api-service"
    "chat_service"         = "chat-service"
    "notification_service" = "notification-service"
    "rig_service"          = "rig-service"
  }

  # Build DATABASE_URL for each service as a list
  # For Kind pods: use container hostname (startupai-postgres:5432)
  # For local dev: use localhost:54321
  database_urls = [
    for db in var.postgres_databases : {
      service  = lookup(local.db_to_service, db, db)
      database = db
      # URL for Kind pods (via Docker network)
      cluster = "postgres://${var.postgres_user}:${var.postgres_password}@startupai-postgres:5432/${db}"
      # URL for local development (via port mapping)
      local = "postgres://${var.postgres_user}:${var.postgres_password}@localhost:${var.postgres_port}/${db}"
    }
  ]

  # Services that need Redis
  redis_services = ["api-service", "rig-service", "chat-service", "notification-service", "web"]

  # Build REDIS_URL for each service as a list (same structure as database_urls)
  # For Kind pods: use container hostname (startupai-redis:6379)
  # For local dev: use localhost:63791
  redis_urls = [
    for service in local.redis_services : {
      service = service
      # URL for Kind pods (via Docker network)
      cluster = "redis://startupai-redis:6379"
      # URL for local development (via port mapping)
      local = "redis://localhost:${var.redis_port}"
    }
  ]
}
