# Stack

Technology choices for this project. This is the "what we use" manifest—agents and humans reference this to understand the technical context.

---

## Language & Runtime

| Choice | Value | Notes |
|--------|-------|-------|
| Language | Rust | Edition 2024 |
| Build tool | Cargo | Workspace with multiple crates |
| Package registry | crates.io | Standard Rust ecosystem |

## Framework

| Choice | Value | Notes |
|--------|-------|-------|
| Game engine | Bevy 0.17+ | ECS-based, data-driven |
| Networking | Lightyear | Client prediction, server authority |
| Physics | Avian 2D | 2D collision and rigidbodies |
| Dialogue | bevy_yarnspinner | Yarn-based dialogue trees |
| Input | bevy_enhanced_input | Context-aware input mapping |
| UI (dev tools) | bevy_egui | Immediate mode UI for tooling |
| Scripting | mlua | Luau dialect, module-first design |

## Tooling

| Choice | Value | Notes |
|--------|-------|-------|
| Issue tracking | beads | Git-native, AI-friendly |
| AI assistant | Claude Code | With project-specific agents |
| Editor backend | axum | Embedded web server |
| Content storage | SQLite (rusqlite) | Source of truth for all content |
| Serialization | bincode, serde | Binary serialization for SQLite blobs |

## Project Structure

```
project/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── {name}-core/        # Platform-agnostic types (no Bevy)
│   ├── {name}-protocol/    # Network protocol, shared components
│   ├── {name}-engine/      # Bevy plugins and systems
│   ├── {name}-editor/      # Web editor backend
│   ├── {name}-client/      # Game client binary
│   └── {name}-server/      # Game server binary
├── assets/                 # Game assets (sprites, audio, data)
├── docs/                   # Project documentation
└── .claude/                # Claude Code configuration
    ├── CLAUDE.md           # Project instructions
    ├── agents/             # Specialized agents
    └── rules/              # Modular rules
```

## Key Patterns

| Pattern | Implementation | Reference |
|---------|---------------|-----------|
| Command bus | `SendCommand<C>` / `CommandExecuted<C>` | See glossary: Framework |
| Message-driven | Bevy events + observers | See CLAUDE.md: Bevy patterns |
| Hot reload | File watching + data pipeline | See build-order: Data Pipeline |
| Plugin architecture | One concept = one plugin | See CLAUDE.md: Bevy code |

## Conventions

### Naming

| Thing | Convention | Example |
|-------|------------|---------|
| Crates | `{project}-{layer}` | `glumglade-engine` |
| Plugins | `{Feature}Plugin` | `CombatPlugin` |
| Components | PascalCase noun | `Health`, `Movement` |
| Systems | snake_case verb | `apply_damage`, `spawn_enemy` |
| Events/Messages | PascalCase verb phrase | `DealDamage`, `SpawnEnemy` |

### File Organization

| Layer | Location | Contents |
|-------|----------|----------|
| Types (no Bevy) | `{name}-core/` | Pure data structures |
| Shared components | `{name}-protocol/` | Replicated components, messages |
| Game logic | `{name}-engine/src/` | Plugins, systems, behaviors |
| Content | `assets/` | RON files, sprites, audio |

## Commands

Common commands for this project:

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run
cargo run -p {name}-client     # Run game client
cargo run -p {name}-server     # Run game server

# Test
cargo test                     # Run all tests
cargo test -p {name}-engine    # Test specific crate

# Check
cargo check                    # Type check without building
cargo clippy                   # Lint check

# Issue tracking
claude                         # Start Claude Code
> /beads:ready                 # Show unblocked issues
> /beads:list                  # List all issues
> /beads:sync                  # Sync with git
```

---

## Customizing for New Projects

When using this as a starter, update:

1. **Framework section** — Your actual dependencies
2. **Project structure** — Your crate layout
3. **Conventions** — Your naming patterns
4. **Commands** — Your build/run commands

This document should reflect reality. If you change tools, update this doc.
