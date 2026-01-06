# Data Architecture

Constraints and patterns for game data to ensure compatibility with persistence, networking, and content authoring tools.

## Principles

1. **Serializable by default** - All game data must serialize/deserialize cleanly
2. **ID-referenced** - Entities referenced by stable IDs, not runtime handles
3. **Schema-driven** - Data structures defined in shareable schemas
4. **Versionable** - Support migrations as schemas evolve
5. **CRUD-compatible** - Data can be created, read, updated, deleted via external tools

## Data Categories

| Category | Persistence | Network Sync | CRUD Editable |
|----------|-------------|--------------|---------------|
| Definitions | Asset files | On connect | Yes (content tools) |
| Runtime state | Save files | Per-tick | No |
| Configuration | Config files | On connect | Yes (settings) |
| Player progress | Save files | On connect | Limited |

## ID Patterns

**Stable IDs**
- Content uses string or UUID identifiers
- Runtime maps IDs to entity handles
- Network transmits IDs, not handles
- Saves store IDs, reconstruct on load

**ID Types**
- `DefinitionId` - References prefabs, items, enemies
- `InstanceId` - Unique per spawned entity
- `PlayerId` - Unique per player/session

## Serialization

**Format Requirements**
- Human-readable for content (RON, JSON, YAML)
- Compact binary for network/saves (bincode, MessagePack)
- Same logical structure, different encodings

**Schema Definition**
- Rust structs as source of truth
- Derive macros for serialization
- Generate schemas for external tools

## CRUD Integration

**Content Server**
- External service manages definitions
- Game fetches on startup or hot-reloads
- Editors push changes, game pulls updates

**Data Flow**
```
Content Editor → CRUD Server → Game Client
                     ↓
              Database (prefabs, items, quests)
```

## Versioning

**Migration Strategy**
- Schema version embedded in data
- Migration functions for version upgrades
- Graceful handling of unknown fields

## Constraints for Features

Features referencing data should:
- Use stable IDs for cross-references
- Implement Serialize/Deserialize
- Avoid runtime-only state in persisted data
- Document their data schemas
- Consider network bandwidth for synced data
