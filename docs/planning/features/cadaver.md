# Cadaver

Persistent death remains that can be pushed around. Enemies leave bodies when killed.

## Core Logic

**State**

- Age (f32) - how long cadaver has existed
- Lifetime (f32) - total time before despawn
- Size (f32) - collision size
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Increment age: age += dt
- If age >= lifetime, mark for removal

`remaining_fraction()` - Get lifetime remaining

- Return (lifetime - age) / lifetime, clamped 0-1

`is_fading()` - Check if in fade phase

- Return true if less than fade_duration remaining

`fade_alpha()` - Get current opacity

- Return 1.0 during most of life
- Fade to 0.0 in final phase (e.g., last 1 second)

`is_expired()` - Check if should despawn

- Return age >= lifetime

**Invariants**

- Age always increases
- Alpha is 1.0 until fade phase, then linear fade to 0
- Physics-enabled (can be pushed)
- Despawns when expired

**Defaults**

| Field    | Value | Description           |
| -------- | ----- | --------------------- |
| Lifetime | 10.0  | Seconds before remove |
| Size     | 12.0  | Collision radius      |
| Fade     | 1.0   | Seconds of fade-out   |

---

## Bevy Integration

**Spawn**

- On enemy death event
- Position: death location
- Includes: physics body, collider, sprite

**Sync**

- Each frame, call `step(delta_time)`
- Apply `fade_alpha()` to sprite alpha
- Despawn entity when `is_expired()`

**Physics**

- Dynamic rigid body (can be pushed)
- Collision with other entities
- High linear damping (slows quickly)
- Rotation locked

**Lifecycle**

- Spawned as new entity on death
- Self-manages lifetime and despawn
- No external cleanup needed
