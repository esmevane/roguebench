---
name: journal
description: Persistent context across sessions. Track decisions, assumptions, observations, and lessons.
---

# Journal Skill

Manage persistent journal for tracking decisions, observations, and learnings across sessions.

## Commands

### remember
Store a new journal entry.

```
/journal:remember <kind> "<content>" [--confidence N] [--tags "tag1,tag2"]
```

Kinds: decision, assumption, observation, question, workflow, pattern, anti_pattern, blocker

Examples:
- `/journal:remember decision "Chose SQLite over RON for content storage"`
- `/journal:remember assumption "Players have stable internet" --confidence 0.7`
- `/journal:remember observation "State machine tests are slower than expected" --tags "performance"`

### recall
Search and retrieve journal entries.

```
/journal:recall [query] [--kind X] [--tags X] [--recent] [--since DATE]
```

Examples:
- `/journal:recall` - Recent entries
- `/journal:recall --kind decision` - All decisions
- `/journal:recall "hot reload"` - Full-text search

### review
Show entries needing attention.

```
/journal:review [--stale] [--contradictions]
```

### synthesize
Generate insights from accumulated entries.

```
/journal:synthesize [--kind X]
```

## Storage

Journal entries are stored in `.claude/journal.db` (SQLite). This database is typically gitignored for per-developer context, but can be committed for shared institutional memory.

## Session Integration

**At session start:**
- Run `/journal:recall --recent` to load context
- Run `/journal:review` to surface open questions

**During work:**
- Remember decisions as they're made
- Remember observations as work progresses

**At session end:**
- Synthesize if enough entries accumulated
- Note open questions for next session
