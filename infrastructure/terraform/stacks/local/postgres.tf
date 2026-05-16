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
  # Build DATABASE_URL for each database.
  # Pre-cleanup this map associated database name -> service name (e.g.
  # "api_service" -> "api-service") so that downstream consumers could look
  # up the right URL by service name. After 0002's deletes, no services
  # remain — the future memoir-server epic re-introduces a single
  # database_name -> service_name binding for memoir_server.
  #
  # For Kind pods: use container hostname (memoir-postgres:5432).
  # For local dev: use localhost:54321.
  database_urls = [
    for db in var.postgres_databases : {
      service  = db
      database = db
      cluster  = "postgres://${var.postgres_user}:${var.postgres_password}@memoir-postgres:5432/${db}"
      local    = "postgres://${var.postgres_user}:${var.postgres_password}@localhost:${var.postgres_port}/${db}"
    }
  ]

  # REDIS_URL per service.
  # Empty until the memoir-server epic adds memoir-server as a Redis consumer.
  # Even then, Memoir's deployment-tier model (README: cache profile is opt-in)
  # means Redis may not be a required consumer for the default profile.
  redis_urls = []
}
