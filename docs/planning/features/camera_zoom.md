# Camera Zoom Bump

Temporary camera zoom that springs back to baseline. Used for impact emphasis - camera briefly zooms in on hits then returns.

## Core Logic

**State**

- Current (f32) - current zoom offset from baseline
- Target (f32) - target zoom offset (usually 0)
- Velocity (f32) - for spring simulation
- Stiffness (f32) - spring force
- Damping (f32) - oscillation reduction
- Base scale (f32) - baseline zoom level
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Compute spring force: (target - current) \* stiffness
- Compute damping force: -velocity \* damping
- Update velocity: velocity + (spring + damping) \* dt
- Update current: current + velocity \* dt

`bump(amount)` - Apply zoom impulse

- Add amount to current (negative = zoom in, positive = zoom out)
- Set target to 0

`set_base(scale)` - Change baseline zoom

- Set base_scale to scale

**Invariants**

- If enabled=false, current is zero
- Output zoom = base_scale + current
- Negative amounts zoom in (smaller scale = closer)
- Springs back to target (0) after bump

**Defaults**

| Field     | Value | Description             |
| --------- | ----- | ----------------------- |
| Stiffness | 300.0 | Spring force            |
| Damping   | 15.0  | Oscillation reduction   |
| Base      | 1.0   | Default zoom level      |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on the camera zoom instance
- Apply (base_scale + current) to camera projection scale
- For OrthographicProjection: set `scale` field
- Typically one instance per camera (or global)

**Triggers**

- Commands: `camera_zoom(amount)`
- Messages: `CameraZoom { amount }`

**Scheduling**

- Runs in a dedicated schedule
- Can run alongside shake and bump

**Lifecycle**

- Can be a Resource (global) or Component (per-camera)
- Must restore base scale on disable
