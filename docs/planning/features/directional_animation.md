# Directional Animation

4-way sprite animation based on entity facing direction. Different sprite row per direction.

## Core Logic

**Directions**

- South (down) - row 0
- West (left) - row 1
- East (right) - row 2
- North (up) - row 3

**State**

- Current facing (direction)
- Current animation (walk, attack, idle)
- Current frame (index)

**Operations**

`update_facing(velocity)` - Set direction from movement

- Determine dominant axis
- Set facing to direction of movement

`get_sprite_row(facing)` - Get spritesheet row

- Return row index for facing direction

`set_animation(type, facing)` - Start animation

- Calculate first/last frame from row + animation type
- Reset to first frame

**Invariants**

- Facing persists when not moving
- Animation row changes with facing
- Frame advances within row bounds
- Idle uses first frame of walk animation

**Sprite Sheet Layout**

| Row | Direction | Frames |
| --- | --------- | ------ |
| 0   | South     | Walk: 0-1, Attack: 7-8 |
| 1   | West      | Walk: 10-11, Attack: 17-18 |
| 2   | East      | Walk: 20-21, Attack: 27-28 |
| 3   | North     | Walk: 30-31, Attack: 37-38 |

---

## Bevy Integration

**Components**

- Facing enum { South, West, East, North }
- AnimationState { current_frame, first_frame, last_frame }

**Systems**

- update_facing reads velocity, updates Facing
- update_animation_state changes row on facing change
- animate_sprites advances frames

**Sprite**

- TextureAtlas with sprite sheet
- Atlas index set from AnimationState
