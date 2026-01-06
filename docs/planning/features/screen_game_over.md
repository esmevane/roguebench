# Game Over Screen

Displayed when player dies. Shows run summary and offers retry or quit options.

## Core Logic

**State**

- Final wave reached
- Enemies defeated (optional)
- Time survived (optional)

**Options**

- Retry - restart game
- Quit to Title - return to main menu

**Operations**

`show(stats)` - Display game over

- Capture run statistics
- Display summary
- Enable menu navigation

`retry()` - Restart game

- Reset all game state
- Return to gameplay

`quit()` - Return to title

- Clean up game state
- Transition to title screen

**Invariants**

- Only shown after player death
- Game is fully stopped
- Statistics reflect completed run
- Must choose an option to proceed

**Defaults**

| Field   | Value        | Description       |
| ------- | ------------ | ----------------- |
| Title   | "GAME OVER"  | Header text       |
| Default | Retry        | Pre-selected option |

---

## Bevy Integration

**Trigger**

- PlayerDied event or EntityDied with Player
- Transition from Gameplay state

**Screen State**

- Screen::GameOver variant
- Or GameplayState::GameOver sub-state

**Input**

- Up/Down or W/S to navigate
- Enter/Space to select
- Gamepad D-pad + A button

**Systems**

- Capture stats on death
- Render game over UI
- Handle menu selection
- Trigger appropriate transition

**UI**

- Centered "GAME OVER" text
- Run statistics below
- Menu options at bottom
