---
name: user-story
description: Write a structured user story with acceptance criteria, edge cases, and technical implementation notes. Saves to docs/user-stories/.
argument-hint: [feature-name]
---

# Write User Story

Create a detailed user story for the given feature and save it to the project docs.

Feature: $ARGUMENTS

## Output File
Save to: `docs/user-stories/US-XXX-$ARGUMENTS.md` (increment XXX based on existing files)

## User Story Template

```markdown
# US-XXX: [Feature Title]

## User Story
As a [customer | store admin | store owner],
I want to [action/feature],
so that [benefit/value].

## Acceptance Criteria

### AC-1: [Criteria name]
- **Given** [precondition]
- **When** [action]
- **Then** [expected result]

### AC-2: [Criteria name]
- **Given** [precondition]
- **When** [action]
- **Then** [expected result]

## Edge Cases
1. [Edge case description] -> [expected behavior]
2. [Edge case description] -> [expected behavior]

## Technical Notes
- **API endpoints needed**: [list]
- **Database changes**: [yes/no, description]
- **Frontend components**: [list]
- **Dependencies**: [other features/stories this depends on]

## Complexity: [S | M | L]
- S = < 1 day implementation
- M = 1-3 days
- L = 3+ days

## Priority: [P0 | P1 | P2 | P3]
```
