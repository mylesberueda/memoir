# Helm Charts

Base Helm chart for deploying all memoir services to Kubernetes.

## Structure

```
infrastructure/
├── kubernetes/
│   ├── helm/charts/service/  # Reusable base chart
│   └── environments/
│       ├── local/
│       │   ├── kind-config.yaml # Kind cluster configuration
│       │   ├── values/*.yaml    # Service values (reference ConfigMaps)
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

```bash
kind load docker-image memoir/api-service:local --name memoir
kind load docker-image memoir/rig-service:local --name memoir
kind load docker-image memoir/chat-service:local --name memoir
kind load docker-image memoir/notification-service:local --name memoir
kind load docker-image memoir/web:local --name memoir
```

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

```bash
helm install api-service infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/api-service.yaml

helm install rig-service infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/rig-service.yaml

helm install chat-service infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/chat-service.yaml

helm install notification-service infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/notification-service.yaml

helm install web infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/web.yaml
```

### Step 8: Verify

```bash
kubectl get pods
kubectl port-forward svc/api-service 5154:5154
curl http://localhost:5154/health
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

# Render templates
helm template api-service infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/api-service.yaml

# Dry-run against K8s API
helm template api-service infrastructure/kubernetes/helm/charts/service \
  -f infrastructure/kubernetes/environments/local/values/api-service.yaml \
  | kubectl apply --dry-run=client -f -
```

---

## Service Ports

| Service | Port |
|---------|------|
| api-service | 5154 |
| rig-service | 5153 |
| chat-service | 5155 |
| notification-service | 5156 |
| web | 3000 |
