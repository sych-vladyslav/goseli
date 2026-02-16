---
name: feature-prioritize
description: Prioritize features using RICE scoring (Reach, Impact, Confidence, Effort). Used by Business Analyst and PM. Outputs a ranked feature backlog.
argument-hint: [feature-list or "backlog"]
---

# Feature Prioritization (RICE Framework)

Prioritize features for the Goseli platform using data-driven scoring.

Features to evaluate: $ARGUMENTS

## RICE Scoring

For each feature, score:

| Factor | Scale | Description |
|--------|-------|-------------|
| **Reach** | # of users/quarter | How many users will this affect? |
| **Impact** | 0.25 / 0.5 / 1 / 2 / 3 | Minimal / Low / Medium / High / Massive |
| **Confidence** | 50% / 80% / 100% | How sure are we about the estimates? |
| **Effort** | person-weeks | How much work is this? |

**RICE Score = (Reach * Impact * Confidence) / Effort**

## Output Template

```markdown
# Feature Prioritization - [Date]

## Scored Features

| Rank | Feature | Reach | Impact | Confidence | Effort | RICE Score |
|------|---------|-------|--------|------------|--------|------------|
| 1 | | | | | | |
| 2 | | | | | | |

## Recommendations

### Must Have (This Phase)
1. [Feature] — Reason: [why now]

### Should Have (Next Phase)
1. [Feature] — Reason: [why next]

### Nice to Have (Future)
1. [Feature] — Reason: [why later]

### Won't Do (Deprioritized)
1. [Feature] — Reason: [why not]

## Dependencies Map
[Feature A] → blocks → [Feature B]
[Feature C] → requires → [Feature D]
```

## Decision Criteria Beyond RICE
- Strategic alignment: does this support our multi-vertical vision?
- Technical debt: does deferring this create problems later?
- Competitive necessity: do competitors all have this?
- Revenue impact: direct or indirect revenue effect?
