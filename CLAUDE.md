# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Teftar is a SaaS application similar to FreshBooks, built as a monorepo with:
- **Frontend**: React Router v7 with SSR, TypeScript, and Tailwind CSS (in `app/`)
- **Backend**: Rust with Axum web framework, Supabase (PostgreSQL) via SQLx (in `api/`)
- **Database**: Supabase with migrations in `supabase/` at project root
- **Auth**: Supabase Auth with JWT verification
- **Storage**: Supabase Storage for file uploads
- **Deployment**: Fly.io with separate apps for frontend and backend

## Product Philosophy

**Ship Complete Features Only**
- Never add placeholder links, buttons, or UI elements for unimplemented features
- Remove any non-functional navigation items, footer links, or CTAs
- Only show features that are fully implemented and working
- Keep the UI clean - if it's visible, it must work
- Examples:
  - ❌ Don't add "About", "Blog", "Contact" links until those pages exist
  - ❌ Don't add "Sign In" button until auth is implemented
  - ❌ Don't add "Pricing" page until pricing model is defined
  - ✅ Do show only functional features and working links
  - ✅ Do progressively enhance the UI as features are completed

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

# Install Shadcn UI Components (use ~ path alias, not @)
pnpm dlx shadcn@latest add <component>
```

### Backend (api/ Directory)

```bash
cd api

# Load environment variables from .env
# (DATABASE_URL for local Supabase connection)

# Check compilation
cargo check

# Run development server (http://localhost:8080)
cargo run

# Build release binary
cargo build --release

# Run tests (when added)
cargo test

# Database migrations (use Supabase CLI, NOT sqlx-cli)
# Install: https://supabase.com/docs/guides/cli/getting-started
# Run these commands from PROJECT ROOT (not api/ directory)

# Initialize Supabase (first time only)
supabase init  # Creates supabase/ folder at project root

# Link to your Supabase project (first time only)
supabase login
supabase link --project-ref <your-project-ref>

# Create new migration
supabase migration new <migration_name>
# Edit the SQL file in supabase/migrations/

# Push to production (or use GitHub Actions)
supabase db push

# Pull remote schema changes
supabase db pull
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
- **Path Alias**: Use `~/*` for imports (e.g., `~/components/ui/button`), NOT `@/*`
- **Build Version**: Git short SHA injected via Vite and displayed in bottom-right corner

### Backend Structure

- **Axum Web Framework**: Async HTTP server with routing
- **Current Endpoints**:
  - `GET /` - API info
  - `GET /health` - Health check
- **Database**: Supabase PostgreSQL via SQLx
  - Connection string in `DATABASE_URL` environment variable
  - Local dev: `api/.env` file
  - Production: Set via GitHub Actions as Fly.io secret
- **Authentication**: Supabase Auth with JWT verification via `jsonwebtoken` crate
- **File Storage**: Supabase Storage accessed via `reqwest` HTTP client
- **Port Configuration**: Reads `PORT` env var (defaults to 8080), binds to `0.0.0.0`
- **CORS**: Permissive CORS enabled via `tower-http`
- **Logging**: Structured logging with `tracing` and `tracing-subscriber`
- **Environment**: `dotenvy` for loading `.env` files in development
- **Key Dependencies**: `axum`, `sqlx`, `tokio`, `serde`, `validator`, `chrono`, `uuid`, `jsonwebtoken`, `reqwest`

### Deployment

Both apps are deployed separately to Fly.io:

- **Frontend App**: `teftar-frontend`
  - Dockerfile uses pnpm with multi-stage build
  - Requires `PORT=8080` and `HOST=0.0.0.0` environment variables
  - Config: `fly.toml`

- **Backend App**: `teftar-api`
  - Dockerfile uses Rust multi-stage build with Debian runtime
  - Config: `api/fly.toml`
  - Requires `DATABASE_URL` secret (set via GitHub Actions)

Deploy commands:
```bash
# Frontend (from root)
flyctl deploy --remote-only

# Backend (from api/ directory)
cd api
flyctl deploy --remote-only
```

**GitHub Actions CI/CD:**
- Automatically deploys both apps on push/merge to `main` branch
- Runs Supabase migrations before deploying backend
- Sets `DATABASE_URL` secret on Fly.io from GitHub secret before deploying backend
- Workflow file: `.github/workflows/deploy.yml`
- Required GitHub secrets:
  - `FLY_API_TOKEN` - Fly.io deployment token
  - `DATABASE_URL` - Supabase PostgreSQL connection string
  - `SUPABASE_ACCESS_TOKEN` - Supabase personal access token (for migrations)
  - `SUPABASE_PROJECT_REF` - Supabase project reference ID

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

### Supabase Integration

**Database (SQLx + Supabase CLI):**
- Connect to Supabase PostgreSQL using SQLx
- Use `sqlx::query!` macro for compile-time checked queries
- Set `DATABASE_URL` in `api/.env` for local dev (from Supabase dashboard)
- Production `DATABASE_URL` automatically set via GitHub Actions
- **Migrations**: Use Supabase CLI (NOT sqlx-cli) to create and manage migrations
  - Local development: Create migrations with `supabase migration new <name>`
  - Production: Migrations run automatically via GitHub Actions before backend deployment
  - Manual prod push: `supabase db push` (after linking to project)
  - Migration files stored in `supabase/migrations/` directory
  ```bash
  supabase migration new create_users_table
  # Edit the generated SQL file in supabase/migrations/
  # Commit and push - GitHub Actions will apply to prod
  ```

**Authentication (JWT):**
- Use `jsonwebtoken` crate to verify Supabase Auth JWTs
- Get JWT secret from Supabase dashboard (Settings → API → JWT Secret)
- Create middleware to extract and verify JWT from Authorization header
- Example:
  ```rust
  use jsonwebtoken::{decode, DecodingKey, Validation};
  
  // Verify JWT from Supabase Auth
  let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret), &validation)?;
  ```

**File Storage (Supabase Storage):**
- Use `reqwest` HTTP client to interact with Supabase Storage API
- Get Supabase URL and anon key from dashboard
- Example endpoints:
  - Upload: `POST https://<project>.supabase.co/storage/v1/object/<bucket>/<path>`
  - Download: `GET https://<project>.supabase.co/storage/v1/object/public/<bucket>/<path>`
  - List: `POST https://<project>.supabase.co/storage/v1/object/list/<bucket>`

## Project Goals

Building invoice and accounting features similar to FreshBooks:
- Client management
- Invoice creation and tracking
- Expense tracking
- Payment processing
- Reporting and analytics
