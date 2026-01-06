# Input Configuration

Customizable control bindings for keyboard, mouse, and gamepad.

## Core Logic

**Binding Types**

- Keyboard key
- Mouse button
- Mouse axis (look direction)
- Gamepad button
- Gamepad axis (stick)

**Actions**

| Action | Default (KB/M) | Default (Gamepad) |
|--------|----------------|-------------------|
| Move Up | W | Left Stick Up |
| Move Down | S | Left Stick Down |
| Move Left | A | Left Stick Left |
| Move Right | D | Left Stick Right |
| Aim | Mouse Position | Right Stick |
| Attack | Left Click | Right Trigger |
| Dash | Space | Left Trigger |
| Pause | Escape | Start |
| Interact | E | A Button |

**Operations**

`get_binding(action)` - Query current binding

- Return input source for action
- May return multiple (KB + gamepad)

`set_binding(action, input)` - Rebind action

- Update binding map
- Check for conflicts
- Persist to settings

`reset_defaults()` - Restore default bindings

- Clear custom bindings
- Apply default map

`check_conflict(action, input)` - Validate binding

- Check if input used elsewhere
- Return conflicting action if any

**Invariants**

- Each action has at least one binding
- Conflicts warned but allowed (user choice)
- Bindings persist across sessions
- Both KB/M and gamepad configurable independently

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Deadzone | 0.2 | Stick deadzone |
| Sensitivity | 1.0 | Mouse/stick multiplier |

---

## Bevy Integration

**Resources**

- InputBindings { map: HashMap<Action, Vec<Input>> }
- InputConfig { deadzone, sensitivity }

**Input Actions**

```rust
enum GameAction {
    Move,
    Aim,
    Attack,
    Dash,
    Pause,
    Interact,
}
```

**Systems**

- Read input based on bindings
- Translate raw input to actions
- Apply deadzone and sensitivity
- Support simultaneous KB + gamepad

**Persistence**

- Save bindings to config file
- Load on startup
- Default if missing or invalid

**UI**

- List of actions with current bindings
- "Press key to rebind" prompt
- Conflict warning
- Reset to defaults button

**Libraries**

- leafwing-input-manager for action mapping
- Or custom binding system
