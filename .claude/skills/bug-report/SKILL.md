---
name: bug-report
description: Create a structured bug report as a task with reproduction steps, expected vs actual behavior, severity, and teammate assignment.
disable-model-invocation: true
argument-hint: [bug-title]
---

# File Bug Report

Create a detailed bug report and add it as a task.

Bug: $ARGUMENTS

## Steps

1. Gather information about the bug

2. Create a task using TaskCreate with this format in the description:

```markdown
## Bug Report: [Title]

**Severity**: [critical | high | medium | low]
- Critical: app crashes, data loss, security issue
- High: feature broken, no workaround
- Medium: feature broken, workaround exists
- Low: cosmetic, minor inconvenience

**Environment**:
- Component: [backend | frontend | both]
- Browser: [if frontend]
- API endpoint: [if backend]

**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Expected Behavior**:
[What should happen]

**Actual Behavior**:
[What actually happens]

**Error Logs**:
```
[Any relevant error output]
```

**Possible Cause**:
[If known, what might be causing this]
```

3. Assign the task to the appropriate teammate based on the component:
   - Backend bugs -> `backend-core` or `backend-commerce`
   - Frontend bugs -> `frontend-store` or `frontend-admin`
   - Infra bugs -> `devops`
