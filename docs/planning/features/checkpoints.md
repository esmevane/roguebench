# Checkpoints

Auto-save triggers and manual save points throughout gameplay.

## Core Logic

**Relationship to Other Persistence Features**

- `save_system.md`: Overall progress persistence (permanent data)
- `checkpoints.md` (this): Mid-session resume points (where player left off)
- `run_state.md`: Temporary vs permanent data separation

Checkpoints enable players to quit mid-session and resume later. They capture the current run state so players don't lose progress when closing the game.

**Concept**

- Game saves at specific moments
- Mix of automatic and manual triggers
- Checkpoint locations in world
- Recovery point on load

**Checkpoint Types**

| Type | Trigger | Examples |
|------|---------|----------|
| Auto | Game event | Room cleared, boss killed |
| Location | Player reaches | Save shrine, town entrance |
| Manual | Player action | Save menu, quicksave |
| Timed | Interval | Every N minutes |

**Auto-Save Triggers**

| Trigger | When |
|---------|------|
| RoomCleared | All enemies defeated |
| BossKilled | Major enemy defeated |
| QuestCompleted | Quest turned in |
| ItemObtained | Significant item gained |
| AreaEntered | New region reached |

**Checkpoint Properties**

| Property | Type | Description |
|----------|------|-------------|
| type | enum | Checkpoint type |
| location | option | World position |
| timestamp | datetime | When created |
| state_snapshot | data | Game state at checkpoint |

**Operations**

`create_checkpoint(type)` - Save current state

- Capture game state
- Store with metadata
- Manage checkpoint limit

`restore_checkpoint()` - Load latest

- Find most recent checkpoint
- Restore state

`get_checkpoints()` - List available

- Return checkpoint history
- For UI display

**Invariants**

- Checkpoint count limited (rolling)
- Auto-saves don't interrupt gameplay
- Load point is checkpoint, not exact position
- Checkpoints use same serialization as saves

**Design Notes**

- Specific triggers left to design
- Checkpoint limit left to implementation
- Visual feedback left to design

---

## Bevy Integration

**Resources**

- CheckpointConfig { auto_triggers, interval, max_checkpoints }
- CheckpointHistory { checkpoints: VecDeque<Checkpoint> }

**Components**

- CheckpointLocation - marks save points in world

**Systems**

- Listen for auto-save triggers
- Create checkpoint on trigger
- Manage checkpoint rotation

**Messages/Commands**

- CreateCheckpoint { checkpoint_type }
- RestoreCheckpoint { checkpoint_id }
- Quicksave
- Quickload

**Events**

- CheckpointCreated { checkpoint_type, timestamp }
- CheckpointRestored { checkpoint_id }

**Integration with Save System**

- Checkpoints use SaveSystem serialization
- Stored separately from manual saves
- May share slot or dedicated checkpoint slots

**Scripting Compatibility**

- Checkpoint creation as command
- Triggers hookable

*See: architecture/data.md*
