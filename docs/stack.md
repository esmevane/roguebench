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
| Dev console | bevy_egui | In-game dev tools (client-side) |
| Scripting | mlua | Luau dialect, module-first design |

## Tooling

| Choice | Value | Notes |
|--------|-------|-------|
| Issue tracking | beads | Git-native, AI-friendly |
| AI assistant | Claude Code | With project-specific agents |
| Web editor | axum | Embedded in game server, serves HTML forms |
| Content storage | SQLite (rusqlite) | Source of truth for all content |
| Serialization | bincode, serde | Binary serialization for SQLite blobs |

## Project Structure

```
project/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── roguebench-core/        # Platform-agnostic types (no Bevy)
│   ├── roguebench-protocol/    # Network protocol, shared components
│   ├── roguebench-storage/     # Content storage (SQLite, in-memory)
│   ├── roguebench-engine/      # Bevy plugins and systems
│   ├── roguebench-editor/      # Web editor backend
│   ├── roguebench-client/      # Game client binary
│   └── roguebench-server/      # Game server binary
├── assets/                 # Game assets (sprites, audio, data)
├── docs/                   # Project documentation
└── .claude/                # Claude Code configuration
    ├── CLAUDE.md           # Project instructions
    ├── agents/             # Specialized agents
    └── rules/              # Modular rules
```

## Architecture

### Server (roguebench-server)

The game server serves two purposes:
1. **Web Editor**: axum serves HTML forms at `localhost:8080` for content authoring
2. **Game Server**: Bevy runs headless, manages authoritative game state, Lightyear replicates to clients

The editor writes directly to SQLite. The game server reads from SQLite.

### Client (roguebench-client)

The game client:
1. **Renders**: Bevy with sprites, particles, UI
2. **Connects**: Lightyear client connects to server for game state
3. **Dev Console**: bevy_egui provides in-game debugging tools (not the content editor)

Authors use their browser for the web editor. Players use the game client to play.

---

## Key Patterns

| Pattern | Implementation | Reference |
|---------|---------------|-----------|
| Command bus | `SendCommand<C>` / `CommandExecuted<C>` | See glossary: Framework |
| Message-driven | Bevy events + observers | See CLAUDE.md: Bevy patterns |
| Hot reload | File watching + data pipeline | See patterns/content-registry.md |
| Plugin architecture | One concept = one plugin | See CLAUDE.md: Bevy code |

## Conventions

### Naming

| Thing | Convention | Example |
|-------|------------|---------|
| Crates | `roguebench-{layer}` | `roguebench-engine` |
| Plugins | `{Feature}Plugin` | `CombatPlugin` |
| Components | PascalCase noun | `Health`, `Movement` |
| Systems | snake_case verb | `apply_damage`, `spawn_enemy` |
| Events/Messages | PascalCase verb phrase | `DealDamage`, `SpawnEnemy` |

### File Organization

| Layer | Location | Contents |
|-------|----------|----------|
| Types (no Bevy) | `roguebench-core/` | Pure data structures |
| Shared components | `roguebench-protocol/` | Replicated components, messages |
| Game logic | `roguebench-engine/src/` | Plugins, systems, behaviors |
| Content | `assets/` | RON files, sprites, audio |

## Commands

Common commands for this project:

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run
cargo run -p roguebench-client     # Run game client
cargo run -p roguebench-server     # Run game server

# Test
cargo test                         # Run all tests
cargo test -p roguebench-engine    # Test specific crate

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
