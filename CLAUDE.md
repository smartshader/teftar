# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Teftar is a SaaS application similar to FreshBooks, built as a monorepo with:
- **Frontend**: React Router v7 with SSR, TypeScript, and Tailwind CSS
- **Backend**: Rust with Axum web framework, PostgreSQL via SQLx
- **Deployment**: Fly.io with separate apps for frontend and backend

## Development Commands

### Frontend (Root Directory)

```bash
# Install dependencies
pnpm install

# Development server (http://localhost:5173)
pnpm dev

# Type checking
pnpm typecheck

# Production build
pnpm build

# Start production server
pnpm start

# Install Shadcn UI Components
pnpm dlx shadcn@latest add button
```

### Backend (api/ Directory)

```bash
cd api

# Check compilation
cargo check

# Run development server (http://localhost:8080)
cargo run

# Build release binary
cargo build --release

# Run tests (when added)
cargo test
```

## Architecture

### Frontend Structure

- **React Router v7**: File-based routing with SSR enabled
- **Routes**: Defined in `app/routes.ts` using `@react-router/dev/routes` API
- **Root Layout**: `app/root.tsx` provides the HTML shell, error boundary, and global Layout component
- **Route Files**: Located in `app/routes/` (e.g., `home.tsx`)
- **Type Generation**: React Router generates types in `app/+types/` for type-safe route components
- **Styling**: Tailwind CSS v4 via Vite plugin (`@tailwindcss/vite`)
- **Build**: Vite with React Router plugin and tsconfigPaths for path aliases

### Backend Structure

- **Axum Web Framework**: Async HTTP server with routing
- **Current Endpoints**:
  - `GET /` - API info
  - `GET /health` - Health check
- **Database**: PostgreSQL with SQLx (prepared for database operations)
- **Port Configuration**: Reads `PORT` env var (defaults to 8080), binds to `0.0.0.0`
- **CORS**: Permissive CORS enabled via `tower-http`
- **Logging**: Structured logging with `tracing` and `tracing-subscriber`
- **Key Dependencies**: `axum`, `sqlx`, `tokio`, `serde`, `validator`, `chrono`, `uuid`

### Deployment

Both apps are deployed separately to Fly.io:

- **Frontend App**: `teftar-frontend`
  - Dockerfile uses pnpm with multi-stage build
  - Requires `PORT=8080` and `HOST=0.0.0.0` environment variables
  - Config: `fly.toml`

- **Backend App**: `teftar-api`
  - Dockerfile uses Rust multi-stage build with Debian runtime
  - Config: `api/fly.toml`

Deploy commands:
```bash
# Frontend (from root)
flyctl deploy --remote-only

# Backend
flyctl deploy --remote-only --config api/fly.toml --dockerfile api/Dockerfile
```

GitHub Actions automatically deploys both apps on push/merge to `main` branch (`.github/workflows/deploy.yml`).

## Key Patterns

### Adding Frontend Routes

1. Create route file in `app/routes/` (e.g., `invoices.tsx`)
2. Add route to `app/routes.ts`:
   ```ts
   import { route } from "@react-router/dev/routes";
   
   export default [
     index("routes/home.tsx"),
     route("invoices", "routes/invoices.tsx"),
   ] satisfies RouteConfig;
   ```
3. Use generated types from `app/+types/[route-name]` for type-safe loaders/actions

### Adding Backend Endpoints

1. Create handler function in `api/src/main.rs` (or new module)
2. Add route to the Router:
   ```rust
   let app = Router::new()
       .route("/your-path", get(your_handler))
       .layer(CorsLayer::permissive());
   ```
3. Return `Json<Value>` or use extractors for request data

### Database Integration (Future)

The backend is set up with SQLx for PostgreSQL:
- Use `sqlx::query!` macro for compile-time checked queries
- Set `DATABASE_URL` environment variable
- Run migrations with `sqlx migrate run`

## Project Goals

Building invoice and accounting features similar to FreshBooks:
- Client management
- Invoice creation and tracking
- Expense tracking
- Payment processing
- Reporting and analytics
