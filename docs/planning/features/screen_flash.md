# Screen Flash

Full-screen color overlay that fades out. Used for significant events - player damage (red), explosions (white), transitions.

## Core Logic

**State**

- Color (color) - flash color including alpha
- Remaining (f32) - time left in flash
- Duration (f32) - total flash duration
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given remaining > 0
- Compute intensity: remaining / duration (1.0 at start, 0.0 at end)
- Compute output alpha: color.alpha \* intensity
- Decrement remaining: remaining - dt
- If remaining <= 0, set alpha to 0

`flash(color, duration)` - Trigger flash

- Set color (including base alpha)
- Set duration
- Set remaining to duration

**Invariants**

- If enabled=false, alpha is 0
- Alpha fades linearly from initial to 0
- Color's base alpha is the peak intensity
- Covers entire screen at UI layer

**Defaults**

| Field    | Value | Description    |
| -------- | ----- | -------------- |
| Duration | 0.15  | Seconds        |

**Common colors**

| Use case      | Color                  |
| ------------- | ---------------------- |
| Player damage | rgba(1.0, 0.3, 0.3, 0.6) |
| Explosion     | rgba(1.0, 1.0, 0.8, 0.4) |
| Heal          | rgba(0.3, 1.0, 0.3, 0.4) |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on the screen flash instance
- Render a full-screen quad with computed color
- Quad must be in screen space, above game world, below UI

**Triggers**

- Commands: `screen_flash(color, duration)`
- Messages: `ScreenFlash { color, duration }`

**Scheduling**

- Runs in a dedicated schedule
- Rendering is separate from logic

**Lifecycle**

- Typically a single Resource + overlay entity
- Overlay entity can be spawned once at setup
- Must handle overlay visibility (hidden when alpha = 0)
