# Squash & Stretch

A spring-based deformation effect that compresses and stretches entities to convey weight, impact, and motion.

## Core Logic

**State**

- Current scale (vec2)
- Target scale (vec2)
- Velocity (vec2)
- Stiffness (f32)
- Damping (f32)
- Base scale (vec2)
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given current != target
- Compute spring force: (target - current) \* stiffness
- Compute damping force: -velocity \* damping
- Update velocity: velocity + (spring + damping) \* dt
- Update current: current + velocity \* dt

`impulse(scale)` - Immediate deformation

- Set current to scale
- Set target to (1, 1)

`set_target(scale)` - Gradual deformation

- Set target to scale

`squash_axis(axis, amount)` - Volume-conserving deformation

- Set target[axis] to amount
- Set target[other] to 2.0 - amount

`reset()` - Return to rest

- Set target to (1, 1)

**Invariants**

- If enabled=false, step() is no-op
- At rest: current = target, velocity = 0
- Output scale = current \* base_scale

**Presets**

| Name   | Stiffness | Damping | Character                |
| ------ | --------- | ------- | ------------------------ |
| Bouncy | 600       | 8       | Overshoots several times |
| Snappy | 800       | 25      | Fast, minimal overshoot  |
| Soft   | 200       | 12      | Slow, gentle settle      |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on all enabled instances
- Write `current * base_scale` to `Transform.scale`

**Triggers**

- Entity commands: `squash_impulse(scale)`, `squash_x(f32)`, `squash_y(f32)`
- Messages: `SquashImpulse { entity, scale }`, etc.

**Scheduling**

- Runs in a dedicated schedule
- Schedule can be invoked manually or auto-run in Update

**Lifecycle**

- Core state lives in a Component
- No cleanup required on despawn
