# Interaction System

Detection and triggering of player interactions with objects, NPCs, and world elements.

## Core Logic

**Concept**

- Player approaches interactable
- Prompt appears when in range
- Input triggers interaction
- Execute interaction behavior

**Interactable Types**

| Type | Behavior | Examples |
|------|----------|----------|
| Pickup | Collect item | Health, coins, weapons |
| Container | Open/loot | Chests, crates |
| NPC | Start dialogue | Shopkeeper, quest giver |
| Door | Transition | Room exit, secret passage |
| Sign | Read text | Lore, hints |
| Mechanism | Activate | Lever, button, altar |

**Interaction Properties**

| Property | Type | Description |
|----------|------|-------------|
| range | f32 | Detection radius |
| prompt | string | UI hint text |
| requires_input | bool | Auto vs manual trigger |
| one_shot | bool | Single use only |
| cooldown | f32 | Time between uses |

**Operations**

`detect_interactables(player_pos)` - Find nearby

- Query entities in range
- Return closest/priority
- Filter by requirements

`show_prompt(interactable)` - Display UI hint

- Show interaction prompt
- Display required input
- Highlight interactable

`trigger_interaction(player, target)` - Execute

- Run interaction behavior
- Consume if one-shot
- Start cooldown

`can_interact(player, target)` - Validate

- Check range
- Check requirements (key, item)
- Check cooldown

**Priority Rules**

- Closest interactable wins
- NPCs > Containers > Pickups
- Required items prioritized
- Only one active at a time

**Invariants**

- Only one prompt shown at a time
- Must be in range to interact
- One-shot items removed after use
- Cooldown prevents spam

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Default range | 32px | Interaction radius |
| Prompt key | E | Interact input |

---

## Bevy Integration

**Components**

```rust
#[derive(Component)]
struct Interactable {
    interaction_type: InteractionType,
    range: f32,
    prompt: String,
    requires_input: bool,
    one_shot: bool,
}

#[derive(Component)]
struct InteractionCooldown(Timer);
```

**Resources**

- CurrentInteractable(Option<Entity>) - highlighted target
- InteractionConfig { default_range, prompt_key }

**Events/Messages**

- InteractionTriggered { player, target, interaction_type }
- Handlers respond based on type

**Systems**

1. Detect nearby interactables
2. Update current target (closest valid)
3. Show/hide prompt UI
4. Listen for input
5. Trigger interaction on input
6. Execute type-specific behavior

**Prompt UI**

```rust
fn update_prompt(
    current: Res<CurrentInteractable>,
    mut prompt: Query<&mut Visibility, With<InteractionPrompt>>,
) {
    let visible = current.0.is_some();
    // Update prompt visibility and text
}
```

**Auto-Interact (Pickups)**

```rust
if !interactable.requires_input && in_range {
    trigger_interaction(player, target);
}
```
