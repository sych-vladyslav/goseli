---
name: sprint-review
description: Run a sprint/phase review - gather task status, verify acceptance criteria, compile a phase report for the team.
disable-model-invocation: true
---

# Sprint Review

Compile a comprehensive sprint/phase review.

## Steps

1. **Read task list** using TaskList
   - Count: completed, in-progress, pending, blocked

2. **For each completed task**:
   - Verify tests pass (run relevant tests)
   - Check acceptance criteria from the task description
   - Note any partial completions or known issues

3. **For in-progress tasks**:
   - Document current state
   - Identify blockers
   - Estimate remaining work

4. **Compile Phase Report**:

```markdown
# Sprint Review: Phase [N] - [Date]

## Summary
- Tasks completed: X/Y
- Tests passing: X backend, Y frontend
- Blockers: [count]

## Completed Work
| Task | Owner | Status | Notes |
|------|-------|--------|-------|

## In-Progress Work
| Task | Owner | Blocker | ETA |
|------|-------|---------|-----|

## Pending Work
| Task | Blocked By | Priority |
|------|------------|----------|

## Quality Metrics
- Test coverage: [estimate]
- Open bugs: [count]
- Code review findings: [summary]

## Architecture Decisions Made
- [Decision 1]: [rationale]

## Risks & Issues
1. [Risk/issue]: [mitigation]

## Next Phase Priorities
1. [Priority 1]
2. [Priority 2]
```

5. **Broadcast** the report to all active teammates for feedback
