# Entity Identity Framework

Stable entity identification for persistence only.

## Scope Clarification

**Networking is handled by Lightyear.** Lightyear uses component-based replication (`Replicate`, `Replicating`, `Replicated` markers) and handles server/client entity mapping automatically. We do not need a separate ID system for networking.

**This framework is for persistence (save/load)** and is deferred until save system implementation.

## When We Need This

Bevy `Entity` IDs are transient—they change when the app restarts. For save/load, we need stable identifiers that persist across sessions.

Use cases:
- Save files reference entities by stable ID
- Loading reconstructs entity graph with correct references
- Cross-references in saved data (e.g., "this item is in that container")

## When We Don't Need This

- **Networking**: Lightyear handles entity replication
- **Runtime references**: Use Bevy `Entity` directly
- **Content references**: Use `DefinitionId` (string key into content)
- **Scripting references**: Query by component/role, not by ID

## Core Logic (Deferred)

When implemented:

**Concept**

- Entities that need persistence get a `PersistentId` component
- ID allocated on spawn if entity is saveable
- Save files serialize `PersistentId`, not Bevy `Entity`
- Load phase reconstructs `PersistentId` → `Entity` mapping

**What Gets an ID**

Not everything needs persistence:
- Players: Yes (save progress)
- Enemies: Maybe (if mid-combat save)
- Items in inventory: Yes
- Projectiles: No (transient)
- Particles: No (ephemeral)

**Deferred Design Questions**

- ID format: u64 counter? UUID? Hybrid?
- Scope: Per-save-file unique? Globally unique?
- Recycling: Reuse IDs with generation counter?

---

## Implementation Status

**Deferred.** Not needed for vertical slice (no save/load).

Revisit when implementing:
- `features/save_system.md`
- `features/checkpoints.md`

*See: architecture/data.md*
