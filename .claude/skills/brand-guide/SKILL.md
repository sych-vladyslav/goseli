---
name: brand-guide
description: Create or update the brand style guide - logo usage, color palette, typography, tone of voice, imagery guidelines. Used by Designer and Marketing teammates.
argument-hint: [store-name or update-section]
---

# Brand Style Guide

Create or update the brand guide for the Goseli storefront.

Store/Section: $ARGUMENTS

## Output File
Save to: `docs/brand-guide.md`

## Sections

### 1. Brand Identity
- Brand name and tagline
- Mission statement (1-2 sentences)
- Brand personality (3-5 adjectives)
- Target audience profiles

### 2. Logo Usage
- Primary logo placement rules
- Minimum size requirements
- Clear space around logo
- Acceptable logo variations (light/dark/monochrome)
- What NOT to do with the logo

### 3. Color Palette
Define with CSS custom properties and Tailwind config values:
```
Primary:    --color-primary     (#XXXX)  - main brand color
Secondary:  --color-secondary   (#XXXX)  - accent color
Surface:    --color-surface     (#XXXX)  - card/panel backgrounds
Background: --color-background  (#XXXX)  - page background
Text:       --color-text        (#XXXX)  - primary text
Muted:      --color-muted       (#XXXX)  - secondary text
Success:    --color-success     (#XXXX)  - positive actions
Warning:    --color-warning     (#XXXX)  - caution states
Error:      --color-error       (#XXXX)  - error states
```

### 4. Typography
- Heading font: [family, weights]
- Body font: [family, weights]
- Font size scale (matching Tailwind defaults)
- Line height guidelines

### 5. Tone of Voice
- Writing style (formal/casual/playful)
- Do/Don't examples for product descriptions
- CTA button text conventions
- Error message tone

### 6. Photography & Imagery
- Image style (lifestyle, studio, flat-lay)
- Aspect ratios for product images
- Placeholder/fallback image specifications
- Icon style (outline/filled, rounded/sharp)

### 7. Store-Specific Overrides
How to customize per-vertical (boardgames vs irrigation):
- Which tokens change
- Theme switching mechanism
- Per-store imagery guidelines
