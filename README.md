<div id="top" />

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/polypixel-labs/pixel-dashboard">
    <img src=".meta/startup.ai.gif" alt="STARTUP.ai Logo">
  </a>

<h3 align="center">committee.ai</h3>

  <p align="center">
    A starter template for multi-service apps
    <br />
    <a href="https://github.com/polypixel-labs/pixel-dashboard">
      <strong>Explore the docs »</strong>
    </a>
    <br />
    <br />
    <a href="https://github.com/polypixel-labs/pixel-dashboard">View Demo</a>
    ·
    <a href="https://github.com/polypixel-labs/pixel-dashboard/issues">Report Bug</a>
    ·
    <a href="https://github.com/polypixel-labs/pixel-dashboard/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#cicd">CI/CD</a></li>
        <li><a href="#deployment">Deployment</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

This is your basic Rust CLI starter kit. Most of the README isn't going to make
sense until I figure out exactly what and how I want the project to be built. In
the meantime, feel free to use this as-is.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

- [NX](https://nx.dev/)
- [Clap](https://github.com/clap-rs/clap)
- [Next.js](https://nextjs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

Using this template is pretty easy - just take a look at the prerequisites and
installation steps below.

### Prerequisites

- [Rust](https://rust-lang.org)
- [pnpm](https://pnpm.io/)
- [mise](https://mise.jdx.dev/)
- [SeaORM](https://www.sea-ql.org/SeaORM/)

### Installation

### Most brute-force way to use this template

> _This'll get you up and running fairly quickly_

1. Clone the repo

```sh
# with https
git clone https://github.com/polypixel-labs/pixel-dash.git

# or ssh
git clone git@github.com:polypixel-labs/pixel-dash.git
```

1. Remove the `.git` directory

```sh
# cd into the cloned directory
cd pixel-dash

# delete `.git`
rm -rf .git
```

1. Re-init git

```sh
git init
```

1. Create a repo on your preferred git hosting site (e.g. GitHub)
2. Add your new remote to your local git instance

```sh
git remote add origin <your url>
git add .
git commit -m 'init'
git push origin <your branch>
```

1. Start hacking away

### Simpler, since you're here

> _alternatively, the "I have a Github account" way_

1. [Github docs][github-docs] telling you to click the button above,
   labeled "Use this template".
2. ???
3. Profit by hacking away after you clone your new repo.

[github-docs]: https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template

<p align="right">(<a href="#top">back to top</a>)</p>

## Usage

### Starting development

All infrastructure runs inside a Kind cluster (no Docker Compose required).

```bash
# 1. Create the Kind cluster
kind create cluster --config infrastructure/kubernetes/environments/local/kind-config.yaml

# 2. Initialize and apply Terraform (first run requires targeted apply)
cd infrastructure/terraform/stacks/local
terraform init

# First time only: deploy infra first (creates Zitadel machine key)
terraform apply -target=helm_release.postgres -target=helm_release.redis -target=module.zitadel_helm -target=module.argocd

# Full apply (configures Zitadel, Stripe, deploys remaining services)
# Requires port-forward: kubectl port-forward svc/zitadel 5150:8080 -n zitadel &
terraform apply
```

Once your infra is standing, deploy services and set up the environment:

```bash
# from the workspace root
cargo cmd env init    # create all .env files, make sure to check each project
pnpm k8s:configmap   # generate K8s configmaps from .env files
kubectl apply -k infrastructure/kubernetes/environments/local/
pnpm migrate:up       # migrate all project tables
```

For local development with fast iteration, use [mirrord](https://mirrord.dev/)
to run services locally while connected to the cluster's network:

```bash
# Run a service locally with cluster networking
mirrord exec -f .mirrord/api-service.json -- cargo run

# or, if you only need to start a specific service in-cluster:
pnpm nx run api-service:dev
```

### CI/CD

This project uses Github actions by default to run tests and builds. You can
make sure that your changes will pass actions by running the following from
the project root:

```bash
pnpm nx affected --target deps
pnpm nx affected --target build
pnpm nx affected --target test
```

In general, if these pass on your local, CI will pass in Github actions.

**Security Scanning**: The security workflow (`security.yml`) is currently
disabled as it requires GitHub Advanced Security features. See
[`.github/README.md`][workflow-readme] for details on how to enable it when
those features are available.

[workflow-readme]: .github/README.md#enabling-security-features

### Deployment

Work happens on `dev` (staging) and `main` (production). CI builds images, then commits the new image digests to a parallel branch named after the source branch — `deploy/main`, `deploy/dev` — that ArgoCD watches. Devs working on `dev` and `main` never see bot deploy commits in their `git log`, and rebasing onto `dev` does not race against CI.

The deploy branches are auto-created by CI on first push to a new environment — no manual setup needed. See [`infrastructure/DEPLOY.md`][deploy-md] for the rollback runbook, the bootstrap order, and how to customize the bot identity in your fork.

[deploy-md]: ./infrastructure/DEPLOY.md

### Testing

Testing this project is fairly simple. The only _real_ caveat is that for
integration tests, all services _must_ be running, otherwise, they will fail.

```bash
pnpm test
pnpm test:integration
```

<p align="right">(<a href="#top">back to top</a>)</p>

## 🔐 Authentication

This project uses [Zitadel](zitadel) for identity and access management. See
[infrastructure/README.md][infrastructure-readme] for setup instructions.

[infrastructure-readme]: ./infrastructure/README.md

## 🏗️ Infrastructure changes

Before making any infrastructure change (Terraform, Helm chart, provider choice, managed-service selection), read **[infrastructure/IAC_RULES.md](./infrastructure/IAC_RULES.md)**. It documents the forward-compatibility rules every infra change must satisfy — what stays portable across providers, what coupling is allowed, and the bar an exception has to clear.

<p align="right">(<a href="#top">back to top</a>)</p>

## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
  - [ ] Nested Feature

See the [open issues](oi) for a full list of proposed features (and known
issues).

<p align="right">(<a href="#top">back to top</a>)</p>

## Contributing

[![Contributors][c-shield]][contributors]

[c-shield]: https://img.shields.io/github/contributors/mylesberueda/committee-ai.svg?style=for-the-badge
[contributors]: https://github.com/mylesberueda/committee-ai/graphs/contributors

Contributions are what make the open source community such an amazing place to
learn, inspire, and create. Any contributions you make are
**greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and
create a pull request. You can also simply open an issue with the tag
"enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Merging features from projects that use this template with JJ

While possible, it's probable that you'll run into merge conflicts that will
need to be resolved. In the examples below, we'll take this repo and a
fictional committee-ai repo, which was created via template from startup-ai.

```sh
# If you didn't add upstream already:
jj git remote add upstream git@github.com:mylesberueda/startup-ai.git
jj git fetch --remote upstream
```

You can do all your feature work on a branch connected to your repo (i.e.
committee-ai, the repo that was spawned from startup-ai). **Once you've merged
into your repo's main (in this example, committee-ai's main), you can use that
same branch and rebase over upstream.**

```sh
# Rebase over startup-ai's main to isolate the feature
jj rebase -s feat/committee-ai-feature -d main@upstream

# Create a bookmark for the PR
jj bookmark create chore/upstream-to-startup-ai -r feat/committee-ai-feature

# Push to upstream
jj git push --remote upstream --bookmark chore/upstream-to-startup-ai
```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->

<!-- ## License -->
<!---->
<!-- [![LICENSE](https://img.shields.io/github/license/polypixel-labs/pixel-dashboard.svg?style=for-the-badge)](https://github.com/MylesWritesCode/rust-wasm/blob/master/LICENSE) -->
<!---->
<!-- Distributed under the {placeholder}. See `LICENSE` for more information. -->

<!-- <p align="right">(<a href="#top">back to top</a>)</p> -->

<!-- CONTACT -->

## Contact

### Myles Berueda

[![LinkedIn][l-shield]][linkedin]
[![Github][g-shield]][github]
[![Mastodon][m-shield]][mastodon]

[linkedin]: <https://linkedin.com/in/myles-berueda>
[l-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[github]: <https://github.com/mylesberueda>
[g-shield]: https://img.shields.io/github/followers/mylesberueda?style=for-the-badge&label=GITHUB
[mastodon]: <https://mstdn.social/@mylesberueda>
[m-shield]: https://img.shields.io/mastodon/follow/113004977572109573?domain=https%3A%2F%2Fmstdn.social&style=for-the-badge&label=MSTDN.SOCIAL

### History

This template repo started from another starter repo, [pixel-dash][pixel-dash].
There are several improvements here, namely IaC, making the service AI-native,
and converting it to a true microservice template rather than a gateway-based
microservice template.

[pixel-dash]: https://github.com/polypixel-labs/pixel-dash

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->

[product-screenshot]: .meta/screenshot.png
