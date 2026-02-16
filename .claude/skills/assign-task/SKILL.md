---
name: assign-task
description: Create a new task with full Jira-style details and assign to a specific teammate. Includes acceptance criteria and dependencies.
disable-model-invocation: true
argument-hint: [teammate-name] [task-title]
---

# Create & Assign Task

Create a detailed task and assign it to a teammate.

Arguments: $ARGUMENTS[0] = teammate name, rest = task title

## Steps

1. Create the task using TaskCreate with:
   - **Subject**: Clear, imperative title
   - **Description**: Full details including:

```markdown
## Description
[What needs to be done]

## Acceptance Criteria
- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]

## Technical Details
- Files to modify: [list]
- Dependencies: [other tasks or features]
- Skills to use: [relevant /skill-name commands]

## Definition of Done
- [ ] Code written and compiles
- [ ] Tests written and passing
- [ ] Code reviewed (via /review-pr)
- [ ] Documentation updated if needed
```

2. Set the task owner to the specified teammate using TaskUpdate

3. Set any dependencies (addBlockedBy) if this task depends on others

4. Notify the teammate by sending them a message about the new assignment
