# Production Stack

Provisions the complete production environment on GCP: GKE, Cloud SQL, Artifact
Registry, and ArgoCD.

## Environment Variables

```bash
export TF_VAR_gcp_project_id="your-gcp-project-id"
export TF_VAR_domain="yourdomain.com"
export TF_VAR_github_argocd_token="ghp_xxx"  # For private repo access
```

## Setup

```bash
# One-time: create state bucket (shared with staging)
gsutil mb -p $TF_VAR_gcp_project_id -l us-central1 gs://memoir-terraform-state
gsutil versioning set on gs://memoir-terraform-state

# Apply
terraform init
terraform apply
```

## ArgoCD Sync

CI updates values files on push to `main`. ArgoCD picks up changes via polling
(default 3 min).

### Instant Sync (Optional)

For instant sync from CI, set up a Cloudflare Tunnel:

```bash
# 1. Install cloudflared
brew install cloudflared  # or download from GitHub releases

# 2. Authenticate (opens browser)
cloudflared tunnel login

# 3. Create tunnel
cloudflared tunnel create argocd-production

# 4. Route DNS (replace with your domain)
cloudflared tunnel route dns argocd-production argocd-tunnel-production.yourdomain.com

# 5. Get tunnel token
cloudflared tunnel token argocd-production
# Copy the token output

# 6. Apply with tunnel token
export TF_VAR_cloudflare_tunnel_token="<token-from-step-5>"
terraform apply
```

Then in Cloudflare Zero Trust dashboard:

1. Access → Applications → Add application → Self-hosted
2. Domain: `argocd-tunnel-production.yourdomain.com`
3. Add policy with Service Auth + Service Token
4. Access → Service Tokens → Create token for GitHub Actions

Finally, add to GitHub:

| Secret | Value |
|--------|-------|
| `CF_ACCESS_CLIENT_ID` | Service token client ID |
| `CF_ACCESS_CLIENT_SECRET` | Service token secret |

| Variable | Value |
|----------|-------|
| `ARGOCD_TUNNEL_PRODUCTION` | `argocd-tunnel-production.yourdomain.com` |
