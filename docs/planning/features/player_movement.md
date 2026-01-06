# Player Movement

Cardinal direction movement controlled by keyboard input. Player moves at configurable speed in the direction of input.

## Core Logic

**State**

- Velocity (vec2) - current movement velocity
- Speed (f32) - maximum movement speed
- Enabled (bool)

**Operations**

`update(input_direction)` - Set movement direction

- Normalize input direction
- Set velocity to direction * speed

`stop()` - Halt movement

- Set velocity to zero

**Invariants**

- Movement is immediate (no acceleration)
- Diagonal movement is normalized (no speed boost)
- Movement respects physics collisions
- Movement disabled during certain states (dashing, stunned)

**Defaults**

| Field | Value | Description |
| ----- | ----- | ----------- |
| Speed | 300.0 | Pixels per second |

---

## Bevy Integration

**Input**

- WASD or Arrow keys
- Gamepad left stick
- Produces normalized Vec2 direction

**Physics**

- Applies velocity to LinearVelocity component
- Physics engine handles collision response
- Linear damping stops movement when input released

**State Interaction**

- Disabled during Dashing state
- Updates Facing direction based on movement
- Triggers walk animation when moving

---

## Framework Dependencies

- `framework/movement_framework.md` - Velocity, speed caps, movement modifiers
- `framework/collision_framework.md` - Physics collision response
- `framework/state_machine.md` - State-based movement disabling
- `framework/animation_framework.md` - Walk animation triggering

*See: architecture/scripting.md, architecture/data.md*
