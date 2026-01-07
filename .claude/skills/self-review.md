---
name: self-review
description: Quality verification with rule of five. Runs agents to verify work quality before completion.
---

# Self-Review Skill

A structured review process that runs quality agents to verify work before completion.

## Commands

### /self-review
Run full review with all quality agents.

```
/self-review
/self-review --quick          # Critical checks only
/self-review --focus <agent>  # Run specific agent only
```

## The Rule of Five

**Before marking work complete, run self-review five times in sequence.**

Why five?
1. First pass catches obvious issues
2. Second pass catches issues revealed by first fixes
3. Third pass catches patterns across issues
4. Fourth pass verifies fixes didn't introduce new problems
5. Fifth pass confirms stability

If the fifth pass finds new issues, continue until a pass is clean.

## Review Sequence

Each pass runs agents in this order:

1. **Alignment** - Is this work aligned with mission and decisions?
2. **Best Practices** - Does the design follow principles?
3. **Organizer** - Is the code in the right place?
4. **Custodian** - Is there technical debt?
5. **Deferral Guard** - Are there hidden deferrals?

## Severity Levels

- **Critical** - Must fix before proceeding
- **High** - Should fix before committing
- **Low** - Document if not fixing

## Completion Criteria

Work is complete when:
- All critical issues resolved
- All high issues resolved or explicitly accepted
- Low issues documented if not fixed
- Final pass is clean (no new issues)
- Memory updated with decisions and observations

## Quick Review Mode

For frequent checks:
```
/self-review --quick
```

Runs only:
- Deferral Guard (always)
- Custodian (critical issues only)
- Alignment (mission drift only)

## Lifecycle Triggers

| Moment | Depth | Notes |
|--------|-------|-------|
| Session start | Quick | Verify context |
| Before commit | Standard | Full pass |
| Before milestone | Rule of Five | Full depth |
| On uncertainty | Focused | Relevant agents only |
