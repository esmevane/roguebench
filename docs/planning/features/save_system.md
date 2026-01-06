# Save System

Full game state serialization and restoration with slot management.

## Core Logic

**Relationship to Other Persistence Features**

- `save_system.md` (this): Overall progress persistence (permanent data)
- `checkpoints.md`: Mid-session resume points (where player left off)
- `run_state.md`: Temporary vs permanent data separation

Save system handles *what* gets saved. Checkpoints handle *when* to save. Run state defines *which data* survives death.

**Concept**

- Serialize game state to persistent storage
- Multiple save slots supported
- Load restores complete state
- Handles versioning and migration

**Save Data**

| Category | Contents |
|----------|----------|
| Player | Stats, inventory, equipment, position |
| Progress | Unlocks, completed quests, flags |
| World | Current room, explored areas |
| Currency | All persistent currency balances |
| Meta | Save version, timestamp, playtime |

**Slot Properties**

| Property | Type | Description |
|----------|------|-------------|
| slot_id | int | Slot number |
| name | string | Player-defined name |
| timestamp | datetime | Last save time |
| playtime | duration | Total play time |
| preview | data | Quick preview info |
| version | int | Save format version |

**Operations**

`save(slot_id)` - Save to slot

- Gather all saveable state
- Serialize to storage
- Update slot metadata

`load(slot_id)` - Load from slot

- Deserialize from storage
- Restore game state
- Handle version migration

`delete(slot_id)` - Remove save

- Delete save file
- Clear slot

`get_slots()` - List saves

- Return all slot info
- For save/load UI

`export(slot_id)` - Export save

- Create portable save file
- For backup/transfer

`import(file)` - Import save

- Load external save
- Place in available slot

**Invariants**

- Saves are atomic (complete or fail)
- Corruption detected and reported
- Old versions migrated on load
- In-progress save doesn't corrupt existing

**Design Notes**

- Slot count left to implementation
- Storage location left to implementation
- Auto-save frequency left to design

---

## Bevy Integration

**Resources**

- SaveSlots { slots: Vec<SaveSlotInfo> }
- CurrentSlot(Option<SlotId>)

**Systems**

- Gather state from saveable components
- Serialize with serde
- Write to platform storage

**Messages/Commands**

- SaveGame { slot_id }
- LoadGame { slot_id }
- DeleteSave { slot_id }

**Events**

- GameSaved { slot_id }
- GameLoaded { slot_id }
- SaveFailed { slot_id, error }
- LoadFailed { slot_id, error }

**Saveable Marker**

```rust
#[derive(Component, Serialize, Deserialize)]
struct Saveable;
// Entities with this are included in saves
```

**Versioning**

- Save version in file header
- Migration functions per version
- Graceful handling of unknown data

**Scripting Compatibility**

- Save/load exposed as commands
- Events hookable
- State must be serializable

*See: architecture/data.md*
