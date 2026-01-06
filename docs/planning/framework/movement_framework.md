# Movement Framework

High-level movement patterns built on Avian 2D physics.

## Relationship to Avian 2D

**Avian 2D handles physics.** We don't implement custom velocity integration or collision resolution. This framework describes:

1. How input translates to Avian velocity
2. Movement modifiers (slow, haste, knockback)
3. Movement states (locked, dashing)
4. Server authority model

```
Input (bevy_enhanced_input)
    ↓ via Lightyear
Server receives input
    ↓
Movement Framework (this doc)
    ↓ sets LinearVelocity
Avian 2D (physics)
    ↓ resolves collisions
Lightyear replicates result
```

## Server Authority

Movement runs on **server only** in `FixedMain` schedule.

- Server: Receives input via Lightyear, sets Avian velocities, Avian resolves physics
- Client: Receives replicated `Transform` and `LinearVelocity` via lightyear_avian2d
- Client: May run local prediction using same Avian setup, but server is authoritative

---

## Avian Components We Use

Movement uses Avian's built-in components:

```rust
commands.spawn((
    RigidBody::Dynamic,
    LinearVelocity(Vec2::ZERO),
    AngularVelocity(0.0),
    Collider::circle(player_radius),
    // Our custom components on top:
    MovementConfig { max_speed: 200.0, acceleration: 1000.0 },
    Player,
));
```

We set `LinearVelocity`. Avian handles integration and collision.

---

## Core Logic

**Movement Properties**

Our custom component layered on Avian:

```rust
#[derive(Component)]
struct MovementConfig {
    max_speed: f32,        // Speed cap
    acceleration: f32,     // How fast to reach max speed
    friction: f32,         // Deceleration when no input
}
```

**Movement Modes**

| Mode | Behavior | Avian Config |
|------|----------|--------------|
| Normal | 8-directional, friction | `RigidBody::Dynamic`, `LinearDamping` |
| Locked | Cannot move | Set `LinearVelocity` to zero, ignore input |
| Dashing | Override velocity, ignore input | Set `LinearVelocity` directly |
| Knockback | Impulse, then recover | `ExternalImpulse` component |

---

## Input to Velocity

Server receives input via Lightyear, translates to velocity:

```rust
fn apply_movement_input(
    mut query: Query<(&PlayerInput, &MovementConfig, &mut LinearVelocity), Without<MovementLocked>>,
) {
    // Runs in FixedMain on server
    for (input, config, mut velocity) in query.iter_mut() {
        if input.direction != Vec2::ZERO {
            // Accelerate toward input direction
            let target_velocity = input.direction.normalize() * config.max_speed;
            velocity.0 = velocity.0.move_towards(target_velocity, config.acceleration);
        } else {
            // Apply friction when no input
            velocity.0 = velocity.0.move_towards(Vec2::ZERO, config.friction);
        }
        
        // Clamp to max speed
        velocity.0 = velocity.0.clamp_length_max(config.max_speed);
    }
}
```

Avian's `LinearDamping` can also handle friction if preferred.

---

## Movement Modifiers

Modifiers affect movement without replacing the core system:

```rust
#[derive(Component)]
struct MovementModifiers {
    modifiers: Vec<MovementModifier>,
}

struct MovementModifier {
    source: Entity,
    kind: ModifierKind,
    remaining_ticks: Option<u64>, // Tick-based duration
}

enum ModifierKind {
    SpeedMultiplier(f32), // 0.5 = half speed
    Root,                  // Cannot move
}
```

**Applying modifiers:**

```rust
fn calculate_effective_speed(base: f32, modifiers: &MovementModifiers) -> f32 {
    let mut speed = base;
    
    for modifier in &modifiers.modifiers {
        match modifier.kind {
            ModifierKind::SpeedMultiplier(mult) => speed *= mult,
            ModifierKind::Root => speed = 0.0,
        }
    }
    
    speed.max(0.0)
}
```

---

## Knockback and Impulses

Use Avian's `ExternalImpulse` for instant forces:

```rust
fn apply_knockback(
    mut commands: Commands,
    mut events: EventReader<KnockbackEvent>,
) {
    for event in events.read() {
        commands.entity(event.target).insert(
            ExternalImpulse::new(event.direction * event.force)
        );
    }
}
```

Avian applies the impulse and it decays naturally via damping.

---

## Movement Lock

Prevent movement during stuns, cutscenes, etc:

```rust
#[derive(Component)]
struct MovementLocked;

fn enforce_movement_lock(
    mut query: Query<&mut LinearVelocity, With<MovementLocked>>,
) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec2::ZERO;
    }
}
```

Systems that read input should check for `Without<MovementLocked>`.

---

## Dashing

Override velocity for a fixed duration:

```rust
#[derive(Component)]
struct Dashing {
    direction: Vec2,
    speed: f32,
    remaining_ticks: u64,
}

fn process_dash(
    mut query: Query<(&Dashing, &mut LinearVelocity)>,
) {
    for (dash, mut velocity) in query.iter_mut() {
        velocity.0 = dash.direction * dash.speed;
    }
}

fn tick_dash(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Dashing)>,
) {
    for (entity, mut dash) in query.iter_mut() {
        if dash.remaining_ticks > 0 {
            dash.remaining_ticks -= 1;
        } else {
            commands.entity(entity).remove::<Dashing>();
        }
    }
}
```

---

## Replication

Avian components replicated via lightyear_avian2d:

- `Transform` — position
- `LinearVelocity` — current velocity
- `Rotation` — facing (if relevant)

`MovementConfig` is static per archetype, so it's spawned identically on server and client.

`MovementModifiers` should be replicated if clients need to display them (e.g., slow effect icon).

---

## Events

```rust
enum MovementEvent {
    Started { entity: Entity },
    Stopped { entity: Entity },
    Locked { entity: Entity },
    Unlocked { entity: Entity },
    DashStarted { entity: Entity, direction: Vec2 },
    DashEnded { entity: Entity },
    KnockbackApplied { entity: Entity, direction: Vec2, force: f32 },
}
```

---

## What's Not Here

- Path following (separate navigation system)
- AI movement patterns (uses state machine + this framework)
- Complex terrain interaction (Avian handles slopes, etc.)

*See: framework/collision_framework.md (Avian collision), framework/state_machine.md (AI uses movement)*
