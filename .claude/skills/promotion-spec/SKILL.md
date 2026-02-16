---
name: promotion-spec
description: Design a discount/promotion rule with conditions, actions, and limits. Used by Business Analyst to spec promotions that Backend Devs implement.
argument-hint: [promotion-type e.g. "percentage-discount" or "buy-x-get-y"]
---

# Promotion/Discount Specification

Design a promotion rule for the Goseli platform.

Type: $ARGUMENTS

## Promotion Types

### 1. Percentage Discount
- X% off entire order or specific products/categories
- Example: "20% off all boardgames"

### 2. Fixed Amount Discount
- $X off when order exceeds $Y
- Example: "$10 off orders over $50"

### 3. Buy X Get Y
- Buy N items, get M free/discounted
- Example: "Buy 2 boardgames, get 1 free"

### 4. Free Shipping
- Free shipping above threshold or on specific products
- Example: "Free shipping on orders over $75"

### 5. Bundle Discount
- Discount when buying specific product combinations
- Example: "Buy game + expansion, save 15%"

### 6. Flash Sale
- Time-limited discount on specific products
- Example: "50% off for next 24 hours"

## Promotion Rule Schema

```json
{
  "promotion": {
    "id": "promo_xxx",
    "name": "Summer Sale",
    "type": "percentage | fixed | buy_x_get_y | free_shipping | bundle",
    "code": "SUMMER20",          // null for automatic promotions
    "conditions": {
      "min_order_total": 50.00,   // optional
      "min_items": 2,             // optional
      "applies_to": {
        "type": "all | category | product | tag",
        "ids": ["cat_boardgames"]
      },
      "customer_segments": ["new | returning | vip"],  // optional
      "date_range": {
        "start": "2026-06-01T00:00:00Z",
        "end": "2026-08-31T23:59:59Z"
      }
    },
    "action": {
      "discount_type": "percentage | fixed",
      "discount_value": 20,
      "max_discount": 100.00,     // cap for percentage discounts
      "applies_to_cheapest": true  // for buy_x_get_y
    },
    "limits": {
      "total_uses": 1000,
      "per_customer": 1,
      "stackable": false           // can combine with other promos?
    },
    "status": "active | scheduled | expired | disabled"
  }
}
```

## Database Impact
- `promotions` table with JSONB conditions/actions
- `promotion_uses` tracking table
- Cart calculation must check applicable promotions
- Order must record which promotions were applied

## Acceptance Criteria
- [ ] Promotion applies correctly at checkout
- [ ] Limits are enforced (total uses, per-customer)
- [ ] Cannot combine non-stackable promotions
- [ ] Expired/disabled promotions are rejected
- [ ] Admin can create/edit/disable promotions
- [ ] Customer sees discount breakdown in cart
