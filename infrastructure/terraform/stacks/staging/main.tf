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
# - Zitadel (in-cluster)
# - Stripe configuration

locals {
  environment = "staging"

  # Services that need Workload Identity
  services = ["api-service", "rig-service", "chat-service", "notification-service", "web"]
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
  repository_name = "startup-ai"

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

  databases = ["zitadel", "api", "chat", "notification", "rig"]

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

  service_accounts = {
    # Application services
    "api-service" = {
      k8s_namespace         = local.environment
      k8s_service_account   = "api-service"
      secret_manager_access = true
    }
    "rig-service" = {
      k8s_namespace         = local.environment
      k8s_service_account   = "rig-service"
      secret_manager_access = true
    }
    "chat-service" = {
      k8s_namespace         = local.environment
      k8s_service_account   = "chat-service"
      secret_manager_access = true
    }
    "notification-service" = {
      k8s_namespace         = local.environment
      k8s_service_account   = "notification-service"
      secret_manager_access = true
    }
    "web" = {
      k8s_namespace         = local.environment
      k8s_service_account   = "web"
      secret_manager_access = true
    }

    # Infrastructure services
    "external-secrets" = {
      k8s_namespace         = "external-secrets"
      k8s_service_account   = "external-secrets"
      secret_manager_access = true
    }
    "zitadel" = {
      k8s_namespace         = "zitadel"
      k8s_service_account   = "zitadel"
      secret_manager_access = true
      cloud_sql_client      = true
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

  # Create secrets for each service
  # Initial values include database and Redis connection strings
  secrets = {
    "staging/api-service" = {
      initial_value = jsonencode({
        DATABASE_URL = module.cloud_sql.connection_strings["api"]
        REDIS_URL    = module.memorystore.connection_string
      })
    }
    "staging/rig-service" = {
      initial_value = jsonencode({
        DATABASE_URL = module.cloud_sql.connection_strings["rig"]
        REDIS_URL    = module.memorystore.connection_string
      })
    }
    "staging/chat-service" = {
      initial_value = jsonencode({
        DATABASE_URL = module.cloud_sql.connection_strings["chat"]
        REDIS_URL    = module.memorystore.connection_string
      })
    }
    "staging/notification-service" = {
      initial_value = jsonencode({
        DATABASE_URL = module.cloud_sql.connection_strings["notification"]
        REDIS_URL    = module.memorystore.connection_string
      })
    }
    "staging/web" = {
      initial_value = null # Web doesn't need DB or Redis
    }
    "staging/zitadel" = {
      initial_value = null
    }
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
# Zitadel (In-Cluster)
# ============================================================================

resource "random_password" "zitadel_master_key" {
  length  = 32
  special = false
}

module "zitadel_gke" {
  source = "../../modules/zitadel-gke"

  environment               = local.environment
  gcp_service_account_email = module.workload_identity.service_account_emails["zitadel"]

  database_host     = module.cloud_sql.private_ip
  database_name     = "zitadel"
  database_user     = module.cloud_sql.users["zitadel"].name
  database_password = module.cloud_sql.users["zitadel"].password

  external_domain = "auth.${local.environment}.${var.domain}"
  static_ip_name  = "zitadel-${local.environment}-ip"
  master_key      = base64encode(random_password.zitadel_master_key.result)

  depends_on = [module.gke, module.cloud_sql, module.workload_identity]
}

# ============================================================================
# Zitadel Configuration (uses existing module after Zitadel is running)
# ============================================================================

# Note: The existing zitadel module requires Zitadel to be running first.
# For initial bootstrap, you may need to apply in stages or use
# a separate bootstrap process.

# module "zitadel_config" {
#   source = "../../modules/zitadel"
#
#   environment              = local.environment
#   zitadel_domain           = "auth.${local.environment}.${var.domain}"
#   zitadel_port             = "443"
#   zitadel_insecure         = false
#   zitadel_jwt_profile_file = var.zitadel_jwt_profile_file
#   zitadel_org_name         = var.zitadel_org_name
#   zitadel_redirect_uri     = "https://${local.environment}.${var.domain}/api/auth/callback"
#   app_login_url            = "https://${local.environment}.${var.domain}"
#   app_internal_url         = "http://web.${local.environment}.svc.cluster.local:3000"
#
#   discord_client_id     = var.discord_client_id
#   discord_client_secret = var.discord_client_secret
#   github_client_id      = var.github_client_id
#   github_client_secret  = var.github_client_secret
#
#   depends_on = [module.zitadel_gke]
# }

# ============================================================================
# Stripe
# ============================================================================

module "stripe" {
  source = "../../modules/stripe"

  environment                   = local.environment
  stripe_price_plus_cents       = var.stripe_price_plus_cents
  stripe_price_pro_cents        = var.stripe_price_pro_cents
  stripe_price_enterprise_cents = var.stripe_price_enterprise_cents
  stripe_webhook_url            = "https://${local.environment}.${var.domain}"
  app_login_url                 = "https://${local.environment}.${var.domain}"
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
      argocd  = "https://argocd.${local.environment}.${var.domain}"
      zitadel = module.zitadel_gke.external_url
      app     = "https://${local.environment}.${var.domain}"
      api     = "https://api.${local.environment}.${var.domain}"
    }

    static_ips = {
      ingress = module.gke.ingress_ip
      argocd  = module.gke.argocd_ip
      zitadel = module.gke.zitadel_ip
    }

    artifact_registry = {
      url = module.artifact_registry.repository_url
    }

    stripe = {
      prices           = module.stripe.prices
      webhook_secret   = module.stripe.webhook_secret
      portal_config_id = module.stripe.portal_configuration_id
    }

    redis = {
      host             = module.memorystore.host
      port             = module.memorystore.port
      connection_string = module.memorystore.connection_string
    }

    generated_at = timestamp()
  })

  depends_on = [module.secrets]
}
