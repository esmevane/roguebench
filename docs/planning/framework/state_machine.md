# State Machine Framework

Generic state machine for AI behavior, character states, and animation.

## Relationship to Networking

State machines run on the **server** in `FixedMain` schedule. State is replicated to clients via Lightyear.

**Critical constraint:** Transition conditions must be deterministic functions of replicated state. No random checks, no frame-timing dependencies, no local-only data.

```
Server: Evaluate transitions at tick N → State changes → Replicated
Client: Receives state via Lightyear → Drives animation/visuals
```

Clients don't run their own transition logic for server-authoritative entities. They receive the current state and react visually.

---

## Core Logic

**Concept**

- Entities have a current state (enum variant)
- States define behavior (what systems do while in state)
- Transitions triggered by conditions evaluated each tick
- State changes emit events for other systems to observe

**State Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | enum variant | State identifier |
| on_enter | system | Execute on entry (once) |
| on_exit | system | Execute on exit (once) |
| behavior | systems | Active while in state |
| transitions | list | Conditions → next state |

**Transition Properties**

| Property | Type | Description |
|----------|------|-------------|
| target | enum variant | Destination state |
| condition | fn(Entity, &World) -> bool | When to transition |
| priority | i32 | Evaluation order (higher first) |

**Operations**

`force_transition(entity, state)` - Immediate transition

- Exit current state
- Enter new state
- Bypasses conditions

`evaluate_transitions(entity)` - Check conditions

- Evaluate in priority order
- Transition on first match
- If no match, stay in current state

`get_state(entity)` - Current state

- Return active state

`time_in_state(entity)` - How long in current state

- Tick count, not wall time

**Invariants**

- Exactly one active state per entity
- Transitions atomic (exit completes before enter)
- Transition evaluation is deterministic given same world state
- State changes emit events

---

## Tick-Based Timing

State durations and cooldowns use **tick counts**, not elapsed time.

```rust
struct StateTimer {
    entered_at_tick: u64,
    min_duration_ticks: Option<u64>,
}

fn can_transition(timer: &StateTimer, current_tick: u64) -> bool {
    match timer.min_duration_ticks {
        Some(min) => current_tick >= timer.entered_at_tick + min,
        None => true,
    }
}
```

This ensures determinism across server and clients regardless of frame rate.

---

## Bevy Integration

**Components**

```rust
/// The state machine component, generic over state enum
#[derive(Component)]
struct StateMachine<S: StateEnum> {
    current: S,
    entered_at_tick: u64,
}

/// Marker component for entities in a specific state
/// Systems can query for Has<InState<EnemyState::Chasing>>
#[derive(Component)]
struct InState<S>(PhantomData<S>);
```

**Trait Definition**

```rust
trait StateEnum: Clone + PartialEq + Send + Sync + Serialize + DeserializeOwned + 'static {
    /// Transitions to evaluate, in priority order
    fn transitions(&self) -> &'static [Transition<Self>];
}

struct Transition<S> {
    target: S,
    condition: fn(Entity, &World) -> bool,
    priority: i32,
}
```

**Systems (run in FixedMain)**

```rust
/// Evaluate transitions for all state machines of type S
fn evaluate_transitions<S: StateEnum>(
    mut query: Query<(Entity, &mut StateMachine<S>)>,
    world: &World,
    current_tick: Res<GameTick>,
    mut events: EventWriter<StateChanged<S>>,
) {
    for (entity, mut machine) in query.iter_mut() {
        for transition in machine.current.transitions() {
            if (transition.condition)(entity, world) {
                let from = machine.current.clone();
                machine.current = transition.target.clone();
                machine.entered_at_tick = current_tick.0;
                
                events.send(StateChanged { entity, from, to: machine.current.clone() });
                break;
            }
        }
    }
}
```

**Events**

```rust
struct StateChanged<S: StateEnum> {
    entity: Entity,
    from: S,
    to: S,
}
```

---

## Example: Enemy AI

```rust
#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum EnemyState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Stunned,
}

impl StateEnum for EnemyState {
    fn transitions(&self) -> &'static [Transition<Self>] {
        match self {
            Self::Idle => &[
                Transition { 
                    target: Self::Chase, 
                    condition: player_in_aggro_range, 
                    priority: 10 
                },
                Transition { 
                    target: Self::Patrol, 
                    condition: patrol_cooldown_elapsed, 
                    priority: 0 
                },
            ],
            Self::Chase => &[
                Transition { 
                    target: Self::Attack, 
                    condition: player_in_attack_range, 
                    priority: 10 
                },
                Transition { 
                    target: Self::Idle, 
                    condition: player_out_of_range, 
                    priority: 0 
                },
            ],
            // ...
        }
    }
}

// Condition functions - deterministic, based on replicated state
fn player_in_aggro_range(entity: Entity, world: &World) -> bool {
    let enemy_pos = world.get::<Transform>(entity)?.translation.truncate();
    let player_pos = find_player_position(world)?;
    enemy_pos.distance(player_pos) < AGGRO_RANGE
}
```

---

## State-Specific Behavior

Systems observe state and act accordingly:

```rust
fn chase_behavior(
    query: Query<(Entity, &Transform, &StateMachine<EnemyState>), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut velocities: Query<&mut Velocity>,
) {
    let player_pos = player_query.single().translation.truncate();
    
    for (entity, transform, machine) in query.iter() {
        if machine.current == EnemyState::Chase {
            let direction = (player_pos - transform.translation.truncate()).normalize();
            velocities.get_mut(entity).unwrap().linear = direction * CHASE_SPEED;
        }
    }
}
```

Or use marker components for more efficient queries:

```rust
fn chase_behavior(
    query: Query<(Entity, &Transform), (With<Enemy>, With<InState<Chasing>>)>,
    // ...
) {
    // Only iterates entities actually in Chase state
}
```

---

## Animation Integration

Animation systems observe state changes and trigger animations:

```rust
fn on_state_change_trigger_animation(
    mut events: EventReader<StateChanged<EnemyState>>,
    mut animation_query: Query<&mut AnimationPlayer>,
) {
    for event in events.read() {
        if let Ok(mut player) = animation_query.get_mut(event.entity) {
            let clip = match &event.to {
                EnemyState::Idle => "idle",
                EnemyState::Chase => "run",
                EnemyState::Attack => "attack",
                // ...
            };
            player.play(clip);
        }
    }
}
```

Animation is client-side presentation. The replicated state drives what animation plays, but animation frame timing doesn't affect gameplay.

---

## What's Not Here

- Hierarchical states (add if behavior gets complex)
- State stacking (push/pop for interrupts)
- Visual state machine editor (future tooling)

*See: features/ai_state_machine.md, features/character_states.md*
