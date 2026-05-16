# Local Cluster Configuration
#
# One `terraform apply` to stand up everything:
# - ArgoCD for GitOps deployments
# - GHCR pull secret (for container images)
# - Postgres + Redis Docker bridge to Kubernetes
#
# Auth (Zitadel) and billing (Stripe) were removed in epic 0002. The future
# memoir-server epic introduces local auth as part of memoir-server itself,
# not as a separate infrastructure service.

# ============================================================================
# GitHub Container Registry Pull Secret
# ============================================================================
# Creates a Docker registry secret for pulling images from ghcr.io.
# Required for ArgoCD-managed deployments to pull private container images.

resource "kubernetes_secret" "ghcr_pull_secret" {
  count = var.enable_kubernetes && var.argocd_ghcr_token != "" ? 1 : 0

  metadata {
    name      = "ghcr-pull-secret"
    namespace = "default"
  }

  type = "kubernetes.io/dockerconfigjson"

  data = {
    ".dockerconfigjson" = jsonencode({
      auths = {
        "ghcr.io" = {
          auth = base64encode("${var.github_username}:${var.argocd_ghcr_token}")
        }
      }
    })
  }
}

# ============================================================================
# ArgoCD Module
# ============================================================================

module "argocd" {
  source = "../../modules/argocd"
  count  = var.enable_kubernetes ? 1 : 0

  environment         = "local"
  github_repo_url     = var.github_repo_url
  github_argocd_token = var.github_argocd_token
  target_revision     = var.argocd_target_revision
  services            = var.argocd_services
}

# ============================================================================
# Cloudflare Tunnel (Optional - for instant ArgoCD sync from CI)
# ============================================================================
# Only deployed if cloudflare_tunnel_token is set.
# Creates an outbound-only tunnel so GitHub Actions can reach ArgoCD
# without exposing it publicly.

resource "kubernetes_deployment" "cloudflared" {
  count = var.enable_kubernetes && var.cloudflare_tunnel_token != "" ? 1 : 0

  metadata {
    name      = "cloudflared"
    namespace = "argocd"
  }

  spec {
    replicas = 1

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
# Combined Outputs JSON File
# ============================================================================
# Generates the terraform-outputs.json for CLI consumption.
#
# Pre-cleanup this file contained Zitadel client IDs, Stripe prices, and
# per-service keys. After 0002's deletes, only postgres + redis URLs remain
# — and even those will be reshaped by the future memoir-server epic when
# the database naming is decided.

resource "local_file" "terraform_outputs" {
  filename        = "../../../../.data/terraform/development.json"
  file_permission = "0600"
  content = jsonencode({
    postgres = {
      databases = var.postgres_databases
      urls      = local.database_urls
    }
    redis = {
      urls = local.redis_urls
    }
    generated_at = timestamp()
  })
}
