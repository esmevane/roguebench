# Camera Shake

Trauma-based screen shake using Perlin-style noise. Intensity is controlled by a "trauma" value that decays over time. Shake magnitude is trauma squared for non-linear feel.

## Core Logic

**State**

- Trauma (f32) - current shake intensity, 0.0-1.0
- Decay (f32) - trauma reduction per second
- Max offset (vec2) - maximum translation in pixels
- Max rotation (f32) - maximum rotation in radians
- Time (f32) - accumulator for noise sampling
- Frequency (f32) - noise sample rate
- Offset (vec2) - computed translation
- Rotation (f32) - computed rotation
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given trauma > 0
- Compute shake amount: trauma^2 (quadratic falloff)
- Accumulate time: time += dt \* frequency
- Sample noise for x, y, rotation using different offsets
- Compute offset: noise_samples \* max_offset \* shake_amount
- Compute rotation: noise_sample \* max_rotation \* shake_amount
- Decay trauma: trauma - (decay \* dt), min 0

`add_trauma(amount)` - Add shake intensity

- Set trauma to min(trauma + amount, 1.0) - clamped

`set_trauma(amount)` - Set shake intensity

- Set trauma to clamp(amount, 0.0, 1.0)

**Invariants**

- If enabled=false, offset and rotation are zero
- Trauma is clamped to 0.0-1.0
- Shake magnitude is trauma squared (small trauma = subtle, high trauma = intense)
- Noise produces smooth, organic motion (not random jitter)

**Defaults**

| Field        | Value       | Description                 |
| ------------ | ----------- | --------------------------- |
| Decay        | 1.0         | Trauma lost per second      |
| Max offset   | (10.0, 8.0) | Pixels at full trauma       |
| Max rotation | 0.05        | Radians at full trauma      |
| Frequency    | 15.0        | Noise sample rate           |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on the camera shake instance
- Apply offset to camera `Transform.translation`
- Apply rotation to camera `Transform.rotation`
- Typically one instance per camera (or global)

**Triggers**

- Commands: `camera_shake(trauma)`
- Messages: `CameraShake { trauma }`

**Scheduling**

- Runs in a dedicated schedule
- Must apply after camera follow/targeting systems

**Lifecycle**

- Can be a Resource (global) or Component (per-camera)
- Offset/rotation are transient - must restore on disable
