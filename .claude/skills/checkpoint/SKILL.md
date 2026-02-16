---
name: checkpoint
description: Save current progress to CHECKPOINT.md for session resumption. Use at breakpoints and before stopping work.
disable-model-invocation: true
---

# Save Checkpoint

Save the current project state so work can be resumed in a new session.

## Steps

1. Read the current task list (use TaskList tool)
2. Check git status and recent commits:
   ```bash
   git -C /home/r32/goseli log --oneline -10
   git -C /home/r32/goseli status
   git -C /home/r32/goseli branch -a
   ```

3. Check running services:
   ```bash
   pg_isready 2>/dev/null && echo "PostgreSQL: running" || echo "PostgreSQL: stopped"
   redis-cli ping 2>/dev/null && echo "Redis: running" || echo "Redis: stopped"
   ```

4. Write CHECKPOINT.md at /home/r32/goseli/CHECKPOINT.md with:

```markdown
# Goseli - Checkpoint [DATE]

## Completed Tasks
[List all completed tasks with IDs]

## In-Progress Tasks
[List all in-progress tasks with current state]

## Pending Tasks
[List remaining tasks]

## Git State
- Current branch: [branch]
- Last commit: [hash + message]
- Uncommitted changes: [yes/no, what files]

## Services State
- PostgreSQL: [running/stopped]
- Redis: [running/stopped]

## How to Resume
1. Start services: [commands if needed]
2. Current working area: [what was being worked on]
3. Next action: [specific next step to take]
4. Blockers: [any known blockers]
```

5. Also update memory files if any stable patterns were discovered
