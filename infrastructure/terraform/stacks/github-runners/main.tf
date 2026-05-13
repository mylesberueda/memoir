# Local Kind cluster setup (manual prerequisite)
# Run: kind create cluster --name github-runners --config ../../cli/infra/kind-config.yaml

# Wait for cluster to be ready
resource "null_resource" "wait_for_cluster" {
  provisioner "local-exec" {
    command = "kubectl cluster-info --context kind-${var.cluster_name}"
  }
}

# Create arc-systems namespace
resource "kubernetes_namespace" "arc_systems" {
  metadata {
    name = "arc-systems"
    labels = {
      "app.kubernetes.io/name"       = "arc-systems"
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }

  depends_on = [null_resource.wait_for_cluster]
}

# Create arc-runners namespace
resource "kubernetes_namespace" "arc_runners" {
  metadata {
    name = "arc-runners"
    labels = {
      "app.kubernetes.io/name"       = "arc-runners"
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }

  depends_on = [null_resource.wait_for_cluster]
}

# Create test-services namespace
resource "kubernetes_namespace" "test_services" {
  metadata {
    name = "test-services"
    labels = {
      "app.kubernetes.io/name"       = "test-services"
      "app.kubernetes.io/component"  = "test-infrastructure"
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }

  depends_on = [null_resource.wait_for_cluster]
}

# =============================================================================
# BuildKit Pod Template for Container Actions
# =============================================================================
# When ARC runs container actions (docker://...) in kubernetes mode, it creates
# a separate pod. This template configures that pod for BuildKit rootless mode:
# - seccompProfile: Unconfined (required for rootless BuildKit)
# - Docker credentials copied to /home/user/.docker for BuildKit
# - emptyDir for BuildKit cache (local to pod, registry cache persists across pods)

resource "kubernetes_config_map" "buildkit_pod_template" {
  metadata {
    name      = "buildkit-pod-template"
    namespace = kubernetes_namespace.arc_runners.metadata[0].name
  }

  # Pod template for container actions (docker://...)
  # In kubernetes mode, the work volume is shared between runner and workflow pods.
  # The workflow writes credentials to $GITHUB_WORKSPACE/.docker/config.json
  #
  # BuildKit rootless requirements:
  # - seccompProfile: Unconfined (for user namespace operations)
  # - BUILDKITD_FLAGS with --oci-worker-no-process-sandbox
  # - Credentials at /home/user/.docker/config.json (rootless home)
  data = {
    "buildkit.yaml" = <<-YAML
      apiVersion: v1
      kind: Pod
      spec:
        securityContext:
          seccompProfile:
            type: Unconfined
        initContainers:
          - name: setup-buildkit-home
            image: busybox:latest
            resources:
              requests:
                cpu: "50m"
                memory: "64Mi"
              limits:
                cpu: "200m"
                memory: "128Mi"
            command:
              - /bin/sh
              - -c
              - |
                # Create BuildKit rootless directory structure
                # rootlesskit requires /home/user/.local/tmp for state
                mkdir -p /home/user/.docker
                mkdir -p /home/user/.local/tmp
                mkdir -p /home/user/.local/share/buildkit
                chown -R 1000:1000 /home/user
                echo "Created BuildKit home directory structure"

                # Find and copy docker config from work volume
                echo "Searching for docker config in work volume..."
                CONFIG=$(find /work -name "config.json" -path "*/.docker/*" 2>/dev/null | head -1)
                if [ -n "$CONFIG" ]; then
                  cp "$CONFIG" /home/user/.docker/config.json
                  chmod 600 /home/user/.docker/config.json
                  chown 1000:1000 /home/user/.docker/config.json
                  echo "Copied docker config from $CONFIG to /home/user/.docker"
                else
                  echo "No docker config found in /work"
                  ls -la /work/
                fi
            volumeMounts:
              - name: work
                mountPath: /work
              - name: buildkit-home
                mountPath: /home/user
        containers:
          - name: "$job"
            # Override entrypoint to use buildctl-daemonless.sh instead of rootlesskit buildkitd
            # This starts buildkitd, waits for it, then runs buildctl with the provided args
            command: ["buildctl-daemonless.sh"]
            resources:
              requests:
                cpu: "1"
                memory: "2Gi"
              limits:
                cpu: "3"
                memory: "8Gi"
            securityContext:
              runAsUser: 1000
              runAsGroup: 1000
            env:
              - name: BUILDKITD_FLAGS
                value: "--oci-worker-no-process-sandbox"
              - name: HOME
                value: "/home/user"
              - name: DOCKER_CONFIG
                value: "/home/user/.docker"
              # S3/MinIO credentials for BuildKit cache
              - name: AWS_ACCESS_KEY_ID
                value: "${var.minio_access_key}"
              - name: AWS_SECRET_ACCESS_KEY
                value: "${var.minio_secret_key}"
              - name: AWS_EC2_METADATA_DISABLED
                value: "${var.buildkit_aws_ec2_metadata_disabled}"
            volumeMounts:
              - name: buildkit-home
                mountPath: /home/user
        volumes:
          - name: buildkit-home
            emptyDir:
              sizeLimit: 4Gi
    YAML
  }

  depends_on = [kubernetes_namespace.arc_runners]
}

# ARC Controller Helm Release
resource "helm_release" "arc_controller" {
  name             = "arc"
  repository       = "oci://ghcr.io/actions/actions-runner-controller-charts"
  chart            = "gha-runner-scale-set-controller"
  version          = var.arc_controller_version
  namespace        = kubernetes_namespace.arc_systems.metadata[0].name
  create_namespace = false
  wait             = true
  timeout          = 600
  wait_for_jobs    = false
  disable_webhooks = true

  values = [
    yamlencode({
      resources = {
        limits = {
          cpu    = var.controller_cpu_limit
          memory = var.controller_memory_limit
        }
        requests = {
          cpu    = "100m"
          memory = "128Mi"
        }
      }
      # RBAC configuration for API discovery
      rbac = {
        rules = [
          {
            nonResourceURLs = ["/api", "/api/*", "/apis", "/apis/*"]
            verbs           = ["get"]
          }
        ]
      }
    })
  ]

  depends_on = [kubernetes_namespace.arc_systems]
}

# MinIO for GitHub Actions Cache
resource "helm_release" "minio" {
  count = var.minio_enabled ? 1 : 0

  name             = "minio"
  repository       = "https://charts.min.io/"
  chart            = "minio"
  namespace        = "minio-system"
  create_namespace = true
  wait             = true
  timeout          = 600
  wait_for_jobs    = true

  values = [
    yamlencode({
      mode         = "standalone"
      rootUser     = var.minio_access_key
      rootPassword = var.minio_secret_key

      persistence = {
        enabled      = true
        storageClass = "standard"
        size         = "10Gi"
      }

      resources = {
        limits = {
          cpu    = "500m"
          memory = "512Mi"
        }
        requests = {
          cpu    = "100m"
          memory = "256Mi"
        }
      }

      service = {
        type = "ClusterIP"
        port = 9000
      }

      consoleService = {
        type = "ClusterIP"
        port = 9001
      }

      buckets = [
        {
          name   = "github-actions-cache"
          policy = "none"
        },
        {
          name   = "buildkit-cache"
          policy = "none"
        }
      ]
    })
  ]

  depends_on = [null_resource.wait_for_cluster]
}

# ARC Runner Scale Set
resource "helm_release" "arc_runner_set" {
  name             = "arc-runner-set"
  repository       = "oci://ghcr.io/actions/actions-runner-controller-charts"
  chart            = "gha-runner-scale-set"
  version          = var.arc_runner_scale_set_version
  namespace        = kubernetes_namespace.arc_runners.metadata[0].name
  create_namespace = false
  wait             = true
  timeout          = 600
  wait_for_jobs    = false

  # Disable waiting during destroy for faster cleanup
  disable_webhooks = true

  set_sensitive {
    name  = "githubConfigSecret.github_token"
    value = var.github_runner_token
  }

  values = [
    yamlencode({
      githubConfigUrl = var.github_config_url
      runnerGroup     = var.runner_group
      minRunners      = var.runner_min
      maxRunners      = var.runner_max

      containerMode = {
        type = "kubernetes"
        kubernetesModeWorkVolumeClaim = {
          accessModes      = ["ReadWriteOnce"]
          storageClassName = "standard"
          resources = {
            requests = {
              storage = "5Gi"
            }
          }
        }
      }

      controllerServiceAccount = {
        namespace = "arc-systems"
        name      = "arc-gha-rs-controller"
      }

      metrics = {
        enabled = true
      }

      template = {
        spec = {
          containers = [
            {
              name    = "runner"
              image   = "ghcr.io/mylesberueda/github-actions-runner-kubectl:3.1.0"
              command = ["/home/runner/run.sh"]

              resources = {
                limits = {
                  cpu    = var.runner_cpu_limit
                  memory = var.runner_memory_limit
                }
                requests = {
                  cpu    = "1"
                  memory = "4Gi"
                }
              }

              volumeMounts = [
                {
                  name      = "buildkit-pod-template"
                  mountPath = "/home/runner/pod-templates"
                  readOnly  = true
                }
              ]

              env = concat(
                [
                  {
                    name  = "RUNNER_FEATURE_FLAG_ONCE"
                    value = "true"
                  },
                  {
                    name  = "RUNNER_FEATURE_FLAG_EPHEMERAL"
                    value = "true"
                  },
                  {
                    name  = "ACTIONS_RUNNER_PRINT_LOG_TO_STDOUT"
                    value = "true"
                  },
                  {
                    name  = "ACTIONS_RUNNER_REQUIRE_JOB_CONTAINER"
                    value = "false"
                  },
                  {
                    name  = "ACTIONS_RUNNER_CONTAINER_HOOK_TEMPLATE"
                    value = "/home/runner/pod-templates/buildkit.yaml"
                  },
                  {
                    name  = "ENVIRONMENT"
                    value = "local"
                  },
                  {
                    name  = "CURL_IPRESOLVE"
                    value = "4"
                  }
                ],
                var.minio_enabled ? [
                  {
                    name  = "MINIO_ENDPOINT"
                    value = "http://minio.minio-system.svc.cluster.local:9000"
                  },
                  {
                    name  = "MINIO_ACCESS_KEY"
                    value = var.minio_access_key
                  },
                  {
                    name  = "MINIO_SECRET_KEY"
                    value = var.minio_secret_key
                  },
                  {
                    name  = "ACTIONS_CACHE_URL"
                    value = "http://minio.minio-system.svc.cluster.local:9000"
                  }
                ] : []
              )
            }
          ]

          securityContext = {
            fsGroup      = 121
            runAsNonRoot = true
            runAsUser    = 1001
          }

          volumes = [
            {
              name = "buildkit-pod-template"
              configMap = {
                name = kubernetes_config_map.buildkit_pod_template.metadata[0].name
              }
            }
          ]
        }
      }

      listenerTemplate = {
        spec = {
          containers = [
            {
              name = "listener"
              securityContext = {
                runAsNonRoot = true
                runAsUser    = 1000
              }
              resources = {
                limits = {
                  cpu    = "1"
                  memory = "1Gi"
                }
                requests = {
                  cpu    = "250m"
                  memory = "256Mi"
                }
              }
              env = [
                {
                  name  = "ACTIONS_RUNNER_CONTROLLER_IDLE_TIMEOUT"
                  value = "1h"
                },
                {
                  name  = "LOG_LEVEL"
                  value = "debug"
                }
              ]
            }
          ]
        }
      }
    })
  ]

  depends_on = [
    helm_release.arc_controller,
    helm_release.minio
  ]
}

# ClusterRole to allow runners to manage the test-services namespace
# Note: The namespace is pre-created by Terraform, so runners only need read access
resource "kubernetes_cluster_role" "test_services_namespace_admin" {
  metadata {
    name = "test-services-namespace-admin"
  }

  # Allow reading namespaces (needed for kubectl apply to check if namespace exists)
  rule {
    api_groups = [""]
    resources  = ["namespaces"]
    verbs      = ["get", "list", "watch"]
  }

  # Allow updating only the test-services namespace (no create - TF handles that)
  rule {
    api_groups     = [""]
    resources      = ["namespaces"]
    resource_names = ["test-services"]
    verbs          = ["update", "patch"]
  }
}

resource "kubernetes_cluster_role_binding" "arc_runners_namespace_admin" {
  metadata {
    name = "arc-runners-namespace-admin"
  }

  role_ref {
    api_group = "rbac.authorization.k8s.io"
    kind      = "ClusterRole"
    name      = kubernetes_cluster_role.test_services_namespace_admin.metadata[0].name
  }

  subject {
    kind      = "ServiceAccount"
    name      = "arc-runner-set-gha-rs-kube-mode"
    namespace = kubernetes_namespace.arc_runners.metadata[0].name
  }

  depends_on = [helm_release.arc_runner_set]
}

# RBAC for test services
resource "kubernetes_role" "test_services_admin" {
  metadata {
    name      = "test-services-admin"
    namespace = kubernetes_namespace.test_services.metadata[0].name
  }

  # Core API - pods, services, configmaps for test infrastructure
  rule {
    api_groups = [""]
    resources  = ["pods", "pods/log", "pods/exec", "services", "configmaps", "secrets", "persistentvolumeclaims"]
    verbs      = ["get", "list", "watch", "create", "update", "patch", "delete"]
  }

  # Apps API - deployments for test databases
  rule {
    api_groups = ["apps"]
    resources  = ["deployments", "statefulsets"]
    verbs      = ["get", "list", "watch", "create", "update", "patch", "delete"]
  }

  # Batch API - jobs for one-off tasks
  rule {
    api_groups = ["batch"]
    resources  = ["jobs"]
    verbs      = ["get", "list", "watch", "create", "update", "patch", "delete"]
  }
}

resource "kubernetes_role_binding" "arc_runners_test_services" {
  metadata {
    name      = "arc-runners-test-services"
    namespace = kubernetes_namespace.test_services.metadata[0].name
  }

  role_ref {
    api_group = "rbac.authorization.k8s.io"
    kind      = "Role"
    name      = kubernetes_role.test_services_admin.metadata[0].name
  }

  subject {
    kind      = "ServiceAccount"
    name      = "arc-runner-set-gha-rs-kube-mode"
    namespace = kubernetes_namespace.arc_runners.metadata[0].name
  }

  depends_on = [helm_release.arc_runner_set]
}
