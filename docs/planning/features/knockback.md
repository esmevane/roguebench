# Knockback

Physics-based displacement when taking damage. Entity is pushed away from damage source with velocity that decays via friction.

## Core Logic

**State**

- Velocity (vec2) - current knockback velocity
- Friction (f32) - velocity decay factor
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given velocity length > threshold
- Apply friction: velocity \*= friction^(dt \* 60)
- If velocity length < threshold, zero it out
- Return velocity for physics integration

`apply(direction, strength)` - Apply knockback

- Normalize direction
- Add direction \* strength to velocity

`stop()` - Cancel knockback

- Set velocity to (0, 0)

**Invariants**

- If enabled=false, velocity is zero
- Direction is normalized before applying
- Multiple knockbacks accumulate (additive)
- Friction decay is exponential, frame-rate independent

**Defaults**

| Field    | Value | Description              |
| -------- | ----- | ------------------------ |
| Friction | 0.85  | Lower = faster stop      |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on all enabled instances
- Add velocity \* dt to entity position (or feed into physics engine)
- Can integrate with Avian2D or other physics via velocity/impulse

**Triggers**

- Entity commands: `knockback(direction, strength)`
- Messages: `Knockback { entity, direction, strength }`

**Scheduling**

- Runs in a dedicated schedule
- Should run before physics step if using physics engine
- Or directly modify Transform if no physics

**Lifecycle**

- Core state lives in a Component
- Velocity is transient - safe to remove component anytime
