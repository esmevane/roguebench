# Health System

Tracks current and maximum health for entities. Handles damage application and death detection.

## Core Logic

**State**

- Current (i32) - current health points
- Max (i32) - maximum health points

**Operations**

`damage(amount)` - Apply damage

- Subtract amount from current
- Clamp current to minimum 0
- Return actual damage dealt

`heal(amount)` - Restore health

- Add amount to current
- Clamp current to maximum max
- Return actual healing done

`is_dead()` - Check if dead

- Return current <= 0

`percentage()` - Get health fraction

- Return current / max as 0.0-1.0

`set_max(new_max)` - Change max health

- Set max to new_max
- Optionally scale current proportionally

**Invariants**

- Current never exceeds max
- Current never goes below 0
- Dead when current reaches 0
- Max must be positive

**Defaults**

| Entity  | Max Health | Description |
| ------- | ---------- | ----------- |
| Player  | 100        | Configurable |
| Grunt   | 30         | Low health   |
| Archer  | 20         | Fragile      |
| Dasher  | 25         | Medium       |
| Brute   | 80         | Tank         |

---

## Bevy Integration

**Component**

- Health { current: i32, max: i32 }

**Events**

- HealthChanged { entity, current, max, delta }
- EntityDied { entity, position, kind } when current reaches 0

**Systems**

- apply_damage reads DamageEvent, modifies Health
- check_death detects Health <= 0, emits EntityDied

**Rendering**

- Health bar UI reads player Health
- Enemy health bars (optional)
