# Pause Menu

In-game menu displayed when paused. Allows resume, settings access, or quit.

## Core Logic

**State**

- Selected index (int)
- Is visible (bool)

**Options**

- Resume - continue playing
- Options - open settings
- Quit to Title - abandon run

**Operations**

`show()` - Open pause menu

- Pause game time
- Display menu overlay
- Enable menu input

`hide()` - Close pause menu

- Resume game time
- Hide overlay
- Enable game input

`navigate(direction)` - Move selection

- Up/Down changes selection
- Wraps at boundaries

`select()` - Activate option

- Execute selected action

**Invariants**

- Game frozen while menu visible
- Only one option selected
- Resume is default selection
- Escape toggles menu

**Defaults**

| Field    | Value  | Description    |
| -------- | ------ | -------------- |
| Selected | 0      | Resume         |
| Options  | 3      | Resume, Options, Quit |

---

## Bevy Integration

**Relation to Pause Overlay**

- Pause overlay handles dim + freeze
- Pause menu is UI within overlay
- Menu optional (can just show "PAUSED")

**Input**

- Escape or P to toggle
- W/S or Up/Down to navigate
- Enter/Space to select
- Gamepad Start to toggle, D-pad + A

**Systems**

- Toggle visibility on pause input
- Handle navigation when visible
- Execute selection action
- Manage sub-menu stack (Options)

**UI**

- Centered menu panel
- Semi-transparent background
- Button list with highlight
- Appears over dimmed game

**Transitions**

- Resume → hide menu, unpause
- Options → push options menu state
- Quit → Screen::Title (confirm dialog optional)
