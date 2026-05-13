output "namespace" {
  description = "Namespace where External Secrets Operator is installed"
  value       = kubernetes_namespace.external_secrets.metadata[0].name
}

output "cluster_secret_store_name" {
  description = "Name of the ClusterSecretStore"
  value       = "gcp-secret-store"
}

output "service_account_name" {
  description = "Kubernetes service account name"
  value       = kubernetes_service_account.external_secrets.metadata[0].name
}
