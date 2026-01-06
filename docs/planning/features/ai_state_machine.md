# AI State Machine

Finite state machine controlling enemy behavior. Transitions between states based on conditions.

## Core Logic

**States**

- Idle - no target, stationary
- Chasing - moving toward target
- Attacking - executing attack
- Retreating - moving away from target
- Circling - orbiting around target

**Transitions**

| From      | Condition              | To        |
| --------- | ---------------------- | --------- |
| Idle      | Player in aggro range  | Chasing   |
| Chasing   | In attack range        | Attacking |
| Chasing   | Player leaves aggro    | Idle      |
| Attacking | Attack cooldown ready  | Chasing   |
| Attacking | Too close (Archer)     | Retreating |
| Retreating| Reached safe distance  | Chasing   |
| Circling  | Attack opportunity     | Attacking |
| Circling  | Timer expires          | Chasing   |

**Operations**

`update(current_state, conditions)` - Evaluate transitions

- Check all valid transitions from current state
- Return new state if condition met
- Otherwise return current state

`get_behavior(state)` - Get state behavior

- Return movement/action for current state

**Invariants**

- Always in exactly one state
- Transitions evaluated each frame
- State determines movement and actions
- Enemy type influences transition conditions

---

## Bevy Integration

**Component**

- AIState enum { Idle, Chasing, Attacking, Retreating, Circling }

**Systems**

- ai_state_transitions evaluates conditions
- ai_movement executes state-specific movement
- ai_attack triggers attacks in Attacking state

**Per-Enemy Behavior**

- Grunt: Idle → Chase → Attack (simple)
- Archer: adds Retreating when close
- Dasher: uses Circling state
- Brute: same as Grunt but slower
