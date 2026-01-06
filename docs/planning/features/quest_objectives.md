# Quest Objectives

Objective types and progress tracking for quest completion requirements.

## Core Logic

**Concept**

- Objectives are completion conditions
- Multiple objective types supported
- Progress tracked automatically
- Designer defines specific objectives

**Objective Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Objective identifier |
| type | enum | Objective type |
| target | string | What to do/find/kill |
| required | int | Amount needed |
| current | int | Current progress |
| description | string | Display text |
| optional | bool | Not required for completion |
| hidden | bool | Not shown until discovered |

**Objective Types**

| Type | Trigger | Examples |
|------|---------|----------|
| Kill | Entity death | Kill 10 goblins |
| Collect | Item pickup | Gather 5 herbs |
| Deliver | Item transfer | Bring letter to NPC |
| Talk | Dialogue complete | Speak with elder |
| Reach | Area entered | Find hidden cave |
| Interact | Interaction triggered | Activate shrine |
| Escort | Entity reaches goal | Protect merchant |
| Survive | Time elapsed | Survive 5 waves |

**Operations**

`increment(objective_id, amount)` - Add progress

- Increase current by amount
- Cap at required
- Check for completion

`set_progress(objective_id, value)` - Set directly

- For non-incremental objectives
- Value validation

`is_complete(objective_id)` - Check done

- current >= required

`reveal(objective_id)` - Show hidden

- Mark as discovered
- Display in UI

`get_progress(objective_id)` - Query state

- Return current/required

**Invariants**

- Progress never exceeds required
- Progress never negative
- Hidden objectives not shown in UI
- Optional objectives don't block completion

**Design Notes**

- Specific objectives per quest left to design
- Auto-tracking based on game events
- Custom objective types via scripting

---

## Bevy Integration

**Data**

- QuestObjective { id, objective_type, target, required, description, ... }
- ObjectiveProgress { current, revealed }

**Events (Listeners)**

- EntityDied → check Kill objectives
- ItemAdded → check Collect objectives
- DialogueCompleted → check Talk objectives
- AreaEntered → check Reach objectives
- InteractionTriggered → check Interact objectives

**Systems**

- Listen for game events
- Match against active objectives
- Update progress
- Emit ObjectiveUpdated

**Messages/Commands**

- IncrementObjective { quest_id, objective_id, amount }
- SetObjectiveProgress { quest_id, objective_id, value }
- RevealObjective { quest_id, objective_id }

**Events**

- ObjectiveProgressed { quest_id, objective_id, current, required }
- ObjectiveCompleted { quest_id, objective_id }
- ObjectiveRevealed { quest_id, objective_id }

**Scripting Compatibility**

- Progress updates as commands
- Objective state readable
- Events hookable

*See: architecture/scripting.md*
