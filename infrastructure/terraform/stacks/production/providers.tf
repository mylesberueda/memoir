# ============================================================================
# Google Cloud Provider
# ============================================================================

provider "google" {
  project = var.gcp_project_id
  region  = var.gcp_region
}

provider "google-beta" {
  project = var.gcp_project_id
  region  = var.gcp_region
}

# ============================================================================
# Kubernetes Provider (configured after GKE cluster is created)
# ============================================================================

data "google_client_config" "default" {}

provider "kubernetes" {
  host                   = "https://${module.gke.cluster_endpoint}"
  token                  = data.google_client_config.default.access_token
  cluster_ca_certificate = base64decode(module.gke.cluster_ca_certificate)
}

# ============================================================================
# Helm Provider
# ============================================================================

provider "helm" {
  kubernetes {
    host                   = "https://${module.gke.cluster_endpoint}"
    token                  = data.google_client_config.default.access_token
    cluster_ca_certificate = base64decode(module.gke.cluster_ca_certificate)
  }
}

# ============================================================================
# Zitadel Provider (for zitadel module)
# ============================================================================

provider "zitadel" {
  domain           = "auth.staging.${var.domain}"
  port             = "443"
  insecure         = false
  jwt_profile_file = var.zitadel_jwt_profile_file
}

# ============================================================================
# Stripe Provider (uses STRIPE_API_KEY env var)
# ============================================================================

provider "stripe" {}
