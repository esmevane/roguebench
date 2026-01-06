# Controls Hint UI

Text display showing control scheme. Helps new players understand inputs.

## Core Logic

**Content**

- Movement keys (WASD)
- Dash key (Space)
- Attack (Click)
- Pause (P)

**Display**

- Static text, doesn't change
- Semi-transparent for non-intrusiveness
- Bottom of screen positioning

**Invariants**

- Always visible during gameplay
- Does not interfere with gameplay view
- Readable but subtle

**Defaults**

| Field    | Value           | Description    |
| -------- | --------------- | -------------- |
| Position | Bottom-center   | Screen location |
| Font size| 16              | Pixels         |
| Color    | White, 70% alpha| Subtle         |
| Text     | "WASD: Move | Space: Dash | Click: Attack | P: Pause" | |

---

## Bevy Integration

**Components**

- ControlsHintText
- Text component

**Spawning**

- Created once at gameplay start
- Static, no updates needed

**Styling**

- UI Node at bottom of screen
- Centered horizontally
- Semi-transparent text color
