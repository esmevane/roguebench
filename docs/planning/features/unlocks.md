# Unlocks

Meta-progression gate system for content availability based on player progress and milestones.

## Core Logic

**Concept**

- Content locked until conditions met
- Conditions: milestones, progress, purchases
- Unlocks persist permanently
- Gates items, characters, modes, areas

**Unlock Types**

| Type | Examples |
|------|----------|
| Character | New playable characters |
| Item | Starting items, shop stock |
| Mode | Difficulty modes, modifiers |
| Area | New regions, rooms |
| Cosmetic | Skins, effects |

**Condition Types**

| Condition | Example |
|-----------|---------|
| Milestone | "Defeat boss X" |
| Stat threshold | "Reach wave 20" |
| Collection | "Find all artifacts" |
| Currency | "Spend 1000 gold total" |
| Aptitude | "Unlock specific aptitude" |

**Operations**

`check_unlock(unlock_id)` - Evaluate conditions

- Check all required conditions
- Return true if all met

`grant_unlock(unlock_id)` - Mark as unlocked

- Add to unlocked set
- Emit event
- Make content available

`is_unlocked(unlock_id)` - Query status

- Check if previously unlocked

`get_progress(unlock_id)` - Query partial

- For trackable conditions
- Return completion percentage

**Invariants**

- Unlocks are permanent once granted
- Conditions defined by design
- Content availability respects unlocks
- Progress tracked for display

**Design Notes**

- Specific unlocks left to design
- Condition specifics left to design
- UI presentation left to design

---

## Bevy Integration

**Data**

- UnlockDefinition { id, conditions, content_type }
- PlayerUnlocks { unlocked: HashSet<UnlockId> }
- UnlockProgress { tracking: HashMap<UnlockId, Progress> }

**Messages/Commands**

- CheckUnlock { unlock_id }
- GrantUnlock { unlock_id }

**Events**

- UnlockGranted { unlock_id }
- UnlockProgressUpdated { unlock_id, progress }

**Systems**

- Monitor conditions (milestone listener, stat tracker)
- Auto-grant when conditions met
- Filter content by unlock status

**Persistence**

- Unlocked set saved permanently
- Progress tracking saved

**Scripting Compatibility**

- Grant exposed as command
- Unlock status readable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
