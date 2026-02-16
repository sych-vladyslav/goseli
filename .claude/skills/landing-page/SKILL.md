---
name: landing-page
description: Design and spec a landing page with hero section, value props, social proof, and CTA. Used by Marketing + Designer teammates together.
argument-hint: [page-purpose e.g. "homepage" or "product-launch"]
---

# Landing Page Specification

Design a high-converting landing page.

Purpose: $ARGUMENTS

## Page Sections

### 1. Hero Section
- **Headline**: Clear value proposition (max 10 words)
- **Subheadline**: Supporting detail (max 25 words)
- **CTA Button**: Primary action with compelling text
- **Hero Image/Visual**: What should be shown
- **Mobile**: Stack vertically, CTA above the fold

### 2. Trust Bar
- Customer count or social proof number
- Key partners/integrations logos
- Security/guarantee badges

### 3. Value Propositions (3-4 cards)
For each:
- Icon suggestion
- Headline (3-5 words)
- Description (1-2 sentences)
- Relates to a customer pain point

### 4. Product Showcase
- Featured products grid or carousel
- Category highlights
- "New arrivals" or "Best sellers" section

### 5. How It Works (if applicable)
- 3-step process
- Simple icons + short descriptions
- Builds confidence in the purchase flow

### 6. Social Proof
- Customer testimonials (2-3)
- Rating aggregate
- "As seen in" or press mentions

### 7. Final CTA
- Repeat the primary CTA
- Add urgency or incentive if appropriate
- Newsletter signup as secondary CTA

### 8. Footer
- Navigation links
- Contact information
- Legal (privacy, terms)
- Social media links

## Technical Requirements
- Next.js page with ISR (revalidate: 3600)
- All images use next/image with priority for above-fold
- Structured data: Organization + WebSite schemas
- Core Web Vitals targets: LCP < 2.5s, CLS < 0.1
