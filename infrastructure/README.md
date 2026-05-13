# Infrastructure

All infrastructure configuration for the memoir platform: Terraform modules/stacks, Helm charts, and environment-specific Kubernetes manifests.

## Directory Structure

```
infrastructure/
├── README.md
│
├── terraform/
│   ├── modules/                    # Reusable Terraform modules
│   │   ├── argocd/                 # ArgoCD Helm installation + ApplicationSets
│   │   ├── artifact-registry/      # GCP Artifact Registry
│   │   ├── cloud-sql/              # GCP Cloud SQL (PostgreSQL)
│   │   ├── external-secrets/       # External Secrets Operator
│   │   ├── gcp-networking/         # GCP VPC, subnets, NAT, firewall
│   │   ├── gcp-secrets/            # GCP Secret Manager
│   │   ├── gcp-workload-identity/  # GCP Workload Identity for K8s
│   │   ├── gke-cluster/            # GKE cluster with private networking
│   │   └── memorystore/            # GCP Memorystore (Redis)
│   │
│   └── stacks/                     # Deployable root modules
│       ├── local/                  # Local dev (Postgres + Redis bridge to Kind)
│       ├── staging/                # GCP staging environment
│       ├── production/             # GCP production environment
│       └── github-runners/         # GitHub Actions self-hosted runners
│
└── kubernetes/
    ├── helm/
    │   └── charts/
    │       └── service/            # Shared base chart for all services
    │
    └── environments/               # Environment-specific K8s config
        ├── local/
        │   ├── kind-config.yaml    # Kind cluster configuration
        │   ├── configmaps/         # Generated from .env files (gitignored)
        │   └── values/             # Helm value overrides (empty post-cleanup)
        ├── staging/
        │   └── values/
        └── production/
            └── values/
```

Auth (Zitadel) and billing (Stripe) modules were removed by epic 0002. The memoir-server epic will introduce the production auth replacement.

## Quick Reference

### Terraform Commands

```bash
# Local development
cd infrastructure/terraform/stacks/local
terraform init
terraform plan
terraform apply

# Staging
cd infrastructure/terraform/stacks/staging
terraform init
terraform plan -var-file=terraform.tfvars
terraform apply -var-file=terraform.tfvars

# Production
cd infrastructure/terraform/stacks/production
terraform init
terraform plan -var-file=terraform.tfvars
terraform apply -var-file=terraform.tfvars
```

### Kubernetes/Helm Commands

```bash
# Create Kind cluster for local development
kind create cluster --config infrastructure/kubernetes/environments/local/kind-config.yaml --name memoir

# Generate ConfigMaps from .env files
pnpm nx run-many -t k8s:configmap

# Apply ConfigMaps
kubectl apply -k infrastructure/kubernetes/environments/local

# Deploy a service (post memoir-server epic)
# helm install memoir-server infrastructure/kubernetes/helm/charts/service \
#   -f infrastructure/kubernetes/environments/local/values/memoir-server.yaml
```

## Design Principles

| Principle | Implementation |
|-----------|----------------|
| **Modules vs Stacks** | `modules/` = reusable code, `stacks/` = deployable entry points |
| **Environment isolation** | Each environment has its own stack directory with separate state |
| **Single Helm chart** | All services use `charts/service/` with environment-specific values |
| **GitOps ready** | ArgoCD ApplicationSets auto-deploy from `kubernetes/environments/` |

## Environments

| Environment | Terraform Stack | K8s Cluster | Secrets Source |
|-------------|-----------------|-------------|----------------|
| Local | `stacks/local/` | Kind (Docker) | `.env` files → ConfigMaps |
| Staging | `stacks/staging/` | GKE (GCP) | GCP Secret Manager → ExternalSecret |
| Production | `stacks/production/` | GKE (GCP) | GCP Secret Manager → ExternalSecret |

## ArgoCD

ArgoCD is deployed via Terraform and manages all service deployments through GitOps.

### Accessing the UI

```bash
# 1. Get the initial admin password
kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d

# 2. Port-forward the ArgoCD server
kubectl port-forward svc/argocd-server -n argocd 8080:443

# 3. Open browser to https://localhost:8080
#    Username: admin
#    Password: (from step 1)
```

### Using the CLI

```bash
# Install CLI
brew install argocd  # macOS
# or: curl -sSL -o /usr/local/bin/argocd https://github.com/argoproj/argo-cd/releases/latest/download/argocd-linux-amd64 && chmod +x /usr/local/bin/argocd

# Login (while port-forward is running)
argocd login localhost:8080 --insecure

# List apps
argocd app list

# Change target revision for a specific app (useful for testing branches)
argocd app set staging-memoir-server --revision ci/test/my-feature

# Sync an app manually
argocd app sync staging-memoir-server

# Revert to default branch
argocd app set staging-memoir-server --revision HEAD
```

### How Deployments Work

1. CI pushes docker images with SHA tags to the container registry
2. CI updates `infrastructure/kubernetes/environments/{env}/values/{service}.yaml` with the new image tag
3. ArgoCD detects the values file change and syncs only the affected service
4. Other services remain untouched (no unnecessary pod restarts)

## Additional Documentation

- **Helm charts**: See [kubernetes/helm/README.md](./kubernetes/helm/README.md)
- **GitHub runners**: See [terraform/stacks/github-runners/README.md](./terraform/stacks/github-runners/README.md)
