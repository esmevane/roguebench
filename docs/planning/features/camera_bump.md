# Camera Bump

Directional camera displacement that decays with friction. Used for impact feedback - camera briefly shifts in hit direction then returns.

## Core Logic

**State**

- Offset (vec2) - current displacement
- Friction (f32) - decay factor, 0.0-1.0
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given offset length > threshold (0.1)
- Compute friction factor: friction^(dt \* 60) - frame-rate independent
- Apply decay: offset \*= friction_factor
- If offset length < threshold, zero it out

`bump(direction, strength)` - Apply directional bump

- Normalize direction
- Add direction \* strength to offset

`reset()` - Clear bump

- Set offset to (0, 0)

**Invariants**

- If enabled=false, offset is zero
- Direction is normalized before applying
- Multiple bumps accumulate (additive)
- Friction decay is exponential, not linear

**Defaults**

| Field    | Value | Description                        |
| -------- | ----- | ---------------------------------- |
| Friction | 0.85  | Lower = faster return              |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on the camera bump instance
- Add offset to camera `Transform.translation`
- Typically one instance per camera (or global)

**Triggers**

- Commands: `camera_bump(direction, strength)`
- Messages: `CameraBump { direction, strength }`

**Scheduling**

- Runs in a dedicated schedule
- Must apply after camera follow, can combine with camera shake

**Lifecycle**

- Can be a Resource (global) or Component (per-camera)
- Offset is transient - must restore on disable
