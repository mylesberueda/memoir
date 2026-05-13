# syntax=docker/dockerfile:1
# =============================================================================
# Web App Dockerfile
# =============================================================================
# Multi-stage build with test and runtime targets for CI integration.
#
# Usage:
#   Test:  docker build --target test -f apps/web/Dockerfile .
#   Build: docker build --target runtime -t web:local -f apps/web/Dockerfile .
#   Run:   docker run --rm -p 3000:3000 web:local
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
RUN pnpm nx run web:test && \
  rm -rf /app/.nx/workspace-data/d/daemon.log

# -----------------------------------------------------------------------------
# Stage: build - Build the Next.js app
# -----------------------------------------------------------------------------
FROM deps AS build

ENV NEXT_TELEMETRY_DISABLED=1
RUN --mount=type=cache,target=/app/apps/web/.next/cache,sharing=shared \
  pnpm nx run web:build && \
  rm -rf /app/.nx/workspace-data/d/daemon.log

# -----------------------------------------------------------------------------
# Stage: runtime - Minimal production image (CI pushes this)
# -----------------------------------------------------------------------------
FROM node:24-alpine AS runtime

WORKDIR /app

RUN addgroup --system --gid 1001 nodejs && \
  adduser --system --uid 1001 nextjs

# Copy public assets
COPY --from=build /app/apps/web/public ./public

# Copy standalone build output
COPY --from=build --chown=nextjs:nodejs /app/apps/web/.next/standalone ./
COPY --from=build --chown=nextjs:nodejs /app/apps/web/.next/static ./apps/web/.next/static

USER nextjs

EXPOSE 3000

ENV PORT=3000
ENV HOSTNAME="0.0.0.0"
ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

CMD ["node", "apps/web/server.js"]
