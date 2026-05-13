# GCP Secrets Module
#
# Creates:
# - Secret Manager secrets for each service
# - IAM bindings for secret access
#
# Note: Secret VALUES should be populated manually or via CI/CD,
# not stored in Terraform state.

# ============================================================================
# Secret Manager Secrets
# ============================================================================

resource "google_secret_manager_secret" "service_secrets" {
  for_each = var.secrets

  secret_id = replace(each.key, "/", "-") # staging/api-service -> staging-api-service
  project   = var.project_id

  labels = {
    environment = var.environment
    service     = try(split("/", each.key)[1], each.key)
    managed_by  = "terraform"
  }

  replication {
    auto {}
  }
}

# ============================================================================
# IAM - Grant access to External Secrets Operator
# ============================================================================

resource "google_secret_manager_secret_iam_member" "eso_accessor" {
  for_each = var.secrets

  project   = var.project_id
  secret_id = google_secret_manager_secret.service_secrets[each.key].secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${var.eso_service_account_email}"
}

# ============================================================================
# Optional: Seed secrets with initial values from Cloud SQL
# ============================================================================

resource "google_secret_manager_secret_version" "initial_values" {
  for_each = { for k, v in var.secrets : k => v if v.initial_value != null }

  secret      = google_secret_manager_secret.service_secrets[each.key].id
  secret_data = each.value.initial_value

  lifecycle {
    # Don't update if secret has been modified outside Terraform
    ignore_changes = [secret_data]
  }
}
