---
name: review-pr
description: Review code changes against project standards. Use for PR reviews and pre-commit quality checks.
context: fork
agent: Explore
argument-hint: [branch-name or PR-number]
---

# Code Review

Review the current code changes against Goseli project standards.

## Review Checklist

### Rust Backend
- [ ] No `.unwrap()` in production code (use `?` operator or proper error handling)
- [ ] All new endpoints have corresponding tests
- [ ] Request/response types match OpenAPI spec in shared/openapi.yaml
- [ ] SQL queries use parameterized queries (no string interpolation)
- [ ] No hardcoded secrets, URLs, or credentials
- [ ] Error types implement proper error responses
- [ ] New database queries have appropriate indexes

### Next.js Frontend
- [ ] Components follow the design system (use design tokens, not hardcoded colors)
- [ ] All interactive elements have proper accessibility (aria-* attributes)
- [ ] Images have alt text
- [ ] Responsive design works at mobile/tablet/desktop breakpoints
- [ ] No console.log left in code
- [ ] TypeScript types are explicit (no `any`)
- [ ] API calls use the shared API client from lib/

### General
- [ ] Commit messages follow conventional format
- [ ] No large files or binaries committed
- [ ] No sensitive data in code or comments
- [ ] New features have documentation updates

## Steps
1. Run `git diff --stat` to see changed files
2. Read each changed file
3. Check against the checklist above
4. Report findings with severity: BLOCKER, WARNING, or NOTE
5. Provide specific file:line references for each finding
