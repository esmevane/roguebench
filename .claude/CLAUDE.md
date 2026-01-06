# Project: roguebench

A game workbench for collaborative roguelike creation with non-programmers.

## Required Reading

Before any work, understand:

| Document | Purpose |
|----------|---------|
| @docs/mission.md | Goal, users, success criteria |
| @docs/glossary.md | Terms of art |
| @docs/priorities.md | Decision framework |
| @docs/stack.md | Technology choices, project structure |
| @docs/workflows.md | How users accomplish tasks |
| @docs/approach.md | Workflow-first development |

## Core Agents

| Agent | When to Use |
|-------|-------------|
| **mission-lead** | Starting work, architectural decisions, questioning priorities |
| **test-designer** | Designing features, debugging, verification strategies |
| **architect** | Patterns emerging, unclear boundaries, structural issues |

## Quality Agents

For self-review (see @docs/agents/self-review.md):
- **alignment** — Workflow and decision consistency
- **best-practices** — SOLID, hexagonal, component design
- **organizer** — Code organization and naming
- **custodian** — Technical debt and code health
- **deferral-guard** — Prevent stubs and deferrals

## Project Stack

- **Language**: Rust (Edition 2024)
- **Engine**: Bevy 0.17+
- **Networking**: Lightyear
- **Physics**: Avian 2D
- **Scripting**: mlua (Luau)
- **Editor**: axum + egui
- **Storage**: SQLite

## Key Commands

```bash
# Build and run
cargo build
cargo run -p {name}-client
cargo run -p {name}-server

# Test
cargo test
cargo clippy

# Issue tracking
/beads:ready                # Unblocked work
/beads:list --status open   # All open issues
/beads:sync                 # Sync with git
```

## Bevy Patterns

@docs/agents/bevy.md

Key principles:
- One concept = one plugin
- Command-driven architecture (all behavior through command bus)
- Message/event-driven design
- Test workflows, not implementation details
- Prefer `Single<C>` over resources

## Project-Specific Rules

### Authoring-First Design
Every architectural decision should ask: "Does this help someone create content?"

### Vertical Slices
Complete one content type end-to-end before spreading horizontally:
- Editor → Data → Runtime → Persistence → Verification

### No Deferral Without Tracking
When tempted to defer, stub, or work around:
1. Surface the tension
2. Either resolve now or reduce scope
3. If deferring, record explicitly with beads

### Testing Requirements
- Test through commands, not component inspection
- Cross-boundary observation for networked features
- Dependencies stay hidden from tests

## Session Flow

See @docs/getting-started.md for full workflow.

**Start:**
```
/beads:ready
/memory:recall --recent
Use mission-lead to assess priorities
```

**End:**
```
/self-review
/beads:sync --message "description"
/memory:remember decision "..."
```
