# Death Explosion

Large particle burst when enemy dies. Dramatic feedback for kills.

## Core Logic

**Parameters**

- Position (vec2) - death location

**Behavior**

- Spawns at entity death location
- Large burst radius
- Longer lifetime than other particle effects
- Multiple particle types possible (debris, sparks, etc.)
- Omnidirectional

**Invariants**

- One-shot effect
- Larger and longer than hit/damage particles
- Only for enemy deaths (not player)

---

## Bevy Integration

**Trigger**

- On entity death event
- Filter: enemy entities only
- Position: death location

**Spawning**

- Create emitter at death position

**Lifecycle**

- Emitter auto-despawns when complete
