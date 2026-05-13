# Deploy

This template uses a **deploy-branch GitOps pattern**. Humans land code on `dev` (staging) and `main` (production). CI builds images, then commits the new image digests to a parallel branch named after the source branch that ArgoCD watches:

| Source branch | Environment | Deploy branch  |
|---------------|-------------|----------------|
| `main`        | production  | `deploy/main`  |
| `dev`         | staging     | `deploy/dev`   |

Devs working on `dev` and `main` never see bot deploy commits in their `git log`. Rebasing onto `dev` does not race against CI.

The local environment (`ci/test/*` branches) does NOT use the deploy-branch pattern. CI still runs full image builds for validation on `ci/test/*` pushes, but no deploy commit lands. Local cluster deployment is wired manually until the test-branch cluster story is built — see [Future](#future) below.

The mechanism lives in `.github/workflows/docker.yml` — grep for `deploy/main` or `deploy/dev` to find it.

## Convention: humans never push to `deploy/*`

Pushing directly to a deploy branch breaks the next CI deploy (the bot's push will be rejected as non-fast-forward, surfaced as a loud error in the workflow log).

This is enforced by **convention**, not by GitHub branch protection. The template is designed for AI-prototype teams on **GitHub Free with private repos**, where neither branch protection rules nor rulesets are available. If your fork is on a paid plan (Pro / Team / Enterprise), add a native branch protection rule on `deploy/*` as defense-in-depth — see [About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches).

## First deploy (chicken-and-egg)

ArgoCD's `targetRevision` is set to `deploy/<source-branch>` in the per-stack Terraform config (`infrastructure/terraform/stacks/<env>/mise.local.toml`). The deploy branches don't exist on origin yet — CI auto-creates them on first push. So the bootstrap order is:

1. **Push to `dev`** (or `main`) first. CI runs, detects that `deploy/dev` (or `deploy/main`) doesn't exist on origin, creates it from your source commit, and pushes the first image-digest commit to it. The workflow run UI shows a loud "🆕 Bootstrapped `deploy/dev`" summary.
2. **Then `terraform apply`**. ArgoCD finds the deploy branch already exists and reconciles cleanly.

Doing it in the other order works but produces an alarming-looking ArgoCD dashboard state (Application stuck at "OutOfSync, Unknown ref") until the first CI push lands. Not broken — just noisy.

## Rolling back

ArgoCD reconciles whatever's at the tip of `deploy/<source-branch>`. Roll back by reverting the bot's last commit on the deploy branch:

```sh
git fetch origin deploy/main
git checkout deploy/main
git revert HEAD
git push origin deploy/main
```

ArgoCD picks up the revert within ~3 min (default poll), or instantly if the Cloudflare Tunnel sync trigger is configured (see `docker.yml`'s `Trigger ArgoCD sync` step).

This **does not rebuild the image** — it only changes which image tag is deployed. The previous image is still in the registry; ArgoCD just points at it again.

## Forcing a redeploy of the current digest

ArgoCD UI → Sync button. Or:

```sh
argocd app sync <project>-<env>
```

(App naming convention is `<project>-<env>`, e.g., `api-service-staging`.)

## Forking the template — bot identity

The CI workflow commits to `deploy/<source-branch>` as `startup-ai-deploy-bot <deploy-bot@users.noreply.github.com>` by default. To use your own bot identity, set repo variables:

```sh
gh variable set DEPLOY_BOT_NAME --body "myproject-deploy-bot"
gh variable set DEPLOY_BOT_EMAIL --body "deploy-bot@myproject.example"
```

The defaults are fine for most adopters — the noreply email won't bounce, and the name only matters for `git log` readability.

## Why not a separate config repo?

The canonical GitOps pattern is a separate repository for environment config that ArgoCD watches. The deploy-branch pattern is a one-repo simplification: same audit trail, same rollback story, fewer moving parts to maintain. For prototype teams the tradeoff is worth it; for larger setups, splitting to a config repo is a clean migration (move `infrastructure/kubernetes/environments/` to a new repo, point ArgoCD at it, drop the `update-values` job).

## Future

**Test-branch cluster for parallel `ci/test/*` deploys.** Currently `ci/test/*` builds run in CI but no deploy commit lands; you point your local cluster at a deploy branch by hand if you want to test in-cluster. The planned shape: a small dedicated cluster (Hetzner `cpx21`/`cpx31` is the candidate, ~€8-15/mo) with ArgoCD configured to watch a per-branch pattern. Multiple `ci/test/*` branches in flight would each get their own deploy branch (`deploy/ci/test/<branch>`), their own ArgoCD Application (via `pullRequest` or `git+matrix` ApplicationSet generator), and their own deployed services — preview-environments-per-branch.

The workflow's `update-values` job is currently gated on `inputs.environment != 'local'`. Lifting that gate plus changing the deploy-branch derivation for the local case is small (~10 lines). The cluster + ArgoCD ApplicationSet rewire is the bulk of the work (~3-4 days). The template's existing `argocd` Terraform module already uses ApplicationSet (`infrastructure/terraform/modules/argocd/main.tf:174-212`), so the change is "swap the `list` generator for `git`/`pullRequest`/`matrix`" rather than building ApplicationSet machinery from scratch.

Not committed work; whenever this gets built, document the cluster setup here and update the source-branch table at the top.
