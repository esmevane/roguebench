# Project: roguebench

A game workbench for collaborative roguelike creation with non-programmers.

## Required Reading

Before any work, understand:
- docs/mission.md — Goal, users, success criteria
- docs/glossary.md — Precise term definitions
- docs/priorities.md — Decision framework
- docs/approach.md — Workflow-first development

## The Mission

**Primary users:** Non-technical collaborators ("the kids") who want to create game content without writing Rust.

**Success:** A non-programmer can define enemies, create rooms, script behaviors, and save/resume work.

**Guiding principle:** Does this help someone create content?

## Tech Stack

| Layer | Choice |
|-------|--------|
| Language | Rust 2024, Cargo workspace |
| Engine | Bevy 0.17+ |
| Scripting | mlua (Luau) |
| Storage | SQLite (rusqlite + refinery) |
| Networking | Lightyear |
| Editor | axum web backend |
| Physics | Avian 2D |

See docs/stack.md for full details.

## Key Patterns

| Pattern | When to Use | Reference |
|---------|-------------|-----------|
| Command Bus | Game mutations needing logging/replay/scripting | docs/patterns/command-bus.md |
| Content Registry | Loading authored content from SQLite | docs/patterns/content-registry.md |
| State Machine | Data-driven entity behaviors | docs/patterns/state-machine.md |

## Conventions

### Naming

| Thing | Convention | Example |
|-------|------------|---------|
| Crates | `roguebench-{layer}` | `roguebench-engine` |
| Plugins | `{Feature}Plugin` | `CombatPlugin` |
| Components | PascalCase noun | `Health`, `Movement` |
| Systems | snake_case verb | `apply_damage`, `spawn_enemy` |
| Events | PascalCase verb phrase | `DealDamage`, `SpawnEnemy` |

### Project Structure (Target)

```
crates/
  roguebench-core/      # Platform-agnostic types (no Bevy)
  roguebench-protocol/  # Network protocol, shared components
  roguebench-engine/    # Bevy plugins and systems
  roguebench-editor/    # Web editor backend (axum)
  roguebench-client/    # Game client binary
  roguebench-server/    # Game server binary
```

## Development Approach

### Always Start with a Walking Skeleton

The thinnest possible end-to-end implementation:
1. Editor UI (one field, one button)
2. API (one endpoint)
3. Storage (one table)
4. Runtime (one visible result)
5. Hot reload (change propagates)

Prove integration works before adding depth.

### Vertical Over Horizontal

Complete one content type end-to-end before starting another. A user who can define items (but not enemies) can still create. A user with half-built systems for everything can create nothing.

### Frameworks With Consumers

Build frameworks alongside their first consumer, not in isolation. The consumer proves the design.

## Commands

```bash
cargo build                      # Debug build
cargo test                       # Run all tests
cargo run -p roguebench-client   # Run game client (when it exists)

# Issue tracking (bd)
bd ready                         # Show unblocked issues
bd list                          # List all issues
bd show <id>                     # View issue details
bd update <id> --status X        # Update issue status
bd close <id>                    # Complete an issue
bd sync                          # Sync with git
```

## Agents

| Agent | Purpose |
|-------|---------|
| mission-lead | Direction and alignment with goals |
| test-designer | Outside-in test design |
| architect | Pattern observation and structure |

Invoke with: `Use {agent} to {task}`
