# Story Events

Triggered narrative moments, cutscenes, and scripted sequences.

## Core Logic

**Concept**

- Events triggered by game actions
- Control camera, entities, dialogue
- Create narrative moments
- Non-interactive sequences

**Event Types**

| Type | Description | Example |
|------|-------------|---------|
| Cutscene | Scripted sequence | Boss intro |
| Revelation | Show hidden info | Secret discovered |
| Transition | Scene change | Area travel |
| Flashback | Memory sequence | Backstory |
| Trigger | One-time effect | First enemy encounter |

**Event Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| trigger | enum | What starts event |
| actions | list | Sequence of actions |
| skippable | bool | Player can skip |
| once | bool | Only plays once |

**Triggers**

| Trigger | Description |
|---------|-------------|
| DialogueEnd(id) | After conversation |
| AreaEnter(area) | Enter location |
| ItemPickup(id) | Collect specific item |
| EnemyKill(type) | Defeat enemy type |
| QuestComplete(id) | Finish quest |
| Manual | Called from code |

**Actions**

| Action | Description |
|--------|-------------|
| Wait(seconds) | Pause sequence |
| CameraMove(target) | Pan camera |
| CameraZoom(level) | Zoom in/out |
| SpawnEntity(def) | Create entity |
| DespawnEntity(id) | Remove entity |
| PlayDialogue(id) | Start conversation |
| PlaySound(id) | Audio cue |
| SetFlag(flag) | Story flag |
| FadeIn/FadeOut | Screen transition |
| ShowText(text) | Floating text |

**Operations**

`trigger_event(id)` - Start event

- Check if already played (once)
- Begin action sequence
- Lock player input

`advance_event()` - Progress sequence

- Execute next action
- Wait for completion
- Continue to next

`skip_event()` - Player skip

- Jump to end
- Apply final state
- Only if skippable

`is_event_complete(id)` - Check status

- Return if event has played
- For conditional content

**Invariants**

- Once events only play once
- Player input locked during cutscene
- Skip applies end state correctly
- Events can chain

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Skip input | Escape | Skip cutscene |
| Default skippable | true | Most scenes |

---

## Bevy Integration

**Resources**

- ActiveEvent { event_id, current_action, timer }
- CompletedEvents { events: HashSet<String> }
- EventQueue { pending: VecDeque<EventId> }

**Data Format**

```ron
StoryEvent(
    id: "boss_intro",
    trigger: AreaEnter("boss_room"),
    once: true,
    skippable: true,
    actions: [
        FadeOut(0.5),
        CameraMove("boss_spawn"),
        Wait(0.5),
        SpawnEntity("boss_dragon"),
        FadeIn(0.5),
        PlayDialogue("boss_taunt"),
        CameraMove("player"),
        SetFlag("boss_encountered"),
    ],
)
```

**Components**

- EventTriggerZone { event_id } - area trigger
- CutsceneActor { event_id } - participating entity

**States**

```rust
enum GameplayState {
    Playing,
    InDialogue,
    InCutscene,
    Paused,
}
```

**Systems**

- Check trigger conditions
- Execute action sequence
- Handle skip input
- Transition states

**Action Executor**

```rust
fn execute_action(action: &EventAction, world: &mut World) {
    match action {
        EventAction::Wait(secs) => { /* set timer */ },
        EventAction::CameraMove(target) => { /* animate camera */ },
        EventAction::PlayDialogue(id) => { /* start dialogue */ },
        // ...
    }
}
```

**Chaining Events**

- Event end can trigger next event
- Quest completions trigger rewards
- Area transitions trigger intros
