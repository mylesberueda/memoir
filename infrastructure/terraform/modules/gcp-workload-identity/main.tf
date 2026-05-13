# GCP Workload Identity Module
#
# Creates:
# - GCP Service Accounts for Kubernetes workloads
# - IAM bindings for Workload Identity
# - Role bindings (Secret Manager, Cloud SQL, etc.)

# ============================================================================
# GCP Service Accounts
# ============================================================================

resource "google_service_account" "workload" {
  for_each = var.service_accounts

  account_id   = "${each.key}-sa"
  display_name = "${title(replace(each.key, "-", " "))} Service Account (${var.environment})"
  project      = var.project_id
}

# ============================================================================
# Workload Identity Bindings
# ============================================================================

# Allow Kubernetes Service Account to impersonate GCP Service Account
resource "google_service_account_iam_member" "workload_identity_binding" {
  for_each = var.service_accounts

  service_account_id = google_service_account.workload[each.key].name
  role               = "roles/iam.workloadIdentityUser"
  member             = "serviceAccount:${var.project_id}.svc.id.goog[${each.value.k8s_namespace}/${each.value.k8s_service_account}]"
}

# ============================================================================
# IAM Role Bindings
# ============================================================================

# Secret Manager Secret Accessor
resource "google_project_iam_member" "secret_accessor" {
  for_each = { for k, v in var.service_accounts : k => v if lookup(v, "secret_manager_access", false) }

  project = var.project_id
  role    = "roles/secretmanager.secretAccessor"
  member  = "serviceAccount:${google_service_account.workload[each.key].email}"
}

# Cloud SQL Client (for services that connect directly to Cloud SQL)
resource "google_project_iam_member" "cloudsql_client" {
  for_each = { for k, v in var.service_accounts : k => v if lookup(v, "cloud_sql_client", false) }

  project = var.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${google_service_account.workload[each.key].email}"
}

# Storage Object Viewer (for services that need to read from GCS)
resource "google_project_iam_member" "storage_viewer" {
  for_each = { for k, v in var.service_accounts : k => v if lookup(v, "storage_viewer", false) }

  project = var.project_id
  role    = "roles/storage.objectViewer"
  member  = "serviceAccount:${google_service_account.workload[each.key].email}"
}

# Pub/Sub Publisher (for services that publish messages)
resource "google_project_iam_member" "pubsub_publisher" {
  for_each = { for k, v in var.service_accounts : k => v if lookup(v, "pubsub_publisher", false) }

  project = var.project_id
  role    = "roles/pubsub.publisher"
  member  = "serviceAccount:${google_service_account.workload[each.key].email}"
}

# Pub/Sub Subscriber (for services that consume messages)
resource "google_project_iam_member" "pubsub_subscriber" {
  for_each = { for k, v in var.service_accounts : k => v if lookup(v, "pubsub_subscriber", false) }

  project = var.project_id
  role    = "roles/pubsub.subscriber"
  member  = "serviceAccount:${google_service_account.workload[each.key].email}"
}
