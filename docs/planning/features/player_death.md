# Player Death

Handling of player reaching zero health. Triggers game over state and associated feedback.

## Core Logic

**Trigger**

- Player health reaches 0

**Operations**

`on_death()` - Handle player death

- Stop player input
- Play death effects
- Transition to game over state
- Record run statistics (optional)

**Death Sequence**

1. Health reaches 0
2. EntityDied event for player
3. Death visual effects (screen flash, particles)
4. Brief pause for impact
5. Transition to game over

**Invariants**

- Player can only die once per run
- Death is not reversible
- All gameplay stops on death
- Death effects play before transition

**Effects**

- Screen flash (red)
- Camera shake (large)
- Death particles
- Slow motion (optional)
- Death sound

---

## Bevy Integration

**Detection**

- EntityDied event with EntityKind::Player
- Or Health component reaching 0

**State Change**

- Set gameplay sub-state to GameOver
- Or transition to GameOver screen

**Entity Handling**

- Player entity can despawn or persist
- Enemies may freeze or continue briefly

**Events**

- PlayerDied custom event (optional)
- Triggers game over flow
