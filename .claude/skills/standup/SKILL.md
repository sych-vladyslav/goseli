---
name: standup
description: Run a quick standup - each teammate reports what they did, what they're doing next, and any blockers. Used by PM/Lead to coordinate the team.
disable-model-invocation: true
---

# Team Standup

Quick sync across all active teammates.

## Process

1. **Read current task list** (TaskList)

2. **For each active teammate**, compile:
   - **Done**: Tasks completed since last standup
   - **Doing**: Current in-progress task
   - **Blocked**: Any blockers or waiting-on items

3. **Output standup report**:

```markdown
# Standup - [Date/Time]

## Team Status

### architect
- Done: [completed tasks]
- Doing: [current task]
- Blocked: [blockers or "none"]

### backend-core
- Done: [completed tasks]
- Doing: [current task]
- Blocked: [blockers or "none"]

### backend-commerce
- Done: [completed tasks]
- Doing: [current task]
- Blocked: [blockers or "none"]

### frontend-store
- Done: [completed tasks]
- Doing: [current task]
- Blocked: [blockers or "none"]

### frontend-admin
- Done: [completed tasks]
- Doing: [current task]
- Blocked: [blockers or "none"]

### designer
- Done: [completed tasks]
- Doing: [current task]
- Blocked: [blockers or "none"]

[... other active teammates]

## Action Items
1. [Blocker resolution action] → assigned to [who]
2. [Coordination needed] → [details]

## Next Milestone
- [What's the next major deliverable?]
- [ETA / tasks remaining]
```

4. **Resolve blockers**:
   - If a teammate is blocked by another teammate, send a message to unblock
   - If blocked by missing info, escalate to the user
   - If blocked by a bug, create a bug task via /bug-report
