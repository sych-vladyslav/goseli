---
name: test
description: Run the full test suite - backend (cargo test) and frontend (yarn test). Use after implementing features or before committing.
disable-model-invocation: true
allowed-tools: Bash(cargo *), Bash(yarn *)
---

# Run Tests

Run the complete test suite for both backend and frontend.

## Steps

1. Run Rust tests:
   ```bash
   cd /home/r32/goseli/backend && cargo test --workspace 2>&1
   ```

2. Run frontend tests:
   ```bash
   cd /home/r32/goseli/frontend && yarn test 2>&1
   ```

3. Parse and summarize output:
   - Total tests: X passed, Y failed, Z skipped
   - List any failing test names with error messages
   - If all pass: "All tests passing"
