# Invincibility Flicker

Rapid alpha oscillation when entity has invincibility frames. Visual feedback that entity cannot be damaged.

## Core Logic

**State**

- Frequency (f32) - blinks per second
- Time (f32) - accumulator for oscillation
- Min alpha (f32) - lowest opacity during blink
- Max alpha (f32) - highest opacity during blink
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Accumulate time: time += dt
- Compute phase: time * frequency * TAU
- Compute alpha: lerp between min and max using sin(phase)
- Return current alpha

`reset()` - Stop flickering

- Set time to 0

**Invariants**

- Oscillates smoothly between min and max alpha
- Frequency determines blink speed
- Effect only active while Invincibility component present
- Restores full alpha when invincibility ends

**Defaults**

| Field     | Value | Description           |
| --------- | ----- | --------------------- |
| Frequency | 10.0  | Blinks per second     |
| Min alpha | 0.3   | Dimmest point         |
| Max alpha | 1.0   | Brightest point       |

---

## Bevy Integration

**Sync**

- Query entities with Invincibility component
- Apply computed alpha to Sprite.color.alpha
- Restore alpha to 1.0 when Invincibility removed

**Trigger**

- Automatic when Invincibility component present
- No explicit trigger needed

**Scheduling**

- Runs each frame while invincibility active
- Must restore alpha on component removal (use RemovedComponents)

**Lifecycle**

- Stateless - can compute from Invincibility timer directly
- Or add FlickerState component alongside Invincibility
