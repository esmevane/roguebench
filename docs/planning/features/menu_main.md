# Main Menu

Primary navigation menu on title screen. Provides access to game start and settings.

## Core Logic

**State**

- Selected index (int)
- Options list

**Options**

- Start Game - begin gameplay
- Options - open settings menu
- Credits - show credits
- Quit - exit application

**Operations**

`navigate(direction)` - Move selection

- Up decrements index (wraps)
- Down increments index (wraps)

`select()` - Activate current option

- Execute action for selected item
- Transition or open submenu

**Invariants**

- Always one option selected
- Selection wraps at boundaries
- Visual indicator on selected item
- Keyboard and gamepad supported

**Defaults**

| Field    | Value      | Description         |
| -------- | ---------- | ------------------- |
| Selected | 0          | Start Game          |
| Options  | 4          | Start, Options, Credits, Quit |

---

## Bevy Integration

**Input**

- W/S or Up/Down arrows
- Enter/Space to select
- Gamepad D-pad + A button

**Systems**

- Handle navigation input
- Update selected state
- Render menu with highlight
- Execute selection action

**UI**

- Vertical button list
- Highlight on selected
- Optional hover effects
- Sound on navigate/select

**Transitions**

- Start → Screen::Loading
- Options → push MenuState::Options
- Credits → push MenuState::Credits
- Quit → AppExit event
