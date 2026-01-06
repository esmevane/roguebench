# Run State

Separation of temporary run data from permanent progression.

## Core Logic

**Relationship to Other Persistence Features**

- `save_system.md`: Overall progress persistence (permanent data)
- `checkpoints.md`: Mid-session resume points (where player left off)
- `run_state.md` (this): Temporary vs permanent data separation

Run state defines the roguelite core: what persists through death (currency, unlocks, quest progress) vs what resets each run (current items, room, buffs).

**Concept**

- Some progress is permanent (survives death)
- Some progress is temporary (lost on death)
- Clear distinction enables roguelike feel
- Managed separately in persistence

**Permanent State**

| Data | Description |
|------|-------------|
| Insight | Investable currency |
| Aptitudes | Unlocked abilities |
| Unlocks | Meta-progression |
| Quest progress | Story advancement |
| Persistent currency | Gold, gems, etc. |
| Completed milestones | One-time unlocks |

**Temporary State**

| Data | Description |
|------|-------------|
| Current room | Where in the run |
| Run inventory | Items found this run |
| Temporary buffs | Run-specific effects |
| Enemy states | Current combat |
| Run currency | Non-persistent currency |
| Temporary stats | Gained this run |

**Operations**

`on_death()` - Handle death

- Preserve permanent state
- Clear temporary state
- Return player to hub/start

`start_run()` - Begin new run

- Initialize temporary state
- Keep permanent state
- Enter run area

`end_run(success)` - Complete run

- If success: convert some temp to permanent?
- Clear temporary state
- Return to hub

`get_permanent()` - Query permanent

- Access permanent state

`get_temporary()` - Query temporary

- Access run state

**Invariants**

- Death never loses permanent progress
- Runs always start with clean temp state
- Conversion rules design-defined
- Clear boundary between states

**Design Notes**

- What converts on success left to design
- Run rewards left to design
- Hub behavior left to design

---

## Bevy Integration

**Resources**

- PermanentProgress { insight, aptitudes, unlocks, quests, ... }
- RunState { inventory, room, buffs, temp_currency, ... }

**State Separation**

- Different resources for permanent vs temporary
- Or tagged components

**Messages/Commands**

- StartRun
- EndRun { success }
- ResetRunState

**Events**

- RunStarted
- RunEnded { success, stats }
- DeathOccurred

**Systems**

- On death: preserve permanent, clear temporary
- On run start: initialize fresh temporary
- Separate save handling for each

**Death Flow**

```
Death → Preserve Permanent → Clear Temporary → Return to Hub → Run Stats Display
```

**Scripting Compatibility**

- Run lifecycle as commands/events
- Both states readable
- Clear/preserve hookable

*See: architecture/data.md*
