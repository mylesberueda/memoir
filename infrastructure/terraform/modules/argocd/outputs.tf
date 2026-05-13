output "namespace" {
  description = "Namespace where ArgoCD is installed"
  value       = var.argocd_namespace
}

output "server_service" {
  description = "ArgoCD server service name"
  value       = "argocd-server"
}

output "admin_password_command" {
  description = "Command to retrieve the initial admin password"
  value       = "kubectl -n ${var.argocd_namespace} get secret argocd-initial-admin-secret -o jsonpath='{.data.password}' | base64 -d"
}

output "port_forward_command" {
  description = "Command to port-forward the ArgoCD UI"
  value       = "kubectl port-forward svc/argocd-server -n ${var.argocd_namespace} 8080:443"
}
