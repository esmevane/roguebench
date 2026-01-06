# Skill: Memory

A persistent memory system for tracking decisions, assumptions, observations, and lessons across sessions.

---

## Purpose

Sessions are ephemeral. Without persistent memory:
- Decisions get re-made differently
- Assumptions drift without notice
- Lessons learned are forgotten
- Context must be re-established every session

Memory provides institutional continuity.

## Schema

```sql
-- Flexible memory entries
CREATE TABLE memories (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    content TEXT NOT NULL,
    context TEXT,
    tags TEXT,                  -- JSON array
    confidence REAL DEFAULT 1.0,
    created_at INTEGER DEFAULT (unixepoch()),
    updated_at INTEGER DEFAULT (unixepoch()),
    superseded_by TEXT,
    FOREIGN KEY (superseded_by) REFERENCES memories(id)
);

-- Synthesized insights
CREATE TABLE insights (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    category TEXT,              -- lesson, pattern, anti_pattern, discovery
    source_memories TEXT,       -- JSON array of memory IDs
    created_at INTEGER DEFAULT (unixepoch())
);

-- Indexes for common queries
CREATE INDEX idx_memories_kind ON memories(kind);
CREATE INDEX idx_memories_created ON memories(created_at);
CREATE INDEX idx_insights_category ON insights(category);
```

## Memory Kinds

| Kind | Purpose | Example |
|------|---------|---------|
| `decision` | Choice made + rationale | "Chose mlua over Rhai because of ecosystem maturity" |
| `assumption` | Something treated as true | "Assuming hot reload latency under 100ms is acceptable" |
| `observation` | Something noticed | "The ContentRegistry pattern is used inconsistently" |
| `question` | Open question to revisit | "Should state machines support hierarchical states?" |
| `workflow` | Status of a workflow | "Enemy authoring skeleton: editor done, runtime pending" |
| `pattern` | Positive recurring pattern | "UserData wrappers work well for Lua interop" |
| `anti_pattern` | Thing that didn't work | "Building frameworks before proving integration" |
| `blocker` | Something preventing progress | "Hot reload not working for scripts" |
| *(custom)* | Emergent kinds as needed | Model can create new kinds |

## Operations

### Remember

Store a new memory:

```
/memory:remember decision "Chose SQLite over RON files for content storage because queries enable editor features"

/memory:remember assumption "Players will have stable internet for multiplayer" --confidence 0.7

/memory:remember observation "State machine tests are slower than expected" --tags "performance,testing"
```

### Recall

Search and retrieve memories:

```
/memory:recall                              # Recent memories
/memory:recall --kind decision              # All decisions
/memory:recall --tags "scripting"           # By tag
/memory:recall "hot reload"                 # Full-text search
/memory:recall --since "2024-01-01"         # By date
```

### Synthesize

Generate insights from accumulated memories:

```
/memory:synthesize                          # Synthesize recent memories
/memory:synthesize --kind observation       # Synthesize observations into patterns
```

Synthesis process:
1. Gather related memories (by kind, tags, or content similarity)
2. Identify themes and patterns
3. Generate insight with category (lesson, pattern, anti_pattern, discovery)
4. Link to source memories
5. Optionally mark source memories as synthesized

### Review

Show memories needing attention:

```
/memory:review                              # Open questions, low-confidence assumptions
/memory:review --stale                      # Old memories that might be outdated
/memory:review --contradictions             # Memories that seem to conflict
```

### Supersede

Mark a memory as replaced by a better understanding:

```
/memory:supersede <old_id> <new_id>
```

## Session Integration

### Session Start

1. Load recent insights into context
2. Load active workflow statuses
3. Load open questions
4. Surface any stale or contradictory memories

### During Work

- Remember decisions as they're made
- Remember assumptions when they're identified
- Remember observations as work progresses
- Questions can be created for later resolution

### Session End

1. Review memories created this session
2. Synthesize if enough raw memories accumulated
3. Update workflow statuses
4. Note any open questions for next session

## Agent Integration

| Agent | Memory Usage |
|-------|--------------|
| **Alignment** | Check decisions for drift, flag contradictions |
| **Best Practices** | Reference patterns and anti-patterns |
| **Deferral Guard** | Track open questions, push for resolution |
| **Organizer** | Remember organization decisions |
| **Custodian** | Track tech debt observations |
| **Self-Review** | Verify consistency with prior decisions |

## Example Session Flow

```
# Session start
/memory:recall --recent
> Recent: 3 decisions, 2 observations, 1 open question

/memory:review
> Open question: "Should scripts have access to raw entity components?"
> Low-confidence assumption: "Hot reload latency acceptable" (0.7)

# During work
/memory:remember decision "Scripts access components via typed wrappers, not raw ECS"

/memory:remember observation "Wrapper approach adds ~5% overhead but improves safety"

# Session end
/memory:synthesize
> Created insight: "Wrapper-based scripting API" (pattern)
> - Safe: prevents scripts from corrupting ECS state
> - Overhead acceptable for sandbox benefits
> - Source: 2 decisions, 1 observation
```

## Bootstrap

To set up memory in a new project:

1. Create `.claude/memory.db` (SQLite database)
2. Run schema migration
3. Add memory skill to `.claude/skills/`
4. Configure session hooks to load/save context

The memory database is gitignored by default (local to each developer), but can be committed for shared institutional memory.
