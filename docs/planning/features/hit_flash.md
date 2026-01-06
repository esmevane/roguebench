# Hit Flash

Temporary color tint applied to entities when damaged. Commonly flashes white on hit to indicate damage.

## Core Logic

**State**

- Color (color) - flash tint color
- Remaining (f32) - time left in flash
- Duration (f32) - total flash duration
- Original color (color, optional) - stored for restoration
- Enabled (bool)

**Operations**

`step(dt)` - Advance simulation

- Given remaining > 0
- Compute intensity: remaining / duration (1.0 at start, 0.0 at end)
- Compute output color: lerp(original, flash_color, intensity)
- Decrement remaining: remaining - dt
- If remaining <= 0, restore original color

`flash(color, duration)` - Trigger flash

- Store original color if not already stored
- Set flash color
- Set duration
- Set remaining to duration

`flash_default()` - Trigger with current settings

- Set remaining to duration

**Invariants**

- If enabled=false, restore original and clear remaining
- Original color captured once at flash start, restored at end
- Intensity fades linearly from 1.0 to 0.0
- New flash during active flash updates color/duration

**Defaults**

| Field    | Value | Description        |
| -------- | ----- | ------------------ |
| Color    | White | Default flash tint |
| Duration | 0.1   | Seconds            |

---

## Bevy Integration

**Sync**

- Each frame, call `step(delta_time)` on all enabled instances
- Write computed color to `Sprite.color`
- Must store and restore original sprite color

**Triggers**

- Entity commands: `hit_flash(color, duration)`, `hit_flash_default()`
- Messages: `HitFlash { entity, color, duration }`

**Scheduling**

- Runs in a dedicated schedule
- Should run after sprite setup, before rendering

**Lifecycle**

- Core state lives in a Component
- Must restore original color if component removed mid-flash
