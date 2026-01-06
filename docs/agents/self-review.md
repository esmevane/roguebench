# Skill: Self-Review

A structured review process that runs agents to verify work quality before completion.

---

## Purpose

Work that feels "done" often isn't:
- Organization has drifted
- Technical debt accumulated
- Alignment with mission unclear
- Best practices violated
- Deferrals hiding in the code

Self-review catches these issues before they compound.

## The Rule of Five

**Before marking work complete, run self-review five times in sequence.**

Why five?
- First pass catches obvious issues
- Second pass catches issues revealed by first fixes
- Third pass catches patterns across issues
- Fourth pass verifies fixes didn't introduce new problems
- Fifth pass confirms stability

If the fifth pass finds new issues, continue until a pass is clean.

## Process

### Invocation

```
/self-review                    # Run full review
/self-review --quick            # Run only critical checks
/self-review --focus alignment  # Run specific agent only
```

### Review Sequence

Each review pass runs agents in this order:

1. **Alignment** — Is this work aligned with mission and decisions?
2. **Best Practices** — Does the design follow principles?
3. **Organizer** — Is the code in the right place?
4. **Custodian** — Is there technical debt?
5. **Deferral Guard** — Are there hidden deferrals?

### Pass Results

Each agent reports:
- Issues found
- Severity (critical/high/low)
- Suggestions

### Completion Criteria

Work is complete when:
- [ ] All critical issues resolved
- [ ] All high issues resolved or explicitly accepted
- [ ] Low issues documented if not fixed
- [ ] Final pass is clean (no new issues)
- [ ] Memory updated with decisions and observations

## Lifecycle Triggers

Self-review runs at key moments:

| Moment | Depth | Notes |
|--------|-------|-------|
| **Session start** | Quick | Verify context, check for stale items |
| **Before commit** | Standard | Full pass, fix before committing |
| **Before milestone** | Rule of Five | Full depth, ensure quality |
| **On uncertainty** | Focused | Run relevant agent(s) only |

## Output Format

```
## Self-Review Pass 1/5

### Alignment Agent
✓ Work aligns with mission
✓ Consistent with prior decisions
⚠ Workflow coherence: No end-to-end test for this path

### Best Practices Agent
✓ SOLID compliance
⚠ Dependency Inversion: Direct dependency on concrete AudioPlayer

### Organizer Agent
✓ Files in correct locations
✓ Naming consistent

### Custodian Agent
⚠ 2 TODO comments introduced
⚠ Missing error handling in script validation

### Deferral Guard
✗ CRITICAL: Stub function with todo!() in production path

---

**Pass 1 Summary:**
- 1 critical (must fix)
- 3 warnings (should fix)
- 0 clean

**Action Required:**
Fix critical issue before continuing to Pass 2.
```

## Integration with Memory

During self-review:
- Check memories for relevant decisions
- Note new observations
- Record resolution of issues
- Update workflow status

After self-review:
```
/memory:remember observation "Self-review found recurring audio dependency issue"
/memory:remember decision "Accepted low-severity TODO in test helper"
```

## Configuration

In `.claude/settings.json`:

```json
{
  "self_review": {
    "pre_commit": true,
    "rule_of_five_triggers": ["milestone", "release"],
    "quick_triggers": ["session_start"],
    "agents": ["alignment", "best-practices", "organizer", "custodian", "deferral-guard"]
  }
}
```

## Quick Review Mode

For frequent checks, quick mode runs subset:

```
/self-review --quick
```

Quick mode checks:
- Deferral Guard (always)
- Custodian (critical issues only)
- Alignment (mission drift only)

Takes ~30 seconds instead of full review.

## Focused Review Mode

When you know what to check:

```
/self-review --focus best-practices
/self-review --focus alignment,custodian
```

Runs only specified agents.

## Example Full Session

```
# Work completed, ready for review

/self-review

## Pass 1/5
[Finds 1 critical, 2 warnings]

# Fix critical issue
[Make changes]

/self-review

## Pass 2/5
[Finds 0 critical, 2 warnings]

# Fix warnings
[Make changes]

/self-review

## Pass 3/5
[Finds 0 critical, 1 warning - new issue from fix]

# Fix new warning
[Make changes]

/self-review

## Pass 4/5
[Clean - no issues]

/self-review

## Pass 5/5
[Clean - no issues]

✓ Self-review complete. Work is ready for commit.

# Update memory
/memory:remember decision "Completed enemy spawning system with data-driven approach"
```
