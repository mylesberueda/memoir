output "cluster_name" {
  description = "Name of the Kind cluster"
  value       = var.cluster_name
}

output "arc_controller_version" {
  description = "Installed ARC controller version"
  value       = helm_release.arc_controller.version
}

output "arc_runner_set_version" {
  description = "Installed ARC runner scale set version"
  value       = helm_release.arc_runner_set.version
}

output "minio_enabled" {
  description = "Whether MinIO is enabled"
  value       = var.minio_enabled
}

output "minio_endpoint" {
  description = "MinIO endpoint URL (if enabled)"
  value       = var.minio_enabled ? "http://minio.minio-system.svc.cluster.local:9000" : null
}

output "github_config_url" {
  description = "GitHub repository/organization URL"
  value       = var.github_config_url
}

output "runner_namespace" {
  description = "Namespace where runners are deployed"
  value       = kubernetes_namespace.arc_runners.metadata[0].name
}

output "usage_instructions" {
  description = "How to use the GitHub Actions runners"
  value       = <<-EOT

    🎉 GitHub Actions Runners deployed successfully!

    📋 Usage in GitHub Actions workflows:
       runs-on: arc-runner-set

    🗄️  MinIO cache storage: ${var.minio_enabled ? "http://minio.minio-system.svc.cluster.local:9000" : "disabled"}
    🪣  Cache bucket: github-actions-cache
    🔐 MinIO credentials: Set via TF_VAR_minio_access_key / TF_VAR_minio_secret_key

    🔍 Check runner status:
       kubectl get pods -n arc-runners
       kubectl get pods -n arc-systems | grep listener

  EOT
}
