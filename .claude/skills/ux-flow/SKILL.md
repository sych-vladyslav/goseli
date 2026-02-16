---
name: ux-flow
description: Map out a user experience flow with screens, decisions, and interactions. Used by Designer teammate. Creates flowcharts and screen-by-screen specs.
argument-hint: [flow-name e.g. "checkout" or "product-browse"]
---

# UX Flow Mapping

Map the complete user experience flow for a feature or journey.

Flow: $ARGUMENTS

## Output Format

### 1. Flow Overview
- **Entry points**: How does the user arrive? (direct URL, navigation, CTA)
- **Goal**: What is the user trying to accomplish?
- **Success state**: What does completion look like?
- **Exit points**: Where might users drop off?

### 2. Flow Diagram (ASCII)

```
[Entry] → [Screen 1] → [Decision?]
                            ├── Yes → [Screen 2a] → [Success]
                            └── No  → [Screen 2b] → [Screen 3] → [Success]
```

### 3. Screen-by-Screen Specification

For each screen in the flow:

```markdown
#### Screen: [Name]
- **URL**: /path/to/page
- **Entry condition**: How user gets here
- **Key elements**:
  - [ ] Element 1 (purpose)
  - [ ] Element 2 (purpose)
- **User actions**:
  - Action A → leads to [Screen X]
  - Action B → leads to [Screen Y]
- **Error states**: What can go wrong?
- **Loading state**: What shows while data loads?
- **Empty state**: What if there's no data?
```

### 4. Interaction Details
- Transitions between screens (instant, slide, fade)
- Form validation (inline vs on-submit)
- Progress indicators (steps, progress bar)
- Confirmation dialogs (when needed)

### 5. Edge Cases
- User navigates back
- Session expires mid-flow
- Network error during submission
- User opens in new tab
- Mobile-specific considerations

### 6. Analytics Events
For each key interaction, define the tracking event:
| Screen | Action | Event Name | Properties |
|--------|--------|------------|------------|
| | | | |

### 7. Accessibility Flow
- Tab order through the flow
- Screen reader announcements at key points
- Keyboard shortcuts (if any)
- Focus management on navigation
