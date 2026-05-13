# ============================================================================
# Cluster Outputs
# ============================================================================

output "cluster_name" {
  description = "GKE cluster name"
  value       = module.gke.cluster_name
}

output "cluster_endpoint" {
  description = "GKE cluster endpoint"
  value       = module.gke.cluster_endpoint
  sensitive   = true
}

output "get_credentials_command" {
  description = "Command to get cluster credentials"
  value       = module.gke.get_credentials_command
}

# ============================================================================
# Static IPs (for DNS configuration)
# ============================================================================

output "ingress_ip" {
  description = "Static IP for main application ingress"
  value       = module.gke.ingress_ip
}

output "argocd_ip" {
  description = "Static IP for ArgoCD ingress"
  value       = module.gke.argocd_ip
}

output "zitadel_ip" {
  description = "Static IP for Zitadel ingress"
  value       = module.gke.zitadel_ip
}

# ============================================================================
# URLs
# ============================================================================

output "argocd_url" {
  description = "ArgoCD URL"
  value       = "https://argocd.${var.domain}"
}

output "zitadel_url" {
  description = "Zitadel URL"
  value       = module.zitadel_gke.external_url
}

output "app_url" {
  description = "Application URL"
  value       = "https://${var.domain}"
}

# ============================================================================
# Artifact Registry
# ============================================================================

output "artifact_registry_url" {
  description = "Artifact Registry URL for Docker images"
  value       = module.artifact_registry.repository_url
}

# ============================================================================
# Database
# ============================================================================

output "cloud_sql_instance" {
  description = "Cloud SQL instance connection name"
  value       = module.cloud_sql.instance_connection_name
}

output "cloud_sql_private_ip" {
  description = "Cloud SQL private IP"
  value       = module.cloud_sql.private_ip
}

# ============================================================================
# DNS Records Required
# ============================================================================

output "dns_records_required" {
  description = "DNS A records that need to be created"
  value = {
    "${var.domain}"        = module.gke.ingress_ip
    "api.staging.${var.domain}"    = module.gke.ingress_ip
    "argocd.${var.domain}" = module.gke.argocd_ip
    "auth.${var.domain}"   = module.gke.zitadel_ip
  }
}
