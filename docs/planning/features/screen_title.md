# Title Screen

Main menu screen with game title and start option. Entry point for new games.

## Core Logic

**State**

- Selected option (if multiple)

**Options**

- Start Game - begin gameplay
- (Future: Options, Credits, Quit)

**Operations**

`select_start()` - Begin game

- Transition to Loading or Gameplay screen
- Initialize game state

**Invariants**

- Shown after splash
- Requires player input to proceed
- Returns here after game over (future)

**Defaults**

| Field    | Value       | Description    |
| -------- | ----------- | -------------- |
| Next     | Loading     | On start press |

**Content**

- Game title
- Start prompt ("Press Enter to Start")
- Background art (optional)

---

## Bevy Integration

**Screen State**

- Screen::Title variant

**Input**

- Enter/Space to start
- Gamepad A button

**Systems**

- Listen for start input
- Transition to Screen::Loading

**UI**

- Centered title text
- Start prompt below
- Optional menu buttons
