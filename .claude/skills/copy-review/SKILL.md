---
name: copy-review
description: Review and improve microcopy, UI text, error messages, and CTA buttons across the storefront. Used by Marketing teammate. Checks tone, clarity, and conversion optimization.
argument-hint: [page-or-component-name]
---

# Copy Review

Review all user-facing text in the specified page/component for quality, consistency, and conversion.

Target: $ARGUMENTS

## Review Checklist

### Clarity
- [ ] Text is understandable without context
- [ ] No jargon or technical terms exposed to users
- [ ] Instructions are actionable ("Add to cart" not "Submit")
- [ ] Error messages tell users what to DO, not just what went wrong

### Consistency
- [ ] Tone matches brand guide (see docs/brand-guide.md)
- [ ] Button text uses consistent patterns (verb + noun)
- [ ] Consistent capitalization (Sentence case for UI, Title Case for headings)
- [ ] Same concepts use same words throughout

### Conversion Optimization
- [ ] CTA buttons are compelling ("Get Started" > "Submit")
- [ ] Value propositions are clear
- [ ] Trust signals present where needed (security badges, guarantees)
- [ ] Urgency/scarcity used appropriately (not manipulatively)

### Accessibility
- [ ] Alt text on images is descriptive and useful
- [ ] Link text makes sense out of context ("Read more about X" not "Click here")
- [ ] Form labels are clear
- [ ] Placeholder text is supplementary, not the only instruction

### Localization Ready
- [ ] No hardcoded strings (all text goes through i18n)
- [ ] No idioms that don't translate well
- [ ] Date/number formats are locale-aware
- [ ] Text containers can handle 30% expansion for translations

## Output
For each finding:
- **Location**: file:line or component name
- **Current text**: "..."
- **Suggested text**: "..."
- **Reason**: why the change improves the copy
