---
name: deploy-check
description: Run pre-deployment verification checklist - tests pass, no warnings, env vars set, migrations up to date, no TODOs in committed code.
disable-model-invocation: true
allowed-tools: Bash(cargo *), Bash(yarn *), Read
---

# Pre-Deployment Checklist

Verify the project is ready for deployment.

## Checks

1. **Tests pass**:
   ```bash
   cd /home/r32/goseli/backend && cargo test --workspace 2>&1
   cd /home/r32/goseli/frontend && yarn test 2>&1
   ```

2. **Clippy clean**:
   ```bash
   cd /home/r32/goseli/backend && cargo clippy --workspace -- -D warnings 2>&1
   ```

3. **Frontend lint clean**:
   ```bash
   cd /home/r32/goseli/frontend && yarn lint 2>&1
   ```

4. **No TODO/FIXME in committed code**:
   Search for TODO/FIXME/HACK/XXX in source files (excluding tests and docs)

5. **Migrations up to date**:
   ```bash
   cd /home/r32/goseli/backend && cargo sqlx migrate info --database-url postgres://localhost/goseli_dev
   ```

6. **No uncommitted changes**:
   ```bash
   git -C /home/r32/goseli status --porcelain
   ```

7. **Environment variables documented**:
   Check that all env vars used in code are listed in docs/deployment.md

8. **CHECKPOINT.md updated**:
   Verify CHECKPOINT.md reflects current state

## Output
```
Pre-Deployment Check:
  Tests:          All passing (42 backend, 18 frontend)
  Clippy:         No warnings
  Lint:           Clean
  TODOs:          3 found (list them)
  Migrations:     Up to date
  Git:            Clean working tree
  Env vars:       All documented
  Checkpoint:     Current
```
