# Project: Roguebench

A game workbench for collaborative roguelike creation with non-programmers.

## Required Reading

Before any work, read:
- docs/mission.md — Understand the goal (authoring tools, not a game)
- docs/glossary.md — Understand the terms (vertical slice, framework, TBD, deferral)
- docs/priorities.md — Understand how to prioritize (resolve decisions, frameworks before features, vertical over horizontal)
- docs/build-order.md — Understand dependencies and current blockers

## Current State

### Phase 0 Blockers (TBDs)

These decisions block significant work and must be resolved:

1. **Scripting Language** — Lua vs Rhai vs WASM (blocks all scripting features)
2. **Entity Identity** — How entities are identified for persistence/networking
3. **Content Storage** — File-based vs SQLite vs hybrid

### Frameworks Status

| Framework | Status | Blocks |
|-----------|--------|--------|
| Command Bus | Partial | Scripting, replay, network sync |
| Data Pipeline | Partial (rooms only) | Editor integration, new content types |
| Scripting Runtime | Not started | All behavior authoring |
| State Machine | Ad-hoc | Data-driven enemy AI |
| Persistence | Not started | Save/load |

### First Vertical Slice

**Recommended:** Items (see docs/build-order.md Phase 2)

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

Content is defined in RON files, not hardcoded:
```rust
// assets/enemies/grunt.ron
Enemy(
    name: "Grunt",
    health: 50,
    behavior: "patrol",
)
```

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
