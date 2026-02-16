---
name: product-copy
description: Generate compelling product descriptions, titles, and metadata for store products. Used by Marketing teammate. Optimized for SEO and conversions.
argument-hint: [product-type or product-name]
---

# Generate Product Copy

Create SEO-optimized, conversion-focused product copy.

Product: $ARGUMENTS

## Output Format

### Product Title
- Max 60 characters
- Include primary keyword
- Descriptive but concise

### Short Description (meta/card)
- Max 160 characters
- Hook + key benefit
- Used in product cards and meta descriptions

### Full Description
Structure:
1. **Hook** (1 sentence) — grab attention, state the problem or desire
2. **Key Benefits** (3-5 bullet points) — what the customer gets
3. **Details** (paragraph) — specifications, materials, dimensions
4. **Social Proof** placeholder — "Rated X/5 by Y customers"
5. **CTA** — clear next action

### SEO Metadata
- `<title>`: Product name + category + brand (50-60 chars)
- `<meta description>`: Compelling summary (150-160 chars)
- Focus keyword + 2-3 related keywords
- JSON-LD Product schema data points

### Per-Vertical Adaptation
Adjust copy tone and focus based on store type:
- **Boardgames**: fun, social, challenge, family/friends
- **Irrigation**: professional, efficiency, reliability, ROI
- **Electronics**: specs, innovation, compatibility, performance
- **Generic**: benefits, quality, value

## Guidelines
- Write for the customer, not the search engine
- Benefits before features ("saves you 2 hours" > "automated system")
- Use sensory language where appropriate
- Avoid superlatives without proof ("best" → "rated #1 by...")
