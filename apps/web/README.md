# Web Application

The primary user-facing frontend for Memoir, built with Next.js 14, TypeScript, and Tailwind CSS.

## Overview

The web application serves as the main interface for founders to interact with their AI-powered startup teams. It provides authentication, dashboard views, agent management, and real-time chat capabilities.

### Key Features

- **Server-Side Authentication**: Secure JWT authentication with HTTP-only cookies, leveraging Next.js Server Components for enhanced security.
- **Auth Guards**: Layout-level protection for authenticated routes.
- **Dashboard**: A central hub for viewing agent performance, recent conversations, and API usage.
- **Agent Chat**: A real-time chat interface for interacting with AI agents.
- **Responsive Design**: A mobile-first approach with Tailwind CSS and DaisyUI.
- **Component Library**: Integration with the shared `@polypixel/ui` component library for a consistent look and feel.

## Architecture

### Technology Stack

- **Framework**: Next.js 14 with App Router
- **Language**: TypeScript with strict mode
- **Styling**: Tailwind CSS with DaisyUI
- **State Management**: React Context and hooks
- **Forms**: React Hook Form with validation
- **API Client**: Generated TypeScript clients from OpenAPI specs
- **Testing**: Vitest, React Testing Library, and Storybook

### Directory Structure

```
src/
├── app/                          # Next.js App Router pages
│   ├── (app)/                    # Authenticated routes (protected by layout auth guard)
│   │   ├── dashboard/            # Main dashboard
│   │   ├── assistant/            # AI assistant chat
│   │   └── layout.tsx            # Auth guard layout (server-side protection)
│   ├── (auth)/                   # Authentication routes (public)
│   │   └── auth/                 # Login and registration
│   └── components/               # App-specific components
├── components/                   # Reusable components
├── lib/                          # Utility libraries
│   └── auth.ts                   # Server-side authentication utilities
└── providers/                    # React context providers
```

## Authentication System

### Server-Side Authentication

The web application implements **server-side authentication** using Next.js Server Components for enhanced security:

#### Architecture Overview

```
Login → JWT Token → HTTP-only Cookie → Server Validation → Protected Routes
```

#### Key Components

**Auth Utilities** (`src/lib/auth.ts`):
- **`getAuthenticatedUser()`**: Validates JWT tokens server-side using the `cookies` API.
- **`requireAuth()`**: An auth guard that redirects unauthenticated users to the landing page.
- **`setAuthToken()`**: Securely stores JWT tokens in HTTP-only cookies.
- **`clearAuthToken()`**: Removes authentication tokens for logout.

**Layout-Level Protection** (`src/app/(app)/layout.tsx`):
- **Server Component**: An async layout that validates authentication before rendering.
- **Route Protection**: All `(app)` routes require authentication.
- **Automatic Redirect**: Unauthenticated users are redirected to the landing page (`/`).

## Development

### Getting Started

```bash
# Start the development server
pnpm nx run web:dev

# Or start all services
docker compose --profile dev up -d
pnpm nx run-many --target dev
```

The application will be available at `http://localhost:3000`.

### API Integration

The web application uses generated TypeScript clients for API communication:

```bash
# Generate API clients from OpenAPI specs
pnpm nx run @polypixel/clients:generate
```

## Testing

### Running Tests

```bash
# All tests
pnpm nx run web:test

# Unit tests
pnpm nx run web:test:unit

# Integration tests
pnpm nx run web:test:integration

# Watch mode for development
pnpm nx run web:test --watch
```

## Deployment

The application is built and deployed using Docker. The `Dockerfile` in the root of the `apps/web` directory contains the build instructions.

```bash
# Build the Docker image
pnpm nx run web:docker:build
```
