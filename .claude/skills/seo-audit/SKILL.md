---
name: seo-audit
description: Audit storefront pages for SEO compliance - meta tags, structured data, accessibility, and performance hints.
context: fork
agent: Explore
---

# SEO Audit

Audit all storefront pages in `frontend/src/app/(storefront)/` for SEO best practices.

## Checks

### Meta Tags (per page)
- [ ] `<title>` tag present and unique (50-60 chars)
- [ ] `<meta name="description">` present (150-160 chars)
- [ ] `<meta name="robots">` set appropriately
- [ ] Canonical URL specified

### Open Graph
- [ ] `og:title`, `og:description`, `og:image` present
- [ ] `og:type` set (website, product, etc.)
- [ ] `og:url` matches canonical

### Structured Data (JSON-LD)
- [ ] Product pages: Product schema with price, availability, reviews
- [ ] Category pages: ItemList schema
- [ ] Breadcrumbs: BreadcrumbList schema
- [ ] Organization schema on homepage

### Content
- [ ] One H1 per page
- [ ] Heading hierarchy (H1 > H2 > H3, no skipping)
- [ ] All images have descriptive alt text
- [ ] Internal links use descriptive anchor text

### Technical
- [ ] All pages generate static/ISR where possible
- [ ] Image optimization (next/image with sizes)
- [ ] sitemap.xml generated
- [ ] robots.txt present

## Output
Create a report with:
- Pass/Fail for each check per page
- Specific code fixes needed
- Priority (P0 = critical, P1 = important, P2 = nice-to-have)
