# syntax=docker/dockerfile:1
#
# Dockerfile for api-service
# Multi-stage build with test and runtime targets for CI integration
#
# Usage:
#   Test:  docker build --target test -f apps/api-service/Dockerfile .
#   Build: docker build --target runtime -t api-service:local -f apps/api-service/Dockerfile .
#   Run:   docker run --rm -p 5154:5154 -e PORT=5154 -e DATABASE_URL=... api-service:local

# ------------------------------------------------------------------------------
# Stage: deps - Install dependencies and copy source
# ------------------------------------------------------------------------------
FROM rust:1.93-bookworm AS deps

WORKDIR /app
COPY . .

# Fetch dependencies (warms the cache for subsequent stages)
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  cargo fetch --manifest-path Cargo.toml

# ------------------------------------------------------------------------------
# Stage: test - Run tests (CI uses --target test)
# ------------------------------------------------------------------------------
FROM deps AS test

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  --mount=type=cache,id=rust-workspace-target,target=/app/target,sharing=locked \
  cargo test -p api-service

# ------------------------------------------------------------------------------
# Stage: build - Compile release binary
# ------------------------------------------------------------------------------
FROM deps AS build

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  --mount=type=cache,id=rust-workspace-target,target=/app/target,sharing=locked \
  cargo build --release -p api-service && \
  cp target/release/api-service /api-service

# ------------------------------------------------------------------------------
# Stage: runtime - Minimal production image (CI pushes this)
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 10001 app \
  && useradd --uid 10001 --gid app --shell /bin/false app

WORKDIR /app
COPY --from=build /api-service /app/api-service

RUN chown -R app:app /app
USER app

ENV PORT=5154
EXPOSE 5154

ENTRYPOINT ["/app/api-service"]
CMD ["server", "start"]
