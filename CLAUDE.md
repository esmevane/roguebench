# Roguebench

A game workbench for collaborative roguelike creation with non-programmers.

## Quick Reference

| Topic | Document |
|-------|----------|
| Goal and users | docs/mission.md |
| Technology stack | docs/stack.md |
| Terms and definitions | docs/glossary.md |
| Development approach | docs/approach.md |
| Team structure | docs/roles.md |
| Session workflow | docs/getting-started.md |

## Project Structure

```
roguebench/
├── crates/
│   ├── roguebench-core/        # Platform-agnostic types (no Bevy)
│   ├── roguebench-protocol/    # Network protocol, shared components
│   ├── roguebench-engine/      # Bevy plugins and systems (future)
│   ├── roguebench-editor/      # Web editor backend (future)
│   ├── roguebench-client/      # Game client binary (future)
│   └── roguebench-server/      # Game server binary (future)
├── assets/                     # Game assets (future)
├── docs/                       # Project documentation
└── .claude/                    # Claude Code configuration
    ├── agents/                 # Specialized agents
    └── skills/                 # Custom skills
```

## Key Principles

1. **Workflows First** — Start from user capability, build end-to-end
2. **Walking Skeletons** — Prove integration early with thin vertical slices
3. **Frameworks With Consumers** — Build frameworks alongside their first use case
4. **No Deferral** — Surface tensions, don't bury them

## Rust Patterns

- Edition 2024, workspace with multiple crates
- `roguebench-core` has no Bevy dependency
- Prefer `thiserror` for error types, `anyhow` at boundaries
- Use `serde` with derive for all data types

## Bevy Patterns

- One concept = one plugin
- Components are data, systems are behavior
- Use observers for event-driven logic
- Plugins in `roguebench-engine`, shared components in `roguebench-protocol`

## Naming Conventions

| Thing | Convention | Example |
|-------|------------|---------|
| Crates | `roguebench-{layer}` | `roguebench-engine` |
| Plugins | `{Feature}Plugin` | `CombatPlugin` |
| Components | PascalCase noun | `Health`, `Movement` |
| Systems | snake_case verb | `apply_damage`, `spawn_enemy` |
| Events | PascalCase verb phrase | `DealDamage`, `SpawnEnemy` |

## Commands

```bash
cargo check                        # Type check
cargo build                        # Build all
cargo test                         # Run tests
cargo clippy                       # Lint
cargo run -p roguebench-client     # Run client (when exists)
cargo run -p roguebench-server     # Run server (when exists)
```

## Issue Tracking

Use beads for issue tracking:

```
/beads:ready                       # Unblocked work
/beads:list --status open          # All open issues
/beads:show {id}                   # Issue details
/beads:update {id} --status X      # Update status
/beads:sync --message "X"          # Sync with git
```

## Session Workflow

1. **Start**: Check `/beads:ready`, `/journal:recall --recent`
2. **Work**: Pick item, verify workflow alignment, implement test-first
3. **End**: `/self-review`, commit, update beads, `/journal:remember`

## Agents

Specialized agents in `.claude/agents/`:

- **mission-lead** — Overall coordination, priority, alignment
- **architect** — Pattern observation, structural guidance
- **test-designer** — Test strategy, verification approach
- **best-practices** — SOLID, hexagonal, component design
- **bevy** — Bevy-specific patterns and guidance
- **deferral-guard** — Prevent stubs and deferrals
- **custodian** — Technical debt and code health
- **organizer** — Code organization and naming
- **alignment** — Workflow and decision consistency
