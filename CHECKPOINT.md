# Goseli - Checkpoint 2026-02-19

## Phase 0: COMPLETE

All setup tasks finished:
- Tools: git, gh 2.23, npm 9.2, tmux 3.3a, sqlx-cli 0.8.6, Rust 1.90, Node 18, Yarn 1.22
- Agent Teams enabled (tmux split-pane mode)
- 35 skills created (dev, design, marketing, QA, PM, security)
- Git repo on GitHub: `github.com/sych-vladyslav/goseli` (private)
- PostgreSQL database: `goseli_dev` (local)
- Redis: running on localhost:6379
- docker-compose.yml ready (PG, Redis, Meilisearch, MinIO)

## Phase 1: COMPLETE

### PRs Merged (in order)
1. **PR #1 — API Schema + DB Migrations** — OpenAPI spec, database migrations for all core tables
2. **PR #2 — Design System** — Tailwind design tokens, typography, base components, color-mix() CSS fix
3. **PR #3 — CI Pipeline** — GitHub Actions (Backend Tests + Frontend Tests), branch protection
4. **PR #5 — Auth System** — JWT with refresh rotation, Argon2 passwords, 5 auth endpoints, middleware
5. **PR #6 — Product & Category CRUD** — 10 REST endpoints, pagination/filtering/search, runtime SQL
6. **PR #7 — Next.js Storefront Pages** — Home, product listing, product detail, 404, 8 components

### Bug Fixes on Main
- `fc64a05` — ProductStatus sqlx type_name: `text` → `VARCHAR`
- `530e12a` — Axum 0.7 route params: `{id}` → `:id`, force light mode, UI polish

### What's Built
- **Backend (5 Rust crates)**: api, core, db, auth, storage
  - Auth: JWT access (15min) + refresh rotation (7d), Argon2
  - Products API: list (paginated/filtered/search), get (images+variants), create, update, soft delete
  - Categories API: full CRUD with hierarchical parent_id
- **Frontend (Next.js 15)**:
  - Home page: hero section, featured products grid
  - Products page: category sidebar, search, sort, pagination (SWR client-side)
  - Product detail: breadcrumbs, price/sale display, stock badge, Add to Cart placeholder
  - 404 page, responsive mobile layout, Header/Footer
  - Design system with CSS variables, light mode forced
- **CI**: Green pipeline (Backend Tests + Frontend Tests)
- **Test Data**: 4 categories, 12 products with realistic data seeded

### Technical Decisions
- Runtime SQL (`query_as::<_, T>()`) — no .sqlx compile-time cache
- UUID v7 (`Uuid::now_v7()`)
- `get_default_store_id()` helper (temporary until domain-based routing)
- Hybrid data fetching: server-side for SEO, SWR for interactive filtering
- Light mode forced (`data-theme="light"`) — dark mode CSS needs proper component support
- Axum 0.7 uses `:id` path params (not `{id}`)

## Next: Phase 2 — Shopping Cart & Checkout
- Cart system (Redis-backed)
- Order management
- Payment integration
- Admin dashboard
- Image uploads

## How to Resume
```bash
cd /home/r32/goseli
# Verify services:
pg_isready && redis-cli ping
# Check git state:
git log --oneline -5 && git status
# Start backend:
cd backend && cargo build --release
DATABASE_URL="postgres:///goseli_dev" ./target/release/goseli-api &
# Start frontend:
cd frontend && yarn dev &
# Visit http://localhost:3000
```
