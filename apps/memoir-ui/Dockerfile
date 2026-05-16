# syntax=docker/dockerfile:1
# =============================================================================
# Memoir UI Dockerfile
# =============================================================================
# Multi-stage build with test and runtime targets for CI integration.
#
# Usage:
#   Test:  docker build --target test -f apps/memoir-ui/Dockerfile .
#   Build: docker build --target runtime -t memoir-ui:local -f apps/memoir-ui/Dockerfile .
#   Run:   docker run --rm -p 3000:3000 memoir-ui:local
# =============================================================================

# -----------------------------------------------------------------------------
# Stage: deps - Install dependencies
# -----------------------------------------------------------------------------
FROM node:24-alpine AS deps

RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copy full workspace (Nx needs project graph, pnpm needs workspace structure)
COPY . .

# Install dependencies (cache pnpm store for faster installs)
RUN --mount=type=cache,target=/root/.local/share/pnpm/store,sharing=shared \
  pnpm install --frozen-lockfile

# -----------------------------------------------------------------------------
# Stage: test - Run tests (CI uses --target test)
# -----------------------------------------------------------------------------
FROM deps AS test

ENV NEXT_TELEMETRY_DISABLED=1
RUN pnpm nx run memoir-ui:test && \
  rm -rf /app/.nx/workspace-data/d/daemon.log

# -----------------------------------------------------------------------------
# Stage: build - Build the Next.js app
# -----------------------------------------------------------------------------
FROM deps AS build

ENV NEXT_TELEMETRY_DISABLED=1
RUN --mount=type=cache,target=/app/apps/memoir-ui/.next/cache,sharing=shared \
  pnpm nx run memoir-ui:build && \
  rm -rf /app/.nx/workspace-data/d/daemon.log

# -----------------------------------------------------------------------------
# Stage: runtime - Minimal production image (CI pushes this)
# -----------------------------------------------------------------------------
FROM node:24-alpine AS runtime

WORKDIR /app

RUN addgroup --system --gid 1001 nodejs && \
  adduser --system --uid 1001 nextjs

# Copy public assets
COPY --from=build /app/apps/memoir-ui/public ./public

# Copy standalone build output
COPY --from=build --chown=nextjs:nodejs /app/apps/memoir-ui/.next/standalone ./
COPY --from=build --chown=nextjs:nodejs /app/apps/memoir-ui/.next/static ./apps/memoir-ui/.next/static

USER nextjs

EXPOSE 3000

ENV PORT=3000
ENV HOSTNAME="0.0.0.0"
ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

CMD ["node", "apps/memoir-ui/server.js"]
