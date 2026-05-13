output "secrets" {
  description = "Map of created secrets"
  value = {
    for k, secret in google_secret_manager_secret.service_secrets : k => {
      id        = secret.id
      secret_id = secret.secret_id
      name      = secret.name
    }
  }
}

output "secret_ids" {
  description = "Map of secret keys to their IDs"
  value       = { for k, secret in google_secret_manager_secret.service_secrets : k => secret.id }
}
