output "service_accounts" {
  description = "Map of created service accounts with their details"
  value = {
    for k, sa in google_service_account.workload : k => {
      email      = sa.email
      name       = sa.name
      account_id = sa.account_id
    }
  }
}

output "service_account_emails" {
  description = "Map of service account names to emails"
  value       = { for k, sa in google_service_account.workload : k => sa.email }
}

output "workload_identity_annotations" {
  description = "Map of Kubernetes service account annotations for Workload Identity"
  value = {
    for k, sa in google_service_account.workload : k => {
      "iam.gke.io/gcp-service-account" = sa.email
    }
  }
}
