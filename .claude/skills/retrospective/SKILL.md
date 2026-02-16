---
name: retrospective
description: Run a team retrospective after a phase - what went well, what didn't, and what to change. Used by PM/Lead at the end of each phase.
disable-model-invocation: true
---

# Team Retrospective

Reflect on the completed phase and identify improvements.

## Process

1. **Gather data** from:
   - Completed task list and timelines
   - Bug reports filed during the phase
   - Sprint review findings
   - Any coordination issues observed

2. **Compile retrospective**:

```markdown
# Retrospective: Phase [N] - [Date]

## What Went Well
1. [Positive outcome] — why it worked
2. [Positive outcome] — why it worked
3. [Positive outcome] — why it worked

## What Didn't Go Well
1. [Issue] — what happened and impact
2. [Issue] — what happened and impact
3. [Issue] — what happened and impact

## What to Change
1. [Action item] — assigned to [who] — by [when]
2. [Action item] — assigned to [who] — by [when]
3. [Action item] — assigned to [who] — by [when]

## Metrics
- Tasks planned: [X]
- Tasks completed: [Y] ([Z]%)
- Bugs found: [count]
- Bugs fixed: [count]

## Team Feedback
[Summarize any feedback from teammates about process, tools, or coordination]

## Skills/Process Updates Needed
- [ ] Any skills that need updating based on lessons learned?
- [ ] Any new skills needed?
- [ ] CLAUDE.md conventions to add/change?
- [ ] Workflow improvements?

## Carry-Over
- Unfinished tasks: [list with new priority]
- Technical debt identified: [list]
- Process improvements to try next phase: [list]
```

3. **Update project artifacts**:
   - Update CLAUDE.md if conventions need changing
   - Update relevant skills if processes need adjusting
   - Update PLAN.md if phase scope changed
   - Save key learnings to memory files
