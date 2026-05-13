# Helm Charts

Base Helm chart for deploying memoir services to Kubernetes.

Pre-cleanup this README walked through deploying five template services
(api-service, chat-service, notification-service, rig-service, web). Epic
0002 deleted those services. The memoir-server scaffold + memoir-ui rename
epics will repopulate the deployment examples below.

## Structure

```
infrastructure/
├── kubernetes/
│   ├── helm/charts/service/  # Reusable base chart
│   └── environments/
│       ├── local/
│       │   ├── kind-config.yaml # Kind cluster configuration
│       │   ├── values/*.yaml    # Service values (empty post-0002)
│       │   └── configmaps/      # Generated from .env files (gitignored)
│       ├── staging/values/      # Uses ExternalSecret → GCP Secret Manager
│       └── production/values/   # Uses ExternalSecret → GCP Secret Manager
└── terraform/
    ├── modules/              # Reusable Terraform modules
    └── stacks/               # Deployable root modules (local, staging, production)
```

## Environment Variables

| Environment | Source | How It Works |
|-------------|--------|--------------|
| Local | `apps/<service>/.env` | NX generates ConfigMap YAML, applied to cluster |
| Staging | GCP Secret `staging/<service>` | ExternalSecret extracts all keys from JSON |
| Production | GCP Secret `production/<service>` | ExternalSecret extracts all keys from JSON |

**Adding a new env var:**

- Local: Add to `.env` file, regenerate ConfigMap
- Staging/Prod: Add to GCP Secret Manager JSON — no Helm changes needed

---

## Local Development

### Prerequisites

- Docker
- kind
- kubectl
- helm

### Step 1: Build Docker Images

```bash
pnpm docker:build
```

### Step 2: Generate ConfigMaps from .env Files

```bash
pnpm nx run-many -t k8s:configmap
```

This reads each `apps/<service>/.env` and generates Kubernetes ConfigMap YAML files in `infrastructure/kubernetes/environments/local/configmaps/`.

### Step 3: Create Kind Cluster

```bash
kind create cluster --config infrastructure/kubernetes/environments/local/kind-config.yaml --name memoir
```

### Step 4: Load Images into Kind

The memoir-server epic adds the concrete `kind load docker-image` invocation
here.

### Step 5: Install nginx-ingress

```bash
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
kubectl wait --namespace ingress-nginx \
  --for=condition=ready pod \
  --selector=app.kubernetes.io/component=controller \
  --timeout=90s
```

### Step 6: Apply ConfigMaps

```bash
kubectl apply -f infrastructure/kubernetes/environments/local/configmaps/
```

### Step 7: Deploy Services

The memoir-server epic adds concrete `helm install` invocations here.

### Step 8: Verify

```bash
kubectl get pods
```

### Cleanup

```bash
kind delete cluster --name memoir
```

---

## Validation (No Cluster Required)

```bash
# Lint
helm lint infrastructure/kubernetes/helm/charts/service
```

The memoir-server epic adds concrete `helm template` validation invocations
here.
