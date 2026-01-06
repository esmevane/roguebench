# Motor Accessibility

Options for players with motor impairments including input assistance, timing adjustments, and control simplification.

## Core Logic

**Concept**

- Accommodate motor impairments
- Reduce precision requirements
- Adjust timing demands
- Simplify complex inputs

**Input Assistance**

| Option | Type | Description |
|--------|------|-------------|
| Auto-aim | enum | Off, light, strong |
| Aim assist | float | Sticky aim strength |
| Hold vs toggle | enum | For sustained actions |
| Input buffering | float | Extended input window |
| Double-tap prevention | bool | Ignore rapid repeats |
| One-hand mode | bool | Alternative control scheme |

**Timing Adjustments**

| Option | Type | Description |
|--------|------|-------------|
| Game speed | float | Slow down gameplay |
| Reaction time | enum | Extended windows |
| Auto-pause | bool | Pause on certain events |
| Dash timing | float | Extended i-frames |

**Control Simplification**

| Option | Type | Description |
|--------|------|-------------|
| Auto-attack | bool | Attack nearest enemy |
| Auto-dodge | bool | Dodge obvious attacks |
| Simplified combos | bool | Reduce input complexity |
| Sticky movement | bool | Continue moving without hold |

**Operations**

`apply_aim_assist(input, targets)` - Adjust aim

- Bias toward nearest target
- Strength based on setting

`extend_input_buffer(window)` - Lengthen buffer

- Increase input acceptance window
- For action queuing

**Invariants**

- Assistance doesn't break gameplay
- All options are optional
- Can be combined freely
- Settings persist

**Design Notes**

- Specific assistance algorithms left to implementation
- Test with target users
- Balance between assistance and game feel

---

## Bevy Integration

**Resources**

- MotorAccessibility { auto_aim, game_speed, input_buffer, ... }

**Systems**

- Apply aim assistance to targeting
- Scale time for game speed
- Extend input buffer window
- Handle alternative input modes

**Integration Points**

- `player_aim.md` - aim assist integration
- `input_buffering.md` - extended buffer
- `player_dash.md` - timing adjustments
- `attack_cooldowns.md` - timing relief

**Scripting Compatibility**

- Settings readable
- Can adjust per-content difficulty

*See: settings_input.md*
