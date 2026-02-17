# Goseli - Reusable E-Commerce Platform

## Vision
A modular, multi-vertical e-commerce engine. Deploy once, configure for any product type:
boardgames, irrigation systems, electronics, etc. The product schema, storefront theme,
and checkout flow are all configurable per-store — the engine stays the same.

---

## Tech Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Backend API** | Rust + Axum | Performance, safety, async-first |
| **Frontend** | Next.js 15 (App Router) | SSR/ISR, React Server Components, great DX |
| **Primary DB** | PostgreSQL 15 (local) + JSONB | Relational integrity + flexible product schemas |
| **Cache/Sessions** | Redis 7 (local) | Fast cache, cart state, session store |
| **Search** | Meilisearch | Rust-native, typo-tolerant, instant search |
| **Storage** | S3-compatible (MinIO local) | Product images, assets |
| **Auth** | JWT (access + refresh tokens) | Stateless, scalable |
| **API Style** | REST (OpenAPI 3.1) | Simple, well-tooled, auto-generated client |
| **Monorepo** | Cargo workspace + Yarn workspaces | Single repo, shared tooling |
| **Package Mgr** | Yarn 1.22 | Already installed on system |

### Why PostgreSQL + JSONB (not MongoDB)
- JSONB gives schema-flexible product attributes with full indexing
- We keep relational integrity for orders, users, payments
- One database to operate instead of two
- PostgreSQL's JSONB performance matches MongoDB for our query patterns

### System State (Auto-detected)
- **Installed**: git, PostgreSQL 15, Redis 7, Rust 1.90, Node 18, Yarn 1.22, SSH keys
- **Need to install**: GitHub CLI (gh), sqlx-cli, npm/npx, Meilisearch
- **Docker**: Not installed yet — will add docker-compose.yml for portable setup + future deployments
- **Resources**: 22GB RAM, 57GB free disk — plenty of headroom

---

## Project Structure (Monorepo)

```
goseli/
├── PLAN.md
├── CLAUDE.md                    # Project conventions for all agents
├── CHECKPOINT.md                # Current progress + resume instructions
│
├── .claude/
│   ├── settings.json            # Agent teams enabled + permissions
│   └── skills/                  # Custom skills (see Skills section)
│       ├── build/SKILL.md
│       ├── test/SKILL.md
│       ├── review-pr/SKILL.md
│       ├── checkpoint/SKILL.md
│       ├── db-migrate/SKILL.md
│       ├── new-endpoint/SKILL.md
│       ├── new-component/SKILL.md
│       ├── design-spec/SKILL.md
│       ├── seo-audit/SKILL.md
│       ├── api-doc/SKILL.md
│       ├── health-check/SKILL.md
│       ├── user-story/SKILL.md
│       ├── competitive-analysis/SKILL.md
│       ├── deploy-check/SKILL.md
│       ├── sprint-review/SKILL.md
│       ├── assign-task/SKILL.md
│       ├── test-plan/SKILL.md
│       ├── bug-report/SKILL.md
│       └── token-check/SKILL.md
│
├── .github/
│   └── workflows/
│       ├── ci.yml               # Test + lint on every PR
│       └── deploy.yml           # Deploy pipeline
│
├── backend/                     # Rust workspace
│   ├── Cargo.toml               # Workspace root
│   ├── crates/
│   │   ├── api/                 # Axum HTTP server, routes, middleware
│   │   ├── core/                # Domain logic (products, orders, cart, users)
│   │   ├── db/                  # PostgreSQL queries (sqlx), migrations
│   │   ├── auth/                # JWT, password hashing, middleware
│   │   ├── search/              # Meilisearch integration
│   │   └── storage/             # S3/MinIO file uploads
│   ├── migrations/              # SQL migrations (sqlx-cli)
│   └── tests/                   # Integration tests
│
├── frontend/                    # Next.js app
│   ├── package.json
│   ├── src/
│   │   ├── app/                 # App Router pages
│   │   │   ├── (storefront)/    # Customer-facing pages
│   │   │   └── (admin)/         # Admin dashboard
│   │   ├── components/          # Shared UI components
│   │   ├── lib/                 # API client, utils, hooks
│   │   └── styles/              # Theme system (CSS variables + Tailwind)
│   └── tests/                   # Jest + Playwright E2E
│
├── shared/                      # Shared types/schemas
│   └── openapi.yaml             # API contract (source of truth)
│
└── docs/                        # Project documentation
    ├── architecture.md
    ├── api-reference.md
    ├── store-configuration.md
    ├── design-system.md
    ├── seo-guide.md
    ├── deployment.md
    ├── user-stories/            # One .md per feature/epic
    │   ├── US-001-user-auth.md
    │   ├── US-002-product-catalog.md
    │   ├── US-003-cart-checkout.md
    │   └── ...
    ├── test-plans/              # QA test plans per feature
    │   ├── TP-001-auth-tests.md
    │   └── ...
    └── test-reports/            # Manual tester findings
        └── ...
```

---

## Custom Skills System

Skills are the backbone of our workflow efficiency. Each skill automates a repeatable task
so any teammate can invoke it consistently.

### Project Skills (`.claude/skills/`)

#### 1. `/build` — Build & Compile Everything
```yaml
name: build
description: Build the entire project - backend (cargo build) and frontend (yarn build). Use when you need to verify the project compiles.
disable-model-invocation: true
allowed-tools: Bash(cargo *), Bash(yarn *), Bash(cd *)
```
Instructions: Run `cargo build --workspace` in backend/, `yarn build` in frontend/. Report errors clearly.

#### 2. `/test` — Run All Tests
```yaml
name: test
description: Run the full test suite - backend (cargo test) and frontend (yarn test). Use after implementing features.
disable-model-invocation: true
allowed-tools: Bash(cargo *), Bash(yarn *), Bash(cd *)
```
Instructions: Run `cargo test --workspace` + `yarn test`. Parse output, summarize pass/fail counts, highlight failures.

#### 3. `/review-pr` — Code Review Checklist
```yaml
name: review-pr
description: Review code changes against project standards. Use for PR reviews and pre-commit checks.
context: fork
agent: Explore
```
Instructions: Check git diff, verify: no unwrap() in production Rust code, proper error handling, types match OpenAPI spec, tests exist for new endpoints, no hardcoded secrets, components follow design system.

#### 4. `/checkpoint` — Save Progress
```yaml
name: checkpoint
description: Save current progress to CHECKPOINT.md for session resumption. Use at breakpoints and before stopping.
disable-model-invocation: true
```
Instructions: Read task list, git log, check running services. Write CHECKPOINT.md with: completed tasks, in-progress tasks, next steps, git branch state, how to resume. Also update memory files.

#### 5. `/db-migrate` — Database Migration
```yaml
name: db-migrate
description: Create and run database migrations using sqlx-cli. Use when schema changes are needed.
disable-model-invocation: true
allowed-tools: Bash(cargo sqlx *), Bash(sqlx *), Read, Write
```
Instructions: Create migration file in backend/migrations/, run `sqlx migrate run`, verify with `sqlx migrate info`.

#### 6. `/new-endpoint` — Scaffold a New API Endpoint
```yaml
name: new-endpoint
description: Scaffold a new REST API endpoint with handler, route, types, and test stub. Follows project conventions.
disable-model-invocation: true
```
Instructions: Given endpoint name + method, create: route registration in api crate, handler function, request/response types in core crate, SQL query in db crate, test stub. Follow existing patterns.

#### 7. `/new-component` — Scaffold a New React Component
```yaml
name: new-component
description: Scaffold a new React component following the design system. Creates component file, types, and optional test.
disable-model-invocation: true
```
Instructions: Create component in appropriate directory with: TypeScript types, Tailwind classes using design tokens, responsive by default, accessibility attributes (aria-*), test stub.

#### 8. `/design-spec` — Generate Design Specification
```yaml
name: design-spec
description: Generate a UI/UX design specification for a feature. Includes layout, responsive breakpoints, color tokens, spacing, and accessibility notes.
user-invocable: true
```
Instructions: Output a design spec with: wireframe (ASCII), color tokens from theme, spacing scale, typography, responsive behavior at mobile/tablet/desktop, accessibility requirements (WCAG AA), component hierarchy.

#### 9. `/seo-audit` — SEO Analysis
```yaml
name: seo-audit
description: Audit pages for SEO compliance - meta tags, structured data, accessibility, performance hints.
context: fork
agent: Explore
```
Instructions: Check all page files for: title/description meta, Open Graph tags, JSON-LD structured data, heading hierarchy, image alt text, canonical URLs, robots directives.

#### 10. `/api-doc` — Generate API Documentation
```yaml
name: api-doc
description: Generate or update OpenAPI spec and API reference docs from the current codebase.
context: fork
agent: Explore
```
Instructions: Scan all route handlers, extract endpoints/methods/types, update shared/openapi.yaml, generate docs/api-reference.md.

#### 11. `/health-check` — System Health Check
```yaml
name: health-check
description: Check all services are running - PostgreSQL, Redis, backend server, frontend dev server.
disable-model-invocation: true
allowed-tools: Bash(pg_isready *), Bash(redis-cli *), Bash(curl *), Bash(lsof *)
```
Instructions: Check PG (pg_isready), Redis (redis-cli ping), backend (curl localhost:3001/health), frontend (curl localhost:3000). Report status of each.

#### 12. `/user-story` — Write User Story
```yaml
name: user-story
description: Write a structured user story with acceptance criteria, edge cases, and technical notes.
user-invocable: true
```
Instructions: Format as: "As a [role], I want [feature], so that [benefit]". Include: acceptance criteria (Given/When/Then), edge cases, technical implementation notes, estimated complexity (S/M/L).

#### 13. `/competitive-analysis` — Competitive Analysis
```yaml
name: competitive-analysis
description: Research and compare our approach against competitors (Shopify, WooCommerce, etc.) for a given feature.
context: fork
agent: general-purpose
```
Instructions: For the given feature, research how Shopify/WooCommerce/Medusa handle it. Compare approaches. Recommend our implementation strategy with pros/cons.

#### 14. `/deploy-check` — Pre-Deployment Checklist
```yaml
name: deploy-check
description: Run pre-deployment verification - tests pass, no warnings, env vars set, migrations up to date.
disable-model-invocation: true
```
Instructions: Verify: all tests pass, cargo clippy clean, no TODO/FIXME in committed code, migrations match schema, env vars documented, CHECKPOINT updated.

#### 15. `/test-plan` — Create Test Plan
```yaml
name: test-plan
description: Create a structured test plan for a feature. Includes unit tests, integration tests, edge cases, and E2E scenarios.
user-invocable: true
```
Instructions: For the given feature, create a test plan in docs/test-plans/ with: test categories (unit/integration/E2E), test cases with Given/When/Then format, edge cases, data requirements, expected results, priority (P0-P3).

#### 16. `/bug-report` — File a Bug
```yaml
name: bug-report
description: Create a structured bug report as a task with reproduction steps, expected vs actual behavior.
disable-model-invocation: true
```
Instructions: Create a task with: title, severity (critical/high/medium/low), steps to reproduce, expected behavior, actual behavior, environment details, screenshots/logs if available. Assign to relevant dev.

#### 17. `/sprint-review` — Sprint/Phase Review
```yaml
name: sprint-review
description: Run a sprint review - gather status from all tasks, check acceptance criteria, compile phase report.
disable-model-invocation: true
```
Instructions: Read all tasks via TaskList. For each completed task: verify acceptance criteria met, check tests pass. For in-progress: report blockers. Compile a phase review report. Broadcast summary to all teammates for feedback.

#### 18. `/assign-task` — Create & Assign Task
```yaml
name: assign-task
description: Create a new task with full Jira-style details and assign to a teammate. Format: /assign-task [teammate] [title]
disable-model-invocation: true
```
Instructions: Create task with: title, description, acceptance criteria, assigned owner, priority, blocked-by dependencies. Use TaskCreate + TaskUpdate to set owner and dependencies.

### Personal Skills (`~/.claude/skills/`) — Cross-project

#### 19. `/token-check` — Token Usage Reminder
```yaml
name: token-check
description: Remind the user to check their daily token usage and decide whether to continue or checkpoint.
disable-model-invocation: true
```
Instructions: Output a reminder to check token usage at console.anthropic.com. If past 80%, run /checkpoint and recommend stopping.

---

## Business-Critical Features (CEO Priorities)

These are the features that separate a toy project from a real competitor to Shopify.
Every one is essential for launch. Listed in priority order:

### Must-Have for MVP
1. **Discount & Promotion Engine** — coupons, percentage/fixed discounts, flash sales, bundle pricing, automatic rules (buy 2 get 1)
2. **Transactional Email System** — order confirmation, shipping updates, password reset, welcome email, abandoned cart recovery
3. **Inventory Management** — real-time stock tracking, low-stock alerts, variant-level inventory, backorder support
4. **Shipping Integration** — carrier rate calculation, label generation, tracking numbers, multiple shipping methods
5. **Tax Calculation** — automatic tax by region, tax-exempt customers, tax reports
6. **i18n / Multi-Currency** — translatable content, currency conversion, locale-aware formatting from day one
7. **Analytics & Tracking** — event tracking (page views, add-to-cart, purchases), conversion funnels, real-time dashboard

### Must-Have for Production
8. **Security Layer** — rate limiting, CSRF protection, Content Security Policy, input sanitization, SQL injection prevention (already via sqlx), XSS prevention
9. **GDPR / Legal Compliance** — cookie consent, data export, account deletion, privacy policy, terms of service
10. **Customer Support** — contact form, FAQ/knowledge base, order inquiry system
11. **Content System** — blog/news for SEO, help center, store-owner onboarding guides
12. **Webhook System** — event-driven integrations (order.created, payment.completed, etc.)
13. **API Keys & Rate Limiting** — for third-party integrations and store-owner API access

### Differentiators (What Makes Us Better Than Shopify)
14. **Dynamic Product Schemas** — our core advantage, already planned
15. **Rust Performance** — 10-100x faster API than Node/PHP competitors
16. **Open Source & Self-Hosted** — no transaction fees, own your data
17. **Plugin Architecture** — extensible without modifying core (future phase)
18. **AI-Powered** — smart product recommendations, auto-generated descriptions, intelligent search (future phase)

---

## Deployment Architecture (Container-per-Store)

Each store is an **isolated deployment** — its own backend + frontend containers.
This is the Goseli platform model: one codebase, many deployments.

### How It Works
```
                    ┌─────────────────┐
                    │  Reverse Proxy   │  (Traefik / nginx)
                    │  Domain Router   │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            ▼                ▼                ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │ Store A      │ │ Store B      │ │ Store C      │
    │ backend:3001 │ │ backend:3002 │ │ backend:3003 │
    │ frontend:3000│ │ frontend:3010│ │ frontend:3020│
    │ epic-games.co│ │ irri-shop.co │ │ tech-mart.co │
    └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
           │                │                │
           ▼                ▼                ▼
    ┌─────────────────────────────────────────────┐
    │          Shared Services                     │
    │  PostgreSQL (DB per store OR shared + schema)│
    │  Redis (namespaced keys per store)           │
    │  Meilisearch (index per store)               │
    │  Image Storage (local → S3 later)            │
    └─────────────────────────────────────────────┘
```

### Key Decisions for Architect
1. **Container isolation**: Each store = docker-compose with backend + frontend containers
2. **Database strategy**: Architect must decide between:
   - **Option A**: Shared DB, `store_id` column on every table (simpler, fewer resources)
   - **Option B**: Separate DB per store (full isolation, easier backups/migration)
   - **Option C**: Shared DB with PostgreSQL schemas per store (middle ground)
3. **Domain routing**: Reverse proxy maps custom domains → store containers
   - Default: `{slug}.goseli.com` subdomain
   - Custom: merchant's own domain via DNS CNAME
4. **Image storage**: Start with local filesystem, migrate to S3/MinIO later
   - Phase 1: `/uploads/{store_id}/` on local disk
   - Phase 3+: S3-compatible (MinIO dev, AWS S3 prod)
5. **Store provisioning**: Admin API to create new stores → spin up containers
6. **Shared config**: Store configuration (product schema, theme, checkout flow) stored in DB, read at container startup

### Store Configuration (drives product schema per store)

```json
{
  "store": {
    "name": "Epic Board Games",
    "slug": "epic-boardgames",
    "theme": "classic",
    "currency": "USD",
    "domain": "epicboardgames.com",
    "product_schema": {
      "type": "boardgame",
      "attributes": [
        { "key": "players_min", "type": "integer", "label": "Min Players", "filterable": true },
        { "key": "players_max", "type": "integer", "label": "Max Players", "filterable": true },
        { "key": "play_time_minutes", "type": "integer", "label": "Play Time (min)" },
        { "key": "complexity", "type": "enum", "options": ["light","medium","heavy"], "filterable": true }
      ]
    },
    "checkout_flow": ["cart", "shipping", "payment", "confirmation"],
    "features": { "reviews": true, "wishlist": true, "compare": false }
  }
}
```

This means:
- **Product attributes** are dynamic (JSONB in PostgreSQL, validated against schema)
- **Storefront** renders fields based on the schema (no code change per vertical)
- **Filters/facets** auto-generated from `filterable` attributes
- **Theme** is swappable via CSS variables + layout variants
- **Checkout flow** is configurable (steps can be added/removed)
- **Each store** runs in its own containers with its own domain

---

## Database Schema (Core Tables)

```sql
-- Store configuration
stores (id, slug, name, config JSONB, theme, created_at, updated_at)

-- Users & Auth
users (id, email, password_hash, role, store_id, created_at)
refresh_tokens (id, user_id, token_hash, expires_at, created_at)

-- Product Catalog
categories (id, store_id, name, slug, parent_id, sort_order)
products (id, store_id, category_id, name, slug, description, price,
          compare_at_price, attributes JSONB, status, created_at, updated_at)
product_images (id, product_id, url, alt_text, sort_order)
product_variants (id, product_id, name, sku, price, stock_quantity, attributes JSONB)

-- Commerce
carts (id, user_id, store_id, session_id, created_at, updated_at)
cart_items (id, cart_id, product_id, variant_id, quantity)
orders (id, store_id, user_id, status, total, shipping_address JSONB,
        billing_address JSONB, payment_status, created_at)
order_items (id, order_id, product_id, variant_id, quantity, unit_price)

-- Reviews
reviews (id, product_id, user_id, rating, title, body, status, created_at)
```

---

## Agent Team Structure

We'll use **CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS** with the following team:

### Core Team (Always Active)

| Role | Teammate Name | Model | Responsibility | Files/Artifacts Owned |
|------|--------------|-------|----------------|----------------------|
| **Team Lead / PM** | (main session) | Opus | Coordination, task management, PR reviews, progress tracking, delegate mode | CLAUDE.md, PLAN.md, CHECKPOINT.md |
| **Architect** | `architect` | Opus | System design, API contracts, DB schema, tech decisions, code reviews | `shared/openapi.yaml`, `docs/architecture.md`, schema designs |
| **Backend Dev 1** | `backend-core` | Sonnet | Rust API server, auth, middleware, user management | `backend/crates/api/`, `backend/crates/auth/` |
| **Backend Dev 2** | `backend-commerce` | Sonnet | Products, cart, orders, payments, search integration | `backend/crates/core/`, `backend/crates/db/`, `backend/crates/search/` |
| **Frontend Dev 1** | `frontend-store` | Sonnet | Storefront pages, product listing/detail, cart UI, responsive | `frontend/src/app/(storefront)/`, `frontend/src/components/` |
| **Frontend Dev 2** | `frontend-admin` | Sonnet | Admin dashboard, product management, analytics dashboards | `frontend/src/app/(admin)/` |

### Support Team (Activated Per Phase)

| Role | Teammate Name | Model | Responsibility | Files/Artifacts Owned |
|------|--------------|-------|----------------|----------------------|
| **Designer** | `designer` | Sonnet | Design system, UI component library, theme tokens, UX flows, accessibility | `frontend/src/styles/`, `docs/design-system.md`, Tailwind config |
| **DevOps** | `devops` | Sonnet | CI/CD, GitHub Actions, migrations, service config | `.github/`, `backend/migrations/` |
| **Marketing / SEO** | `marketing` | Sonnet | SEO meta tags, copy/microcopy, landing page content, structured data | Layout metadata, `docs/seo-guide.md` |
| **Business Analyst** | `analyst` | Opus | Requirements docs, user stories, acceptance criteria, KPIs | `docs/requirements/`, `docs/user-stories.md` |
| **QA / Tester** | `qa` | Sonnet | Test strategy, unit/integration/E2E tests, test data generation, bug reports | `backend/tests/`, `frontend/tests/`, `docs/test-plans/` |
| **Manual Tester** | `tester` | Sonnet | Exploratory testing, UX flow validation, edge case discovery, bug filing as tasks | `docs/test-reports/`, bug task creation |
| **Content Writer** | `content` | Sonnet | Product descriptions, blog posts, help articles, onboarding guides, email templates | `docs/content/`, email templates, blog posts |
| **Security Engineer** | `security` | Opus | Security audits, rate limiting config, CSRF/CSP/XSS prevention, GDPR compliance | Security middleware, CSP headers, compliance docs |

### Team Rotation Strategy
- **Phase 1 (Foundation)**: Architect + Backend Dev 1 + Backend Dev 2 + DevOps + Designer
- **Phase 2 (Commerce)**: Backend Dev 1 + Backend Dev 2 + Frontend Dev 1 + Frontend Dev 2 + Analyst
- **Phase 3 (Reusability)**: Architect + Frontend Dev 1 + Designer + Marketing + QA
- **Phase 4 (Polish)**: Marketing + QA + DevOps + Analyst

### Teammate Skill Access
Each teammate can invoke project skills relevant to their role:
- **Backend Devs**: `/build`, `/test`, `/new-endpoint`, `/db-migrate`, `/review-pr`
- **Frontend Devs**: `/build`, `/test`, `/new-component`, `/review-pr`
- **Architect**: `/api-doc`, `/review-pr`, `/competitive-analysis`
- **Designer**: `/design-spec`, `/new-component`
- **DevOps**: `/build`, `/test`, `/deploy-check`, `/health-check`
- **Marketing**: `/seo-audit`, `/competitive-analysis`
- **Analyst**: `/user-story`, `/competitive-analysis`
- **QA**: `/test`, `/test-plan`, `/deploy-check`, `/health-check`
- **Tester**: `/bug-report`, `/health-check`, `/test`, `/exploratory-test`, `/test-report`
- **Content Writer**: `/product-copy`, `/copy-review`, `/email-template`, `/onboarding-flow`
- **Security Engineer**: `/security-audit`, `/deploy-check`, `/review-pr`
- **Lead/PM**: `/standup`, `/sprint-review`, `/retrospective`, `/assign-task`, `/checkpoint`, `/feature-prioritize`

### How Teammates Collaborate
- **Architect** designs API contracts → Backend Devs implement → QA tests
- **Analyst** writes user stories → Architect creates specs → Devs build
- **Designer** creates component specs → Frontend Devs build → Designer reviews
- **Marketing** provides SEO/copy requirements → Frontend Devs integrate
- **All teammates** use shared task list + direct messaging + skills

---

## Development Phases

### Phase 0: Setup (Pre-team)
**Goal**: All tooling installed and configured

Tasks:
- [ ] P0.1: Install missing tools (gh CLI, sqlx-cli, npm)
- [ ] P0.2: Enable Agent Teams in settings.json
- [ ] P0.3: Create all custom skills in .claude/skills/
- [ ] P0.4: Initialize git repo + create GitHub remote
- [ ] P0.5: Create CLAUDE.md with project conventions
- [ ] P0.6: Set up PostgreSQL database + Redis verify

### Phase 1: Foundation
**Goal**: Runnable monorepo with basic CRUD

Tasks:
- [ ] P1.1: Set up monorepo structure (Cargo workspace + Yarn)
- [ ] P1.2: Rust backend scaffold (Axum server, health check, DB pool)
- [ ] P1.3: Database migrations (core tables) using /db-migrate
- [ ] P1.4: Auth system (register, login, JWT, middleware)
- [ ] P1.5: Product CRUD API using /new-endpoint
- [ ] P1.6: Design system foundation (tokens, typography, components) using /design-spec
- [ ] P1.7: Next.js scaffold with Tailwind + theme system
- [ ] P1.8: Storefront: product listing + product detail pages using /new-component
- [ ] P1.9: GitHub Actions CI (cargo test + clippy + yarn test)
- [ ] P1.10: /checkpoint — save progress

### Phase 2: Core Commerce
**Goal**: Working shopping experience

Tasks:
- [ ] P2.1: /user-story for cart + checkout flows (Analyst)
- [ ] P2.2: Cart API + UI (add, remove, update quantity)
- [ ] P2.3: Checkout API + UI (multi-step flow)
- [ ] P2.4: Order management API + admin UI
- [ ] P2.5: Payment integration (Stripe)
- [ ] P2.6: Order confirmation + email notifications
- [ ] P2.7: /test full suite + /review-pr
- [ ] P2.8: /checkpoint — save progress

### Phase 3: Reusability & Admin
**Goal**: Multi-vertical support + management dashboard

Tasks:
- [ ] P3.1: Store configuration system (dynamic schemas)
- [ ] P3.2: Admin dashboard (product CRUD, orders, settings)
- [ ] P3.3: /design-spec for admin components (Designer)
- [ ] P3.4: Meilisearch integration (product search + filters)
- [ ] P3.5: Review system
- [ ] P3.6: Second store config demo (irrigation systems)
- [ ] P3.7: /seo-audit (Marketing)
- [ ] P3.8: /checkpoint — save progress

### Phase 4: Polish & Production
**Goal**: Production-ready platform

Tasks:
- [ ] P4.1: /seo-audit fix all issues (Marketing)
- [ ] P4.2: Performance optimization (caching, ISR, images)
- [ ] P4.3: Error handling + monitoring
- [ ] P4.4: /api-doc generate full docs
- [ ] P4.5: /deploy-check + deployment pipeline
- [ ] P4.6: E2E tests with Playwright (QA)
- [ ] P4.7: /competitive-analysis final comparison
- [ ] P4.8: Security audit
- [ ] P4.9: /checkpoint — final state

---

## Workflow & Quality

### Git Workflow
- `main` branch: protected, requires PR + /review-pr
- Feature branches: `feature/P1.1-project-scaffold`
- All commits with conventional commit messages
- PRs reviewed by Architect or relevant teammate

### CI Pipeline (GitHub Actions)
```yaml
on: [pull_request]
jobs:
  backend:
    - cargo fmt --check
    - cargo clippy -- -D warnings
    - cargo test --workspace
  frontend:
    - yarn lint
    - yarn type-check
    - yarn test
```

### Documentation
- All docs in `/docs` — maintained by relevant role
- CLAUDE.md = project conventions (all agents read this)
- CHECKPOINT.md = session continuity
- OpenAPI spec in `/shared/openapi.yaml` via /api-doc

### Progress Tracking ("Jira" Workflow)

We use Claude's built-in Task system as our Jira board. Every piece of work follows this lifecycle:

**Task States**: `pending` → `in_progress` → `completed`

**Workflow per Feature**:
1. **Analyst** creates user story task (via /user-story) with acceptance criteria
2. **Architect** reviews story → creates technical spec task → assigns to devs
3. **Dev** claims task, sets `in_progress`, implements on feature branch
4. **Dev** runs /test + /build → creates PR
5. **Architect or peer** runs /review-pr → approves or sends back with feedback
6. **Lead** merges PR, marks task `completed`
7. **QA** verifies in integration tests

**Task Dependencies** (blockedBy/blocks):
- Design spec tasks block Frontend implementation tasks
- API contract tasks block Backend implementation tasks
- Backend API tasks block Frontend integration tasks
- All feature tasks block the phase /checkpoint task

**Sprint Reviews** (per phase):
- Lead broadcasts to all teammates: "Phase N complete — review findings"
- Each teammate reports: what was done, blockers hit, suggestions
- Architect reviews overall architecture coherence
- Analyst verifies acceptance criteria met
- Lead synthesizes into CHECKPOINT.md

**Additional tracking**:
- **CHECKPOINT.md** updated via /checkpoint at each breakpoint
- **Memory files** updated with stable patterns and decisions
- **/token-check** reminders at natural breakpoints

---

## Prerequisites (Phase 0)

1. **Enable Agent Teams**: `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in `.claude/settings.json`
2. **Install gh CLI**: For GitHub integration
3. **Install sqlx-cli**: `cargo install sqlx-cli` for DB migrations
4. **Install npm**: Needed alongside Yarn for npx/tooling
5. **Create GitHub repo**: `goseli` repository
6. **Verify local PG + Redis**: Both already installed, just need a database created
7. **Create docker-compose.yml**: PostgreSQL + Redis + Meilisearch for portable dev setup & future deploys

---

## Token Usage Strategy

- **Max 5-6 active teammates** at any time (rotate per phase)
- Lead uses **delegate mode** (Shift+Tab) — coordinates only
- **Sonnet for implementation** (Backend/Frontend Devs, DevOps, QA) — cheaper
- **Opus for decisions** (Architect, Analyst, Lead) — better reasoning
- **/checkpoint at every breakpoint** — never lose progress
- **/token-check reminders** — pause and save before 80% daily usage
- Skills reduce token waste by standardizing repeatable operations

---

## First Session Execution Plan

1. **Phase 0**: Install tools, enable Agent Teams, create skills, init repo
2. **Spawn team** for Phase 1 (Architect + 2 Backend + DevOps + Designer)
3. **Architect** designs API contracts + DB schema first (plan approval required)
4. **Parallel work**: Backend Devs + DevOps implement while Designer creates design system
5. **Frontend** scaffold after design system + API are ready
6. **Lead** reviews all PRs, runs /test, coordinates
7. **/checkpoint** before session ends
