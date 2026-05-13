# ============================================================================
# ArgoCD Outputs
# ============================================================================

output "argocd_namespace" {
  value = var.enable_kubernetes ? module.argocd[0].namespace : null
}

output "argocd_admin_password_command" {
  value = var.enable_kubernetes ? module.argocd[0].admin_password_command : null
}

output "argocd_port_forward_command" {
  value = var.enable_kubernetes ? module.argocd[0].port_forward_command : null
}

# ============================================================================
# PostgreSQL Outputs
# ============================================================================

output "database_urls" {
  description = "DATABASE_URL for each service (cluster = Kind pods, local = host development)"
  value       = local.database_urls
  sensitive   = true
}

output "databases_created" {
  description = "List of databases created in PostgreSQL"
  value       = [for db in postgresql_database.service_dbs : db.name]
}

# ============================================================================
# Combined Outputs
# ============================================================================

output "terraform_outputs_location" {
  description = "Location of terraform outputs JSON file for CLI"
  value       = local_file.terraform_outputs.filename
}
