# Goseli — Project Conventions

## Overview
Goseli is a reusable, multi-vertical e-commerce platform built with Rust (Axum) + Next.js.
See PLAN.md for full architecture. See CHECKPOINT.md for current progress.

## Tech Stack
- **Backend**: Rust 1.90 + Axum (async web framework) + SQLx (compile-time checked SQL)
- **Frontend**: Next.js 15 (App Router) + TypeScript + Tailwind CSS
- **Database**: PostgreSQL 15 with JSONB for flexible product schemas
- **Cache**: Redis 7 for sessions, cart state, and caching
- **Package Manager**: Yarn (frontend), Cargo (backend)

## Project Structure
```
backend/crates/api/     → HTTP routes, handlers, middleware
backend/crates/core/    → Domain types, business logic
backend/crates/db/      → SQL queries, migrations
backend/crates/auth/    → JWT, password hashing
backend/crates/search/  → Meilisearch integration
backend/crates/storage/ → S3/file uploads
frontend/src/app/       → Next.js pages (storefront + admin)
frontend/src/components/ → React components
frontend/src/lib/       → API client, hooks, utilities
shared/openapi.yaml     → API contract (source of truth)
docs/                   → All documentation
```

## Coding Standards

### Rust Backend
- Use `?` operator for error propagation — NEVER use `.unwrap()` in production code
- All SQL via sqlx macros (`query!`, `query_as!`) — no string interpolation
- Handlers return `Result<Json<T>, ApiError>`
- Use Axum extractors: `State`, `Path`, `Query`, `Json`
- Request/response types in `core` crate, derive `Serialize`/`Deserialize`
- All endpoints need tests in `backend/tests/`
- Run `cargo clippy -- -D warnings` before committing
- Pagination: `?page=1&per_page=20` → `PaginatedResponse<T>`
- Prices stored as integers (cents), not floats

### Next.js Frontend
- TypeScript strict mode — no `any` types
- Components use Tailwind CSS with design tokens (not hardcoded colors)
- Responsive mobile-first (use `sm:`, `md:`, `lg:` breakpoints)
- All interactive elements need `aria-*` attributes for accessibility
- Images use `next/image` with proper `sizes` and `alt` text
- API calls go through the shared client in `lib/api-client.ts`
- No `console.log` in committed code
- Server Components by default, Client Components only when needed (`'use client'`)

### Database
- Migrations in `backend/migrations/` via sqlx-cli
- Table names: plural snake_case (`products`, `order_items`)
- Primary keys: UUID v7 (time-sortable)
- Timestamps: `created_at` and `updated_at` on every table
- JSONB columns get GIN indexes
- Always include `store_id` for multi-tenant queries

### Git
- Branch naming: `feature/P{phase}.{task}-short-description`
- Commit messages: conventional format (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`)
- PRs reviewed via `/review-pr` skill before merge
- Never commit secrets, .env files, or credentials

## Skills Available
Run `/help` or ask "what skills are available" to see all 35+ project skills.
Key skills: `/build`, `/test`, `/review-pr`, `/checkpoint`, `/new-endpoint`, `/new-component`

## File Ownership (Prevent Merge Conflicts)
Each team member owns specific directories. Check PLAN.md for the full ownership map.
If you need to modify files outside your ownership, coordinate with the file owner first.

## Environment
- Database URL: `postgres://localhost/goseli_dev`
- Redis: `localhost:6379`
- Backend server: `localhost:3001`
- Frontend dev server: `localhost:3000`

## Important Commands
```bash
# Backend
cd backend && cargo build --workspace     # Build
cd backend && cargo test --workspace      # Test
cd backend && cargo clippy -- -D warnings # Lint
cd backend && cargo sqlx migrate run --database-url postgres://localhost/goseli_dev  # Migrate

# Frontend
cd frontend && yarn dev    # Dev server
cd frontend && yarn build  # Production build
cd frontend && yarn test   # Tests
cd frontend && yarn lint   # Lint
```
