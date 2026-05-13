provider "helm" {
  kubernetes {
    config_path    = var.kubeconfig_path
    config_context = "kind-${var.cluster_name}"
  }
}

provider "kubernetes" {
  config_path    = var.kubeconfig_path
  config_context = "kind-${var.cluster_name}"
}
