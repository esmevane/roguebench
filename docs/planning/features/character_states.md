# Character States

Enum-based state machine for character behavior modes.

## Core Logic

**Relationship to Animation State Machine**

- Character states define *what the character is doing* (gameplay)
- Animation state machine handles *how animations play* (rendering)
- Character state changes trigger animation changes
- See `animation_state_machine.md` for the animation framework

**Concept**

- Single enum represents current state
- Only one state active at a time
- Illegal states unrepresentable
- Cleaner than boolean flags

**States**

| State | Description | Transitions To |
|-------|-------------|----------------|
| Idle | Standing still | Walking, Running, Jumping |
| Walking | Moving at walk speed | Idle, Running, Jumping |
| Running | Moving at run speed | Idle, Walking, Jumping |
| Jumping | Airborne | Idle, Walking, Running |
| Dashing | Invincible dash | Idle, Walking |
| Attacking | Attack animation | Idle, Walking |
| Stunned | Hit reaction | Idle |
| Dead | No actions | None |

**State Properties**

| State | Can Move | Can Attack | Can Dash | Is Grounded |
|-------|----------|------------|----------|-------------|
| Idle | Yes | Yes | Yes | Yes |
| Walking | Yes | Yes | Yes | Yes |
| Running | Yes | Yes | Yes | Yes |
| Jumping | Yes | No | No | No |
| Dashing | Fixed | No | No | Yes |
| Attacking | No | No | No | Yes |
| Stunned | No | No | No | Yes |

**Operations**

`can_transition(from, to)` - Check validity

- Return if transition allowed
- Prevents invalid state changes

`enter_state(state)` - Begin state

- Set new state
- Trigger enter effects
- Reset state timer

`exit_state(state)` - Leave state

- Clean up state
- Trigger exit effects

`is_grounded()` - Query property

- Check if state allows ground actions
- Used for jump validation

**Invariants**

- Exactly one state active
- Dead state is terminal
- Transitions validated
- State duration tracked

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Initial | Idle | Starting state |
| Stun duration | 0.3s | Hit reaction time |
| Dash duration | 0.2s | Dash length |

---

## Bevy Integration

**Component**

```rust
#[derive(Component)]
enum CharacterState {
    Idle,
    Walking,
    Running,
    Jumping { velocity: f32 },
    Dashing { direction: Vec2, timer: f32 },
    Attacking { attack: AttackType, timer: f32 },
    Stunned { timer: f32 },
    Dead,
}
```

**Change Detection**

- Use `Changed<CharacterState>` filter
- Trigger animation changes on state change
- Avoids per-frame checks

**Systems**

- Input system sets state based on input
- State update system manages timers
- Animation system responds to changes
- Movement system reads state for speed

**State Queries**

```rust
impl CharacterState {
    fn can_move(&self) -> bool {
        matches!(self, Idle | Walking | Running | Jumping { .. })
    }

    fn can_attack(&self) -> bool {
        matches!(self, Idle | Walking | Running)
    }

    fn movement_speed(&self, stats: &CharacterStats) -> f32 {
        match self {
            Walking => stats.speed_walk,
            Running => stats.speed_run,
            _ => 0.0,
        }
    }
}
```

**Events**

- StateChanged { entity, from, to }
- Used for sound, particles, etc.
