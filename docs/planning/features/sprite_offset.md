# Sprite Offset

A visual displacement effect with friction decay. Used for attack recoil - entity briefly shifts opposite to attack direction then returns.

## Core Logic

**State**

- Offset (vec2) - current displacement
- Friction (f32) - decay factor per frame, 0.0-1.0
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given offset length > threshold (0.1)
- Compute friction factor: friction^(dt \* 60) - frame-rate independent
- Apply decay: offset \*= friction_factor
- If offset length < threshold, zero it out

`apply(direction, amount)` - Add offset

- Normalize direction
- Add direction \* amount to offset

`reset()` - Clear offset

- Set offset to (0, 0)

**Invariants**

- If enabled=false, offset is zero
- Direction is normalized before applying
- Friction is exponential decay, not linear
- Multiple applies accumulate (additive)

**Defaults**

| Field    | Value | Description                          |
| -------- | ----- | ------------------------------------ |
| Friction | 0.8   | Lower = faster decay, 0.9 = floaty   |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on all enabled instances
- Add offset to `Transform.translation` (must track and remove previous offset)

**Triggers**

- Entity commands: `sprite_offset(direction, amount)`
- Messages: `SpriteOffset { entity, direction, amount }`

**Scheduling**

- Runs in a dedicated schedule
- Must apply offset after, and remove before, other transform modifications

**Lifecycle**

- Core state lives in a Component
- Offset is transient - must be cleaned up if component removed mid-effect
