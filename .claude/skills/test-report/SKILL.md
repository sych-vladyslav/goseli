---
name: test-report
description: Generate a test execution report summarizing test results, coverage, and quality assessment. Used by QA and Tester teammates. Saves to docs/test-reports/.
argument-hint: [phase-or-feature]
---

# Test Execution Report

Generate a comprehensive test report for the specified phase or feature.

Scope: $ARGUMENTS

## Output File
Save to: `docs/test-reports/TR-XXX-$ARGUMENTS.md`

## Report Template

```markdown
# Test Report: [Scope] - [Date]

## Executive Summary
- **Overall Status**: PASS / FAIL / PARTIAL
- **Tests Executed**: X/Y (Z% coverage)
- **Bugs Found**: X critical, Y high, Z medium, W low
- **Recommendation**: Ready for release / Needs fixes / Block release

## Test Execution Summary

### Backend Tests (cargo test)
| Test Suite | Total | Pass | Fail | Skip |
|-----------|-------|------|------|------|
| auth | | | | |
| products | | | | |
| cart | | | | |
| orders | | | | |

### Frontend Tests (yarn test)
| Test Suite | Total | Pass | Fail | Skip |
|-----------|-------|------|------|------|
| components | | | | |
| pages | | | | |
| utils | | | | |

### E2E Tests (Playwright)
| Scenario | Status | Notes |
|----------|--------|-------|
| | | |

## Bug Summary
| ID | Title | Severity | Status | Assigned |
|----|-------|----------|--------|----------|
| | | | | |

## Test Coverage Gaps
- [Areas not covered and why]
- [Planned future coverage]

## Performance Observations
- API response times: [summary]
- Page load times: [summary]
- Any regressions noted: [yes/no]

## Accessibility Findings
- [Summary from a11y-audit if run]

## Risks
1. [Risk]: [mitigation]

## Sign-off
- QA: [approved/not approved]
- Recommended next steps: [list]
```
