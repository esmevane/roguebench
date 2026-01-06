# Entity Interpolation

Smooth rendering of remote entities between server snapshots.

## Core Logic

**Concept**

- Server sends snapshots at 20hz (50ms apart)
- Rendering at 60fps needs positions between snapshots
- Interpolate between known positions

**Interpolation Buffer**

- Store last N snapshots
- Render "in the past" by buffer time
- Always have two snapshots to interpolate between

**Operations**

`buffer_snapshot(snapshot)` - Store received state

- Add to interpolation buffer
- Maintain chronological order
- Discard old snapshots

`get_interpolated_position(entity, render_time)` - Query

- Find snapshots bracketing render_time
- Linear interpolate between them
- Return smooth position

`calculate_render_time()` - Determine display time

- Current time minus interpolation delay
- Ensures buffer has data

**Render Time**

```
Server: [t=100] -------- [t=150] -------- [t=200]
                              ^
Client renders here: t=150 (50ms behind latest)
```

**Invariants**

- Render time always behind server time
- Buffer must contain two snapshots minimum
- Extrapolation only as fallback (jittery)
- Local player not interpolated (predicted)

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Buffer time | 100ms | Render delay |
| Buffer size | 10 | Snapshots stored |
| Extrapolation limit | 50ms | Max forward guess |

---

## Bevy Integration

**Components**

- Interpolated - marker for interpolated entities
- InterpolationBuffer { snapshots: VecDeque<(Tick, State)> }

**Resources**

- InterpolationConfig { buffer_time, extrapolation_limit }
- RenderTime(f64)

**Systems**

- Buffer incoming snapshots per entity
- Calculate render time each frame
- Query interpolated positions for rendering
- Handle buffer underrun (extrapolate or freeze)

**Visual Components**

- Separate "render transform" from "simulation transform"
- Simulation: authoritative/predicted
- Render: interpolated, used by sprite

**Extrapolation Fallback**

- If buffer underrun, extrapolate from velocity
- Clamp extrapolation duration
- Visual indicator of connection issues (optional)
