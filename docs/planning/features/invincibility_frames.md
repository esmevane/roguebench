# Invincibility Frames

Temporary damage immunity after taking damage or during certain actions. Prevents rapid consecutive damage.

## Core Logic

**State**

- Duration (f32) - total invincibility time
- Timer (f32) - remaining time

**Operations**

`grant(duration)` - Start invincibility

- Set timer to duration

`tick(dt)` - Advance timer

- Decrement timer by dt
- If timer <= 0, invincibility ends

`is_active()` - Check if invincible

- Return timer > 0

**Invariants**

- Entity cannot take damage while active
- New grant extends or resets timer (take max)
- Automatically removed when timer expires

**Triggers**

| Trigger       | Duration | Description           |
| ------------- | -------- | --------------------- |
| Take damage   | 0.2s     | Post-hit immunity     |
| Dash          | Dash duration | Full dash immunity |

**Defaults**

| Field            | Value | Description       |
| ---------------- | ----- | ----------------- |
| Damage immunity  | 0.2   | Seconds           |

---

## Bevy Integration

**Component**

- Invincibility { timer: Timer }

**Systems**

- tick_invincibility decrements timer
- Removes component when timer expires
- apply_damage skips entities with Invincibility

**Visual**

- Triggers invincibility flicker effect (separate spec)
