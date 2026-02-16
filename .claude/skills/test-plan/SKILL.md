---
name: test-plan
description: Create a structured test plan for a feature. Includes unit, integration, and E2E test scenarios. Saves to docs/test-plans/.
argument-hint: [feature-name]
---

# Create Test Plan

Generate a comprehensive test plan for the given feature.

Feature: $ARGUMENTS

## Output File
Save to: `docs/test-plans/TP-XXX-$ARGUMENTS.md`

## Template

```markdown
# TP-XXX: [Feature] Test Plan

## Scope
- Feature: [description]
- Components: [backend API, frontend pages, database]
- Related user stories: [US-XXX]

## Unit Tests

### Backend (Rust)
| ID | Test Case | Input | Expected | Priority |
|----|-----------|-------|----------|----------|
| UT-1 | | | | P0 |

### Frontend (React)
| ID | Test Case | Input | Expected | Priority |
|----|-----------|-------|----------|----------|
| UT-1 | | | | P0 |

## Integration Tests
| ID | Test Case | Steps | Expected | Priority |
|----|-----------|-------|----------|----------|
| IT-1 | | | | P0 |

## E2E Tests (Playwright)
| ID | Scenario | Steps | Expected | Priority |
|----|----------|-------|----------|----------|
| E2E-1 | | | | P0 |

## Edge Cases
| ID | Scenario | Expected Behavior |
|----|----------|-------------------|
| EC-1 | | |

## Test Data Requirements
- [What test data/fixtures are needed]

## Priority Legend
- P0: Must pass before merge
- P1: Should pass, can have known issues
- P2: Nice to have
- P3: Future coverage
```
