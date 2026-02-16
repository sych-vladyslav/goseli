---
name: db-migrate
description: Create and run database migrations using sqlx-cli. Use when schema changes are needed.
disable-model-invocation: true
allowed-tools: Bash(cargo sqlx *), Bash(sqlx *), Read, Write
argument-hint: [migration-name]
---

# Database Migration

Create and run a SQL migration for the Goseli database.

## Steps

1. Create a new migration:
   ```bash
   cd /home/r32/goseli/backend && cargo sqlx migrate add $ARGUMENTS
   ```

2. Edit the generated migration file in `backend/migrations/` with the SQL

3. Run the migration:
   ```bash
   cd /home/r32/goseli/backend && cargo sqlx migrate run --database-url postgres://localhost/goseli_dev
   ```

4. Verify migration status:
   ```bash
   cd /home/r32/goseli/backend && cargo sqlx migrate info --database-url postgres://localhost/goseli_dev
   ```

5. If using sqlx compile-time checking, update the offline query data:
   ```bash
   cd /home/r32/goseli/backend && cargo sqlx prepare --database-url postgres://localhost/goseli_dev
   ```

## Conventions
- Migration names: descriptive snake_case (e.g., `create_products_table`)
- Always include both UP and DOWN (reversible) where possible
- Use IF NOT EXISTS for CREATE TABLE
- Add indexes in the same migration as the table
- JSONB columns should have GIN indexes for query performance
