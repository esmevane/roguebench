# Player Dash

Invincible dash ability that moves player quickly in a direction. Grants temporary invincibility and has a cooldown.

## Core Logic

**State**

- Direction (vec2) - dash direction
- Duration (f32) - how long dash lasts
- Speed (f32) - dash movement speed
- Cooldown (f32) - time before dash available again
- Timer (f32) - remaining dash time
- Cooldown timer (f32) - remaining cooldown time

**Operations**

`start(direction)` - Begin dash

- Set direction (from movement input or aim)
- Set timer to duration
- Grant invincibility for duration

`step(dt)` - Advance dash

- Move in direction at speed
- Decrement timer
- If timer <= 0, end dash

`tick_cooldown(dt)` - Advance cooldown

- Decrement cooldown timer
- If cooldown timer <= 0, dash available

`is_available()` - Check if can dash

- Return cooldown timer <= 0

`is_active()` - Check if dashing

- Return timer > 0

**Invariants**

- Cannot dash while already dashing
- Cannot dash during cooldown
- Invincibility lasts exactly dash duration
- Direction locked once dash starts
- Overrides normal movement during dash

**Defaults**

| Field    | Value | Description           |
| -------- | ----- | --------------------- |
| Duration | 0.15  | Seconds               |
| Speed    | 800.0 | Pixels per second     |
| Cooldown | 0.5   | Seconds between dashes |

---

## Bevy Integration

**Input**

- Space or Shift key
- Gamepad button

**Components**

- Dashing (active dash state)
- DashCooldown (cooldown timer)
- Adds Invincibility component during dash

**Events**

- DashStarted { entity, direction }
- DashEnded { entity }

**State Interaction**

- Sets PlayerState to Dashing
- Triggers dash visual effects (slow-mo, squash, trail)

---

## Framework Dependencies

- `framework/movement_framework.md` - Dash velocity override
- `framework/timer_framework.md` - Duration and cooldown timers
- `framework/state_machine.md` - Dashing state
- `framework/command_bus.md` - Dash command for networking
- `features/invincibility_frames.md` - Invincibility during dash

*See: architecture/scripting.md*
