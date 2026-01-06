# Slow Motion

Temporary time slowdown for dramatic moments. Unlike hitstop which freezes completely, slow motion reduces time scale to a fraction.

## Core Logic

**State**

- Factor (f32) - target time scale multiplier (0.0-1.0)
- Remaining (f32) - slow-mo time left
- Ease in (f32) - transition time to reach target
- Ease out (f32) - transition time to return to normal
- Current (f32) - current interpolated factor
- Enabled (bool)

**Operations**

`step(dt)` - Advance using real/unscaled time

- Given remaining > 0
- Compute phase: easing in, holding, or easing out
- Interpolate current toward target (ease in) or toward 1.0 (ease out)
- Decrement remaining

`trigger(factor, duration)` - Start slow motion

- Set factor (target time scale)
- Set remaining to duration

`get_scale()` - Get current time scale

- Return current

**Invariants**

- If enabled=false, current is 1.0 (normal speed)
- Uses real/unscaled time for duration tracking
- Smooth transitions in and out (not jarring)
- Factor of 0.3 means 30% speed (slower = more dramatic)
- Can be interrupted by new trigger

**Defaults**

| Field    | Value | Description             |
| -------- | ----- | ----------------------- |
| Factor   | 0.3   | 30% speed               |
| Ease in  | 0.05  | Seconds to reach target |
| Ease out | 0.1   | Seconds to return       |

---

## Bevy Integration

**Sync**

- Apply current to `Time<Virtual>` scale
- Step uses `Time<Real>` (unscaled)
- Multiplies with other time scale effects (hitstop takes priority)

**Triggers**

- Commands: `slow_motion(factor, duration)`
- Messages: `SlowMotion { factor, duration }`

**Scheduling**

- Must run before systems that read delta time
- Coordinate with hitstop (hitstop = 0 overrides slow-mo)

**Lifecycle**

- Typically a global Resource
- Must restore time scale on disable/cleanup
