# Local Kubernetes Environment

Kubernetes manifests and Kustomize overlays for the local Kind cluster.

## Applying ConfigMaps

ConfigMaps are generated from each service's `.env` file, then patched with
Kubernetes-resolvable URLs via Kustomize.

```bash
# 1. Generate base ConfigMaps from service .env files
pnpm k8s:configmap

# 2. Apply with Kustomize (patches localhost URLs → K8s service URLs)
kubectl apply -k infrastructure/kubernetes/environments/local/
```

Or as a one-liner:

```bash
pnpm k8s:configmap && kubectl apply -k infrastructure/kubernetes/environments/local/
```

## How It Works

1. **`pnpm k8s:configmap`** runs `kubectl create configmap --from-env-file` for
   each service, generating YAML files in `configmaps/`.

2. **`kubectl apply -k`** uses `kustomization.yaml` to:
   - Load the base ConfigMaps from `configmaps/`
   - Apply strategic merge patches that override localhost URLs with K8s
     service names

## Network Architecture

Services in the Kind cluster connect to infrastructure services (Postgres,
Redis, Zitadel) running in Docker Compose via the shared `kind` Docker network:

```
┌─────────────────────────────────────────────────────────────┐
│                     Kind Cluster                            │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐   │
│  │ api-service │  │ chat-service │  │ notification-svc  │   │
│  └──────┬──────┘  └──────┬───────┘  └─────────┬─────────┘   │
│         │                │                    │             │
│         └────────────────┼────────────────────┘             │
│                          │                                  │
└──────────────────────────┼──────────────────────────────────┘
                           │ kind network
┌──────────────────────────┼──────────────────────────────────┐
│                          │      Docker Compose              │
│  ┌───────────────────┐   │   ┌─────────────────────────┐    │
│  │ startupai-postgres│◄──┼──►│   startupai-zitadel     │    │
│  │     :5432         │   │   │        :8080            │    │
│  └───────────────────┘   │   └─────────────────────────┘    │
│                          │                                  │
│  ┌───────────────────┐   │                                  │
│  │  startupai-redis  │◄──┘                                  │
│  │      :6379        │                                      │
│  └───────────────────┘                                      │
└─────────────────────────────────────────────────────────────┘
```

## URL Mappings

The Kustomize patches override these localhost URLs with K8s-resolvable names:

| Local Dev (.env)       | K8s Cluster (patched)           |
|------------------------|---------------------------------|
| `localhost:54321`      | `startupai-postgres:5432`       |
| `localhost:63791`      | `startupai-redis:6379`          |
| `localhost:5150`       | `startupai-zitadel:8080`        |
| `localhost:5153`       | `rig-service:5153`              |
| `localhost:5154`       | `api-service:5154`              |
| `localhost:5155`       | `chat-service:5155`             |
| `localhost:5156`       | `notification-service:5156`     |

## Directory Structure

```
local/
├── kustomization.yaml    # Kustomize config with URL patches
├── configmaps/           # Generated ConfigMaps (gitignored)
│   ├── api-service.yaml
│   ├── chat-service.yaml
│   └── ...
├── values/               # Helm values per service
│   ├── api-service.yaml
│   └── ...
└── kind-config.yaml      # Kind cluster configuration
```

## Updating URL Mappings

If you need to add or change URL overrides, edit the `patches` section in
`kustomization.yaml`. Each patch targets a ConfigMap by name and merges
additional `data` keys.

## Files

| File | Service | Database |
|------|---------|----------|
| `configmaps/api-service.yaml` | Main API | `api_service` |
| `configmaps/chat-service.yaml` | Chat/messaging | `chat_service` |
| `configmaps/notification-service.yaml` | Notifications | `notification_service` |
| `configmaps/rig-service.yaml` | AI/LLM service | `rig_service` |
| `configmaps/web.yaml` | Next.js frontend | - |
