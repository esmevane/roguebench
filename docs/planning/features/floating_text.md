# Floating Text

Text that pops up from a position and floats upward while fading. Used for damage numbers, pickup notifications, status effects.

## Core Logic

**State**

- Text (string) - content to display
- Position (vec2) - current world position
- Velocity (vec2) - movement direction and speed
- Age (f32) - time since spawn
- Lifetime (f32) - total duration
- Color (color) - text color with alpha
- Scale (f32) - text size

**Operations**

`step(dt)` - Advance simulation

- Update position: position += velocity * dt
- Increment age: age += dt
- Compute alpha: fade based on age/lifetime
- If age >= lifetime, mark for removal

`spawn(text, position, color)` - Create floating text

- Set text content
- Set initial position
- Set default upward velocity
- Set color
- Reset age to 0

**Invariants**

- Text floats upward by default (positive Y velocity)
- Alpha fades from 1.0 to 0.0 over lifetime
- Multiple texts can overlap (no collision)
- Despawns when lifetime expires

**Defaults**

| Field    | Value       | Description              |
| -------- | ----------- | ------------------------ |
| Lifetime | 0.5-1.0     | Seconds visible          |
| Velocity | (0, 30-50)  | Pixels per second upward |
| Scale    | 1.0         | Base text size           |

**Variants**

| Type       | Color  | Example      |
| ---------- | ------ | ------------ |
| Damage     | Red    | "-15"        |
| Heal       | Green  | "+10"        |
| Crit       | Yellow | "CRIT! -30"  |
| Pickup     | White  | "+1 Key"     |
| Status     | Purple | "Poisoned!"  |

---

## Bevy Integration

**Spawn**

- On damage event (show damage amount)
- On heal event (show heal amount)
- On pickup (show item name)
- Position: entity location or event position

**Rendering**

- Text entity with Transform
- Apply velocity and fade each frame
- Despawn when lifetime expires

**Triggers**

- Commands: `floating_text(text, position, color)`
- Messages: `FloatingText { text, position, color, lifetime }`

**Lifecycle**

- Each text is a separate entity
- Self-manages lifetime and despawn
