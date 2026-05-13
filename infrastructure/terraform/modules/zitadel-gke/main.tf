# Zitadel GKE Deployment Module
#
# Deploys Zitadel in GKE cluster via Helm with:
# - PostgreSQL backend (Cloud SQL)
# - Workload Identity for GCP access
# - Ingress with managed certificates

# ============================================================================
# Namespace
# ============================================================================

resource "kubernetes_namespace" "zitadel" {
  metadata {
    name = "zitadel"

    labels = {
      "app.kubernetes.io/name"       = "zitadel"
      "app.kubernetes.io/managed-by" = "terraform"
      "environment"                  = var.environment
    }
  }
}

# ============================================================================
# Kubernetes Service Account (for Workload Identity)
# ============================================================================

resource "kubernetes_service_account" "zitadel" {
  metadata {
    name      = "zitadel"
    namespace = kubernetes_namespace.zitadel.metadata[0].name

    annotations = {
      "iam.gke.io/gcp-service-account" = var.gcp_service_account_email
    }

    labels = {
      "app.kubernetes.io/name"       = "zitadel"
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }
}

# ============================================================================
# Database Secret
# ============================================================================

resource "kubernetes_secret" "zitadel_db" {
  metadata {
    name      = "zitadel-db-credentials"
    namespace = kubernetes_namespace.zitadel.metadata[0].name
  }

  data = {
    username = var.database_user
    password = var.database_password
  }

  type = "Opaque"
}

# ============================================================================
# Helm Release
# ============================================================================

resource "helm_release" "zitadel" {
  name             = "zitadel"
  repository       = "https://charts.zitadel.com"
  chart            = "zitadel"
  version          = var.chart_version
  namespace        = kubernetes_namespace.zitadel.metadata[0].name
  create_namespace = false
  wait             = true
  timeout          = 600

  values = [
    file("${path.module}/values/${var.environment}.yaml")
  ]

  # Database configuration
  set {
    name  = "zitadel.configmapConfig.Database.Postgres.Host"
    value = var.database_host
  }

  set {
    name  = "zitadel.configmapConfig.Database.Postgres.Port"
    value = "5432"
  }

  set {
    name  = "zitadel.configmapConfig.Database.Postgres.Database"
    value = var.database_name
  }

  set {
    name  = "zitadel.configmapConfig.Database.Postgres.User.Username"
    value = var.database_user
  }

  set {
    name  = "zitadel.configmapConfig.Database.Postgres.User.SSL.Mode"
    value = "disable" # Cloud SQL uses private IP, no SSL needed
  }

  set_sensitive {
    name  = "zitadel.configmapConfig.Database.Postgres.User.Password"
    value = var.database_password
  }

  # External domain
  set {
    name  = "zitadel.configmapConfig.ExternalDomain"
    value = var.external_domain
  }

  set {
    name  = "zitadel.configmapConfig.ExternalSecure"
    value = "true"
  }

  set {
    name  = "zitadel.configmapConfig.ExternalPort"
    value = "443"
  }

  # Service Account for Workload Identity
  set {
    name  = "serviceAccount.create"
    value = "false"
  }

  set {
    name  = "serviceAccount.name"
    value = kubernetes_service_account.zitadel.metadata[0].name
  }

  # Bootstrap configuration - creates initial admin user
  set {
    name  = "zitadel.masterkey"
    value = var.master_key
  }

  depends_on = [
    kubernetes_namespace.zitadel,
    kubernetes_service_account.zitadel,
    kubernetes_secret.zitadel_db
  ]
}

# ============================================================================
# GKE Managed Certificate
# ============================================================================

resource "kubernetes_manifest" "zitadel_managed_cert" {
  manifest = {
    apiVersion = "networking.gke.io/v1"
    kind       = "ManagedCertificate"

    metadata = {
      name      = "zitadel-cert"
      namespace = kubernetes_namespace.zitadel.metadata[0].name
    }

    spec = {
      domains = [var.external_domain]
    }
  }

  depends_on = [kubernetes_namespace.zitadel]
}

# ============================================================================
# Ingress
# ============================================================================

resource "kubernetes_ingress_v1" "zitadel" {
  metadata {
    name      = "zitadel"
    namespace = kubernetes_namespace.zitadel.metadata[0].name

    annotations = {
      "kubernetes.io/ingress.class"                 = "gce"
      "kubernetes.io/ingress.global-static-ip-name" = var.static_ip_name
      "networking.gke.io/managed-certificates"      = "zitadel-cert"
    }
  }

  spec {
    rule {
      host = var.external_domain

      http {
        path {
          path      = "/*"
          path_type = "ImplementationSpecific"

          backend {
            service {
              name = "zitadel"
              port {
                number = 8080
              }
            }
          }
        }
      }
    }
  }

  depends_on = [
    helm_release.zitadel,
    kubernetes_manifest.zitadel_managed_cert
  ]
}
