# Dialogue Choices

Branching dialogue with player choices, conditions, and consequences.

## Core Logic

**Concept**

- Choices presented at branch points
- Choices may have requirements
- Selections affect game state
- Enable non-linear storytelling

**Choice Properties**

| Property | Type | Description |
|----------|------|-------------|
| text | string | Choice display text |
| target | node_id | Where choice leads |
| condition | option | Requirement to show |
| effect | option | Action when chosen |
| once | bool | Remove after choosing |

**Conditions**

| Type | Example | Description |
|------|---------|-------------|
| HasItem | HasItem("key") | Player has item |
| HasFlag | HasFlag("met_king") | Story flag set |
| StatCheck | Stat("gold", >=, 100) | Stat threshold |
| QuestState | Quest("rescue", Active) | Quest status |

**Effects**

| Type | Example | Description |
|------|---------|-------------|
| SetFlag | SetFlag("allied_rebels") | Set story flag |
| GiveItem | GiveItem("sword") | Add to inventory |
| TakeItem | TakeItem("gold", 50) | Remove from inventory |
| StartQuest | StartQuest("rescue") | Begin quest |
| Reputation | Reputation("guild", +10) | Modify standing |

**Operations**

`get_available_choices(node, player)` - Filter choices

- Evaluate all conditions
- Return only valid choices
- Order by priority

`select_choice(choice)` - Process selection

- Apply effects
- Record in history
- Navigate to target

`is_choice_visible(choice, player)` - Check display

- Some hidden until condition met
- Others shown but disabled

**Choice Display States**

| State | Appearance | Behavior |
|-------|------------|----------|
| Available | Normal text | Selectable |
| Locked | Grayed + reason | Not selectable |
| Hidden | Not shown | Condition not met |

**Invariants**

- At least one choice always available
- Effects applied immediately
- Choices recorded for flags
- Locked choices show requirements

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Max choices | 4 | Per branch point |
| Default fallback | "..." | If all locked |

---

## Bevy Integration

**Data Structures**

```rust
struct DialogueChoice {
    text: String,
    target: NodeId,
    condition: Option<Condition>,
    effect: Option<Effect>,
    once: bool,
}

enum Condition {
    HasItem(String),
    HasFlag(String),
    StatCheck { stat: String, op: Comparison, value: i32 },
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
}

enum Effect {
    SetFlag(String),
    GiveItem(String),
    TakeItem(String, u32),
    ModifyStat(String, i32),
    Multiple(Vec<Effect>),
}
```

**Resources**

- StoryFlags { flags: HashSet<String> }
- ChoiceHistory { choices: HashMap<ConversationId, Vec<ChoiceId>> }

**Systems**

- Evaluate conditions against player state
- Filter and display valid choices
- Process selection effects
- Update story flags

**Condition Evaluation**

```rust
fn evaluate_condition(
    condition: &Condition,
    inventory: &Inventory,
    flags: &StoryFlags,
    stats: &PlayerStats,
) -> bool {
    match condition {
        Condition::HasItem(id) => inventory.contains(id),
        Condition::HasFlag(flag) => flags.contains(flag),
        Condition::And(conds) => conds.iter().all(|c| evaluate(c, ...)),
        // ...
    }
}
```

**UI Selection**

- Up/Down or W/S to navigate
- Enter/Space to select
- Visual highlight on current
- Locked choices show lock icon + requirement

**Choice Persistence**

- Save choice history to save file
- Affects future dialogue availability
- Enables story consequences
