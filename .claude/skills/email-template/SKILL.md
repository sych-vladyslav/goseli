---
name: email-template
description: Create transactional or marketing email templates. Includes HTML email layout, text fallback, variable placeholders, and responsive design. Used by Content Writer and Marketing.
argument-hint: [email-type e.g. "order-confirmation" or "abandoned-cart"]
---

# Email Template Design

Create a professional, responsive email template.

Email type: $ARGUMENTS

## Supported Email Types
- `welcome` — New account registration
- `order-confirmation` — Order placed successfully
- `shipping-update` — Order shipped with tracking
- `abandoned-cart` — Cart reminder after 1h/24h/72h
- `password-reset` — Password reset link
- `review-request` — Ask for product review after delivery
- `low-stock-alert` — Admin: product running low
- `newsletter` — Marketing: promotions, new arrivals

## Template Structure

### Subject Line
- Max 50 chars, compelling, personalized with {{customer_name}}
- A/B test variants: provide 2 options

### Preheader Text
- Max 100 chars, supplements subject line
- Visible in email client previews

### HTML Layout (600px max width, responsive)
```html
<!-- Inline CSS for email client compatibility -->
<table role="presentation" width="100%" style="max-width:600px; margin:0 auto;">
  <!-- Header: Logo + store name -->
  <!-- Hero: Key message or image -->
  <!-- Body: Main content with variables -->
  <!-- CTA: Primary action button -->
  <!-- Footer: Unsubscribe, social links, legal -->
</table>
```

### Variables (Handlebars-style)
- `{{customer_name}}` — Customer's first name
- `{{order_number}}` — Order ID
- `{{order_total}}` — Formatted total
- `{{order_items}}` — Loop of items with images
- `{{tracking_url}}` — Shipping tracking link
- `{{store_name}}` — Store name from config
- `{{unsubscribe_url}}` — Required for marketing emails

### Plain Text Fallback
Every email must have a text-only version.

## Requirements
- Works in Gmail, Outlook, Apple Mail, Yahoo
- Responsive down to 320px
- Dark mode compatible (use both light and dark logo variants)
- CAN-SPAM compliant (physical address, unsubscribe link)
- Accessibility: sufficient contrast, alt text on images
