# Dialogue System

Core dialogue display and flow for conversations with NPCs and story moments.

## Core Logic

**Concept**

- Dialogue is sequence of nodes
- Each node has text and next action
- Player advances through nodes
- May branch based on choices

**Dialogue Node**

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier |
| speaker | string | Character name |
| portrait | option | Speaker image |
| text | string | Dialogue content |
| next | enum | Next action |

**Next Actions**

| Action | Behavior |
|--------|----------|
| Continue(id) | Go to specific node |
| Choice(options) | Show player choices |
| End | Close dialogue |
| Event(id) | Trigger story event |

**Operations**

`start_dialogue(conversation_id)` - Begin conversation

- Load dialogue data
- Show dialogue UI
- Display first node
- Pause gameplay (optional)

`advance()` - Progress dialogue

- Move to next node
- Or wait for choice
- Handle text animation

`select_choice(index)` - Player chooses

- Record choice (optional)
- Go to choice's target node

`end_dialogue()` - Close conversation

- Hide dialogue UI
- Resume gameplay
- Trigger post-dialogue events

**Text Display**

| Mode | Description |
|------|-------------|
| Instant | All text appears immediately |
| Typewriter | Characters appear over time |
| Word-by-word | Words appear sequentially |

**Invariants**

- One dialogue active at a time
- Dialogue pauses gameplay
- Player must complete or exit
- Choices recorded for flags

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Text speed | 30 cps | Characters per second |
| Advance input | Space/Enter | Next node |
| Skip input | Hold Space | Fast forward |

---

## Bevy Integration

**Resources**

- ActiveDialogue { conversation: Handle, current_node: NodeId }
- DialogueHistory { completed: HashSet, choices: HashMap }

**States**

```rust
enum GameplayState {
    Playing,
    InDialogue,
    Paused,
}
```

**Components**

- DialogueTrigger { conversation_id } - on NPCs
- DialoguePortrait - UI portrait display

**Data Format**

```ron
Conversation(
    id: "shopkeeper_greeting",
    nodes: [
        Node(
            id: "start",
            speaker: "Shopkeeper",
            text: "Welcome, traveler! Looking to buy?",
            next: Choice([
                ("Yes, show me your wares", "shop"),
                ("Just browsing", "browse"),
                ("Goodbye", "end"),
            ]),
        ),
        Node(
            id: "shop",
            speaker: "Shopkeeper",
            text: "Take a look!",
            next: Event("open_shop"),
        ),
        // ...
    ],
)
```

**Systems**

- Handle dialogue triggers
- Animate text display
- Process input (advance/choice)
- Update UI elements
- Trigger events on end

**UI Layout**

```
┌─────────────────────────────────┐
│ [Portrait]  Speaker Name        │
│                                 │
│  Dialogue text appears here,   │
│  with typewriter effect...     │
│                                 │
│  > Choice 1                    │
│    Choice 2                    │
│    Choice 3                    │
└─────────────────────────────────┘
```

**Typewriter Effect**

```rust
struct TextAnimation {
    full_text: String,
    visible_chars: usize,
    timer: Timer,
}
```
