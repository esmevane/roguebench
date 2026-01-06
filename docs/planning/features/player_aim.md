# Player Aim

Targeting system that determines attack direction based on mouse cursor or gamepad stick.

## Core Logic

**State**

- Aim direction (vec2) - normalized direction to target
- Aim position (vec2) - world position of cursor/target

**Operations**

`update_mouse(cursor_screen_pos, camera, player_pos)` - Update from mouse

- Convert screen position to world position
- Calculate direction from player to cursor
- Normalize direction

`update_gamepad(stick_input)` - Update from gamepad

- Use right stick as direct direction
- Normalize if magnitude > deadzone

`get_direction()` - Get current aim direction

- Return normalized aim direction

**Invariants**

- Direction is always normalized
- Falls back to facing direction if no input
- Mouse takes priority over gamepad when both present

**Defaults**

| Field    | Value | Description        |
| -------- | ----- | ------------------ |
| Deadzone | 0.2   | Gamepad stick dead zone |

---

## Bevy Integration

**Input**

- Mouse cursor position (via Window query)
- Gamepad right stick

**Camera**

- Requires camera transform for screen-to-world conversion
- Uses Camera2d projection

**Usage**

- Attacks use aim direction for targeting
- Visual indicator can show aim direction (optional)
