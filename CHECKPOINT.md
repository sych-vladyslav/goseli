# Goseli - Checkpoint 2026-02-17

## Phase 0: COMPLETE

All setup tasks finished:
- Tools: git, gh 2.23, npm 9.2, tmux 3.3a, sqlx-cli 0.8.6, Rust 1.90, Node 18, Yarn 1.22
- Agent Teams enabled (tmux split-pane mode)
- 35 skills created (dev, design, marketing, QA, PM, security)
- Git repo on GitHub: `github.com/sych-vladyslav/goseli` (private)
- PostgreSQL database: `goseli_dev` (local)
- Redis: running on localhost:6379
- docker-compose.yml ready (PG, Redis, Meilisearch, MinIO)

## Phase 1: IN PROGRESS (Backend + CI + Design done, Frontend pending)

### Completed Today (2026-02-17)

**PR #5 — Auth System (MERGED)**
- Axum server scaffold: async main, PgPool + Redis ConnectionManager, health check
- CORS + tracing middleware, graceful shutdown
- JWT auth: access tokens (15min) + refresh token rotation (7d)
- Argon2 password hashing, SHA-256 token storage
- 5 auth endpoints: POST register, login, refresh, logout; GET /me
- Auth middleware (AuthUser extractor)
- DB queries: users, tokens (runtime SQL)
- Security hardened: race condition fix, token invalidation on login, validation, active user checks

**PR #6 — Product & Category CRUD (MERGED)**
- 10 REST endpoints for products and categories
- Products: list (paginated/filtered/search), get (with images+variants), create, update, soft delete
- Categories: list, get, create, update, delete
- All SQL converted to runtime queries (no .sqlx cache needed)
- Request validation via validator crate
- From<Model> impls for response DTOs

**PR #3 — CI Pipeline (MERGED earlier)**
- GitHub Actions: Backend Tests (fmt, clippy, test) + Frontend Tests (lint, type-check, build)
- Branch protection: CI checks required

**PR #1 — API Schema + DB Migrations (MERGED earlier)**
- OpenAPI spec, database migrations for all core tables

**PR #2 — Design System (MERGED earlier)**
- Tailwind design tokens, typography, base components
- Fixed Tailwind opacity modifiers with color-mix() CSS

### Current State of Main Branch
```
2261283 feat: Product & Category CRUD API endpoints (#6)
a2a4244 Merge pull request #5 - Server + Auth
a7faf1e docs: add container-per-store architecture and checkpoint
57dcbe7 feat: initial project setup
```

### What's Built
- **5 Rust crates**: api, core, db, auth, storage
- **Auth**: Full JWT flow with refresh token rotation
- **Products API**: CRUD with pagination, filtering, search, images, variants
- **Categories API**: Full CRUD with hierarchical support (parent_id)
- **CI**: Green pipeline (Backend Tests + Frontend Tests)
- **Frontend**: Next.js 15 scaffold with Tailwind, design tokens, base components

### Technical Decisions Made During Phase 1
- Runtime SQL (`query_as::<_, T>()`) instead of compile-time macros — avoids .sqlx cache for CI
- UUID v7 everywhere (`Uuid::now_v7()`) — no v4
- Temporary `get_default_store_id()` helper in each handler module (until domain-based routing in P2)
- CORS permissive for dev (TODO for production)
- Soft delete for products (status → archived)

### Remaining Phase 1 Tasks
- **Task #7**: Build Next.js storefront pages (product listing + detail)
- Integration tests for API endpoints
- Wire up frontend API client to backend

## How to Resume
```bash
cd /home/r32/goseli
# Verify services:
pg_isready && redis-cli ping
# Check git state:
git log --oneline -5 && git status
# Build backend:
cd backend && cargo build --workspace && cargo test --workspace
# Build frontend:
cd frontend && yarn build
# Next: Task #7 — Build storefront pages
```
