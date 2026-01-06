# Animation State Machine

General framework for controlling which animation plays based on entity state. Handles transitions between animations.

## Core Logic

**Relationship to Character States**

- This is the animation *framework* (how animations work)
- `character_states.md` is the gameplay state machine (what the character is doing)
- Character states *drive* animation selection
- Animation state machine handles playback mechanics

**Animation Types** (examples, design-defined)

- Idle - standing still
- Walk - moving
- Run - moving fast
- Attack - attacking
- Dash - dashing
- Hurt - taking damage
- Death - dying

**State**

- Current animation type
- Frame index
- Frame timer
- Looping flag

**Operations**

`set_animation(type)` - Change animation

- Set new animation type
- Reset frame to start if different
- Apply transition rules

`advance_frame(dt)` - Progress animation

- Tick frame timer
- If timer complete, next frame
- Loop or stop based on animation type

`on_animation_complete()` - Handle end

- Called when non-looping animation finishes
- Notify listeners (for state transitions)

`get_frame_index()` - Query current frame

- Return atlas index for current frame

**Invariants**

- One animation active at a time
- Animations play at defined frame rate
- Looping animations repeat, one-shot stop
- Frame index always valid for animation

**Defaults**

| Field          | Value | Description |
| -------------- | ----- | ----------- |
| Frame duration | 0.15  | Seconds per frame |

**Design Notes**

- Specific animations defined per entity type
- Frame counts defined in animation data
- Priority/interruption rules left to design

---

## Bevy Integration

**Components**

- AnimationController { current_animation, frame_index, timer }
- AnimationDefinition { frames, duration, looping }

**Data**

- AnimationSet { animations: HashMap<AnimationType, AnimationDefinition> }
- Loaded from data files, design-defined

**Systems**

- Advance frame timer
- Update sprite atlas index
- Handle animation completion events

**Integration with Character States**

```rust
fn update_animation_from_state(
    mut query: Query<(&CharacterState, &mut AnimationController), Changed<CharacterState>>,
) {
    for (state, mut animation) in &mut query {
        let anim_type = match state {
            CharacterState::Idle => AnimationType::Idle,
            CharacterState::Walking => AnimationType::Walk,
            CharacterState::Attacking { .. } => AnimationType::Attack,
            // ...
        };
        animation.set_animation(anim_type);
    }
}
```

**Scripting Compatibility**

- Animations definable in data
- Animation events hookable

*See: character_states.md, data_driven_characters.md*
