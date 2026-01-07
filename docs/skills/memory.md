---
name: memory
description: Persistent context across sessions. Track decisions, assumptions, observations, and lessons.
---

# Memory Skill

Manage persistent memory for tracking decisions, observations, and learnings across sessions.

## Commands

### remember
Store a new memory entry.

```
/memory:remember <kind> "<content>" [--confidence N] [--tags "tag1,tag2"]
```

Kinds: decision, assumption, observation, question, workflow, pattern, anti_pattern, blocker

Examples:
- `/memory:remember decision "Chose SQLite over RON for content storage"`
- `/memory:remember assumption "Players have stable internet" --confidence 0.7`
- `/memory:remember observation "State machine tests are slower than expected" --tags "performance"`

### recall
Search and retrieve memories.

```
/memory:recall [query] [--kind X] [--tags X] [--recent] [--since DATE]
```

Examples:
- `/memory:recall` - Recent memories
- `/memory:recall --kind decision` - All decisions
- `/memory:recall "hot reload"` - Full-text search

### review
Show memories needing attention.

```
/memory:review [--stale] [--contradictions]
```

### synthesize
Generate insights from accumulated memories.

```
/memory:synthesize [--kind X]
```

## Storage

Memories are stored in `.claude/memory.db` (SQLite). This database is typically gitignored for per-developer memory, but can be committed for shared institutional memory.

## Session Integration

**At session start:**
- Run `/memory:recall --recent` to load context
- Run `/memory:review` to surface open questions

**During work:**
- Remember decisions as they're made
- Remember observations as work progresses

**At session end:**
- Synthesize if enough memories accumulated
- Note open questions for next session
