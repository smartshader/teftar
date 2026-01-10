# Teftar

A modern SaaS invoicing and accounting platform, similar to FreshBooks.

## Tech Stack

- **Frontend**: React Router v7 (SSR), TypeScript, Tailwind CSS
- **Backend**: Rust, Axum, PostgreSQL
- **Deployment**: Fly.io

## Quick Start

### Prerequisites

- Node.js 20+ and pnpm
- Rust 1.84+
- PostgreSQL (for backend development)

### Frontend Development

```bash
pnpm install
pnpm dev
```

Visit http://localhost:5173

### Backend Development

```bash
cd api
cargo run
```

API runs on http://localhost:8080

## Deployment

Both apps deploy automatically to Fly.io on push to `main` branch via GitHub Actions.

Manual deployment:
```bash
# Frontend
flyctl deploy --remote-only

# Backend
flyctl deploy --remote-only --config api/fly.toml --dockerfile api/Dockerfile
```

## Project Structure

```
├── app/              # Frontend routes and components
├── api/              # Rust backend
│   └── src/
├── supabase/         # Database migrations (at root)
│   └── migrations/
├── public/           # Static assets
└── .github/
    └── workflows/    # CI/CD
```

## License

Private - All Rights Reserved
