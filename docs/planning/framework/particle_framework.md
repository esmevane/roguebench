# Particle Framework

Particle effect spawning, configuration, and lifecycle.

## Client-Side Execution

Particles are **purely visual** and run **entirely on the client**. They are not replicated.

```
Server: Game event (damage, death, spawn)
    ↓
Lightyear replication or message
    ↓
Client: Observes event → Spawns particles locally
```

**No particles on server.** Server doesn't spawn, track, or know about particles.

---

## Triggering Particles

Particles are triggered by observing replicated state or events:

```rust
// Client-side system
fn spawn_hit_particles(
    mut damage_events: EventReader<DamageApplied>, // Replicated event
    mut commands: Commands,
    library: Res<ParticleLibrary>,
) {
    for event in damage_events.read() {
        // Spawn particle effect at damage location
        spawn_effect(&mut commands, &library, "hit_spark", event.position);
    }
}
```

Particles are **ephemeral**—they don't go through the command bus. The server already validated the event; the client just renders feedback.

---

## Core Logic

**Concept**

- Visual effects via particle systems
- Data-driven effect definitions
- Attach to entities or world positions
- Pooling for performance

**Particle System Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Effect identifier |
| emission_rate | f32 | Particles per second |
| lifetime | range | Particle lifespan |
| spawn_shape | enum | Point, circle, rect |
| velocity | range | Initial velocity |
| acceleration | Vec2 | Constant acceleration |
| color | gradient | Color over lifetime |
| scale | curve | Size over lifetime |
| sprite | option | Particle sprite |

**Emission Modes**

| Mode | Behavior |
|------|----------|
| Continuous | Emit while active |
| Burst | Emit N particles once |
| Timed | Emit for duration |

**Operations**

`spawn(effect_id, position)` - Create at position

- Instantiate from definition
- Begin emission

`spawn_attached(effect_id, entity)` - Attach to entity

- Follow entity position
- Despawn with entity

`burst(effect_id, position, count)` - One-shot burst

- Emit count particles
- No persistent emitter

`stop(effect_handle)` - Stop emission

- Let existing particles finish

---

## Bevy Integration

**Components**

```rust
#[derive(Component)]
struct ParticleEmitter {
    definition_id: String,
    active: bool,
    timer: f32,
}

#[derive(Component)]
struct Particle {
    lifetime: f32,
    velocity: Vec2,
    // ... other per-particle state
}

#[derive(Component)]
struct AttachedToEntity(Entity);
```

**Resources**

```rust
#[derive(Resource)]
struct ParticleLibrary {
    effects: HashMap<String, ParticleDefinition>,
}
```

**Particle Definition**

```rust
struct ParticleDefinition {
    id: String,
    emission_mode: EmissionMode,
    emission_rate: f32,
    lifetime: Range<f32>,
    spawn_shape: SpawnShape,
    initial_velocity: VelocityConfig,
    acceleration: Vec2,
    color_over_lifetime: ColorGradient,
    scale_over_lifetime: Curve,
    sprite: Option<Handle<Image>>,
}

enum SpawnShape {
    Point,
    Circle { radius: f32 },
    Rectangle { size: Vec2 },
}
```

**Systems (client-side, Update schedule)**

```rust
fn emit_particles(
    time: Res<Time>,
    mut emitters: Query<(&mut ParticleEmitter, &Transform)>,
    mut commands: Commands,
    library: Res<ParticleLibrary>,
) {
    for (mut emitter, transform) in emitters.iter_mut() {
        if !emitter.active { continue; }

        let def = library.get(&emitter.definition_id)?;
        emitter.timer += time.delta_secs();
        let interval = 1.0 / def.emission_rate;

        while emitter.timer >= interval {
            emitter.timer -= interval;
            spawn_particle(&mut commands, def, transform.translation);
        }
    }
}

fn update_particles(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.lifetime -= time.delta_secs();

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Update position
        transform.translation += particle.velocity.extend(0.0) * time.delta_secs();
        particle.velocity += particle.acceleration * time.delta_secs();
    }
}
```

---

## Common Trigger Patterns

**On damage:**
```rust
fn on_damage_particles(mut events: EventReader<DamageApplied>, ...) {
    for event in events.read() {
        spawn_effect("blood_spray", event.position);
    }
}
```

**On death:**
```rust
fn on_death_particles(mut events: EventReader<EntityDied>, ...) {
    for event in events.read() {
        spawn_effect("death_explosion", event.position);
    }
}
```

**On movement (dust, trails):**
```rust
fn movement_dust(
    query: Query<(&LinearVelocity, &Transform), With<Player>>,
    ...
) {
    for (velocity, transform) in query.iter() {
        if velocity.0.length() > DUST_THRESHOLD {
            spawn_effect("run_dust", transform.translation);
        }
    }
}
```

---

## Implementation Options

Consider these crates for actual particle rendering:
- **bevy_hanabi** — GPU-accelerated particles
- **bevy_enoki** — Simpler CPU particles
- **Custom** — If needs are simple

This framework provides the pattern; choose implementation based on performance needs.

---

## What's Not Here

- Particle physics collision (add if needed)
- Particle lighting/shadows
- LOD for distant particles

*See: framework/animation_framework.md (triggers effects), framework/audio_framework.md (similar client-side pattern)*
