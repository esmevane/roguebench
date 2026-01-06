# Entity Shake

High-frequency oscillation applied to entities, typically when taking damage. Creates rapid back-and-forth movement that conveys impact.

## Core Logic

**State**

- Amplitude (f32) - current shake intensity, decays over time
- Frequency (f32) - oscillation speed in Hz
- Decay (f32) - amplitude reduction per second
- Time (f32) - accumulator for oscillation phase
- Offset (vec2) - computed displacement
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given amplitude > threshold (0.01)
- Accumulate time: time += dt
- Compute phase: time \* frequency \* TAU
- Compute offset.x: sin(phase) \* amplitude
- Compute offset.y: cos(phase \* 1.3 + 0.5) \* amplitude \* 0.8
- Decay amplitude: amplitude - (decay \* dt), min 0

`shake(amplitude, frequency, decay)` - Trigger full shake

- Set amplitude to max(current, new) - allows stacking
- Set frequency
- Set decay

`shake_simple(amplitude)` - Trigger with current settings

- Set amplitude to max(current, new)

`stop()` - Immediate stop

- Set amplitude to 0

**Invariants**

- If enabled=false, offset is zero
- If amplitude < 0.01, offset is zero and amplitude resets to 0
- X and Y oscillate at different frequencies for organic feel
- Amplitude stacks (takes max) rather than resets

**Defaults**

| Field     | Value | Description            |
| --------- | ----- | ---------------------- |
| Frequency | 30.0  | Hz, oscillations/sec   |
| Decay     | 20.0  | Amplitude lost per sec |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on all enabled instances
- Add offset to `Transform.translation` (must track and remove previous offset)

**Triggers**

- Entity commands: `entity_shake(amplitude, frequency, decay)`, `entity_shake_simple(amplitude)`
- Messages: `EntityShake { entity, amplitude, frequency, decay }`, `EntityShakeSimple { entity, amplitude }`

**Scheduling**

- Runs in a dedicated schedule
- Must apply offset after, and remove before, other transform modifications

**Lifecycle**

- Core state lives in a Component
- Offset is transient - must be cleaned up if component removed mid-shake
