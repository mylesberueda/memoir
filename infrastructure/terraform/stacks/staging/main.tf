# Staging Cluster Configuration
#
# One `terraform apply` to stand up everything:
# - GCP networking (VPC, subnets, NAT)
# - GKE cluster with Workload Identity
# - Cloud SQL PostgreSQL
# - Artifact Registry
# - GCP Secret Manager secrets
# - External Secrets Operator
# - ArgoCD for GitOps
#
# Auth (Zitadel) and billing (Stripe) were removed in epic 0002. The future
# memoir-server epic will introduce the production auth replacement.
# Service-specific blocks are intentionally empty until the memoir-server
# epic populates them.

locals {
  environment = "staging"
}

# ============================================================================
# GCP Networking
# ============================================================================

module "networking" {
  source = "../../modules/gcp-networking"

  project_id  = var.gcp_project_id
  region      = var.gcp_region
  environment = local.environment

  subnet_cidr         = "10.0.0.0/20"
  pods_cidr_range     = "10.1.0.0/16"
  services_cidr_range = "10.2.0.0/20"
}

# ============================================================================
# Artifact Registry
# ============================================================================

module "artifact_registry" {
  source = "../../modules/artifact-registry"

  project_id      = var.gcp_project_id
  region          = var.gcp_region
  environment     = local.environment
  repository_name = "memoir"

  # GKE nodes can pull images after cluster is created
  gke_service_account_email = ""
}

# ============================================================================
# Cloud SQL (PostgreSQL)
# ============================================================================

module "cloud_sql" {
  source = "../../modules/cloud-sql"

  project_id     = var.gcp_project_id
  region         = var.gcp_region
  environment    = local.environment
  instance_name  = "${local.environment}-postgres"
  vpc_network_id = module.networking.vpc_id

  tier         = "db-custom-2-4096" # 2 vCPU, 4GB RAM
  disk_size_gb = 20

  # Databases are populated by the memoir-server epic. The template's
  # ["zitadel", "api", "chat", "notification", "rig"] list was deleted with
  # the corresponding services in epic 0002.
  databases = []

  depends_on = [module.networking]
}

# ============================================================================
# Memorystore (Redis)
# ============================================================================

module "memorystore" {
  source = "../../modules/memorystore"

  project_id     = var.gcp_project_id
  region         = var.gcp_region
  environment    = local.environment
  instance_name  = "${local.environment}-redis"
  vpc_network_id = module.networking.vpc_id

  memory_size_gb = 1 # 1GB for staging

  depends_on = [module.networking]
}

# ============================================================================
# GKE Cluster
# ============================================================================

module "gke" {
  source = "../../modules/gke-cluster"

  project_id   = var.gcp_project_id
  region       = var.gcp_region
  environment  = local.environment
  cluster_name = "${local.environment}-cluster"

  network_id         = module.networking.vpc_id
  subnetwork_id      = module.networking.subnet_id
  pods_cidr_name     = module.networking.pods_cidr_name
  services_cidr_name = module.networking.services_cidr_name

  node_machine_type = var.gke_node_machine_type
  min_node_count    = var.gke_min_nodes
  max_node_count    = var.gke_max_nodes

  depends_on = [module.networking]
}

# ============================================================================
# Workload Identity
# ============================================================================

module "workload_identity" {
  source = "../../modules/gcp-workload-identity"

  project_id  = var.gcp_project_id
  environment = local.environment

  # Application service accounts are populated by the memoir-server epic.
  # Only infrastructure SAs remain here.
  service_accounts = {
    "external-secrets" = {
      k8s_namespace         = "external-secrets"
      k8s_service_account   = "external-secrets"
      secret_manager_access = true
    }
  }

  depends_on = [module.gke]
}

# ============================================================================
# GCP Secret Manager
# ============================================================================

module "secrets" {
  source = "../../modules/gcp-secrets"

  project_id  = var.gcp_project_id
  environment = local.environment

  eso_service_account_email = module.workload_identity.service_account_emails["external-secrets"]

  # Per-service secrets are populated by the memoir-server epic.
  # Only the terraform-outputs placeholder remains.
  secrets = {
    "staging/terraform-outputs" = {
      initial_value = null
    }
  }

  depends_on = [module.workload_identity, module.cloud_sql, module.memorystore]
}

# ============================================================================
# External Secrets Operator
# ============================================================================

module "external_secrets" {
  source = "../../modules/external-secrets"

  environment               = local.environment
  gcp_project_id            = var.gcp_project_id
  gcp_service_account_email = module.workload_identity.service_account_emails["external-secrets"]

  depends_on = [module.gke, module.workload_identity]
}

# ============================================================================
# ArgoCD
# ============================================================================

module "argocd" {
  source = "../../modules/argocd"

  environment         = local.environment
  github_repo_url     = var.github_repo_url
  github_argocd_token = var.github_argocd_token
  services            = var.argocd_services

  depends_on = [module.external_secrets]
}

# ============================================================================
# Cloudflare Tunnel (Optional - for instant ArgoCD sync from CI)
# ============================================================================
# Only deployed if cloudflare_tunnel_token is set.
# Creates an outbound-only tunnel so GitHub Actions can reach ArgoCD
# without exposing it publicly.

resource "kubernetes_deployment" "cloudflared" {
  count = var.cloudflare_tunnel_token != "" ? 1 : 0

  metadata {
    name      = "cloudflared"
    namespace = "argocd"
  }

  spec {
    replicas = 2 # HA for production environments

    selector {
      match_labels = {
        app = "cloudflared"
      }
    }

    template {
      metadata {
        labels = {
          app = "cloudflared"
        }
      }

      spec {
        container {
          name  = "cloudflared"
          image = "cloudflare/cloudflared:latest"
          args  = ["tunnel", "--no-autoupdate", "run", "--token", var.cloudflare_tunnel_token]

          resources {
            requests = {
              cpu    = "10m"
              memory = "32Mi"
            }
            limits = {
              cpu    = "100m"
              memory = "128Mi"
            }
          }
        }
      }
    }
  }

  depends_on = [module.argocd]
}

# ============================================================================
# Store Terraform Outputs in Secret Manager
# ============================================================================

resource "google_secret_manager_secret_version" "terraform_outputs" {
  secret = module.secrets.secret_ids["staging/terraform-outputs"]

  secret_data = jsonencode({
    environment = local.environment
    gcp_project = var.gcp_project_id
    gcp_region  = var.gcp_region

    cluster = {
      name     = module.gke.cluster_name
      endpoint = module.gke.cluster_endpoint
      location = module.gke.cluster_location
    }

    urls = {
      argocd = "https://argocd.${local.environment}.${var.domain}"
      app    = "https://${local.environment}.${var.domain}"
      api    = "https://api.${local.environment}.${var.domain}"
    }

    static_ips = {
      ingress = module.gke.ingress_ip
      argocd  = module.gke.argocd_ip
    }

    artifact_registry = {
      url = module.artifact_registry.repository_url
    }

    redis = {
      host              = module.memorystore.host
      port              = module.memorystore.port
      connection_string = module.memorystore.connection_string
    }

    generated_at = timestamp()
  })

  depends_on = [module.secrets]
}
