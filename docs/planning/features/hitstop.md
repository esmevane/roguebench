# Hitstop

Brief game freeze on significant impacts. Creates punch and weight by pausing all motion for a few frames.

## Core Logic

**State**

- Remaining (f32) - freeze time left
- Enabled (bool)

**Operations**

`step(dt)` - Advance using real/unscaled time

- Given remaining > 0
- Decrement remaining: remaining - dt
- If remaining <= 0, effect ends

`freeze(duration)` - Trigger hitstop

- Set remaining to max(remaining, duration) - stacks by taking longer

`is_active()` - Check if frozen

- Return remaining > 0

**Invariants**

- If enabled=false, remaining is 0
- Uses real/unscaled time for countdown (otherwise never unfreezes)
- Multiple freezes take the longer duration (max, not additive)
- During freeze, game time scale is 0

**Defaults**

| Field    | Value | Description        |
| -------- | ----- | ------------------ |
| Duration | 0.05  | Typical hit freeze |

---

## Bevy Integration

**Sync**

- When active, set `Time<Virtual>` scale to 0
- Step uses `Time<Real>` (unscaled)
- When inactive, restore time scale

**Triggers**

- Commands: `hitstop(duration)`
- Messages: `Hitstop { duration }`

**Scheduling**

- Must run before systems that read delta time
- Uses real time, not virtual time

**Lifecycle**

- Typically a global Resource
- Must restore time scale on disable/cleanup
