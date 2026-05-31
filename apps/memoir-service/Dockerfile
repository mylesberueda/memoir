# syntax=docker/dockerfile:1
#
# Dockerfile for memoir-service
# Multi-stage build with test and runtime targets for CI integration.
#
# Usage:
#   Test:  docker build --target test    -f apps/memoir-service/Dockerfile .
#   Build: docker build --target runtime -t memoir-service:local -f apps/memoir-service/Dockerfile .
#   Run:   docker run --rm -p 5153:5153 \
#            -e DATABASE_URL=postgres://... \
#            -e QDRANT_URL=http://... \
#            -e MEMOIR_JWT_SECRET=... \
#            memoir-service:local
#
# Notes:
# - Migrations run automatically at server startup (see apps/memoir-service/
#   src/context.rs `AppContext::new`); no separate migration step is needed.
# - The embedding model (BGE-small-en-v1.5, ~50 MB) is downloaded on first
#   use by `fastembed` and cached under $HOME. The runtime user `app` owns
#   /app and has a writable HOME so the cache lands somewhere it can write.

# ------------------------------------------------------------------------------
# Stage: deps - Fetch dependencies and warm the cargo cache
# ------------------------------------------------------------------------------
FROM rust:1.93-trixie AS deps

WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  cargo fetch --manifest-path Cargo.toml

# ------------------------------------------------------------------------------
# Stage: test - Run unit tests (CI uses --target test)
# Integration tests need live Postgres + Qdrant and run outside the image.
# ------------------------------------------------------------------------------
FROM deps AS test

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  --mount=type=cache,id=rust-workspace-target,target=/app/target,sharing=locked \
  cargo test --features unit -p memoir-service

# ------------------------------------------------------------------------------
# Stage: build - Compile release binary
# ------------------------------------------------------------------------------
FROM deps AS build

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  --mount=type=cache,id=rust-workspace-target,target=/app/target,sharing=locked \
  cargo build --release -p memoir-service && \
  cp target/release/memoir /memoir

# ------------------------------------------------------------------------------
# Stage: runtime - Minimal production image (CI pushes this)
# ------------------------------------------------------------------------------
FROM debian:trixie-slim AS runtime

# Static OCI labels. Per-build labels (version, revision) are injected by the
# CI workflow via --opt=label=... so the same Dockerfile produces correctly
# labeled images on every build without source edits.
LABEL org.opencontainers.image.source="https://github.com/mylesberueda/memoir"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
LABEL org.opencontainers.image.title="memoir-service"
LABEL org.opencontainers.image.description="Memoir gRPC service — memory substrate for AI agents"

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 10001 app \
  && useradd --uid 10001 --gid app --create-home --shell /bin/false app

WORKDIR /app
COPY --from=build /memoir /app/memoir

RUN chown -R app:app /app
USER app

ENV PORT=5153
EXPOSE 5153

ENTRYPOINT ["/app/memoir"]
CMD ["server", "start"]
