# IaC Rules

The rules that keep this codebase forward-compatible across cloud providers and managed-service migrations. Read before any infrastructure change.

The goal is a single one: **never spend engineering hours migrating because of a coupling we should have avoided.** Specific provider choices, prices, and SKUs change. The rules below should not.

---

## The core invariant

**Compute is a commodity. State lives off the cluster.**

Every architectural decision in this repo should be checked against that sentence. If a change makes the compute provider harder to swap, or pulls state into the cluster, it's adding migration cost we'll pay later.

---

## Forward-compatibility principles

### 1. Stateful services are external, addressed by URL

Postgres, auth, object storage, queues, search, and (when not in-cluster) cache are reached via standard URLs from environment variables. The app must not know or care which provider hosts them.

**Why:** moving Postgres from Supabase to RDS, or object storage from R2 to S3, becomes a config change instead of a code change. Same for moving compute to a new provider — the stateful URLs come along unchanged.

**How to apply:** every stateful dependency goes through env-var-configured connection strings. No SDK lock-in for things that have a standard wire protocol (Postgres → use `postgres://`, not provider SDKs; Redis-compatible → use `redis://`, not provider client libs; S3-compatible → use S3 API against R2/MinIO/S3 alike).

### 2. The compute provider is one Terraform module

The cluster — VM, networking, firewall, cloud-init — lives in a single `modules/<provider>/` tree. Workload definitions (Helm charts, ApplicationSets, NetworkPolicies, per-environment values) reference the cluster only via remote-state outputs.

**Why:** swapping providers is "rewrite one module, leave production stack untouched." If workload definitions reach into provider-specific resources directly, the migration is no longer scoped.

**How to apply:** bootstrap stack creates the cluster + edge wiring; production stack reads bootstrap's outputs (`data.terraform_remote_state.bootstrap`) for kubeconfig and shared identifiers. Production stack never imports provider modules directly.

### 3. Workload manifests are portable across Kubernetes distributions

Standard k8s resources (Deployments, Services, ConfigMaps, Secrets, NetworkPolicies, HPAs, PDBs, ServiceAccounts, Ingresses) work everywhere. Use those by default.

CRDs from cluster add-on platforms (service mesh, secrets operators, ingress controllers, GitOps, cert managers) are allowed **only if the platform itself runs on any compliant k8s cluster** — not if it's tied to a specific cloud provider's managed offering. ArgoCD, Istio, Linkerd, cert-manager, ExternalSecrets, KEDA all qualify. Provider-specific CRDs (`BackendConfig`, `AWSLoadBalancerController` annotations, `gke-*` resources) do not.

**Why:** the goal is "this workload can run on any k8s cluster I bring up next." That's true for portable platform CRDs (you install the same operator on the new cluster) but false for provider-specific ones (you'd rewrite manifests for the new provider).

**How to apply:**

- The base Helm chart for services uses only standard k8s resources.
- Platform CRDs go in dedicated charts (`infrastructure/kubernetes/helm/charts/<platform>/`) so it's clear what add-ons a cluster requires.
- The bootstrap stack documents which platforms must be installed for the workloads to run. A new cluster brought up on a new provider installs the same platform set.
- Cloud-specific resources are gated behind environment-specific values overlays, never in the base chart.

**On adding a new platform CRD dependency:** the question to answer is "does the value of this platform exceed the cost of carrying it on every future cluster?" Lightweight, well-scoped tools (cert-manager, ExternalSecrets) usually pay back. Heavy platforms (full service mesh) usually don't until the workload's complexity demands them.

### 4. Edge concerns live at the edge, not in-cluster

TLS termination, DNS, WAF, CDN, identity gating, and external ingress route through a fourth-party edge provider (Cloudflare, Fastly, Akamai). Compute-provider-bundled edge offerings (AWS Route53/WAF/CloudFront, GCP Cloud DNS/Armor/CDN) defeat this principle by recoupling edge to the compute provider. The cluster speaks plain HTTP internally and is reachable only via outbound tunnels.

**Why:** the cluster doesn't need a public IP, doesn't need cert-manager, doesn't need a cloud LoadBalancer Service, doesn't need to know about DNS. Moving compute providers doesn't disturb any of those concerns. Same edge config wraps any cluster on any provider.

**How to apply:** use Cloudflare Tunnel (or equivalent outbound-only ingress) for inbound traffic. Never expose Service type=LoadBalancer publicly. TLS is the edge's problem.

### 5. In-cluster state is rebuildable or disposable

Anything that lives in the cluster — in-cluster Redis/Valkey, ephemeral caches, queue state, session stores — must be tolerable to lose on a node restart. The app handles cold-cache and missing-job scenarios gracefully.

**Why:** this is what makes "destroy and recreate the cluster" a routine operation rather than a data-loss event. It's also what makes the move from in-cluster cache to managed cache a one-line URL change instead of a data migration.

**How to apply:** in-cluster Redis runs without persistence (`dataStorage: disabled`). Cache layer implements rebuild-on-miss with request coalescing. Anything that genuinely needs durability goes to Postgres or external storage, not in-cluster.

### 6. Secrets and config are env-var shaped

Every service reads its config from environment variables. Secrets land as Kubernetes Secrets, mounted as env vars. No provider-specific secret-fetching SDKs in app code.

**Why:** provider secret managers (GCP Secret Manager, AWS Secrets Manager, Vault, etc.) all have sync mechanisms that can land values into a Kubernetes Secret. As long as the app reads env vars, the source of those secrets is swappable without app changes.

**How to apply:** ExternalSecrets / SecretStore CRDs in the cluster, sourcing from whatever provider is current. App code only sees `process.env.X` / `std::env::var("X")`.

### 7. Terraform state is on neutral ground

State lives in S3-compatible object storage that isn't tied to the compute provider (e.g., R2). Same for any artifacts the bootstrap process needs to produce.

**Why:** moving compute providers shouldn't require also moving the state backend. Having state on the same provider as compute creates a chicken-and-egg problem during migration.

**How to apply:** R2 (or any S3-compatible third-party) backend for all stacks. State backend bucket pre-created out-of-band, not managed by the stack that depends on it.

### 8. The bootstrap path is reproducible from a clean environment

A new operator with credentials can stand up the entire infrastructure from `terraform apply` + documented one-time prerequisites. No undocumented manual clicks. No assumed cluster state from a prior bootstrap.

**Why:** if the only person who knows how the cluster came up is the person who ran the original `apply` six months ago, you can't safely move providers, recover from disaster, or onboard help.

**How to apply:** prerequisites documented in a single place (e.g., `DEPLOY.md`); cloud-init handles all in-VM setup; bootstrap stack is idempotent on re-apply.

---

## Decision filter for any IaC change

Before making an infrastructure change, ask:

1. **Does this couple the workload to a specific provider?** (Provider-specific CRDs, SDK imports, IAM-shaped auth in app code, hostnames hardcoded to provider domains.) If yes, can it be expressed in a vanilla way instead?
2. **Does this put recoverable state in an unrecoverable place?** (Local volumes, in-cluster databases used as source of truth, secrets stored only in cluster.) If yes, is the data tolerant of loss, or does it belong externally?
3. **Does this require a manual step that isn't documented?** If yes, document it or automate it before merging.
4. **Does this assume a specific provider's pricing model?** (Pod-per-second billing, free egress, included backups.) If yes, can the workload still function on a provider with different economics?

**The default answer is "find the uncoupled way."** Coupling is a last resort, not a fallback. Most "but the business reason justifies it" arguments are post-hoc rationalizations of the path of least resistance — a managed service that's faster to wire up today, a provider-specific feature that solves the immediate problem without considering the next one, an SDK that's the documented happy path.

A real exception meets all of these:

1. **A specific, named requirement** the uncoupled approach genuinely cannot satisfy (not "would be harder," not "would take longer" — *cannot*).
2. **Explicit owner sign-off** from the engineer who will pay the migration cost later. If that's you and you're tempted to wave it through, that's the signal to take the uncoupled path.
3. **A written entry in the decision log below**, naming the coupling, the requirement that forced it, and the migration cost being accepted.

If a proposed exception can't clear all three bars, take the uncoupled path. "We can always revisit later" is how exceptions accumulate until the principles no longer mean anything.

---

## What this document deliberately doesn't say

- **Which provider to use.** That's a tactical decision driven by current pricing, customer requirements, and operational maturity. The architecture should make that decision cheap to revisit.
- **What stage of growth uses what.** Same reason. The signals that justify a move change as the product evolves.
- **Specific SKUs, prices, or feature comparisons.** All of those go stale within months. When a migration is being planned, do the comparison fresh against current pricing.
- **Whether managed services are worth the cost.** That's a function of headcount, ops time, and revenue at any given moment. The architecture should make managed adoption a config change, not a rewrite.

The plan-of-the-month belongs in a separate doc, a ticket, or a conversation. This document is what stays true regardless of which provider is current.

---

## When to update this document

Add a rule when a real migration would have been cheaper if it had existed beforehand. Remove a rule when a migration proves it was wrong. Don't add tactical advice; that belongs elsewhere.

| Date | Change | Reason |
|------|--------|--------|
| 2026-05-08 | Initial principles. | Codifying forward-compatibility lessons before next provider decision. |
