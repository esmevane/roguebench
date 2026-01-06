# Pattern: State Machine

A data-driven state machine for entity behaviors, with scripting hooks for enter/exit/update events.

---

## Problem

Entities need complex behaviors:
- Enemies patrol, chase, attack based on conditions
- Players have movement states (idle, walking, dashing)
- UI flows have screen transitions

Hardcoding these in Rust means non-programmers can't author them.

## Solution

A data-driven state machine where:
1. States and transitions are defined in data (SQLite)
2. Conditions reference runtime values (flags, thresholds, timers)
3. Lua hooks fire on state enter/exit
4. The runtime evaluates and transitions automatically

## Implementation

### Core Types (in roguebench-core, no Bevy)

```rust
#[derive(Clone)]
pub struct StateId(pub String);

#[derive(Clone)]
pub struct StateDefinition {
    pub id: StateId,
    pub name: String,
    pub metadata: HashMap<String, Value>,  // Animation, speed, etc.
}

#[derive(Clone)]
pub struct TransitionDefinition {
    pub from: StateId,
    pub to: StateId,
    pub condition: TransitionCondition,
    pub priority: i32,
}

#[derive(Clone)]
pub enum TransitionCondition {
    Flag { name: String, value: bool },
    Threshold { name: String, op: CompareOp, value: f32 },
    After { seconds: f32 },
    And(Box<TransitionCondition>, Box<TransitionCondition>),
    Or(Box<TransitionCondition>, Box<TransitionCondition>),
    Not(Box<TransitionCondition>),
}

#[derive(Clone)]
pub struct StateMachineDefinition {
    pub id: String,
    pub initial_state: StateId,
    pub states: Vec<StateDefinition>,
    pub transitions: Vec<TransitionDefinition>,
}
```

### Bevy Component

```rust
#[derive(Component)]
pub struct StateMachine {
    definition: StateMachineDefinition,
    current_state: StateId,
    context: StateContext,  // flags, values, time_in_state
}

impl StateMachine {
    pub fn set_flag(&mut self, name: &str, value: bool);
    pub fn set_value(&mut self, name: &str, value: f32);
    pub fn update(&mut self, dt: f32) -> Option<StateTransition>;
    pub fn current_state(&self) -> &StateId;
}

#[derive(Event)]
pub struct StateChanged {
    pub entity: Entity,
    pub transition: StateTransition,
}
```

### State Update System

```rust
fn update_state_machines(
    time: Res<Time>,
    mut query: Query<(Entity, &mut StateMachine)>,
    mut events: EventWriter<StateChanged>,
) {
    for (entity, mut sm) in query.iter_mut() {
        if let Some(transition) = sm.update(time.delta_secs()) {
            events.send(StateChanged { entity, transition });
        }
    }
}
```

### Scripting Hooks

```lua
-- Scripts can react to state changes
function module.on_state_enter(event)
    if event.to == "attack" then
        effects:spawn("attack_windup", event.entity)
    end
end

function module.on_state_exit(event)
    if event.from == "patrol" then
        sounds:play("alert")
    end
end
```

### Data-Driven Definition

Stored in SQLite, authored via editor:

```json
{
  "id": "enemy_grunt_ai",
  "initial_state": "idle",
  "states": [
    { "id": "idle", "metadata": { "animation": "idle" } },
    { "id": "patrol", "metadata": { "animation": "walk", "speed": 50 } },
    { "id": "chase", "metadata": { "animation": "run", "speed": 100 } }
  ],
  "transitions": [
    { "from": "idle", "to": "patrol", "condition": { "after": 2.0 } },
    { "from": "patrol", "to": "chase", "condition": { "flag": ["player_spotted", true] } },
    { "from": "chase", "to": "patrol", "condition": { "flag": ["player_spotted", false] } }
  ]
}
```

### Editor UI (Functional But Not Fancy)

```
┌─────────────────────────────────────────────┐
│ State Machine: grunt_ai                     │
├─────────────────────────────────────────────┤
│ States:                                     │
│   [idle    ] [patrol  ] [chase   ] [+]     │
│                                             │
│ Selected: patrol                            │
│   Animation: [walk         ]                │
│   Speed:     [50           ]                │
│                                             │
│ Transitions from patrol:                    │
│   → chase  when player_spotted = true       │
│   → idle   when after 10 seconds            │
│                                             │
│ [Save] [Test]                               │
└─────────────────────────────────────────────┘
```

## When to Use

- **Entity AI** with multiple behavioral states
- **Player states** (movement, abilities)
- **Any behavior** that non-programmers should be able to author

## When Not to Use

- **Simple behaviors** that don't need multiple states
- **One-off logic** better expressed in Lua directly

## Benefits

| Benefit | How |
|---------|-----|
| Data-driven | Definitions in SQLite, not code |
| Authorable | Editor UI for non-programmers |
| Scriptable | Lua hooks for enter/exit/update |
| Hot-reloadable | Change definition, behavior updates |
| Testable | Create StateMachine in tests, simulate conditions |

## Related Patterns

- **Behavior Trees:** More complex decision-making (state machines are simpler)
- **Hierarchical State Machines:** States containing sub-machines (future extension)
