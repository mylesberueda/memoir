# Artifact Registry Module
#
# Creates:
# - Docker repository for container images
# - IAM bindings for GKE nodes to pull images

# ============================================================================
# Artifact Registry Repository
# ============================================================================

resource "google_artifact_registry_repository" "docker" {
  provider = google-beta

  project       = var.project_id
  location      = var.region
  repository_id = var.repository_name
  description   = "Docker repository for ${var.environment} environment"
  format        = "DOCKER"

  cleanup_policy_dry_run = false

  # Cleanup old images to save storage costs
  cleanup_policies {
    id     = "delete-untagged"
    action = "DELETE"

    condition {
      tag_state  = "UNTAGGED"
      older_than = "604800s" # 7 days
    }
  }

  cleanup_policies {
    id     = "keep-recent-tagged"
    action = "KEEP"

    most_recent_versions {
      keep_count = 10
    }
  }

  labels = {
    environment = var.environment
    managed_by  = "terraform"
  }
}

# ============================================================================
# IAM - Allow GKE nodes to pull images
# ============================================================================

resource "google_artifact_registry_repository_iam_member" "gke_reader" {
  provider = google-beta

  project    = var.project_id
  location   = var.region
  repository = google_artifact_registry_repository.docker.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${var.gke_service_account_email}"
}

# Allow CI/CD to push images (if service account provided)
resource "google_artifact_registry_repository_iam_member" "cicd_writer" {
  count    = var.cicd_service_account_email != "" ? 1 : 0
  provider = google-beta

  project    = var.project_id
  location   = var.region
  repository = google_artifact_registry_repository.docker.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${var.cicd_service_account_email}"
}
