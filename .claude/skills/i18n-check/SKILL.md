---
name: i18n-check
description: Audit the codebase for internationalization readiness - hardcoded strings, date/number formatting, RTL support, currency handling. Used by any teammate.
context: fork
agent: Explore
argument-hint: [area e.g. "frontend" or "backend" or "all"]
---

# Internationalization (i18n) Audit

Check the codebase for i18n readiness.

Area: $ARGUMENTS

## Checks

### Frontend (Next.js)

#### String Externalization
- [ ] No hardcoded user-facing strings in JSX/TSX
- [ ] All strings go through translation function: `t('key')`
- [ ] Translation files exist: `locales/{en,es,de,...}.json`
- [ ] Dynamic strings use interpolation: `t('greeting', { name })`
- [ ] Pluralization handled: `t('items', { count })`

#### Date/Time Formatting
- [ ] All dates use `Intl.DateTimeFormat` or date-fns with locale
- [ ] No hardcoded date formats (MM/DD/YYYY)
- [ ] Relative time ("2 hours ago") uses locale-aware formatter

#### Number/Currency Formatting
- [ ] All prices use `Intl.NumberFormat` with currency
- [ ] No hardcoded currency symbols ($, €)
- [ ] Number formatting respects locale (1,000.00 vs 1.000,00)

#### Layout
- [ ] CSS supports RTL (use logical properties: margin-inline-start, not margin-left)
- [ ] Text containers can handle 30% expansion
- [ ] No text in images
- [ ] Icons don't have cultural bias

### Backend (Rust)

#### API Responses
- [ ] Error messages are translatable (use error codes, not strings)
- [ ] API accepts `Accept-Language` header
- [ ] Dates returned in ISO 8601 (UTC)
- [ ] Prices stored as integers (cents) with currency code

#### Data Storage
- [ ] Product names/descriptions support multiple languages (JSONB)
- [ ] Email templates support locale selection
- [ ] Locale stored in user preferences

### Multi-Currency
- [ ] Store config supports multiple currencies
- [ ] Exchange rates source defined
- [ ] Prices can be set per-currency or auto-converted
- [ ] Payment processor handles currency correctly

## Output
For each finding:
- **File**: path:line
- **Issue**: what's hardcoded or missing
- **Fix**: specific code change needed
- **Priority**: P0 (blocks i18n) / P1 (important) / P2 (nice-to-have)
