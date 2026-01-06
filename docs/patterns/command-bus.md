# Pattern: Command Bus

A pattern for routing all game mutations through a central bus, enabling logging, replay, scripting hooks, and network synchronization.

---

## Problem

Game mutations happen in many places:
- Player input causes damage
- Scripts modify entity state
- Network messages apply remote changes
- AI decisions trigger actions

Without centralization:
- Hard to log what happened
- Hard to replay for debugging
- Hard to hook scripts into mutations
- Hard to synchronize across network

## Solution

Route all mutations through a Command Bus:

```
Intent → Command → Validation → Execution → Event
```

1. **Intent:** Something wants to change state
2. **Command:** Structured request describing the change
3. **Validation:** Check if the change is allowed
4. **Execution:** Apply the change
5. **Event:** Notify that the change happened

## Implementation

### Command Trait

```rust
pub trait Command: Send + Sync + 'static {
    type Result;
    fn execute(&self, world: &mut World) -> Self::Result;
}
```

### Sending Commands

```rust
// Define the command
pub struct DealDamage {
    pub target: Entity,
    pub amount: i32,
    pub source: Option<Entity>,
}

impl Command for DealDamage {
    type Result = DamageResult;

    fn execute(&self, world: &mut World) -> DamageResult {
        // Apply damage, return result
    }
}

// Send via resource
commands.send(DealDamage { target, amount: 10, source: Some(player) });
```

### Event on Execution

```rust
// Automatically fired after command executes
#[derive(Event)]
pub struct CommandExecuted<C: Command> {
    pub command: C,
    pub result: C::Result,
}

// Systems can react
fn on_damage_dealt(
    mut events: EventReader<CommandExecuted<DealDamage>>,
) {
    for event in events.read() {
        // Play sound, spawn particles, etc.
    }
}
```

### Scripting Integration

Scripts subscribe to command events:

```lua
function module.on_deal_damage(event)
    -- event.target, event.amount, event.source available
    if event.amount > 50 then
        effects:spawn("big_hit", event.target)
    end
end
```

### Validation Layer

```rust
pub trait Validator<C: Command> {
    fn validate(&self, command: &C, world: &World) -> Result<(), ValidationError>;
}

// Example: Can't damage invincible entities
impl Validator<DealDamage> for InvincibilityValidator {
    fn validate(&self, cmd: &DealDamage, world: &World) -> Result<(), ValidationError> {
        if world.get::<Invincible>(cmd.target).is_some() {
            return Err(ValidationError::TargetInvincible);
        }
        Ok(())
    }
}
```

## When to Use

- **Any game mutation** that should be loggable, replayable, or scriptable
- **Networked games** where commands need to sync across clients
- **Debugging** where you need to understand what happened

## When Not to Use

- **Internal state updates** that don't represent game events (camera position, UI state)
- **High-frequency updates** where the overhead matters (particle positions)

## Benefits

| Benefit | How |
|---------|-----|
| Logging | Commands are data, easy to serialize and log |
| Replay | Play back logged commands to reproduce bugs |
| Scripting | Scripts hook into command events |
| Network | Commands are the replication unit |
| Testing | Send commands, assert results |

## Related Patterns

- **Event Sourcing:** Commands as the source of truth (we use events, not full sourcing)
- **CQRS:** Separating reads from writes (commands are writes)
