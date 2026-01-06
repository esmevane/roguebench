# Quest Framework

Scriptable quest structure enabling designer-defined quest behaviors and flows.

## Core Logic

**Concept**

- Quests are state machines
- States and transitions defined by design
- Objectives drive progression
- Integrates with scripting system

**Quest Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| name | string | Display name |
| description | string | Quest summary |
| objectives | list | Completion requirements |
| rewards | list | Granted on completion |
| prerequisites | list | Required to start |
| repeatable | bool | Can redo after completion |
| persists | bool | Survives death |

**Quest States**

| State | Description |
|-------|-------------|
| Unavailable | Prerequisites not met |
| Available | Can be started |
| Active | In progress |
| Completed | All objectives done |
| Failed | Failed condition met |
| Turned In | Rewards claimed |

**Operations**

`start_quest(quest_id)` - Begin quest

- Check prerequisites
- Set state to Active
- Initialize objectives

`update_objective(quest_id, objective_id, progress)` - Track progress

- Update objective state
- Check for completion
- May trigger state change

`complete_quest(quest_id)` - Mark complete

- Verify all objectives done
- Set state to Completed

`claim_rewards(quest_id)` - Get rewards

- Grant defined rewards
- Set state to Turned In

`fail_quest(quest_id)` - Mark failed

- Set state to Failed
- Handle cleanup

`get_active_quests()` - List in-progress

- Return quests in Active state

**Invariants**

- State transitions validated
- Objectives tracked per quest
- Rewards granted once
- Persistence respected

**Design Notes**

- Specific quests left to design
- Objective types left to design
- Quest flow connects to scripting

---

## Bevy Integration

**Data**

- QuestDefinition { id, name, objectives, rewards, ... }
- QuestState { quest_id, state, objective_progress }

**Resources**

- ActiveQuests { quests: HashMap<QuestId, QuestState> }
- CompletedQuests { quests: HashSet<QuestId> }

**Messages/Commands**

- StartQuest { quest_id }
- UpdateObjective { quest_id, objective_id, progress }
- CompleteQuest { quest_id }
- ClaimRewards { quest_id }
- FailQuest { quest_id }

**Events**

- QuestStarted { quest_id }
- ObjectiveUpdated { quest_id, objective_id }
- QuestCompleted { quest_id }
- QuestFailed { quest_id }
- RewardsClaimed { quest_id, rewards }

**Scripting Compatibility**

- All operations exposed as commands
- Quest state readable
- Events hookable for custom logic
- Quest behaviors scriptable

---

## Framework Dependencies

- `framework/command_bus.md` - Quest operations as commands
- `framework/state_machine.md` - Quest state transitions
- `framework/event_hooks.md` - Quest event hooks for scripting
- `framework/data_loading.md` - Quest definitions from data

*See: architecture/scripting.md, architecture/data.md*
