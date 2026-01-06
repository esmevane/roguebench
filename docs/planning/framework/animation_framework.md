# Animation Framework

Sprite animation, state-driven playback, and visual feedback.

## Client-Side Rendering, Server-Side State

Animation *playback* is client-side (visual), but animation *state* is determined by replicated game state.

```
Server: State machine transition → New state replicated
    ↓
Client: Receives new state → Triggers animation change → Renders frames
```

**Key principle:** Clients don't independently decide animations. They observe replicated state (character state, velocity, etc.) and animate accordingly.

**Gameplay events (attack frames, hitboxes)** are NOT driven by animation. The server controls hitbox timing via tick-based timers. Animation visually represents what the server has already determined.

---

## Core Logic

**Concept**

- Sprites animate through frame sequences
- Animations tied to replicated entity state
- Client renders at local frame rate
- Server doesn't care about animation frames

**Animation Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Animation identifier |
| frames | list | Sprite indices |
| frame_duration | f32 | Seconds per frame (client time) |
| loop_mode | enum | Once, Loop, PingPong |

**Loop Modes**

| Mode | Behavior |
|------|----------|
| Once | Play then stop on last frame |
| Loop | Restart from beginning |
| PingPong | Reverse at ends |

**Animation Triggers**

Animations change based on replicated state:

| Replicated State | Animation |
|------------------|-----------|
| `CharacterState::Idle` | "idle" |
| `CharacterState::Moving` + `LinearVelocity` | "run" |
| `CharacterState::Attacking` | "attack" |
| `CharacterState::Stunned` | "stunned" |

---

## No Gameplay Events from Animation

**Important:** Animation frames do NOT trigger gameplay.

Wrong approach:
```rust
// DON'T DO THIS - animation frame triggers damage
if anim.frame == ATTACK_FRAME {
    apply_damage(target); // Client-driven, will desync!
}
```

Correct approach:
```rust
// Server uses tick-based timer for attack window
// animation/timer.rs on SERVER:
if attack_timer.remaining_ticks == 0 {
    command_bus.send(ApplyDamage { ... }); // Server-authoritative
}

// Client just plays animation based on replicated CharacterState
// Client sees CharacterState::Attacking, plays "attack" animation
// Visual timing may differ slightly from server - that's OK
```

---

## Bevy Integration

**Components**

```rust
#[derive(Component)]
struct SpriteAnimation {
    current: AnimationId,
    frame: usize,
    timer: f32,      // Client-local timer, uses wall time
    speed: f32,
    paused: bool,
}
```

**Resources**

```rust
#[derive(Resource)]
struct AnimationLibrary {
    animations: HashMap<AnimationId, AnimationDefinition>,
}

struct AnimationDefinition {
    id: String,
    frames: Vec<usize>,
    frame_duration: f32,  // Seconds (client frame rate)
    loop_mode: LoopMode,
}
```

**Systems (client-side, Update schedule)**

```rust
fn animate_sprites(
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut query: Query<(&mut SpriteAnimation, &mut Sprite)>,
) {
    for (mut anim, mut sprite) in query.iter_mut() {
        if anim.paused { continue; }

        let def = library.get(&anim.current)?;
        anim.timer += time.delta_secs() * anim.speed;

        while anim.timer >= def.frame_duration {
            anim.timer -= def.frame_duration;
            anim.frame += 1;

            // Handle loop
            if anim.frame >= def.frames.len() {
                match def.loop_mode {
                    LoopMode::Loop => anim.frame = 0,
                    LoopMode::Once => { 
                        anim.frame = def.frames.len() - 1; 
                        anim.paused = true; 
                    }
                    // ...
                }
            }
        }

        sprite.index = def.frames[anim.frame];
    }
}
```

**State → Animation Mapping**

```rust
fn sync_animation_to_state(
    query: Query<(&CharacterState, &mut SpriteAnimation), Changed<CharacterState>>,
    mapping: Res<StateAnimationMapping>,
) {
    for (state, mut anim) in query.iter_mut() {
        // CharacterState is replicated from server
        if let Some(animation_id) = mapping.get(state) {
            if anim.current != *animation_id {
                anim.current = *animation_id;
                anim.frame = 0;
                anim.timer = 0.0;
                anim.paused = false;
            }
        }
    }
}
```

---

## Replication

**What's replicated:**
- `CharacterState` (enum: Idle, Moving, Attacking, etc.)
- `LinearVelocity` (from Avian, affects run animation speed)

**What's NOT replicated:**
- `SpriteAnimation` (current frame, timer) — client-local

Clients derive animation from replicated state. They don't need to be frame-synced with server.

---

## Visual Events (Client-Only)

For visual feedback (dust puffs on footsteps, screen shake on attack), use client-local events:

```rust
fn spawn_footstep_dust(
    query: Query<(&SpriteAnimation, &Transform)>,
    library: Res<AnimationLibrary>,
    mut commands: Commands,
) {
    for (anim, transform) in query.iter() {
        let def = library.get(&anim.current)?;
        // Check if current frame is a "footstep" frame (defined in animation data)
        if def.footstep_frames.contains(&anim.frame) {
            spawn_dust_particle(&mut commands, transform.translation);
        }
    }
}
```

These are purely visual and don't affect gameplay.

---

## What's Not Here

- Animation blending/transitions (add when needed)
- Animation layers (e.g., upper body + lower body)
- Skeletal animation (sprite-based for now)

*See: framework/state_machine.md (drives animation state), framework/particle_framework.md (visual events)*
