# GitHub Actions Workflow Setup Guide

This guide will help you configure the necessary secrets, variables, and infrastructure to use the CI/CD workflows in this template repository.

## Table of Contents

- [Quick Start Checklist](#quick-start-checklist)
- [Required GitHub Secrets](#required-github-secrets)
- [Optional GitHub Variables](#optional-github-variables)
- [Runner Configuration](#runner-configuration)
- [Infrastructure Setup](#infrastructure-setup)
- [Testing Your Setup](#testing-your-setup)
- [Troubleshooting](#troubleshooting)

## Quick Start Checklist

### Minimum Required Setup

- [ ] Add `NX_KEY` secret
- [ ] Choose and configure runner strategy (self-hosted or GitHub-hosted)
- [ ] Set up caching strategy (MinIO or GitHub Actions Cache)

### Required for Releases

- [ ] Add `CARGO_REGISTRY_TOKEN` secret (publishes Rust crates to crates.io)
- [ ] Add `NPM_TOKEN` secret (publishes TS SDK to npmjs)

### Optional Setup

- [ ] Add `PROJECTS_TOKEN` secret (for GitHub Projects automation)
- [ ] Add `PROJECT_NAME` variable (for GitHub Projects)
- [ ] Add `NX_POWERPACK_LICENSE` secret (for Nx Powerpack features)

## Required GitHub Secrets

### 1. NX_KEY (Required)

**Purpose**: Enables Nx Cloud distributed caching and task execution for faster builds.

**How to get it**:

1. Go to [Nx Cloud](https://nx.app/)
2. Sign up or log in with your GitHub account
3. Connect your repository
4. Copy the access token from your workspace settings

**How to add it**:

```bash
# Via GitHub CLI
gh secret set NX_KEY

# Via GitHub UI
Repository → Settings → Secrets and variables → Actions → New repository secret
Name: NX_KEY
Value: <your-nx-cloud-token>
```

**Used in**: `ci.yml`, `build.yml`, `test.yml`, `lint.yml`

### 2. CARGO_REGISTRY_TOKEN (Required for releases)

**Purpose**: Authenticates `cargo publish` to crates.io for the release workflow (`release.yml`).

**How to get it** — token scoping is lifecycle-dependent:

**First release (bootstrap token):**

1. Sign in to [crates.io](https://crates.io/) with your GitHub account.
2. Go to [Account Settings → API Tokens](https://crates.io/settings/tokens).
3. Click **New Token**:
   - Token name: `memoir-bootstrap`.
   - Scopes: **`publish-new` AND `publish-update`**. `publish-new` is required because the first release of each crate creates it on the registry — `publish-update`-only tokens cannot create new crate names.
   - Crates: leave unrestricted. Per-crate scoping only applies after the crate exists; it cannot scope a `publish-new` operation to names that don't exist yet.
4. Copy the token (only shown once).
5. After the first successful release publishes both crates, **revoke this bootstrap token** and create a steady-state one (next subsection).

**Steady-state releases (after first publish lands):**

1. Same flow, new token:
   - Token name: `memoir-release-ci`.
   - Scopes: **`publish-update`** only. Bootstrap is done; the token never needs to create new crate names again.
   - Crates: restrict to `polypixel-memoir-core` and `polypixel-memoir-sdk` (least privilege — token cannot publish anything else).
2. Replace the bootstrap token in GitHub Secrets with this one.

**How to add it (either token type)**:

```bash
# Via GitHub CLI
gh secret set CARGO_REGISTRY_TOKEN

# Via GitHub UI
Repository → Settings → Secrets and variables → Actions → New repository secret
Name: CARGO_REGISTRY_TOKEN
Value: <your-crates.io-token>
```

**Rotation**: Rotate after each contributor with token access leaves, or annually. Revoke via the same crates.io tokens page. Adding a third publishable crate later means temporarily swapping back to a `publish-new`-scoped token for that first publish, then back to the per-crate steady-state token.

**Used in**: `release.yml` (only on `v*` tag pushes).

### 3. NPM_TOKEN (Required for releases)

**Purpose**: Authenticates `pnpm publish` to npmjs.com for `@polypixel/memoir-sdk` (used in `release.yml`).

**Prerequisite**: The `@polypixel` org must exist on npmjs.com (free) and your account must be a member/owner with publish rights to it. Claim the scope before generating any token.

**How to get it** — token scoping is lifecycle-dependent:

**First release (bootstrap token):**

1. Sign in to [npmjs.com](https://www.npmjs.com/).
2. Go to **Access Tokens** → **Generate New Token** → **Granular Access Token**.
3. Configure:
   - Token name: `memoir-bootstrap`.
   - Expiration: 30 days (you'll replace it after first publish anyway).
   - Permissions: **Read and write** on packages.
   - Packages and scopes: **Selected packages and scopes → `@polypixel` (whole scope)**. The per-package picker can't select `@polypixel/memoir-sdk` until it exists on the registry. Scope-level grant lets the token create the package on first publish.
4. Copy the token (only shown once).
5. After the first successful release publishes the package, **revoke this bootstrap token** and create a steady-state one (next subsection).

**Steady-state releases (after first publish lands):**

1. Same flow, new token:
   - Token name: `memoir-release-ci`.
   - Expiration: 1 year (or your security policy's max).
   - Permissions: **Read and write** on packages.
   - Packages and scopes: **Selected packages → `@polypixel/memoir-sdk`** (now selectable; least privilege).
2. Replace the bootstrap token in GitHub Secrets with this one.

**Alternative — Classic Automation token** (simpler but broader): npmjs also offers Classic "Automation" tokens that bypass 2FA challenges. They have no package-level scoping (your whole account's publish power) but are easier to set up. Acceptable for a one-account org; not recommended once contributors share the workflow.

**How to add it (either token type)**:

```bash
# Via GitHub CLI
gh secret set NPM_TOKEN

# Via GitHub UI
Repository → Settings → Secrets and variables → Actions → New repository secret
Name: NPM_TOKEN
Value: <your-npmjs-token>
```

**Rotation**: Rotate annually or when the granular token expires (npmjs sends email reminders ~30 days out). Revoke via the same npmjs Access Tokens page. Adding a second `@polypixel/...` package later: bootstrap-token cycle again unless the steady-state token is scope-level rather than package-level.

**Used in**: `release.yml` (only on `v*` tag pushes).

## Optional GitHub Secrets

### 1. PROJECTS_TOKEN (Optional)

**Purpose**: GitHub Personal Access Token for automating GitHub Projects (used in `set-end-date-on-close.yml`).

**When you need it**: Only if you want automatic project field updates when issues close.

**How to generate it**:

1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate new token with scopes:
   - `repo` (Full control of private repositories)
   - `project` (Full control of projects)
3. Copy the token

**How to add it**:

```bash
# Via GitHub CLI
gh secret set PROJECTS_TOKEN

# Via GitHub UI
Repository → Settings → Secrets and variables → Actions → New repository secret
Name: PROJECTS_TOKEN
Value: <your-github-pat>
```

**Used in**: `set-end-date-on-close.yml`

### 2. NX_POWERPACK_LICENSE (Optional)

**Purpose**: Enables Nx Powerpack features for enhanced performance and capabilities.

**When you need it**: Only if you have an Nx Powerpack license and want to use premium features.

**How to get it**:

1. Purchase Nx Powerpack from [Nx](https://nx.dev/pricing)
2. Copy your license key

**How to add it**:

```bash
# Via GitHub CLI
gh secret set NX_POWERPACK_LICENSE

# Via GitHub UI
Repository → Settings → Secrets and variables → Actions → New repository secret
Name: NX_POWERPACK_LICENSE
Value: <your-nx-powerpack-license>
```

**Used in**: `release.yml`, `update-cache.yml`

## Optional GitHub Variables

### 1. PROJECT_NAME (Optional)

**Purpose**: Name of your GitHub Project for automated issue tracking.

**When you need it**: Only if using GitHub Projects automation with `set-end-date-on-close.yml`.

**How to add it**:

```bash
# Via GitHub CLI
gh variable set PROJECT_NAME --body "Your Project Name"

# Via GitHub UI
Repository → Settings → Secrets and variables → Actions → Variables tab → New repository variable
Name: PROJECT_NAME
Value: Your Project Name
```

**Example**: `"Memoir Development"`

**Used in**: `set-end-date-on-close.yml`

## Runner Configuration

The workflows are configured to use **self-hosted runners** by default. You have two options:

### Option 1: Self-Hosted Runners (Default - Recommended for Production)

**Runner Label**: `arc-runner-set`

**Advantages**:

- Faster builds with persistent caching
- Lower costs for high-volume CI/CD
- Full control over runner environment

**Requirements**:

1. Kubernetes cluster
2. Actions Runner Controller (ARC) installed
3. MinIO for S3-compatible caching

**Setup Instructions**: See [Infrastructure Setup](#infrastructure-setup) below.

### Option 2: GitHub-Hosted Runners (Easier Setup)

**How to switch**:

1. Find and replace `arc-runner-set` with `ubuntu-latest` in all workflow files:

   ```bash
   # From repository root
   find .github/workflows -name "*.yml" -type f -exec sed -i 's/arc-runner-set/ubuntu-latest/g' {} +
   ```

2. Modify cache action usage:
   - The smart cache setup in `.github/actions/setup-cache/action.yml` will automatically detect GitHub-hosted runners and use GitHub Actions Cache instead of MinIO.

**Trade-offs**:

- Easier setup (no infrastructure required)
- Slower builds (no persistent caching between runners)
- Higher costs for high-volume usage
- GitHub's usage limits apply

## Infrastructure Setup

### Self-Hosted Runner Setup with ARC

If using self-hosted runners (recommended), you'll need to set up the following infrastructure:

#### 1. Actions Runner Controller (ARC)

**Purpose**: Manages self-hosted GitHub Actions runners in Kubernetes.

**Installation**:

```bash
# Add the ARC Helm repository
helm repo add actions-runner-controller https://actions-runner-controller.github.io/actions-runner-controller
helm repo update

# Install ARC
helm install arc \
  --namespace actions-runner-system \
  --create-namespace \
  actions-runner-controller/actions-runner-controller \
  --set authSecret.github_token="<GITHUB_PAT>"
```

**Create Runner Deployment**:

```yaml
# arc-runner-set.yaml
apiVersion: actions.summerwind.dev/v1alpha1
kind: RunnerDeployment
metadata:
  name: arc-runner-set
  namespace: actions-runner-system
spec:
  replicas: 3
  template:
    spec:
      repository: <your-org>/<your-repo>
      labels:
        - arc-runner-set
      dockerEnabled: true
      dockerdWithinRunnerContainer: true
```

Apply the configuration:

```bash
kubectl apply -f arc-runner-set.yaml
```

**Resources**:

- [ARC Documentation](https://github.com/actions/actions-runner-controller)
- [ARC Quickstart](https://github.com/actions/actions-runner-controller/blob/master/docs/quickstart.md)

#### 2. MinIO S3 Cache

**Purpose**: Provides S3-compatible storage for workflow caching on self-hosted runners.

**Installation**:

```bash
# Add MinIO Helm repository
helm repo add minio https://charts.min.io/
helm repo update

# Install MinIO
helm install minio minio/minio \
  --namespace minio-system \
  --create-namespace \
  --set accessKey=admin \
  --set secretKey=minio123 \
  --set persistence.size=100Gi
```

**Create Cache Bucket**:

```bash
# Port forward to MinIO
kubectl port-forward -n minio-system svc/minio 9000:9000

# Install MinIO client
brew install minio/stable/mc  # macOS
# or
wget https://dl.min.io/client/mc/release/linux-amd64/mc

# Configure MinIO client
mc alias set local http://localhost:9000 admin minio123

# Create cache bucket
mc mb local/github-actions-cache

# Set public policy (for internal cluster use)
mc anonymous set download local/github-actions-cache
```

**Note**: The default credentials (`admin/minio123`) are configured in `.github/actions/setup-cache/action.yml`. For production, you should:

1. Change these credentials
2. Update the action file accordingly
3. Use Kubernetes secrets for sensitive values

**Resources**:

- [MinIO Documentation](https://min.io/docs/minio/kubernetes/upstream/)
- [MinIO Helm Chart](https://github.com/minio/minio/tree/master/helm/minio)

#### 3. Test Services Namespace

**Purpose**: Isolated namespace for PostgreSQL and Redis test databases during CI/CD.

The test services are automatically set up by the workflow using `.github/actions/setup-services/action.yml`. No manual setup required, but ensure your Kubernetes cluster has:

- Sufficient resources for test pods
- Network policies allowing runner → test-services communication
- Storage class for PostgreSQL persistent volumes

## Testing Your Setup

### 1. Verify Secrets Are Set

```bash
# List all secrets (values are hidden)
gh secret list

# Expected output should include:
# NX_KEY
```

### 2. Verify Variables Are Set (Optional)

```bash
# List all variables
gh variable list

# Expected output (if using GitHub Projects):
# PROJECT_NAME
```

### 3. Test Workflow Execution

Create a test branch and push to trigger CI:

```bash
# Create a test branch
git checkout -b test/workflow-setup

# Make a trivial change
echo "# Test" >> README.md

# Commit and push
git add README.md
git commit -m "test: verify workflow setup"
git push origin test/workflow-setup

# Open a PR to trigger CI
gh pr create --title "Test: Verify workflow setup" --body "Testing CI/CD configuration"
```

### 4. Monitor Workflow Execution

```bash
# Watch workflow runs
gh run list --limit 5

# View specific run logs
gh run view <run-id> --log
```

### 5. Expected Workflow Behavior

On a successful setup, you should see:

1. ✅ **detect-changes** - Identifies affected projects
2. ✅ **lint** - Runs linters on affected code
3. ✅ **format** - Checks code formatting
4. ✅ **build** - Builds affected projects
5. ✅ **test** - Runs test suites
6. ✅ **ci-summary** - Overall status report

## Troubleshooting

### Common Issues

#### 1. "NX_KEY is not set"

**Symptom**: Workflows fail with Nx Cloud errors.

**Solution**:

```bash
# Verify secret is set
gh secret list | grep NX_KEY

# If missing, add it
gh secret set NX_KEY
```

#### 2. "Runner not found: arc-runner-set"

**Symptom**: Workflows are queued but never start.

**Solutions**:

- **Option A**: Switch to GitHub-hosted runners (see [Option 2](#option-2-github-hosted-runners-easier-setup))
- **Option B**: Set up ARC (see [Infrastructure Setup](#infrastructure-setup))

Verify runners are online:

```bash
kubectl get pods -n actions-runner-system
```

#### 3. "MinIO cache connection failed"

**Symptom**: Cache operations fail on self-hosted runners.

**Solution**:

1. Verify MinIO is running:

   ```bash
   kubectl get pods -n minio-system
   ```

2. Check bucket exists:

   ```bash
   kubectl exec -n minio-system deploy/minio -- mc ls local/
   ```

3. Verify credentials in `.github/actions/setup-cache/action.yml` match your MinIO setup

### Getting Help

If you continue experiencing issues:

1. **Check workflow logs**:

   ```bash
   gh run view <run-id> --log
   ```

2. **Review the specific workflow file** in `.github/workflows/` for configuration details

3. **Check project documentation**: See main project README and CLAUDE.md for project-specific setup

4. **Open an issue**: Include workflow logs and error messages

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Nx Cloud Documentation](https://nx.dev/ci/intro/ci-with-nx)
- [Actions Runner Controller](https://github.com/actions/actions-runner-controller)
- [MinIO Documentation](https://min.io/docs/minio/kubernetes/upstream/)

## Maintenance

### Updating Secrets

Secrets should be rotated periodically for security:

```bash
# Update a secret
gh secret set SECRET_NAME --body "new-value"

# Or interactively
gh secret set SECRET_NAME
```

### Monitoring Usage

**Nx Cloud**: Monitor cache usage and CI analytics at [nx.app](https://nx.app/)

**GitHub Actions**: Check usage limits at Repository → Settings → Actions → General

**MinIO**: Monitor storage usage:

```bash
kubectl exec -n minio-system deploy/minio -- mc admin info local
```

## Next Steps

After completing this setup:

1. Review the [main project README](../../README.md) for project-specific configuration
2. Check [CLAUDE.md](../../CLAUDE.md) for development workflow guidelines
3. Review individual workflow files for customization options
4. Consider setting up branch protection rules requiring CI to pass
