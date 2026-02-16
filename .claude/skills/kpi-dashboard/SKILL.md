---
name: kpi-dashboard
description: Define KPIs and metrics for the store. Tracks conversion rate, AOV, cart abandonment, page views, etc. Used by Business Analyst teammate.
argument-hint: [metric-category or "all"]
---

# KPI Dashboard Specification

Define and track key performance indicators for the Goseli platform.

Category: $ARGUMENTS

## Core E-Commerce KPIs

### Conversion Metrics
| KPI | Formula | Target | Data Source |
|-----|---------|--------|-------------|
| Conversion Rate | orders / unique_visitors * 100 | > 2.5% | orders + analytics |
| Add-to-Cart Rate | add_to_cart_events / product_views * 100 | > 8% | events |
| Cart Abandonment Rate | (carts_created - orders) / carts_created * 100 | < 70% | carts + orders |
| Checkout Completion | orders / checkout_started * 100 | > 60% | events + orders |

### Revenue Metrics
| KPI | Formula | Target | Data Source |
|-----|---------|--------|-------------|
| Average Order Value (AOV) | total_revenue / total_orders | varies | orders |
| Revenue Per Visitor | total_revenue / unique_visitors | varies | orders + analytics |
| Customer Lifetime Value | avg_order_value * purchase_frequency * customer_lifespan | varies | orders + users |
| Refund Rate | refunded_orders / total_orders * 100 | < 3% | orders |

### Product Metrics
| KPI | Formula | Target | Data Source |
|-----|---------|--------|-------------|
| Top Products | sorted by revenue or units sold | - | order_items |
| Product View to Purchase | orders_with_product / product_views | - | events + orders |
| Out-of-Stock Rate | out_of_stock_products / total_products * 100 | < 5% | product_variants |
| Review Rate | reviews / orders * 100 | > 10% | reviews + orders |

### Technical Metrics
| KPI | Formula | Target | Data Source |
|-----|---------|--------|-------------|
| Page Load (LCP) | Core Web Vitals | < 2.5s | Lighthouse |
| API Response Time (p95) | - | < 200ms | backend logs |
| Error Rate | 5xx_responses / total_requests * 100 | < 0.1% | backend logs |
| Uptime | - | > 99.9% | monitoring |

## Implementation Notes
- Events tracked via custom analytics (or integrate Plausible/PostHog)
- Backend exposes `/api/admin/analytics` endpoints
- Admin dashboard shows real-time and historical KPI views
- Define alerts for KPIs exceeding thresholds

## Per-Vertical Adjustments
Different store types may prioritize different KPIs:
- **Boardgames**: focus on review rate, repeat purchases, wishlist saves
- **Irrigation**: focus on AOV (higher-ticket), quote requests, B2B metrics
