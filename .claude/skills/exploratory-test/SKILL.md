---
name: exploratory-test
description: Run an exploratory testing session on a feature. Tests edge cases, unexpected inputs, unusual flows. Used by Tester teammate. Files bugs found via /bug-report.
argument-hint: [feature-or-page]
---

# Exploratory Testing Session

Perform structured exploratory testing on the given feature.

Target: $ARGUMENTS

## Testing Charter
- **Mission**: Find bugs, UX issues, and edge cases in $ARGUMENTS
- **Time box**: Focused exploration session
- **Areas**: functionality, usability, edge cases, error handling

## Exploration Strategies

### 1. Happy Path
Walk through the primary user flow as intended:
- Does it work as described in the user story?
- Is the experience smooth and intuitive?
- Do success states display correctly?

### 2. Boundary Testing
- Empty inputs / missing required fields
- Maximum length inputs (very long names, descriptions)
- Special characters (unicode, emoji, HTML, SQL injection attempts)
- Zero, negative, very large numbers for quantities/prices
- Boundary values (exactly at limits)

### 3. State Testing
- Refresh the page mid-flow
- Navigate back/forward
- Open in multiple tabs
- Session timeout during action
- Slow network simulation
- Offline behavior

### 4. Role-Based Testing
- Unauthenticated user access
- Regular customer vs admin
- Accessing another user's resources (IDs in URLs)

### 5. Device/Responsive Testing
- Mobile viewport (375px)
- Tablet viewport (768px)
- Desktop viewport (1440px)
- Touch interactions vs mouse
- Landscape orientation

### 6. Data Variation
- No data (empty states)
- One item
- Many items (pagination)
- Long text content
- Missing images
- Slow-loading images

## Bug Reporting
For each issue found, use /bug-report to file it with:
- Steps to reproduce
- Expected vs actual
- Severity
- Screenshots/logs if applicable

## Session Output
```markdown
# Exploratory Test Report: [Feature] - [Date]

## Session Summary
- Duration: [time]
- Bugs found: [count by severity]
- Areas covered: [list]

## Findings
| # | Type | Severity | Description | Bug Task |
|---|------|----------|-------------|----------|
| 1 | Bug | high | ... | #XX |

## Areas NOT Tested
[List anything skipped and why]

## Recommendations
[Overall quality assessment and suggestions]
```
