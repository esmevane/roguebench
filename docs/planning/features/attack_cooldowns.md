# Attack Cooldowns

Timers that prevent rapid attack spam. Each attack type has independent cooldown.

## Core Logic

**State**

- Melee cooldown (f32) - time until melee available
- Ranged cooldown (f32) - time until ranged available
- Dash cooldown (f32) - time until dash available

**Operations**

`start_cooldown(attack_type, duration)` - Begin cooldown

- Set cooldown timer for attack type

`tick(dt)` - Advance cooldowns

- Decrement all active cooldown timers
- Clamp to minimum 0

`is_ready(attack_type)` - Check availability

- Return cooldown timer <= 0 for attack type

`remaining(attack_type)` - Get remaining time

- Return current cooldown timer value

**Invariants**

- Each attack type has independent timer
- Cooldowns tick down regardless of player state
- New attack resets timer to full duration
- Zero or negative means ready

**Defaults**

| Attack | Cooldown | Description |
| ------ | -------- | ----------- |
| Melee  | 0.3s     | Fast attacks |
| Ranged | 0.2s     | Rapid fire   |
| Dash   | 0.5s     | Movement ability |

---

## Bevy Integration

**Component**

- AttackCooldown { melee: Timer, ranged: Timer }
- DashCooldown (separate component, added during dash)

**Systems**

- Tick all cooldown timers each frame
- Attack systems check cooldown before allowing attack

**UI**

- Optional cooldown indicators
- Input buffering works with cooldowns
