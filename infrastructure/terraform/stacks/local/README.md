# Local Stack

Provisions the local development environment on Kind: ArgoCD, Zitadel, and
Stripe.

## Quick Start

```bash
# 1. Start Docker Compose services
docker compose up -d

# 2. Create Kind cluster
kind create cluster --config ../../../kubernetes/environments/local/kind-config.yaml

# 3. Set context (if needed)
kubectl config use-context kind-memoir

# 4. Set environment variables
export TF_VAR_github_argocd_token="ghp_xxx"
export TF_VAR_argocd_target_revision="your-branch"

# 5. Apply Terraform
terraform init
terraform apply

# 6. Generate and apply ConfigMaps (with Kustomize URL patches)
pnpm k8s:configmap && kubectl apply -k ../../kubernetes/environments/local/
```

## Re-apply After Docker Restart

If Docker container IPs change (e.g., after `docker compose down && up`):

```bash
terraform apply
```

## ArgoCD Sync

CI updates values files on push to `ci/test/*` branches. ArgoCD picks up changes
via polling (default 3 min).

### Instant Sync (Optional)

For instant sync from CI, set up a Cloudflare Tunnel:

```bash
# 1. Install cloudflared
brew install cloudflared  # or download from GitHub releases

# 2. Authenticate (opens browser)
cloudflared tunnel login

# 3. Create tunnel
cloudflared tunnel create argocd-local

# 4. Route DNS (replace with your domain)
cloudflared tunnel route dns argocd-local argocd-tunnel-local.yourdomain.com

# 5. Get tunnel token
cloudflared tunnel token argocd-local
# Copy the token output

# 6. Apply with tunnel token
export TF_VAR_cloudflare_tunnel_token="<token-from-step-5>"
terraform apply
```

Then in Cloudflare Zero Trust dashboard:

1. Access → Applications → Add application → Self-hosted
2. Domain: `argocd-tunnel-local.yourdomain.com`
3. Add policy with Service Auth + Service Token
4. Access → Service Tokens → Create token for GitHub Actions

Finally, add to GitHub:

| Secret | Value |
|--------|-------|
| `CF_ACCESS_CLIENT_ID` | Service token client ID |
| `CF_ACCESS_CLIENT_SECRET` | Service token secret |

| Variable | Value |
|----------|-------|
| `ARGOCD_TUNNEL_LOCAL` | `argocd-tunnel-local.yourdomain.com` |
