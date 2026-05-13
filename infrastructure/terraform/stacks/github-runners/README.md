# GitHub Actions Runners - Terraform Configuration

This Terraform configuration deploys GitHub Actions self-hosted runners on a
local Kind cluster using Actions Runner Controller (ARC).

## Prerequisites

1. **Install required tools:**

   ```bash
   # From the workspace root:
   mise install
   ```

2. **Create Kind cluster:**

   ```bash
   kind create cluster --config config/kind-config.yaml
   ```

## Quick Start

```bash

# Set required environment variables
export TF_VAR_github_runner_token="ghp_your_token_here"
export TF_VAR_minio_access_key="admin"
export TF_VAR_minio_secret_key="minio123"

# Initialize Terraform
terraform init

# Review the plan
terraform plan

# Deploy
terraform apply
```

## Configuration

Edit `local.tfvars` to customize:

- **GitHub repository URL**
- **Runner min/max counts**
- **Resource limits**
- **MinIO settings**
- **ARC versions**

## Managing the Infrastructure

```bash
# Show current state
terraform show

# List resources
terraform state list

# Update infrastructure
terraform apply 

# Destroy everything
terraform destroy
```

## Troubleshooting

### Check runner status

```bash
kubectl get pods -n arc-runners
kubectl get pods -n arc-systems
```

### View logs

```bash
# Controller logs
kubectl logs -n arc-systems -l app.kubernetes.io/component=controller

# Listener logs
kubectl logs -n arc-systems -l app.kubernetes.io/component=listener

# Runner logs
kubectl logs -n arc-runners <runner-pod-name>
```

### Common issues

**Local-path-provisioner errors:**

```bash
kubectl delete pod -n local-path-storage -l app=local-path-provisioner
```

**Controller authentication errors:**
Check RBAC permissions are applied correctly in the Terraform state.

**BuildKit cache not working:**

```bash
# Check MinIO is running
kubectl get pods -n minio-system

# View MinIO console (port-forward)
kubectl port-forward -n minio-system svc/minio-console 9001:9001
# Open http://localhost:9001 (admin / minio123)

# Check bucket contents
kubectl exec -n minio-system deploy/minio -- mc ls local/buildkit-cache
```
