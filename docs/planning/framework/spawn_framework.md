# Spawn Framework

Entity spawning from archetypes, with support for procedural generation.

## Relationship to Other Systems

- **Lightyear**: Server spawns entities with `Replicate` component; Lightyear syncs to clients
- **Avian**: Spawned entities get physics components (colliders, rigid bodies) as needed
- **bevy_rand**: Procedural spawning uses seeded RNG for determinism
- **Command Bus**: Spawn events flow through command bus for scripting hooks

## Core Concepts

### Archetypes, Not Prefabs

An **archetype** defines what components an entity has and their default values. It's a template, not a pre-placed instance.

```rust
struct EnemyArchetype {
    id: String,           // "grunt", "archer", "brute"
    health: i32,
    speed: f32,
    collider_size: Vec2,
    ai_behavior: AiBehavior,
    // ...
}
```

Archetypes are data-driven (loaded from RON/JSON files). Spawning takes an archetype + position + modifiers.

### Spawn Strategies

| Strategy | Description | Use Case |
|----------|-------------|----------|
| Direct | Spawn at specific position | Scripted events, boss arenas |
| Procedural | Position determined by generation rules | Regular room encounters |
| Pattern | Spawn multiple in formation | Wave spawning |

For procedural generation, the **generation system** determines what and where to spawn. The spawn framework just executes the spawn.

### No Fixed Spawn Points (Usually)

Fixed spawn points are rare in procedural generation. Instead:

1. Room generator produces a tile grid
2. Spawn density rules determine entity counts
3. Valid spawn positions calculated from walkable tiles
4. Entities placed using seeded RNG

For authored content (boss arenas, tutorial rooms), fixed spawn points still work.

---

## Core Logic

**Spawn Properties**

| Property | Type | Description |
|----------|------|-------------|
| archetype_id | String | Which archetype to spawn |
| position | Vec2 | World position |
| modifiers | Vec | Spawn-time adjustments |
| rng_seed | Option | For deterministic spawning |

**Operations**

`spawn(archetype_id, position, modifiers)` - Create entity

- Load archetype definition
- Build entity with components
- Apply modifiers
- Add `Replicate` for networking
- Emit spawn event

`spawn_batch(requests)` - Spawn multiple entities

- Process in deterministic order
- Use provided RNG seeds for consistency

`despawn(entity, reason)` - Remove entity

- Emit despawn event
- Lightyear handles network cleanup

**Modifiers**

```rust
enum SpawnModifier {
    ScaleDifficulty(f32),    // Adjust stats by difficulty
    AddEffect(EffectId),      // Apply status effect on spawn
    SetTeam(Team),            // Assign team affiliation
    OverrideComponent(Box<dyn Component>), // Replace default component
}
```

**Invariants**

- All spawns go through framework (for event consistency)
- Server spawns, Lightyear replicates
- Deterministic spawning requires explicit RNG seed

---

## Bevy Integration

**Components**

```rust
/// Marker indicating what archetype this entity came from
struct SpawnedFrom(String);

/// Marker for entities that despawn when room clears
struct DespawnOnRoomClear;

/// Marker for entities that despawn after duration
struct DespawnAfter(Timer);
```

**Resources**

```rust
struct ArchetypeRegistry {
    archetypes: HashMap<String, EntityArchetype>,
}

struct SpawnQueue {
    pending: VecDeque<SpawnRequest>,
}
```

**Spawn Request**

```rust
struct SpawnRequest {
    archetype_id: String,
    position: Vec2,
    modifiers: Vec<SpawnModifier>,
}
```

**Systems**

```rust
/// Runs in FixedMain, processes spawn queue
fn process_spawn_queue(
    mut commands: Commands,
    mut queue: ResMut<SpawnQueue>,
    archetypes: Res<ArchetypeRegistry>,
    mut events: EventWriter<EntitySpawned>,
) {
    for request in queue.pending.drain(..) {
        let archetype = archetypes.get(&request.archetype_id)?;
        
        let entity = commands.spawn((
            SpawnedFrom(request.archetype_id.clone()),
            Transform::from_translation(request.position.extend(0.0)),
            Replicate::default(), // Lightyear replication
            // ... components from archetype
        )).id();

        // Apply modifiers
        for modifier in request.modifiers {
            modifier.apply(&mut commands.entity(entity));
        }

        events.send(EntitySpawned { 
            entity, 
            archetype_id: request.archetype_id,
            position: request.position,
        });
    }
}
```

**Events**

```rust
struct EntitySpawned { 
    entity: Entity, 
    archetype_id: String,
    position: Vec2,
}

struct EntityDespawned { 
    entity: Entity, 
    reason: DespawnReason,
}

enum DespawnReason {
    Killed,
    RoomCleared,
    TimedOut,
    Scripted,
}
```

---

## Procedural Generation Integration

The spawn framework doesn't know about room generation. It receives spawn requests from the generation system.

```
Generation Rules → Room Generator → Spawn Requests → Spawn Framework
     (data)         (bevy_rand)        (queue)         (execution)
```

**Example Flow:**

1. Server starts new room
2. Generation system reads density rules: "3-5 grunts, 0-1 archer"
3. Generation system picks valid positions using seeded RNG
4. Generation system queues spawn requests
5. Spawn framework processes queue
6. Lightyear replicates spawned entities to clients

**Seed Distribution:**

For client-side prediction (if needed), server can send generation seed. Client regenerates same spawn pattern locally. But typically, server just spawns and replicates—simpler.

---

## Networking

**Server Authority:**
- Server processes spawn queue
- Server adds `Replicate` component
- Lightyear syncs entity to clients

**No spawn commands over network.** Clients don't send "spawn enemy" requests. They receive replicated entities.

**Exception:** Client-owned entities (player's projectiles in some designs). But for server-authoritative, server spawns everything.

---

## What's Not Here

- Object pooling (optimization, add when needed)
- Spawn animations/effects (handled by visual systems observing SpawnedFrom)
- Fixed spawn point registry (add if/when authoring fixed encounters)

*See: framework/data_loading.md, features/wave_system.md, features/arena_generation.md*
