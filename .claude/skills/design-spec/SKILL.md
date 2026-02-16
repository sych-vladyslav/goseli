---
name: design-spec
description: Generate a UI/UX design specification for a feature or page. Includes layout wireframes, responsive breakpoints, color tokens, spacing, and accessibility notes.
argument-hint: [feature-or-page-name]
---

# Generate Design Specification

Create a comprehensive UI/UX design spec for the given feature or page.

Feature: $ARGUMENTS

## Output Format

### 1. Overview
- Purpose of this feature/page
- Target users and their goals
- Key interactions

### 2. Layout Wireframe (ASCII)
```
+-------------------------------------+
|  Header / Navigation                |
+-------------------------------------+
|  +---------+  +------------------+  |
|  | Sidebar |  |  Main Content    |  |
|  |         |  |                  |  |
|  +---------+  +------------------+  |
+-------------------------------------+
|  Footer                             |
+-------------------------------------+
```

### 3. Design Tokens
- Colors: use from theme (--color-primary, --color-surface, etc.)
- Spacing: 4px grid (space-1 through space-16)
- Typography: heading sizes, body text, labels
- Border radius, shadows

### 4. Responsive Breakpoints
- Mobile (< 640px): single column, stacked layout
- Tablet (640-1024px): adjusted grid
- Desktop (> 1024px): full layout

### 5. Component Hierarchy
- List all components needed with their props
- Mark which are existing vs new

### 6. Accessibility (WCAG AA)
- Color contrast ratios (min 4.5:1 for text)
- Keyboard navigation flow (tab order)
- Screen reader announcements
- Focus indicators
- ARIA attributes needed

### 7. States & Interactions
- Loading states
- Empty states
- Error states
- Hover/focus/active states
- Animations (prefer reduced-motion)
