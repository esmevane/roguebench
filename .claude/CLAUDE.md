# Project: Roguebench

A game workbench for collaborative roguelike creation with non-programmers.

## Required Reading

Before any work, read:
- docs/mission.md — Understand the goal (authoring tools, not a game)
- docs/glossary.md — Understand the terms (vertical slice, framework, TBD, deferral)
- docs/priorities.md — Understand how to prioritize (resolve decisions, frameworks before features, vertical over horizontal)
- docs/build-order.md — Understand dependencies and current blockers

## Current State

### Resolved Decisions

These Phase 0 decisions have been made:

**Scripting Language → mlua (Luau)**
- Use `mlua` crate with Luau dialect (sandbox-friendly)
- Lean heavily on `UserData` trait for Rust-Lua interop
- Module-first design: scripts export objects with hook handlers
- Globals namespaced by purpose (e.g., `inventory:player()`)

```lua
local module = {}

module.recipes = { { itemA = 300 } }

function module.handle_craft(craft_event)
    inventory:player(craft_event.player.id):addCurrency(craft_event.items[1])
end

return module
```

**Entity Identity → SQLite + Lightyear**
- SQLite for persistence identity (templates, prefabs, save data)
- Lightyear handles networked entity identity separately
- Template/prefab identity scheme for editor workflows

**Content Storage → SQLite**
- SQLite as the source of truth (not RON files)
- Editor writes directly to SQLite
- Can store blobs for assets
- Queries for content management

### Frameworks Status

| Framework | Status | Blocks |
|-----------|--------|--------|
| Command Bus | Not started | Scripting hooks, replay, network sync |
| Data Pipeline | Not started | Editor integration, content loading |
| Scripting Runtime | Not started | All behavior authoring |
| State Machine | Not started | Data-driven enemy AI |
| Persistence | Not started | Save/load |

### First Vertical Slice

**Target:** Items (see docs/build-order.md Phase 2)

## Project Structure

```
roguebench/
├── crates/
│   ├── roguebench-core/        # Platform-agnostic types (no Bevy)
│   ├── roguebench-protocol/    # Network protocol, shared components
│   ├── roguebench-engine/      # Bevy plugins and systems
│   ├── roguebench-editor/      # Web editor backend
│   ├── roguebench-client/      # Game client binary
│   └── roguebench-server/      # Game server binary
├── assets/                     # Game assets
├── docs/                       # Project documentation
└── .claude/                    # Claude Code configuration
```

## Key Patterns

### Command Bus

All game mutations flow through commands:
```rust
commands.send_command(DealDamage { target, amount });
// Later: CommandExecuted<DealDamage> event fires
```

### Message-Driven

Use Bevy events and observers. Systems react to events, not direct calls.

### One Plugin = One Concept

Each plugin owns one bounded concept:
```rust
pub struct CombatPlugin;
pub struct HealthPlugin;
pub struct MovementPlugin;
```

### Data-Driven Content

Content is stored in SQLite, not hardcoded:
```rust
// Content loaded from SQLite at runtime
let enemy_template = db.query_template::<Enemy>("grunt")?;
// Template has: name, health, behavior, etc.
```

Editor UI writes to SQLite, runtime reads from it. Hot reload watches for DB changes.

## Conventions

### Naming

| Thing | Convention | Example |
|-------|------------|---------|
| Crates | `roguebench-{layer}` | `roguebench-engine` |
| Plugins | `{Feature}Plugin` | `CombatPlugin` |
| Components | PascalCase noun | `Health`, `Movement` |
| Systems | snake_case verb | `apply_damage`, `spawn_enemy` |
| Events | PascalCase verb phrase | `DealDamage`, `SpawnEnemy` |

### Testing

Outside-in, test-driven:
1. Define what the user does (action)
2. Define what should happen (effect)
3. Write test asserting action → effect
4. Implementation is a black box

### Deferral

**Deferral is an anti-pattern.** When tempted to defer:
1. Stop
2. Surface the tension
3. Resolve or explicitly acknowledge cost

## Commands

```bash
# Build
cargo build
cargo build --release

# Run
cargo run -p roguebench-client
cargo run -p roguebench-server

# Test
cargo test
cargo test -p roguebench-engine

# Check
cargo check
cargo clippy

# Issues
/beads:ready                    # Unblocked work
/beads:list --status open       # All open
/beads:sync                     # Sync with git
```

## Agents

Consult agents for their domains:

| Agent | When to Use |
|-------|-------------|
| mission-lead | Starting work, priority decisions, alignment checks |
| test-designer | Designing features, verification strategies, debugging |
| architect | Pattern emergence, boundary confusion, structural issues |
| combat-system | Combat features, damage, health, abilities |
| editor-system | Web UI, API, content management |
| data-system | Persistence, serialization, content pipeline |
