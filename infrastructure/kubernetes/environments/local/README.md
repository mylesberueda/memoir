# Local Kubernetes Environment

Kubernetes manifests and Kustomize overlays for the local Kind cluster.

Pre-cleanup this environment hosted ConfigMaps and Helm value overrides for
five services (api-service, chat-service, notification-service, rig-service,
web). Epic 0002 deleted those services and emptied the Kustomize patches +
Helm values. The memoir-server scaffold and memoir-ui rename epics
repopulate these.

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
Redis) running in Docker Compose via the shared `kind` Docker network:

```
┌─────────────────────────────────────────────────────────────┐
│                     Kind Cluster                            │
│  ┌─────────────┐                                            │
│  │ memoir-     │  (populated by the memoir-server epic)     │
│  │   server    │                                            │
│  └──────┬──────┘                                            │
│         │                                                   │
└─────────┼───────────────────────────────────────────────────┘
          │ kind network
┌─────────┼───────────────────────────────────────────────────┐
│         ▼              Docker Compose                       │
│  ┌────────────────┐   ┌────────────────┐                    │
│  │ memoir-postgres│   │  memoir-redis  │                    │
│  │     :5432      │   │     :6379      │                    │
│  └────────────────┘   └────────────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

## URL Mappings

The Kustomize patches override these localhost URLs with K8s-resolvable names.
Empty until the memoir-server epic adds its mapping.

| Local Dev (.env)       | K8s Cluster (patched)           |
|------------------------|---------------------------------|
| `localhost:54321`      | `memoir-postgres:5432`          |
| `localhost:63791`      | `memoir-redis:6379`             |

## Directory Structure

```
local/
├── kustomization.yaml    # Kustomize config with URL patches (empty post-0002)
├── configmaps/           # Generated ConfigMaps (gitignored)
├── values/               # Helm values per service (empty post-0002)
└── kind-config.yaml      # Kind cluster configuration
```

## Updating URL Mappings

If you need to add or change URL overrides, edit the `patches` section in
`kustomization.yaml`. Each patch targets a ConfigMap by name and merges
additional `data` keys.
