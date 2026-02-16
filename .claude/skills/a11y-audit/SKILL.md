---
name: a11y-audit
description: Accessibility audit against WCAG 2.1 AA standards. Used by Designer and QA teammates. Checks color contrast, keyboard nav, screen readers, and ARIA usage.
context: fork
agent: Explore
argument-hint: [page-or-component]
---

# Accessibility Audit (WCAG 2.1 AA)

Audit the specified page or component for accessibility compliance.

Target: $ARGUMENTS

## Audit Categories

### 1. Perceivable
- [ ] **Color contrast**: Text meets 4.5:1 ratio (3:1 for large text)
- [ ] **Non-text content**: All images, icons have alt text or aria-label
- [ ] **Captions**: Media content has captions/transcripts
- [ ] **Responsive**: Content reflows at 320px without horizontal scroll
- [ ] **Text spacing**: Content works with increased text spacing

### 2. Operable
- [ ] **Keyboard accessible**: All interactive elements reachable via Tab
- [ ] **No keyboard traps**: Can navigate away from any element
- [ ] **Focus visible**: Clear focus indicator on all interactive elements
- [ ] **Skip links**: "Skip to main content" link present
- [ ] **Touch targets**: Min 44x44px for touch interactions
- [ ] **No timing**: No time-limited interactions without controls

### 3. Understandable
- [ ] **Language**: `lang` attribute on `<html>`
- [ ] **Labels**: All form inputs have visible labels
- [ ] **Error identification**: Errors clearly described with suggestions
- [ ] **Consistent navigation**: Same nav pattern across pages
- [ ] **Predictable**: No unexpected context changes

### 4. Robust
- [ ] **Valid HTML**: Proper semantic elements (nav, main, article, etc.)
- [ ] **ARIA usage**: ARIA roles/properties used correctly
- [ ] **Name/Role/Value**: Custom widgets have proper ARIA
- [ ] **Status messages**: Dynamic content uses aria-live regions

## For Each Finding
- **Issue**: Description
- **WCAG Criterion**: e.g., 1.4.3 Contrast
- **Severity**: Critical / Major / Minor
- **Location**: file:line or component
- **Fix**: Specific code change needed

## Tools to Use
- Check color contrast with computed styles
- Verify heading hierarchy (h1 > h2 > h3)
- Check all `<img>` for alt attributes
- Verify form `<label>` associations
- Check for `role` attributes on custom widgets
