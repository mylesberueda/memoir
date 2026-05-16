# GKE Cluster Module
#
# Creates:
# - GKE cluster with Workload Identity enabled
# - Node pool with autoscaling
# - Private cluster configuration

# ============================================================================
# GKE Cluster
# ============================================================================

resource "google_container_cluster" "primary" {
  name     = var.cluster_name
  location = var.region
  project  = var.project_id

  # We manage node pools separately
  remove_default_node_pool = true
  initial_node_count       = 1

  # Network configuration
  network    = var.network_id
  subnetwork = var.subnetwork_id

  # Private cluster - nodes have no public IPs
  private_cluster_config {
    enable_private_nodes    = true
    enable_private_endpoint = false # Allow public access to control plane
    master_ipv4_cidr_block  = var.master_cidr_block
  }

  # IP allocation for pods and services
  ip_allocation_policy {
    cluster_secondary_range_name  = var.pods_cidr_name
    services_secondary_range_name = var.services_cidr_name
  }

  # Workload Identity - CRITICAL for GCP service integration
  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }

  # Addons
  addons_config {
    http_load_balancing {
      disabled = false
    }
    horizontal_pod_autoscaling {
      disabled = false
    }
    network_policy_config {
      disabled = false
    }
    gce_persistent_disk_csi_driver_config {
      enabled = true
    }
  }

  # Network policy
  network_policy {
    enabled  = true
    provider = "CALICO"
  }

  # Release channel for automatic upgrades
  release_channel {
    channel = var.release_channel
  }

  # Maintenance window - Sunday 3-7 AM
  maintenance_policy {
    recurring_window {
      start_time = "2024-01-01T03:00:00Z"
      end_time   = "2024-01-01T07:00:00Z"
      recurrence = "FREQ=WEEKLY;BYDAY=SU"
    }
  }

  # Logging and monitoring
  logging_config {
    enable_components = ["SYSTEM_COMPONENTS", "WORKLOADS"]
  }

  monitoring_config {
    enable_components = ["SYSTEM_COMPONENTS"]
    managed_prometheus {
      enabled = true
    }
  }

  # Security
  master_authorized_networks_config {
    dynamic "cidr_blocks" {
      for_each = var.master_authorized_networks
      content {
        cidr_block   = cidr_blocks.value.cidr_block
        display_name = cidr_blocks.value.display_name
      }
    }
  }

  # Protect production clusters from accidental deletion
  deletion_protection = var.environment == "production"

  resource_labels = {
    environment = var.environment
    managed_by  = "terraform"
  }
}

# ============================================================================
# Node Pool
# ============================================================================

resource "google_container_node_pool" "primary" {
  name     = "${var.cluster_name}-primary"
  location = var.region
  cluster  = google_container_cluster.primary.name
  project  = var.project_id

  # Autoscaling
  autoscaling {
    min_node_count = var.min_node_count
    max_node_count = var.max_node_count
  }

  # Node management
  management {
    auto_repair  = true
    auto_upgrade = true
  }

  # Node configuration
  node_config {
    machine_type = var.node_machine_type
    disk_size_gb = var.node_disk_size_gb
    disk_type    = "pd-standard"

    # Workload Identity requires this metadata config
    workload_metadata_config {
      mode = "GKE_METADATA"
    }

    # OAuth scopes
    oauth_scopes = [
      "https://www.googleapis.com/auth/cloud-platform"
    ]

    # Labels
    labels = {
      environment = var.environment
      node_pool   = "primary"
    }

    # Tags for firewall rules
    tags = ["gke-node", "${var.cluster_name}-node"]

    # Shielded instance
    shielded_instance_config {
      enable_secure_boot          = true
      enable_integrity_monitoring = true
    }

    # Use container-optimized OS
    image_type = "COS_CONTAINERD"
  }

  # Upgrade settings
  upgrade_settings {
    max_surge       = 1
    max_unavailable = 0
  }
}

# ============================================================================
# Static IP for Ingress
# ============================================================================

resource "google_compute_global_address" "ingress" {
  name    = "${var.cluster_name}-ingress-ip"
  project = var.project_id
}

resource "google_compute_global_address" "argocd" {
  name    = "argocd-${var.environment}-ip"
  project = var.project_id
}
