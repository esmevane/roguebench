# Pause Overlay

Full-screen overlay that appears when game is paused. Dims gameplay and shows pause state.

## Core Logic

**State**

- Is paused (bool)

**Operations**

`pause()` - Activate pause

- Show overlay
- Stop game time
- Enable pause input only

`resume()` - Deactivate pause

- Hide overlay
- Resume game time
- Enable all inputs

`toggle()` - Switch pause state

- If paused, resume
- If not paused, pause

**Invariants**

- Overlay covers entire screen
- Game logic frozen while paused
- Only pause input works while paused
- Visual indication of paused state

**Defaults**

| Field       | Value         | Description |
| ----------- | ------------- | ----------- |
| Color       | Black, 80%    | Dim overlay |
| Text        | "PAUSED"      | Optional    |

---

## Bevy Integration

**Input**

- P key or Escape
- Gamepad Start button

**Implementation Options**

1. State-based: Pause screen state
2. Resource-based: IsPaused resource

**Systems**

- Pausable systems check pause state
- Overlay entity visibility toggled

**Time**

- Can use Time<Virtual> pause
- Or manual system disabling
