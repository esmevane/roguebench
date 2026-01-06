# Build Order

What must exist before what. This isn't a task list—it's a dependency map that helps identify blockers and valid next steps.

---

## Phase 0: Resolve Decisions

Before implementation can proceed, these decisions must be made. Each blocks significant work.

### Scripting Language

**Decision needed:** Lua vs Rhai vs WASM vs other

**Blocked work:**
- All scripting features
- Behavior authoring
- Event hooks
- Hot-reload for logic

**Considerations:**
| Option | Pros | Cons |
|--------|------|------|
| Lua | Mature, well-known, good Bevy bindings | Heavier runtime |
| Rhai | Rust-native, simple, good for embedding | Less mature ecosystem |
| WASM | Language-agnostic, sandboxed | Complex tooling |

**To resolve:** Build a spike with each, evaluate developer experience and performance.

### Entity Identity

**Decision needed:** How entities are identified for persistence and networking

**Blocked work:**
- Save/load system
- Cross-session references
- Networked entity sync

**Considerations:**
- UUID: Simple, globally unique, but opaque
- Incremental: Simple, human-readable, but session-dependent
- Composite: Meaningful (room:entity:instance), but complex

**To resolve:** Examine what Lightyear already provides. Design should complement, not duplicate.

### Content Storage

**Decision needed:** File-based (RON/JSON) vs SQLite vs hybrid

**Blocked work:**
- Editor persistence
- Content queries
- Multi-user editing

**Considerations:**
- Files: Simple, git-friendly, limited querying
- SQLite: Rich queries, transactions, single-file DB
- Hybrid: Files for version control, DB for runtime

**To resolve:** Consider multi-user needs. If single-author, files may suffice.

---

## Phase 1: Frameworks

These must exist before features that depend on them. Order matters—earlier items may block later ones.

### Command Bus (Foundation)

**Status:** Partially implemented

**What exists:**
- Basic SendCommand/CommandExecuted pattern
- Command queue and logging

**What's missing:**
- Validation layer (reject invalid commands)
- Replay mechanism (playback from log)
- Hook system (scripts subscribe to commands)

**Blocks:**
- Scripting integration
- Replay/debugging
- Network command sync

**Dependency:** None (foundational)

### Data Pipeline

**Status:** Partially implemented (rooms only)

**What exists:**
- Room file loading
- File change detection (hot reload for rooms)

**What's missing:**
- Unified loading for all content types
- Schema validation
- Version migration
- Integration with editor

**Blocks:**
- Editor hot-reload integration
- New content types
- Data validation

**Dependency:** Content Storage decision

### Scripting Runtime

**Status:** Not started

**What exists:** Nothing

**What's needed:**
- Language runtime integration
- Safe API surface (what scripts can access)
- Event hooks (scripts subscribe to game events)
- Hot-reload for scripts

**Blocks:**
- All behavior authoring
- Dynamic content
- User-created logic

**Dependency:** Scripting Language decision, Command Bus (hooks)

### State Machine Framework

**Status:** Ad-hoc implementations exist

**What exists:**
- Enemy AI uses state-like patterns
- No reusable framework

**What's needed:**
- Reusable state machine abstraction
- Data-driven state definitions
- Transition hooks for scripting

**Blocks:**
- Data-driven enemy AI
- Complex player states
- UI flow states

**Dependency:** None (can build independently)

### Persistence Framework

**Status:** Not started

**What exists:** Nothing

**What's needed:**
- Save slot management
- Entity serialization
- World state capture/restore
- Checkpoint support

**Blocks:**
- Save/load functionality
- Session resume
- Progress persistence

**Dependency:** Entity Identity decision, Data Pipeline

---

## Phase 2: First Vertical Slice

Pick ONE content type and build it end-to-end. This validates the entire pipeline.

**Recommended first slice:** Items

**Why Items:**
- Simple data model (name, type, effects)
- Minimal runtime behavior (spawn, pickup, use)
- Tests the full authoring pipeline
- Not blocked by state machine or AI

**Slice components:**
1. **Schema:** Item definition structure
2. **Editor UI:** Form for item CRUD
3. **Editor API:** REST endpoints for items
4. **Data Pipeline:** Load items, hot-reload on change
5. **Runtime:** Spawn items, pickup handling, effect application
6. **Persistence:** Save item definitions
7. **Verification:** Tests for authoring workflow

**Completion criteria:**
- User can define item in editor
- Item spawns in game with correct properties
- Changes to item hot-reload
- Item definition persists across restarts

---

## Phase 3: Expand Content Types

Repeat Phase 2 pattern for additional content types, in order of dependency.

### Order recommendation:

1. **Items** (Phase 2)
   - Minimal dependencies
   - Validates pipeline

2. **Enemies**
   - Depends on: State Machine framework (for AI)
   - Uses: Items (for drops)

3. **Rooms**
   - Depends on: Data Pipeline improvements
   - Uses: Enemies (spawn points), Items (placement)

4. **Scripts/Behaviors**
   - Depends on: Scripting Runtime
   - Uses: Command Bus hooks

5. **Quests**
   - Depends on: Scripting (for conditions/triggers)
   - Uses: All above (objectives involve enemies, items, rooms)

6. **Dialogues**
   - Depends on: Scripting (for conditionals)
   - Partially exists: Yarnspinner integration

---

## Phase 4: Integration & Polish

Only after core content types work end-to-end.

### Categories:

**Networking:**
- Multi-client content sync
- Collaborative editing
- Server-authoritative content validation

**Persistence:**
- Full save/load
- Checkpoints
- Progress tracking

**Observability:**
- Metrics and diagnostics
- Performance profiling
- Debug visualization

**Accessibility:**
- Consider after core workflows are stable

---

## Dependency Graph

```
                    ┌─────────────────┐
                    │ Scripting Lang  │ (Decision)
                    │    Decision     │
                    └────────┬────────┘
                             │
                             ▼
┌──────────────┐    ┌─────────────────┐
│ Command Bus  │───▶│ Scripting       │
│ (Foundation) │    │ Runtime         │
└──────┬───────┘    └────────┬────────┘
       │                     │
       │            ┌────────┴────────┐
       │            ▼                 ▼
       │    ┌─────────────┐   ┌─────────────┐
       │    │ Behaviors   │   │ Quests      │
       │    │ (Scripts)   │   │             │
       │    └─────────────┘   └─────────────┘
       │
       ▼
┌──────────────┐    ┌─────────────────┐
│ Data         │───▶│ Content Types   │
│ Pipeline     │    │ (Items, etc.)   │
└──────┬───────┘    └─────────────────┘
       │
       │            ┌─────────────────┐
       └───────────▶│ Persistence     │
                    │                 │
                    └─────────────────┘

┌──────────────┐    ┌─────────────────┐
│ State        │───▶│ Enemy AI        │
│ Machine      │    │                 │
└──────────────┘    └─────────────────┘
```

---

## How to Use This Document

### When starting work:

1. Find your work item in the phases
2. Check its dependencies
3. If dependencies aren't complete, work on those first
4. If blocked by a decision, resolve the decision

### When adding new work:

1. Identify what it depends on
2. Add it to the appropriate phase
3. Update the dependency graph if needed

### When stuck:

1. Trace back through dependencies
2. Find the earliest unresolved item
3. That's the actual blocker—work on it instead
