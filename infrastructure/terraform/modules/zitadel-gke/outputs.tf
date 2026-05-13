output "namespace" {
  description = "Namespace where Zitadel is installed"
  value       = kubernetes_namespace.zitadel.metadata[0].name
}

output "service_name" {
  description = "Zitadel service name"
  value       = "zitadel"
}

output "external_url" {
  description = "External URL for Zitadel"
  value       = "https://${var.external_domain}"
}

output "internal_url" {
  description = "Internal URL for Zitadel (within cluster)"
  value       = "http://zitadel.${kubernetes_namespace.zitadel.metadata[0].name}.svc.cluster.local:8080"
}
