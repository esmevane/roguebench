# Scripting Architecture

Constraints and patterns for exposing game systems to a scripting runtime for content authoring.

## Principles

1. **Command pattern** - Game actions expressed as dispatchable commands
2. **Observable events** - Game state changes emit hookable events
3. **Safe API surface** - Scripts interact through defined interfaces
4. **Hot-reloadable** - Script changes apply without restart
5. **Content authoring focus** - Designed for designers, not modders (initially)

## Command Pattern

**Structure**
- Actions are data (command structs)
- Commands dispatched through central system
- Handlers execute commands
- Results observable as events

**Benefits**
- Scripts emit commands, don't touch internals
- Commands are serializable (network-safe)
- Command history enables replay/undo
- Same commands work local or networked

## Hook System

**Event Hooks**
- Game emits events at key moments
- Scripts subscribe to event types
- Handlers receive event data, can respond

**Hook Points**
| Category | Events |
|----------|--------|
| Entity | Spawned, Despawned, Damaged, Died |
| Player | Moved, Attacked, InteractedWith |
| Quest | Started, ObjectiveComplete, Completed |
| Room | Entered, Cleared, Exited |
| Combat | HitLanded, DamageTaken, EffectApplied |

**Script Response**
- Read event data
- Query game state (read-only)
- Emit commands (write via commands only)

## API Surface

**Readable State**
- Entity positions, stats, inventory
- Quest progress, flags
- Room state, world data

**Commandable Actions**
- Spawn/despawn entities
- Modify stats, apply effects
- Start dialogue, trigger events
- Grant items, currency
- Update quest state

**Restricted**
- Direct component access
- Physics internals
- Render state

## Hot Reload

**Requirements**
- Detect script file changes
- Reload without restart
- Preserve game state across reload
- Graceful error handling

**Scope**
- Script logic reloads
- Data definitions reload
- Core systems don't reload

## Runtime Considerations

**Language (TBD)**
- Implementation exercise
- Options: Lua, Rhai, WASM
- Must support hot reload

**Sandboxing**
- Scripts can't crash game
- Resource limits (execution time, memory)
- No filesystem/network access from scripts

## Constraints for Features

Features should:
- Express actions as commands/messages
- Emit events for state changes
- Expose readable state through defined API
- Avoid side effects that bypass command system
- Document hookable events and available commands
