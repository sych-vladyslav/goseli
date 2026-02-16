---
name: health-check
description: Check all services are running - PostgreSQL, Redis, backend API server, frontend dev server. Reports status of each.
disable-model-invocation: true
allowed-tools: Bash(pg_isready *), Bash(redis-cli *), Bash(curl *), Bash(lsof *)
---

# System Health Check

Verify all Goseli services are running and healthy.

## Checks

1. **PostgreSQL**:
   ```bash
   pg_isready -h localhost -p 5432
   ```
   Also check: `psql -d goseli_dev -c "SELECT 1;" 2>/dev/null`

2. **Redis**:
   ```bash
   redis-cli ping
   ```
   Expected: PONG

3. **Backend API** (if running):
   ```bash
   curl -s http://localhost:3001/health
   ```
   Expected: {"status":"ok"}

4. **Frontend Dev Server** (if running):
   ```bash
   curl -s -o /dev/null -w "%{http_code}" http://localhost:3000
   ```
   Expected: 200

5. **Meilisearch** (if installed):
   ```bash
   curl -s http://localhost:7700/health
   ```

## Output Format
```
Service Health Check:
  PostgreSQL:  running (port 5432)
  Redis:       running (port 6379)
  Backend:     not running
  Frontend:    not running
  Meilisearch: not installed
```
