# -----------------------------------------------------------------------------
# Nginx Ingress Controller - Required for Kind cluster (local environment only)
# In cloud environments, ingress is typically managed by the cloud provider
# -----------------------------------------------------------------------------

resource "helm_release" "ingress_nginx" {
  count = var.environment == "local" ? 1 : 0

  name             = "ingress-nginx"
  repository       = "https://kubernetes.github.io/ingress-nginx"
  chart            = "ingress-nginx"
  version          = var.ingress_nginx_version
  namespace        = "ingress-nginx"
  create_namespace = true
  wait             = true
  timeout          = 300

  # Kind-specific configuration
  values = [yamlencode({
    controller = {
      # Use host network for Kind compatibility
      hostPort = {
        enabled = true
      }
      service = {
        type = "NodePort"
      }
      # Schedule on nodes with ingress-ready label (Kind control-plane)
      nodeSelector = {
        "ingress-ready" = "true"
      }
      tolerations = [
        {
          key      = "node-role.kubernetes.io/control-plane"
          operator = "Equal"
          effect   = "NoSchedule"
        },
        {
          key      = "node-role.kubernetes.io/master"
          operator = "Equal"
          effect   = "NoSchedule"
        }
      ]
      # Publish status for ingress resources
      publishService = {
        enabled = false
      }
      extraArgs = {
        "publish-status-address" = "localhost"
      }
    }
  })]
}

resource "helm_release" "argocd" {
  name             = "argocd"
  repository       = "https://argoproj.github.io/argo-helm"
  chart            = "argo-cd"
  version          = var.argocd_version
  namespace        = var.argocd_namespace
  create_namespace = true # Idempotent - creates if missing, no-op if exists
  wait             = true
  timeout          = 600

  # Values are merged in order: environment file first, then OIDC config
  values = [
    file("${path.module}/values/${var.environment}.yaml"),
    # ---------------------------------------------------------------------------
    # GitHub Actions OIDC Authentication
    # Allows GitHub Actions to authenticate without static secrets
    # Workflow exchanges GitHub OIDC token for ArgoCD token via Dex
    # ---------------------------------------------------------------------------
    yamlencode({
      configs = {
        cm = {
          "dex.config" = yamlencode({
            connectors = [{
              type = "oidc"
              id   = "github-actions"
              name = "GitHub Actions"
              config = {
                issuer                    = "https://token.actions.githubusercontent.com/"
                scopes                    = ["openid"]
                userNameKey               = "sub"
                insecureSkipEmailVerified = true
              }
            }]
          })
        }
        rbac = {
          # RBAC: GitHub Actions from this repo can sync and get applications
          # The "sub" claim format is: repo:<owner>/<repo>:ref:refs/heads/<branch>
          "policy.csv" = <<-EOT
            p, repo:${var.github_repo_owner}/${var.github_repo_name}:*, applications, sync, */*, allow
            p, repo:${var.github_repo_owner}/${var.github_repo_name}:*, applications, get, */*, allow
          EOT
        }
      }
    })
  ]

  # GitHub repo credentials for private repos
  dynamic "set_sensitive" {
    for_each = var.github_argocd_token != "" ? [1] : []
    content {
      name  = "configs.credentialTemplates.github.password"
      value = var.github_argocd_token
    }
  }

  dynamic "set" {
    for_each = var.github_argocd_token != "" ? [1] : []
    content {
      name  = "configs.credentialTemplates.github.url"
      value = var.github_repo_url
    }
  }
}

# -----------------------------------------------------------------------------
# ArgoCD Apps - Deploys AppProject and ApplicationSet via separate Helm chart
# This chart depends on ArgoCD CRDs being installed by the argo-cd chart above
# -----------------------------------------------------------------------------

locals {
  # Filter to only enabled services
  enabled_services = [for svc in var.services : svc if svc.enabled]

  # Destination namespace varies by environment
  destination_namespace = var.environment == "local" ? "default" : var.environment

  # Source repos - permissive for local, restricted for other environments
  source_repos = var.environment == "local" ? ["*"] : [var.github_repo_url]
}

resource "helm_release" "argocd_apps" {
  name             = "argocd-apps"
  repository       = "https://argoproj.github.io/argo-helm"
  chart            = "argocd-apps"
  version          = var.argocd_apps_version
  namespace        = var.argocd_namespace
  create_namespace = false
  wait             = true
  timeout          = 300

  values = [yamlencode({
    # Projects are keyed by name (the key becomes the project name)
    projects = {
      (var.environment) = {
        namespace   = var.argocd_namespace
        description = "${title(var.environment)} environment"
        sourceRepos = local.source_repos
        destinations = [
          {
            namespace = local.destination_namespace
            server    = "https://kubernetes.default.svc"
          },
          {
            namespace = var.environment
            server    = "https://kubernetes.default.svc"
          }
        ]
        clusterResourceWhitelist = [
          { group = "", kind = "Namespace" },
          { group = "networking.k8s.io", kind = "Ingress" }
        ]
        namespaceResourceWhitelist = [
          { group = "*", kind = "*" }
        ]
      }
    }

    # ApplicationSets are keyed by name (the key becomes the applicationset name)
    applicationsets = {
      "${var.environment}-services" = {
        namespace = var.argocd_namespace
        generators = [
          {
            list = {
              elements = [for svc in local.enabled_services : { name = svc.name }]
            }
          }
        ]
        template = {
          metadata = {
            name = "${var.environment}-{{name}}"
          }
          spec = {
            project = var.environment
            source = {
              repoURL        = var.github_repo_url
              targetRevision = var.target_revision
              path           = "infrastructure/kubernetes/helm/charts/service"
              helm = {
                valueFiles = ["../../../environments/${var.environment}/values/{{name}}.yaml"]
              }
            }
            destination = {
              server    = "https://kubernetes.default.svc"
              namespace = local.destination_namespace
            }
            syncPolicy = {
              automated = {
                prune    = true
                selfHeal = true
              }
              syncOptions = ["CreateNamespace=true"]
            }
          }
        }
      }
    }
  })]

  # This release depends on the main ArgoCD release which installs the CRDs
  depends_on = [helm_release.argocd]
}
