# Local Cluster Configuration
#
# One `terraform apply` to stand up everything:
# - ArgoCD for GitOps deployments
# - GHCR pull secret (for container images)
# - Zitadel configuration (auth)
# - Stripe configuration (billing)

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
# Zitadel Module
# ============================================================================

module "zitadel" {
  source = "../../modules/zitadel"

  environment              = "local"
  zitadel_domain           = var.zitadel_domain
  zitadel_port             = var.zitadel_port
  zitadel_insecure         = var.zitadel_insecure
  zitadel_jwt_profile_file = var.zitadel_jwt_profile_file
  zitadel_org_name         = var.zitadel_org_name
  zitadel_redirect_uri     = var.zitadel_redirect_uri
  app_login_url            = var.app_login_url
  app_internal_url         = var.app_internal_url
  discord_client_id        = var.discord_client_id
  discord_client_secret    = var.discord_client_secret
  github_client_id         = var.github_client_id
  github_client_secret     = var.github_client_secret
}

# ============================================================================
# Stripe Module
# ============================================================================

module "stripe" {
  source = "../../modules/stripe"

  environment                   = "local"
  stripe_price_plus_cents       = var.stripe_price_plus_cents
  stripe_price_pro_cents        = var.stripe_price_pro_cents
  stripe_price_enterprise_cents = var.stripe_price_enterprise_cents
  stripe_webhook_url            = var.stripe_webhook_url
  app_login_url                 = var.app_login_url
}

# ============================================================================
# Combined Outputs JSON File
# ============================================================================
# Generates the terraform-outputs.json for CLI consumption

resource "local_file" "terraform_outputs" {
  filename        = "../../../../.data/terraform/development.json"
  file_permission = "0600"
  content = jsonencode({
    zitadel_url = module.zitadel.zitadel_url
    project_id  = module.zitadel.project_id
    services = {
      api = {
        user_id            = module.zitadel.api_user_id
        key_id             = module.zitadel.api_key_id
        private_key_base64 = module.zitadel.api_private_key_base64
        client_id          = module.zitadel.api_client_id
      }
      agent_api = {
        user_id            = module.zitadel.agent_api_user_id
        key_id             = module.zitadel.agent_api_key_id
        private_key_base64 = module.zitadel.agent_api_private_key_base64
        client_id          = module.zitadel.agent_api_client_id
      }
      api_service = {
        user_id            = module.zitadel.api_service_user_id
        key_id             = module.zitadel.api_service_key_id
        private_key_base64 = module.zitadel.api_service_private_key_base64
        client_id          = module.zitadel.api_service_client_id
      }
      rig_service = {
        user_id            = module.zitadel.rig_service_user_id
        key_id             = module.zitadel.rig_service_key_id
        private_key_base64 = module.zitadel.rig_service_private_key_base64
        client_id          = module.zitadel.rig_service_client_id
      }
      chat_service = {
        user_id            = module.zitadel.chat_service_user_id
        key_id             = module.zitadel.chat_service_key_id
        private_key_base64 = module.zitadel.chat_service_private_key_base64
        client_id          = module.zitadel.chat_service_client_id
      }
      notification_service = {
        user_id            = module.zitadel.notification_service_user_id
        key_id             = module.zitadel.notification_service_key_id
        private_key_base64 = module.zitadel.notification_service_private_key_base64
        client_id          = module.zitadel.notification_service_client_id
      }
      web = {
        client_id           = module.zitadel.web_client_id
        key_details         = module.zitadel.web_key_details
        service_user_id     = module.zitadel.web_service_user_id
        service_key_id      = module.zitadel.web_service_key_id
        service_key_details = module.zitadel.web_service_key_details
        webhook_signing_key = module.zitadel.zitadel_webhook_signing_key
      }
    }
    api_audiences = {
      profile_service = module.zitadel.profile_service_client_id
    }
    cli = {
      user_id = module.zitadel.cli_user_id
      pat     = module.zitadel.cli_pat
    }
    roles = module.zitadel.roles
    stripe = {
      prices           = module.stripe.prices
      webhook_secret   = module.stripe.webhook_secret
      portal_config_id = module.stripe.portal_configuration_id
    }
    postgres = {
      databases = var.postgres_databases
      urls      = local.database_urls
    }
    redis = {
      urls = local.redis_urls
    }
    project_id   = module.zitadel.project_id
    generated_at = timestamp()
  })
}
