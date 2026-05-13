# External Secrets Operator Module
#
# Installs:
# - External Secrets Operator via Helm
# - ClusterSecretStore for GCP Secret Manager

# ============================================================================
# Namespace
# ============================================================================

resource "kubernetes_namespace" "external_secrets" {
  metadata {
    name = "external-secrets"

    labels = {
      "app.kubernetes.io/name"       = "external-secrets"
      "app.kubernetes.io/managed-by" = "terraform"
      "environment"                  = var.environment
    }
  }
}

# ============================================================================
# Kubernetes Service Account (for Workload Identity)
# ============================================================================

resource "kubernetes_service_account" "external_secrets" {
  metadata {
    name      = "external-secrets"
    namespace = kubernetes_namespace.external_secrets.metadata[0].name

    annotations = {
      "iam.gke.io/gcp-service-account" = var.gcp_service_account_email
    }

    labels = {
      "app.kubernetes.io/name"       = "external-secrets"
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }
}

# ============================================================================
# Helm Release
# ============================================================================

resource "helm_release" "external_secrets" {
  name             = "external-secrets"
  repository       = "https://charts.external-secrets.io"
  chart            = "external-secrets"
  version          = var.chart_version
  namespace        = kubernetes_namespace.external_secrets.metadata[0].name
  create_namespace = false
  wait             = true
  timeout          = 300

  set {
    name  = "installCRDs"
    value = "true"
  }

  # Use the service account we created with Workload Identity
  set {
    name  = "serviceAccount.create"
    value = "false"
  }

  set {
    name  = "serviceAccount.name"
    value = kubernetes_service_account.external_secrets.metadata[0].name
  }

  # Webhook service account also needs the annotation
  set {
    name  = "webhook.serviceAccount.create"
    value = "true"
  }

  set {
    name  = "certController.serviceAccount.create"
    value = "true"
  }

  depends_on = [kubernetes_service_account.external_secrets]
}

# ============================================================================
# ClusterSecretStore for GCP Secret Manager
# ============================================================================

resource "kubernetes_manifest" "cluster_secret_store" {
  manifest = {
    apiVersion = "external-secrets.io/v1beta1"
    kind       = "ClusterSecretStore"

    metadata = {
      name = "gcp-secret-store"
    }

    spec = {
      provider = {
        gcpsm = {
          projectID = var.gcp_project_id
        }
      }
    }
  }

  depends_on = [helm_release.external_secrets]
}
