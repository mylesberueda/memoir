# syntax=docker/dockerfile:1
#
# Dockerfile for rig-service
# Multi-stage build with test and runtime targets for CI integration
#
# Usage:
#   Test:  docker build --target test -f apps/rig-service/Dockerfile .
#   Build: docker build --target runtime -t rig-service:local -f apps/rig-service/Dockerfile .
#   Run:   docker run --rm -p 5153:5153 -e PORT=5153 -e DATABASE_URL=... rig-service:local

# ------------------------------------------------------------------------------
# Stage: pdfium - Download pre-built PDFium shared library
# Required by kreuzberg for PDF text extraction at runtime.
# Source: https://github.com/bblanchon/pdfium-binaries
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS pdfium

RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz \
  | tar xz -C /opt \
  && ls /opt/lib/libpdfium.so

# ------------------------------------------------------------------------------
# Stage: deps - Install dependencies and copy source
# ------------------------------------------------------------------------------
FROM rust:1.93-trixie AS deps

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
  cargo test -p rig-service

# ------------------------------------------------------------------------------
# Stage: build - Compile release binary
# ------------------------------------------------------------------------------
FROM deps AS build

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=shared \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=shared \
  --mount=type=cache,id=rust-workspace-target,target=/app/target,sharing=locked \
  cargo build --release -p rig-service && \
  cp target/release/rig-service /rig-service

# ------------------------------------------------------------------------------
# Stage: runtime - Minimal production image (CI pushes this)
# ------------------------------------------------------------------------------
FROM debian:trixie-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# PDFium shared library - kreuzberg loads this at runtime via libloading
COPY --from=pdfium /opt/lib/libpdfium.so /usr/lib/

RUN groupadd --gid 10001 app \
  && useradd --uid 10001 --gid app --shell /bin/false app

WORKDIR /app
COPY --from=build /rig-service /app/rig-service

RUN chown -R app:app /app
USER app

ENV PORT=5153
EXPOSE 5153

ENTRYPOINT ["/app/rig-service"]
CMD ["server", "start"]
