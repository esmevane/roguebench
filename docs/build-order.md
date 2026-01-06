# Build Order

What must exist before what. This isn't a task list—it's a dependency map that helps identify blockers and valid next steps.

---

## Phase 0: Resolved Decisions

These foundational decisions have been made. Reference these when implementing dependent features.

### Scripting Language → mlua (Luau)

**Decision:** Use `mlua` crate with Luau dialect.

**Key points:**
- Luau for sandbox-friendly scripting
- Heavy use of `UserData` trait for Rust-Lua interop
- Module-first design: scripts export objects with hook handlers
- Globals namespaced by purpose (e.g., `inventory:player()`)

**Script structure:**
```lua
local module = {}

module.recipes = { { itemA = 300 } }

function module.handle_craft(craft_event)
    inventory:player(craft_event.player.id):addCurrency(craft_event.items[1])
end

return module
```

**Unblocked work:** Scripting runtime, behavior authoring, event hooks, hot-reload for logic.

### Entity Identity → SQLite + Lightyear

**Decision:** SQLite for persistence, Lightyear for networking.

**Key points:**
- SQLite stores template/prefab identity for editing
- Lightyear handles networked entity identity (don't duplicate)
- Template IDs are stable, instance IDs are session-dependent

**Unblocked work:** Save/load system, cross-session references, editor persistence.

### Content Storage → SQLite

**Decision:** SQLite as the source of truth.

**Key points:**
- Editor writes directly to SQLite (not RON files)
- Rich queries for content management
- Can store blobs for assets
- Hot reload watches for DB changes

**Unblocked work:** Editor persistence, content queries, data pipeline.

---

## Phase 1: Frameworks

These must exist before features that depend on them. Order matters—earlier items may block later ones.

### Command Bus (Foundation)

**Status:** Not started

**What's needed:**
- SendCommand/CommandExecuted pattern
- Command queue and logging
- Validation layer (reject invalid commands)
- Replay mechanism (playback from log)
- Hook system (scripts subscribe to commands)

**Blocks:**
- Scripting integration
- Replay/debugging
- Network command sync

**Dependency:** None (foundational)

### Data Pipeline

**Status:** Not started

**What's needed:**
- SQLite connection management
- Unified loading for all content types
- Schema validation
- Version migration
- Hot reload (watch for DB changes)
- Integration with editor

**Blocks:**
- Editor hot-reload integration
- New content types
- Data validation

**Dependency:** None (Content Storage decision resolved → SQLite)

### Scripting Runtime

**Status:** Not started

**What's needed:**
- mlua integration with Luau dialect
- UserData implementations for game types
- Module loader (scripts return module objects)
- Hook dispatcher (invoke `handle_*` functions)
- Global APIs namespaced by purpose
- Hot-reload for scripts (watch for changes)

**Blocks:**
- All behavior authoring
- Dynamic content
- User-created logic

**Dependency:** Command Bus (for hook events)

### State Machine Framework

**Status:** Not started

**What's needed:**
- Reusable state machine abstraction
- Data-driven state definitions (from SQLite)
- Transition hooks for scripting
- Bevy integration (component-based)

**Blocks:**
- Data-driven enemy AI
- Complex player states
- UI flow states

**Dependency:** None (can build independently)

### Persistence Framework

**Status:** Not started

**What's needed:**
- Save slot management (SQLite tables)
- Entity serialization (template ID + instance state)
- World state capture/restore
- Checkpoint support

**Blocks:**
- Save/load functionality
- Session resume
- Progress persistence

**Dependency:** Data Pipeline (SQLite infrastructure)

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
1. **Schema:** Item table in SQLite, Rust struct
2. **Editor UI:** Form for item CRUD (bevy_egui)
3. **Editor API:** REST endpoints for items (axum)
4. **Data Pipeline:** Load items from SQLite, hot-reload on change
5. **Runtime:** Spawn items, pickup handling, effect application
6. **Persistence:** Items stored in SQLite (inherent)
7. **Verification:** Tests for authoring workflow

**Completion criteria:**
- User can define item in editor
- Item spawns in game with correct properties
- Changes to item hot-reload
- Item definition persists across restarts (SQLite)

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
┌──────────────┐    ┌─────────────────┐
│ Command Bus  │───▶│ Scripting       │ (mlua/Luau)
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
│ (SQLite)     │    └─────────────────┘
└──────┬───────┘
       │
       │            ┌─────────────────┐
       └───────────▶│ Persistence     │
                    │ (SQLite)        │
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
