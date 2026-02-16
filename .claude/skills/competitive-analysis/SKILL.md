---
name: competitive-analysis
description: Research and compare our approach against competitors (Shopify, WooCommerce, Medusa) for a given feature.
context: fork
agent: general-purpose
argument-hint: [feature-name]
---

# Competitive Analysis

Research how major e-commerce platforms handle the given feature.

Feature: $ARGUMENTS

## Platforms to Compare
1. **Shopify** - market leader, SaaS
2. **WooCommerce** - open source, WordPress
3. **Medusa.js** - open source, headless, Node.js
4. **Saleor** - open source, headless, Python/GraphQL

## Analysis Template

### Feature: [name]

| Aspect | Shopify | WooCommerce | Medusa | Saleor | Goseli (ours) |
|--------|---------|-------------|--------|--------|---------------|
| Approach | | | | | |
| API | | | | | |
| Flexibility | | | | | |
| Performance | | | | | |
| DX | | | | | |

### Key Insights
- What do they all do well?
- Where are the gaps?
- What can we do better?

### Recommendation for Goseli
- Approach: [description]
- Pros: [list]
- Cons: [list]
- Differentiator: [what makes ours better]
