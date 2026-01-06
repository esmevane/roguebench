# Hit Impact

Spark burst when an attack connects. Visual confirmation that a hit landed.

## Core Logic

**Parameters**

- Position (vec2) - hit location

**Behavior**

- Spawns at target location when hit lands
- Omnidirectional burst (no rotation)
- Small bright particles
- Short travel distance
- Quick fade

**Invariants**

- One-shot effect
- Position is target's location, not attack origin
- Does not depend on hit direction

---

## Bevy Integration

**Trigger**

- On hit event (attack connects with target)
- Position: target entity location

**Spawning**

- Create emitter entity at hit position
- No rotation needed

**Lifecycle**

- Emitter auto-despawns when complete
