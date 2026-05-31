# Releasing Memoir

## What gets published on a release

| Artifact | Registry | Triggered by |
|---|---|---|
| `polypixel-memoir-core` | crates.io | `v*` tag |
| `polypixel-memoir-sdk` | crates.io | `v*` tag |
| `@polypixel/memoir-sdk` | npmjs | `v*` tag |
| `ghcr.io/mylesberueda/memoir/memoir-service` | GHCR | `v*` tag (re-tags existing per-commit image) |
| GitHub Release with notes | GitHub | `v*` tag |

Two workflows fire in parallel on `v*`: `.github/workflows/release.yml` (cargo + npm + GitHub Release) and `.github/workflows/docker-release.yml` (Docker re-tag).

## Prerequisites (one-time setup)

1. `CARGO_REGISTRY_TOKEN` GitHub Secret. See `.github/workflows/README.md` for token scoping (bootstrap vs steady-state).
2. `NPM_TOKEN` GitHub Secret. Requires `@polypixel` npm scope ownership. See same README.
3. The tagged commit must already have its per-commit `:<sha>` Docker image in GHCR. This happens automatically when CI runs on `dev` or `main`.

## Cutting a release

Releases are tag-driven. Merging to `dev`/`main` does NOT release.

### 1. Bump versions

Three manifests must agree with the tag. All three must show the same version, sans the leading `v`.

```bash
# For v0.1.0:
packages/memoir-core/Cargo.toml      → version = "0.1.0"
packages/memoir-sdk-rs/Cargo.toml    → version = "0.1.0"
packages/memoir-sdk-ts/package.json  → "version": "0.1.0"

# For v0.1.0-rc.1:
# Same three files, each set to "0.1.0-rc.1"
```

Commit the bumps and merge to `main`.

### 2. Tag and push

Always cut a release candidate first. Pre-release tags publish to dist-tag `next` (npm) and do NOT bump sliding Docker tags (`:0.1`, `:0`, `:latest`) — much lower blast radius than a clean release.

```bash
git tag v0.1.0-rc.1 main
git push origin v0.1.0-rc.1
```

Both workflows fire. Watch:

```bash
gh run watch
```

### 3. Verify the rc

```bash
cargo info polypixel-memoir-core           # shows 0.1.0-rc.1
cargo info polypixel-memoir-sdk            # shows 0.1.0-rc.1
npm view @polypixel/memoir-sdk dist-tags   # shows next: 0.1.0-rc.1
docker pull ghcr.io/mylesberueda/memoir/memoir-service:v0.1.0-rc.1
gh release view v0.1.0-rc.1                # shows Pre-release badge
```

The sliding Docker tags must NOT point at the rc digest. Verify:

```bash
crane manifest ghcr.io/mylesberueda/memoir/memoir-service:0.1 2>/dev/null
# Either 404 (no prior clean release) or points at a prior clean release digest.
```

### 4. Cut the clean release

If the rc is clean:

```bash
git tag v0.1.0 main
git push origin v0.1.0
```

This publishes to npm dist-tag `latest`, bumps Docker `:0.1`/`:0`/`:latest` to the new digest, and creates a non-prerelease GitHub Release.

## Recovery

### A publish step failed

`release.yml` accepts `workflow_dispatch` with a `steps` input. Re-run only the failed steps:

```bash
gh workflow run release.yml \
  -f tag=v0.1.0 \
  -f steps=cargo,npm   # any subset of cargo,npm,github-release
```

Default is all three.

### A bad version was published

crates.io and npm versions are immutable. Yank and ship the next patch:

```bash
cargo yank --version 0.1.0 polypixel-memoir-core
cargo yank --version 0.1.0 polypixel-memoir-sdk
npm unpublish @polypixel/memoir-sdk@0.1.0  # only within 72 hours
# Then bump to 0.1.1 and cut a new release.
```

### Docker re-tag failed but cargo/npm succeeded

`docker-release.yml` re-runs cleanly via the Actions UI without touching cargo/npm. The per-commit `:<sha>` image is the source; `crane copy` is idempotent.

### Workflow rejected the tag

Common causes:

- **Version mismatch.** All three manifests must match the tag's version string. Fix the manifest, commit, retag at the new commit.
- **Tag not reachable from `main`.** The `validate` job runs `git merge-base --is-ancestor` against `origin/main`. Re-tag a `main` commit.
- **Per-commit Docker image missing.** The tagged commit must have been merged through `dev`/`main` first so `ci.yml` produced `:<sha>`. Merge the commit, wait for CI, then re-push the tag.

## File reference

- `.github/workflows/release.yml` — cargo + npm + GitHub Release
- `.github/workflows/docker-release.yml` — Docker version-tag re-tagging
- `.github/workflows/README.md` — secret setup, token scoping
- `apps/memoir-service/Dockerfile` — runtime image
- `.tasks/1000-release-operator-runbook.md` — verbose runbook with rationale for each decision
