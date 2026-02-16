---
name: onboarding-flow
description: Design the store-owner onboarding experience - from signup to first sale. Used by Designer, Content Writer, and Business Analyst together.
argument-hint: [step e.g. "full" or "store-setup" or "first-product"]
---

# Store Owner Onboarding Flow

Design the experience for a new store owner setting up their Goseli store.

Step: $ARGUMENTS

## Onboarding Goals
- **Time to first product**: < 10 minutes
- **Time to store live**: < 30 minutes
- **Completion rate**: > 80%
- **Zero-confusion**: No step should require documentation

## Onboarding Steps

### Step 1: Sign Up (2 min)
- Email + password (or OAuth: Google, GitHub)
- Store name
- What do you sell? (category picker or free text)
- Skip everything else for now

### Step 2: Store Setup Wizard (5 min)
- **Store info**: Name, description, logo upload
- **Product type**: Select or define custom attributes
- **Theme**: Pick from 3-5 starter themes, preview live
- **Shipping**: Basic settings (flat rate to start)
- **Payments**: Connect Stripe (guided flow)
- Each step has "Skip for now" option

### Step 3: Add First Product (3 min)
- Guided form with inline help
- Image upload with drag-and-drop
- Auto-suggest fields based on product type
- Preview showing how it'll look on storefront
- "Publish" button with celebration animation

### Step 4: Launch Checklist
Interactive checklist showing completion:
- [ ] Store name and logo set
- [ ] At least 1 product added
- [ ] Shipping configured
- [ ] Payment method connected
- [ ] Store preview reviewed
- [ ] Legal pages added (auto-generated templates)
- [ ] **Go Live** button (becomes active when essentials done)

### Step 5: Post-Launch
- Dashboard with "Next steps" suggestions
- "Share your store" social cards
- First sale celebration
- Tips for adding more products, SEO, marketing

## UX Principles
- Progress indicator visible at all times
- Every step saveable (can leave and return)
- Smart defaults (pre-fill where possible)
- Contextual help tooltips (not walls of text)
- Mobile-friendly (store owners might set up on phone)
- Celebrate milestones (animation, confetti on first sale)

## Content Needed
- Welcome email sequence (days 0, 1, 3, 7)
- Tooltip copy for each wizard field
- Help article: "Setting up your first store"
- Video placeholder: "Quick start in 5 minutes"
- Legal page templates (Privacy Policy, Terms of Service, Return Policy)

## Technical Implementation
- Onboarding state stored in `stores.onboarding_status JSONB`
- `/api/onboarding/progress` endpoint
- Frontend wizard component with step navigation
- Analytics events for each step (started, completed, skipped)
- A/B testable step order (feature flag)
